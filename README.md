# mutiny-data-recovery

This is a tool for recovering Mutiny Wallets that end up in an unrecoverable state. If you enter your seed into this
tool it'll cause your channels to force close next time you open the wallet. This will allow you to recover your funds
from the channels.

## Usage

First install the binary from the [releases page](https://github.com/mutinywallet/mutiny-data-recovery/releases).

then run with:

```bash
mutiny-data-recovery
```

It will prompt you to enter your seed and this will get your wallet into a recoverable state.
