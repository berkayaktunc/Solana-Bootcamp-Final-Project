# Solana Review Project - README

Welcome to the **Web3 Review** project repository! This decentralized solana-based project from RiseIn's Solana Bootcamp leverages blockchain technology to implement a review platform on the Solana network. Participants can add, update and view reviews.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [Usage](#usage)
- [Smart Contracts](#smart-contracts)
- [Frontend](#frontend)
- [Contributing](#contributing)

## Overview

The **Web3 Review** provides a user-friendly interface to participate in Solana-based reviews. This project ensures transparency and trust in the local business or product review process through the use of smart contracts. Think of it as the Decentralised Yelp. Users can add and update reviews, as well as see other reviews.

## Features

- Add reviews from 1 to 5.
- Update reviews.
- Browse reviews by other reviewers.
- Solana Wallet Integration: Connect your Solana wallet to participate directly (For now, you have to be on the Solana Devnet).

## Getting Started

Follow these steps to set up the project locally and start participating in the decntralised solana-based reviews.

### Prerequisites

1. Crypto Wallet: Ensure you have a [Phantom](https://phantom.app/) or [Solflare](https://solflare.com/) wallets installed on your browser.

2. Navigate to testnet (Devnet) network in your choosen wallet.

3. Get your to SOL tokens at [https://faucet.solana.com](https://faucet.solana.com/)

### Installation

1. Clone the repository:

```bash
  git clone https://github.com/berkayaktunc/Solana-Bootcamp-Final-Project.git
```

2. Navigate to the project directory in your terminal.
```bash
cd Solana-Bootcamp-Final-Project/final-project/
```

4. Install required npm packages:

```bash
 npm install
```

If you encounter any npm errors during install, run

```bash
npm update
```

## Usage

1. Start the development server:

    ```bash
    npm run dev
    ```

2. Open your web browser and navigate to `http://localhost:3000` to access the page.

3. Connect your Solana wallet via button on top right corner.

4. Browse ongoing auctions, place bids, and monitor your auction activity.

## Smart Contracts

The Rust smart contracts in this project facilitate the review process. They handle adding and updating reviews. These contracts are deployed on the Solana Devnet.

- `lib.rs`: Handles the broad logic of creating transactions, adding and updating reviews.
- `instruction.rs`: Defines the program's instruction data for executing the program.
- `state.rs`: Oversees the basic state of the program and handles some common errors.

## Frontend

The review page frontend is built using modern web technologies including React.js. 

- **React.js**: Powers the DApp's user interface.
- **solana/web3.js , solana/wallet-adapter-react**: The Solana JavaScript API for smart contract interaction.
- **Phantom / Solflare**: Solana wallets for secure transactions.