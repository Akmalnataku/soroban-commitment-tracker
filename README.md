# Commitment Tracker Smart Contract

## 📌 Overview

This project is a decentralized smart contract built using Soroban on the Stellar network. The application allows users to create and manage personal commitments with enforced rules such as deadlines and immutable completion status.

## 🚀 Features

* Create a new commitment with a deadline
* View all stored commitments
* Mark a commitment as completed (cannot be undone)
* Prevent deletion before the deadline
* Ensure data integrity through smart contract logic

## 🧠 Smart Contract Logic

This smart contract introduces rule-based behavior beyond basic CRUD operations:

* Commitments cannot be deleted before their deadline
* Completed commitments cannot be modified again
* Deadlines must be set in the future

## 🛠️ Tech Stack

* Rust (Soroban SDK)
* Stellar Smart Contracts (Soroban)
* Stellar CLI

## 🌐 Deployment

* Network: Stellar Testnet
* Contract ID:
  `CD7XKTOYZKQ7KT3NW7S2GZGJFPAQ7J3GRH24FLBVDI6SAZZ72XFROYM`

## ⚙️ Example Commands

### Create Commitment

```
stellar contract invoke --id CONTRACT_ID --source-account YOUR_ACCOUNT --network testnet -- create_commitment --title "Study Rust" --deadline 2000000000
```

### Get Commitments

```
stellar contract invoke --id CONTRACT_ID --source-account YOUR_ACCOUNT --network testnet -- get_commitments
```

### Complete Commitment

```
stellar contract invoke --id CONTRACT_ID --source-account YOUR_ACCOUNT --network testnet -- complete_commitment --id 123
```

### Delete Commitment

```
stellar contract invoke --id CONTRACT_ID --source-account YOUR_ACCOUNT --network testnet -- delete_commitment --id 123
```

## 📸 Screenshots

(Add your CLI output screenshots here)

## 📖 Use Case

This application helps users build discipline by enforcing commitment rules through decentralized logic, ensuring that actions such as deletion or completion follow predefined constraints.

## 📄 License

This project is developed for educational purposes.
