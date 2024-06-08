use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(version, author, about)]
/// A tool for recovering a mutiny wallet from a corrupted state.
pub struct Config {
    /// URL for mutiny authentication
    #[clap(default_value = "https://auth.mutinywallet.com", short, long)]
    pub auth_url: String,
    /// URL for VSS storage
    #[clap(default_value = "https://storage.mutinywallet.com/v2", short, long)]
    pub vss_url: String,
}
