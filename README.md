# Arthur Labs

<p align="center">
    <img src="Logo.png" alt="Arthur Labs Logo" width="150">
</p>

---

This project leverages the **Polkadot EVM/Frontier** repository as its foundational infrastructure, providing a robust and scalable environment for dapps. Arthur Labs is building a blockchain focused on real-world utility and transparency, with a custom set of pallets designed to bridge the digital and physical worlds.

### Core Ideology

The Arthur Labs ecosystem is built on a set of core principles that guide its development and use:

1.  **Scalability and Security over Decentralization:** Prioritizing performance and safety to create a reliable platform.
2.  **Accessibility and Ease of Use:** Making blockchain technology simple and intuitive for all users.
3.  **Quality of Adoption and Utilization:** Focusing on meaningful use cases rather than just a high volume of interactions.
4.  **Separation of Governance:** Ensuring core contributors and true participants have distinct roles in the network's direction.
5.  **Transparency in Transactions:** Providing clear identification for legal entities and their activities on-chain.
6.  **Minimally Inflationary, Primarily Deflationary:** Creating a sustainable economic model for the network.

---

### Key Infrastructure Pallets

The following custom pallets form the backbone of the Arthur Labs blockchain, extending its functionality beyond the standard EVM:

* **Oracle Validators:** A core component that validates the authenticity of real-world transactions and services.
* **Metric Scoring:** A built-in reputation system that assigns a trust score to accounts based on factors like account type, interaction length, and community reviews.
* **Account Type Registrar:** A pallet for wallet creators to officially register their account type (e.g., LLC, C-Corp, 501c3, Project, Startup).
* **Account Type Verification Registrar:** A mechanism for accounts to be verified as legitimate actors through DAO governance.
* **Insurance (Float):** A pallet designed to moderate and handle insurance claims, using staked funds and oracle validators to determine if transactions were fraudulent.
* **Product/Service/Delivery Registrar:** Pallets for registering and managing real-world products, services, and deliveries on the blockchain.
* **Geotracking Node:** A specialized node for verifying the live physical location of individuals or packages.
* **Moderation:** A pallet that uses deterministic metrics to identify and remove malicious actors, with funds being transferred to the Insurance Pallet.
* **Public/Private Code Deployments:** This feature allows smart contract creators to choose whether to make their code public or private upon deployment, which can influence their trust score.
* **Verifiably Random:** A pallet for generating random strings or numbers, useful for a variety of dapps and functionalities.

---

### Getting Started

This guide will walk you through the steps to install and connect to your local Arthur Labs development node.

#### Steps to Install the Node

1.  **Clone the repository:**
    `git clone git@github.com:5eh/Vault-Blockchain.git`

2.  **Build the project:**
    `cargo build --release`

3.  **Run the local development node:**
    `./target/release/frontier-template-node --dev --enable-dev-signer --tmp`

#### Steps to Connect a Wallet Provider

1.  **Import account using seed phrase:**
    You can import an official Substrate development account (no real value) with the following seed phrase:
    `bottom drive obey lake curtain smoke basket hold race lonely fit walk`

2.  **Add the local Frontier network:**
    * **Network ID:** `42`
    * **RPC URL:** `127.0.0.1:9944`
    * **Token:** `$DEV`

#### Steps to Connect to PolkadotJS Apps Frontend

1.  **Follow the instructions to install PolkadotJS Apps:**
    * `git clone https://github.com/polkascan/polkadot-js-apps`
    * `yarn install`
    * `yarn start`
2.  **Connect to your local node:**
    * Navigate to the settings in the frontend.
    * Switch to the local RPC: `127.0.0.1:9944`. This option will only be available once your node is running.
3.  **Confirm custom pallets:**
    * Go to `Developers > Extrinsics` and validate that the `productRegistry` custom pallet is present.

#### Steps to Use `remix.ethereum.org` to Test Contracts

1.  **Connect your wallet provider** to Remix.
2.  **Confirm the localhost network changes** in Remix.
3.  **Deploy your smart contract** using your funded account.

---

### Confirming Transactions on the Local Blockchain

You can use `curl` commands to interact with your local node and confirm transaction details.

#### Get a Transaction Receipt

Check the status of a transaction with its hash:
```bash
curl -H "Content-Type: application/json" -d '{
  "id":1,
  "jsonrpc":"2.0",
  "method":"eth_getTransactionReceipt",
  "params":["TRANSACTION_HASH"]
}' http://localhost:9944
```
**Result: 0x0 indicates failure, 0x1 indicates success.**
#### Get the Contract Code.

Retrieve the encrypted Solidity code stack of a deployed contract:

```bash
curl -H "Content-Type: application/json" -d '{
  "id":1,
  "jsonrpc":"2.0",
  "method":"eth_getCode",
  "params":["CONTRACT_ADDRESS", "latest"]
}' http://localhost:9944
```
