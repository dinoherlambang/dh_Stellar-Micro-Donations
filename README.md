# Micro-Donations Platform

A decentralized application (dApp) leveraging Stellar smart contracts for micro-donations.

## Overview

This project implements a micro-donations platform using Stellar blockchain technology. Users can interact with the application to make donations, which are stored in a smart contract.

## Features

- Create donation projects
- Accept donations from users
- Store donations in a Stellar smart contract
- View donation history and project status

## Technical Stack

- Smart Contract: Rust with Soroban SDK
- Frontend: React with TypeScript
- Blockchain: Stellar

## Smart Contract Functionality

The smart contract, implemented in Rust, provides the following functions:

1. `init`: Initialize the contract with an admin address
2. `create_project`: Create a new donation project (admin only)
3. `donate`: Make a donation to a specific project
4. `get_project_status`: Retrieve the current amount and goal for a project

## Frontend

The frontend is built with React and TypeScript, providing a user interface for interacting with the smart contract. It uses the Stellar SDK to communicate with the Stellar network.

## Getting Started

### Prerequisites

- Rust
- Node.js and npm
- Stellar account and testnet tokens

### Installation

1. Clone the repository
2. Install Rust dependencies:
   ```
   cargo build
   ```
3. Install frontend dependencies:
   ```
   cd frontend
   npm install
   ```

### Environment Configuration

1. Create a `.env` file in the `frontend` directory
2. Add the following variables:
   ```
   REACT_APP_CONTRACT_ID=your_contract_id
   REACT_APP_HORIZON_URL=https://horizon-testnet.stellar.org
   ```
   Replace `your_contract_id` with the actual contract ID after deployment.

### Running the Application

1. Deploy the smart contract to the Stellar testnet:
   ```
   stellar contract deploy --wasm target/wasm32-unknown-unknown/release/micro_donations.wasm --network testnet
   ```
   Note the contract ID and update your `.env` file.

2. Start the frontend development server:
   ```
   npm start
   ```

## Testing

### Smart Contract Testing

To run the smart contract tests:
cargo test


### Frontend Testing

To run the frontend tests:
cd frontend
npm test


## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License.
