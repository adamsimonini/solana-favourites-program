# solana-favourites-program

solana bootcamp project

## Project Structure

- `/programs/favourites/src/lib.rs` - Main program logic
- `/tests/favourites.ts` - Program tests
- `Anchor.toml` - Project configuration
- `package.json` - JavaScript dependencies

## Getting Started

### Prerequisites

- Rust and Cargo
- Solana CLI tools
- Node.js and npm
- Anchor Framework

### Installation

1. Clone the repository:

### Development Workflow

1. Start the local validator:

```bash
solana-test-validator
```

2. In a new terminal, build and deploy:

```bash
# Build the program
anchor build

# Deploy to local network
anchor deploy
```

3. Run the program:

```bash
# Configure Solana to use localhost
solana config set --url localhost

# Airdrop SOL to your local wallet for testing
solana airdrop 2

# Run tests
anchor test

# For live reload during development
anchor test --skip-local-validator
```

## Notes

- Make sure the test validator is running before deploying or running tests
- The program ID in `lib.rs` and `Anchor.toml` should match your deployed program
- Keep your keypair file secure and never commit it to version control
