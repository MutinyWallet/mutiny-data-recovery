use crate::config::Config;
use clap::Parser;
use mutiny_core::auth::MutinyAuthClient;
use mutiny_core::bitcoin::bip32::ExtendedPrivKey;
use mutiny_core::bitcoin::Network;
use mutiny_core::lnurlauth::AuthManager;
use mutiny_core::logging::MutinyLogger;
use mutiny_core::vss::MutinyVssClient;
use std::io::Write;
use std::str::FromStr;
use std::sync::Arc;

mod config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let logger = Arc::new(MutinyLogger::default());
    let config: Config = Config::parse();

    // Create a mutable string to store the user input
    let mut input = String::new();

    // Prompt the user for input
    print!("Enter your mnemonic seed: ");
    // Flush stdout to ensure the prompt is displayed before input is read
    std::io::stdout().flush()?;

    // Read the input from stdin and handle potential errors
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let seed = bip39::Mnemonic::from_str(input.trim())?;

    let xprivkey = ExtendedPrivKey::new_master(Network::Bitcoin, &seed.to_seed("")).unwrap();

    let auth_manager = AuthManager::new(xprivkey).unwrap();

    let lnurl_client = Arc::new(
        lnurl::Builder::default()
            .build_async()
            .expect("failed to make lnurl client"),
    );

    let auth_client = Arc::new(MutinyAuthClient::new(
        auth_manager,
        lnurl_client,
        logger.clone(),
        config.auth_url,
    ));

    let vss = MutinyVssClient::new_authenticated(
        auth_client,
        config.vss_url,
        xprivkey.private_key,
        logger,
    );

    let key_versions = vss.list_key_versions(Some("monitor".to_string())).await?;

    let mut new_objects = Vec::new();
    for kv in key_versions {
        // skip closed channels
        if kv.version >= i32::MAX as u32 {
            continue;
        }
        // take the object, bump the version numbers, and put it back
        let mut obj = vss.get_object(&kv.key).await?;
        // bump version by 100, this should make us think our
        // channel counterparty is out of date and force close the channel
        obj.version = obj.version.saturating_add(100);

        // modify the object
        let mut bytes: Vec<u8> = serde_json::from_value(obj.value.clone())?;

        // change bytes 2-10 to new version
        let new_version: u64 = obj.version as u64;
        bytes[2..10].copy_from_slice(&new_version.to_be_bytes());

        obj.value = serde_json::to_value(bytes)?;
        new_objects.push(obj);
    }

    // put the new objects back
    vss.put_objects(new_objects).await?;

    println!("\nDone! Open Mutiny Wallet and your channels should force close and be recoverable on-chain in 3-14 days.");

    Ok(())
}
