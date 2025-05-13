# Tickr - Blockchain Ticketing System

Tickr is a decentralized ticketing platform built on Solana that facilitates event management, ticket issuance, and secondary market transactions with full transparency and reduced fraud.

## Overview

Tickr leverages blockchain technology to create a secure, transparent, and efficient system for event organizers and attendees. The platform allows organizers to create events, issue tickets as NFTs, and provides attendees with verifiable ownership of tickets that can be easily transferred or resold based on the organizer's preferences.

## Features

- **Event Creation**: Create on-chain events with customizable attributes (venue, date, capacity)
- **NFT Ticketing**: Issue tickets as NFTs with configurable transferability
- **Secondary Market**: Enable secure peer-to-peer ticket reselling with royalties
- **Seat Assignment**: Support for assigned seating (screen, row, seat)
- **Fee Management**: Configurable marketplace fees and secure treasury system
- **Organizer Controls**: Managed access and permissions for event creators

## Technology Stack

- **Blockchain**: [Solana](https://solana.com)
- **Development Framework**: [Anchor](https://www.anchor-lang.com/)
- **Token Standard**: [Metaplex Core](https://developers.metaplex.com/)
- **Testing**: Mocha & Chai

## Project Structure

```
anchor/
  Anchor.toml
  Cargo.lock
  Cargo.toml
  package.json
  tsconfig.json
  app/
  migrations/
    deploy.ts
  programs/
    tickr/
      Cargo.toml
      Xargo.toml
      src/
        constants.rs
        error.rs
        lib.rs
        instructions/
        state/
  target/
    debug/
    deploy/
    idl/
    release/
    sbf-solana-solana/
    types/
  tests/
    tickr.ts
```

## Architecture

Tickr is built around key components:

- **Marketplace**: Central entity managing platform configuration and fees
- **Manager**: Controls access for event organizers
- **Events**: Represented as NFT collections
- **Tickets**: Individual NFTs within event collections
- **Treasury**: Secure vault for platform fees and revenue

## Getting Started

### Prerequisites

- Rust and Cargo
- Solana CLI
- Node.js and npm/yarn
- Anchor CLI

### Installation

1. Clone the repository

   ```bash
   git clone https://github.com/yourusername/Tickr.git
   cd Tickr
   ```

2. Install dependencies

   ```bash
   cd anchor
   yarn install
   ```

3. Build the program
   ```bash
   anchor build
   ```

### Testing

Run the test suite to validate functionality:

```bash
anchor test
```
