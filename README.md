# Cypher windexer
**High-Performance ZK-Compressed Solana Indexer with Advanced Storage and Querying Capabilities**

Cypher Windexer is a not just another indexing solution for Solana, but it uses zk compression, Filecoin as a storage solution, and flexible querying interfaces. Built for performance and scalability, it's designed to handle Solana's growing state efficiently.

## Key Features

- ZK compression for Solana account data
- gRPC by default, with support for RPC polling and WebSocket subscriptions
- Advanced binary parsing for accounts and instructions without IDLs
- Multi-tiered storage: Filecoin for long-term storage, Redis for caching, ScyllaDB for high-throughput data handling
- ClickHouse compatibility for advanced analytics
- WebAssembly (WASM) support using Wasmer for custom indexing logic
- Real-time processing pipeline with support for custom parsers
- GraphQL and REST API for flexible data querying
- Rust-based implementation for high performance

## Technical Stack

- **Language**: Rust
- **RPC**: gRPC, with fallback to HTTP/JSON-RPC
- **Storage**: 
  - Filecoin: Long-term, decentralized storage
  - Redis: Caching and fast data retrieval
  - ScyllaDB: High-throughput data processing
  - ClickHouse: Analytics-compatible data warehouse
- **Custom Logic**: WebAssembly (WASM) with Wasmer runtime
- **APIs**: GraphQL and REST
- **Parsing**: Custom binary parsers for Solana accounts and instructions

## Quick Start (for Hackathon Demo)

1. Clone the repository:
   ```
   git clone https://github.com/cypherwg/windexer.git
   cd cypher-windexer
   ```

2. Install dependencies:
   ```
   cargo build
   ```

3. Run the indexer:
   ```
   cargo run --release -- --config config/demo.toml
   ```

4. Query data (example using GraphQL):
   ```
   curl -X POST -H "Content-Type: application/json" \
        -d '{"query": "{ account(pubkey: \"Your_Account_Pubkey\") { balance, owner } }"}' \
        http://localhost:8080/graphql
   ```

## Contributing

We welcome contributions! For the research and development period, please focus on bug reports and small enhancements. See `CONTRIBUTING.md` for guidelines.

## License

Cypher Windexer is released under the GNU-GPLv3 License. See the `LICENSE` file for details.
