# RMCP CoinMarketCap

A Rust-based MCP service that provides real-time cryptocurrency market data query using the CoinMarketCap API.

## API Features

- /cryptocurrency
  - [x] /quotes/latest

## Requirements

- Rust toolchain (recommended to use the latest stable version)
- CoinMarketCap API key

## Usage

1. Install Crate

```bash
cargo install rmcp-coinmarketcap
```

2. MCP Client Config

```json
{
  "mcpServers": {
    "rmcp-coinmarketcap": {
        "command": "rmcp-coinmarketcap",
        "args": [],
        "env": {
            "COINMARKETCAP_API_KEY": "YOUR_COINMARKETCAP_API_KEY"
        }
    }
  }
}
```

## Devleopment

### Setup

Clone the repository:

```bash
git clone https://github.com/mulander/cmc-rmcp.git
cd cmc-rmcp
```

Build the project:

```bash
cargo build
```

Manual Run

```bash
export COINMARKETCAP_API_KEY="your-api-key"
cargo run
```

Run with Inspector

```bash
# run dev
npx @modelcontextprotocol/inspector -e COINMARKETCAP_API_KEY=xxxx cargo run
# run build
npx @modelcontextprotocol/inspector -e COINMARKETCAP_API_KEY=xxxx rmcp-coinmarketcap
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Issues and Pull Requests are welcome!
