# EPIC - Ethereum RPC CLI Client

EPIC is a Ethereum RPC CLI client written in Rust.

## Installation

### From Source

```bash
$ git clone https://github.com/natsuneko-laboratory/epic.git
$ cd /path/to/epic/repository
$ cargo build --release
```

## Usage

```bash
$ epic block-number --endpoint=https://mainnet.infura.io/v3/YOUR-PROJECT-ID
14189140

$ epic block-number --endpoint=https://mainnet.infura.io/v3/YOUR-PROJECT-ID --json
{
    "result": "0xD88254"
}
```
