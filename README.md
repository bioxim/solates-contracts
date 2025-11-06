# 🧠 Solates Smart Contracts

This repository contains the on-chain programs powering **Solates**, a DeFi education & incentive platform built on **Solana**.

---

## 📦 Structure

solates-contracts/ <br>
├─ points_program/ → Tracks user XP, level and achievements <br>
├─ ola_token/ → Core $OLA token mint and distribution logic <br>
└─ ola_economy/ → Staking, liquidity and post-TGE economy


---

## ⚙️ Programs Overview

### 🪩 Points Program
Handles gamified user progression.

- Register users and their wallets  
- Track earned XP and levels  
- Verify achievements to unlock the Mining Room  

### 💠 OLA Token Program
Implements the $OLA mint and pre-TGE tokenomics.

- Initialize and mint $OLA  
- Control total supply and mint authority  
- Enable airdrops and presales  

### 🔥 OLA Economy Program
Manages token utility and sustainability after the TGE.

- Staking and rewards  
- Liquidity and lending (future modules)  
- On-chain incentives powered by $OLA  

---

## 🧩 Stack

- **Solana CLI** v2.x  
- **Anchor Framework** v0.32+  
- **Rust** stable toolchain  
- **IDLs** generated for Solates frontend integration

---

## 🧪 Development

```bash
# Build
anchor build

# Run local test validator
solana-test-validator

# Deploy to Devnet
anchor deploy --provider.cluster devnet
