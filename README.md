# Artchain - NFT Marketplace

A full-stack NFT marketplace built with Rust, featuring Solana blockchain integration and modern web technologies.

## 🚀 Project Overview

Artchain is a decentralized NFT marketplace that allows artists to mint, list, and sell digital artwork on the Solana blockchain. The project demonstrates modern blockchain development practices with a focus on performance, security, and user experience.

## 🏗️ Architecture

### Current Stack
- **Backend**: Rust with Axum web framework
- **Blockchain**: Solana integration with devnet support
- **Frontend**: Yew (Rust WebAssembly) - *In Development*
- **Smart Contracts**: Anchor framework - *In Development*
- **Database**: PostgreSQL planned for metadata storage

### Project Structure

```
Artchain/
├── backend/           # Axum web server & REST API
│   ├── src/
│   │   └── main.rs    # Main server implementation
│   └── Cargo.toml     # Backend dependencies
├── frontend/          # Yew WASM frontend (planned)
├── shared/            # Shared types & utilities
├── programs/          # Solana smart contracts (in development)
└── README.md          # This file
```

## ✨ Features Implemented

### Backend API (✅ Complete)
- **RESTful API** with multiple endpoints
- **Wallet Integration** - Real Solana wallet balance checking
- **NFT Management** - List, view, and manage NFT data
- **Health Monitoring** - API health checks
- **Type Safety** - Full Rust type system benefits

### API Endpoints
```
GET  /                           # Homepage
GET  /api/health                 # Health check
GET  /api/nfts                   # List all NFTs
GET  /api/nft/{id}              # Get specific NFT
GET  /api/wallet/{address}       # Get wallet balance
GET  /api/nfts/user/{address}   # Get user's NFTs
```

### Blockchain Integration (✅ Working)
- **Solana Devnet** connectivity
- **Real wallet balance** queries
- **SOL/Lamports** conversion handling
- **Error handling** for invalid addresses

## 🛠️ Tech Stack Deep Dive

### Backend Technologies
- **Axum 0.8.4** - Modern async web framework
- **Tokio 1.46.1** - Async runtime with full features
- **Serde 1.0.219** - Serialization/deserialization
- **Solana Client** - Blockchain interaction
- **solana-sdk** - Core Solana types and functions

### Data Structures
```rust
pub struct Nft {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub image_url: String,
    pub artist: String,
    pub price_sol: f64,
    pub owner: String,
    pub mint_address: Option<String>,
    pub is_listed: bool,
}
```

## 🚦 Getting Started

### Prerequisites
- Rust (latest stable)
- Solana CLI tools
- Node.js (for Anchor framework)
- Phantom wallet (for testing)

### Installation

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd Artchain
   ```

2. **Install Solana CLI**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSfL https://solana-install.solana.workers.dev | bash
   ```

3. **Configure for development**
   ```bash
   solana config set --url devnet
   ```

4. **Run the backend**
   ```bash
   cd backend
   cargo run --bin artchain-backend
   ```

### Testing the API

With the server running on `http://localhost:3000`, test these endpoints:

```bash
# Health check
curl http://localhost:3000/api/health

# List NFTs
curl http://localhost:3000/api/nfts

# Check wallet balance (replace with your address)
curl http://localhost:3000/api/wallet/JCK39hgE32NjNTbjCfUfh4uhwP9sm7KRL6GBSCpfWYS4
```

## 🎯 Learning Goals & Progress

### ✅ Completed
- [x] Rust fundamentals and ownership concepts
- [x] Async programming with Tokio
- [x] Web server development with Axum
- [x] JSON API design and implementation
- [x] Solana blockchain integration
- [x] Wallet connectivity and balance queries
- [x] Error handling patterns
- [x] Type-safe API development

### 🔄 In Progress
- [ ] Solana program development with Anchor
- [ ] Smart contract for NFT marketplace
- [ ] On-chain NFT minting and trading
- [ ] Frontend development with Yew

### 📋 Planned Features
- [ ] Database integration (PostgreSQL)
- [ ] Real NFT minting functionality
- [ ] Image upload and IPFS integration
- [ ] User authentication system
- [ ] Advanced trading features (auctions, offers)
- [ ] Analytics dashboard
- [ ] Mobile-responsive design

## 🔧 Development Commands

### Backend Development
```bash
# Check code without building
cargo check

# Run with auto-reload
cargo watch -x run

# Run tests
cargo test

# Build optimized version
cargo build --release
```

### Blockchain Development
```bash
# Build Solana programs
anchor build

# Deploy to devnet
anchor deploy --provider.cluster devnet

# Run tests
anchor test
```

## 🌟 Key Learning Concepts

### Rust Concepts Mastered
- **Ownership & Borrowing** - Memory safety without garbage collection
- **Error Handling** - Result<T, E> and Option<T> patterns
- **Async Programming** - Futures and async/await
- **Type System** - Strong static typing and generics
- **Pattern Matching** - Comprehensive match statements

### Blockchain Concepts
- **Solana Architecture** - Accounts, programs, and transactions
- **Token Standards** - SPL tokens and metadata
- **Wallet Integration** - Phantom wallet connectivity
- **Network Configuration** - Mainnet vs Devnet vs Localnet

### Web Development
- **RESTful API Design** - Resource-based URL structure
- **JSON Serialization** - Type-safe data conversion
- **HTTP Status Codes** - Proper error response handling
- **CORS and Security** - Web security best practices

## 🤝 Contributing

This is a learning project focused on blockchain development and Rust programming. Key areas for exploration:

1. **Smart Contract Development** - Anchor framework and Solana programs
2. **Frontend Integration** - Connecting Yew frontend to Rust backend
3. **Database Design** - Efficient NFT metadata storage
4. **Performance Optimization** - Query optimization and caching
5. **Security Auditing** - Smart contract and API security

## 📚 Resources & References

- [Solana Documentation](https://docs.solana.com/)
- [Anchor Framework](https://project-serum.github.io/anchor/)
- [Axum Documentation](https://docs.rs/axum/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)

## 📄 License

This project is for educational purposes and career development in blockchain engineering.

---

**Built with ❤️ and Rust** | Learning Solana blockchain development for opportunities like Phantom's Blockchain Engineer role