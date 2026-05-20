# EzPay

EzPay is a Stellar-based payment infrastructure that enables customers to pay merchants using stablecoins and fiat-compatible settlement flows even if the merchant is not registered on the platform.

The goal of EzPay is to make digital payments as simple as traditional bank transfers by combining fast Stellar settlement, QR-based payments, stablecoins, and fiat payout infrastructure into one seamless experience.

EzPay is designed as a lightweight MVP focused on real-world usability, low-cost payments, cross-border accessibility, and merchant simplicity.

---

# Contents

- [Problem](#problem)
- [Solution](#solution)
- [Key Features](#key-features)
- [Payment Flows](#payment-flows)
- [Architecture Overview](#architecture-overview)
- [Tech Stack](#tech-stack)
- [Why Stellar](#why-stellar)
- [Security & Compliance](#security--compliance)
- [Project Structure](#project-structure)
- [Getting Started](#getting-started)
- [Project Status](#project-status)
- [Future Improvements](#future-improvements)
- [Use Cases](#use-cases)
- [Contributing](#contributing)
- [License](#license)

---

# Problem

Many merchants and everyday users still struggle with digital payments because:

- Existing crypto payment systems are difficult for non-technical users
- Wallet setup and seed phrases create onboarding friction
- Merchants often prefer receiving fiat directly into their bank accounts
- Cross-border payment fees are expensive and slow
- Many payment platforms require merchants to register before they can receive funds
- Managing both fiat and crypto payments is difficult for small businesses

This creates friction for customers who want to pay using stablecoins while merchants still prefer simple fiat settlement.

---

# Solution

EzPay simplifies digital payments by enabling customers to pay merchants using stablecoins while allowing merchants to receive either crypto or fiat payouts.

The platform supports both wallet-based payments and fiat settlement flows powered by Stellar-compatible infrastructure.

## Wallet Payments

Customers can send stablecoins directly to a merchant wallet using:

- QR codes
- Payment links
- Wallet addresses

## Fiat Settlement

Customers can also pay merchants who do not use crypto by entering the merchant’s bank details.

Payments are processed through Stellar-compatible Anchor infrastructure and payout partners, allowing merchants to receive fiat directly into their bank accounts.

This removes blockchain complexity for merchants while still enabling fast digital payments.

---

# Key Features

## Payment Features

- QR code payments
- Payment request links
- Stablecoin payments
- Fast Stellar settlement
- Fiat-compatible payout flows
- Payments to registered and unregistered merchants
- Cross-border payments
- Low transaction costs

## Merchant Features

- Merchant onboarding
- Wallet or bank payout preferences
- Merchant payment links
- Transaction tracking
- Merchant dashboard
- Payment history

## Customer Features

- Wallet connect
- Simple payment experience
- Instant payment confirmation
- Stablecoin payments
- Cross-border transfers

## Stellar-Native Features

- Fee-sponsored transactions
- Stellar fee-bump transaction support
- Stellar Anchor integrations
- SEP-6 deposit/withdraw support
- SEP-24 hosted on-ramp/off-ramp support
- USDC and Stellar-issued stable asset support

---

# Payment Flows

## Registered Merchant Flow

1. Merchant registers on EzPay
2. Merchant generates a payment link or QR code
3. Customer pays using stablecoins
4. Payment settles on Stellar
5. Merchant receives funds in wallet or bank account

---

## Unregistered Merchant Flow

1. Customer selects **Pay Merchant**
2. Customer enters either:
   - Merchant wallet address
   - Merchant bank details
3. Customer sends stablecoins
4. Payment settles through Stellar infrastructure
5. If bank details are used:
   - Funds are routed through Anchor/off-ramp infrastructure
   - Merchant receives fiat payout in bank account
6. Merchant can optionally register later

---

# Architecture Overview

EzPay follows a lightweight payment infrastructure architecture.

```text
Customer Wallet / Payment Interface
                ↓
             EzPay API
                ↓
   Stellar Network & Anchor Infrastructure
                ↓
 Merchant Wallet or Bank Account
```

The backend manages:

- Payment validation
- Transaction construction
- Stellar transaction submission
- Fee sponsorship
- Fiat settlement routing
- Transaction tracking
- Merchant payout processing

---

# Tech Stack

## Blockchain

- Stellar Network

## Backend

- Rust
- Axum

## Blockchain SDK

- Stellar SDK

## Database

- PostgreSQL

## Stablecoins

- USDC
- Stellar-issued stable assets

## Infrastructure

- Stellar RPC
- Anchor integrations
- Wallet integration services

---

# Why Stellar

The Stellar network is built for payments and financial infrastructure.

Benefits include:

- Fast transaction settlement (2–5 seconds)
- Extremely low fees
- Native stablecoin support
- Efficient cross-border payments
- Asset issuance support
- Built-in payment primitives
- Sponsored transaction capabilities
- Strong remittance ecosystem

These features make Stellar ideal for real-world payment applications.

---

# Security & Compliance

EzPay is designed with security and compliance in mind.

## Security

- Transaction validation
- Secure wallet handling
- Multi-signature support
- Infrastructure monitoring
- Fraud prevention systems
- Audit-ready architecture

## Compliance

- KYC/AML support
- SEP-standard integrations
- Secure authentication flows
- Fiat settlement compliance
- Regulatory-aware infrastructure design

---

# Project Structure

```text
ezpay/
│
├── src/
│   ├── config/
│   ├── controllers/
│   ├── services/
│   ├── routes/
│   ├── models/
│   ├── middleware/
│   ├── utils/
│   └── main.rs
│
├── scripts/
├── tests/
├── .env.example
├── Cargo.toml
├── README.md
├── CONTRIBUTING.md
└── LICENSE
```

---

# Getting Started

## Clone the Repository

```bash
git clone https://github.com/your-username/ezpay
cd ezpay
```

---

## Configure Environment Variables

Create a `.env` file:

```env
PORT=3001
RUST_LOG=info
STELLAR_NETWORK=testnet
STELLAR_RPC_URL=
STELLAR_SECRET_KEY=
DATABASE_URL=
ANCHOR_API_KEY=
```

---

## Install Dependencies

```bash
cargo build
```

---

## Run Development Server

```bash
cargo run
```

---

# Project Status

EzPay is currently in MVP development.

The first version focuses on:

- Stablecoin payments
- QR payments
- Merchant payment flows
- Wallet and fiat payout support
- Unregistered merchant payments
- Stellar settlement infrastructure
- Merchant dashboard
- Cross-border payment usability

---

# Future Improvements

- Mobile applications
- Recurring payments
- Invoice generation
- POS integrations
- Merchant analytics
- Multi-chain support
- SDK/API integrations
- Advanced compliance tooling
- Payment subscriptions

---

# Use Cases

- Paying merchants with stablecoins
- Cross-border remittances
- Paying freelancers globally
- Merchant crypto acceptance
- Fiat settlement for businesses
- Low-cost international payments
- QR-based merchant payments

---

# Contributing

We welcome contributions from developers, designers, and blockchain enthusiasts.

Please read `CONTRIBUTING.md` before submitting a pull request.

---

# License

This project is licensed under the MIT License.