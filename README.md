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

## Project Structure

- `/src`: Contains the Rust smart contract code
- `/frontend`: Contains the React frontend application
- `/tests`: Contains smart contract tests

## Smart Contract Functionality

The smart contract, implemented in Rust, provides the following functions:

1. `init`: Initialize the contract with an admin address
   - Parameters: `admin: Address`
   - Description: Sets up the contract with the specified admin address

2. `create_project`: Create a new donation project (admin only)
   - Parameters: `name: Symbol`, `goal: i128`
   - Description: Creates a new project with the given name and fundraising goal

3. `donate`: Make a donation to a specific project
   - Parameters: `project: Symbol`, `amount: i128`
   - Description: Allows a user to donate the specified amount to the given project

4. `get_project_status`: Retrieve the current amount and goal for a project
   - Parameters: `project: Symbol`
   - Returns: `(i128, i128)` (current amount, goal)
   - Description: Returns the current donation amount and the goal for the specified project

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
   ```
   cargo test
   ```

### Frontend Testing

To run the frontend tests:
```
cd frontend
npm test
```


## Security Considerations

- The smart contract uses Stellar's built-in security features
- Admin functions are protected with access control
- Users should be cautious when interacting with the dApp and verify all transaction details

## Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository
2. Create a new branch: `git checkout -b feature/your-feature-name`
3. Make your changes and commit them: `git commit -m 'Add some feature'`
4. Push to the branch: `git push origin feature/your-feature-name`
5. Submit a pull request

Please make sure to update tests as appropriate and adhere to the existing coding style.

## Troubleshooting

If you encounter any issues while setting up or running the project, please check the following:

- Ensure you have the correct versions of Rust and Node.js installed
- Verify that your Stellar account has sufficient testnet tokens
- Check that the contract ID in your `.env` file is correct
- Make sure you're connected to the Stellar testnet in your wallet

If problems persist, please open an issue on the GitHub repository.

## License

This project is licensed under the MIT License.
