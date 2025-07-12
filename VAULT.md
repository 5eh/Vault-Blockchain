
Core ideology

1. Scalability and Security over Decentralization
2. Accesability and ease of blockchain over features and functionality
3. Quality of adoption and utilization over quantity of interactions
4. Seperation of DAO Governance to core ecosystem contributors, and true participants
5. Transparency in account types and transactions (Clear identification for legal formations and their infrastructure)
6. Minimally inflationary, primarily deflationary.

# Core Blockchain suggestions and ideations for this blockchain:


Definitions:
RW - Real World
RWC - Real World Commerce
RWS - Real World Services
RWD - Real World Deliveries
Pallet - Core blockchain functionality and interactions

## Metric scoring level on chain level
- Built in reviews and scoring mechanism to determine an accounts trust
- Other wallets can review each other (Txn fee, and perhaps Oracle Validator should boost response)
- Scale of 100
- Types of metrics should include: 1. Length of interactivity, 2. Type of account (User, Organization, Collective, Business, Charity), 3. Reviews from others, 4. Governance activity, 5. Snuffed activity (Is there anything suspicious with this?)

## Oracle Validators (Core)
- Review and determine if (real world transactions) are authentic
- Receive incentives for validating transactions

## Smart Contract & !ink Pallet
- Enable EVM backbone into the Vault Blockchain for contract deployment.

## Public/Private code deployments
- Allow creators to deploy contracts, and select upon deployment
- If their code is public, automatically improves their trust metric a little bit
- If their code is not public, there is no change in trust metric
- Private code will remain anonymous, but the contract creator can eventually publish it openly, otherwise it remains in encrypted format
- Private code can be verified and audited through DAO governance

## Insurance (float) Pallet
- Determine if transactions were fraudulant
- Determine if insurance claims were fraudulant
- Determine if transactions were real
- Determine if insurance claims were real
- Moderate if transactions
- Limited initial supply
- Use Oracle Validators in the process
- Core funds are staked to validate network
- Builds treasury over time, using leading cryptocurrencies

## Moderation Pallet
- Retrieves deterministic metrics such as trust, liveliness, responsibility and account type to determine authenticity
- If blocked, account is removed from being able to transact, and funds are punished into the Insurance Pallet

## Account Type Registrar Pallet
- Allows the wallet creator to register an account type
- Account types consist of:
["LLC", "S-Corp", "C-Corp", "Partnership", "LLP", "LP", "Sole-Proprietorship", "501c3", "501c4", "501c6", "Ltd", "PLC", "GmbH", "AG", "SAS", "SARL", "BV", "AB", "Pty-Ltd", "Project", "Solo", "Startup", "Cooperative", "Joint-Venture", "Subsidiary", "Foundation", "Trust", "DAO", "Unincorporated", "Pending", "Investor"]

## Account Tags Registrar Pallet
- Allows the wallet creator to additionally add up two tags to their wallet
- There are two tag with a max char length of 16 characters (emoji/text/numbers included)
- They can create/change/delete it themselves by paying a gas fee

## Account Type Verification Registrar Pallet
- Allows for the wallet creator to go through a DAO governance to upload verfication of the account type
- DAO governance just requires a few people to see it and agree to it, these DAO governances will be required to hold $VAU.A tokens
- After they upload and receive verification, their accounts are considered "Verified", with a new tag next to their name, that displays they are legitimate actors in the ecosystem.

## External API Connection Nodes Pallet

## Product Registrar Pallet

## Service Registrar Pallet

## Delivery Registrar Pallet

## Geotracking Node Pallet
- A custom geotracking and physical node that confirms/validates live location of individuals, or packages
- Used for RW

## Verifiably Random Pallet
- Generates random string / number inputs
- Allows for multiple returns
- Stored in Array, u32, u64, or single bit functions
