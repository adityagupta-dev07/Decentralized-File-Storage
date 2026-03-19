# 📦 Decentralized File Storage (Soroban Smart Contract)

## 🚀 Project Description

This project is a decentralized file storage system built using **Soroban smart contracts** on the Stellar network. It allows users to securely store file metadata (such as file hash and file name) on-chain, ensuring transparency, ownership, and immutability.

Instead of storing actual files on-chain (which is expensive), this system stores **file references (hashes)**, making it efficient and scalable. The actual files can be stored on decentralized platforms like IPFS.

---

## ⚙️ What It Does

* Upload file metadata (file hash + file name)
* Associate files with the user’s wallet address
* Retrieve all stored files
* Retrieve files specific to a user

---

## ✨ Features

* 🔐 **Secure Ownership**: Only the owner can upload their files
* 📁 **Decentralized Metadata Storage**: File hashes stored on-chain
* 🔍 **Efficient Querying**:

  * Get all files
  * Get files by owner
* ⚡ **Gas Efficient**: Stores only metadata, not full files
* 🌐 **IPFS Ready**: Easily integrates with decentralized storage systems

---

## 🧱 Tech Stack

* **Soroban (Stellar Smart Contracts)**
* **Rust**
* **Stellar Testnet**

---

## 📂 Project Structure

```
contracts/
 └── hello-world/
     ├── src/
     │   ├── lib.rs
     │   └── test.rs
     ├── Cargo.toml
     └── target/
```

---

## 🛠 How It Works

1. Upload your file to IPFS (or any storage system)
2. Copy the file hash (CID)
3. Call the smart contract function:

   * `upload_file(owner, file_hash, file_name)`
4. Retrieve files using:

   * `get_files()`
   * `get_files_by_owner(owner)`

---

## ⚡ Build Instructions

```bash
cargo build --target wasm32v1-none --release
```

---

## 🚀 Deploy Contract (PowerShell)

```powershell
stellar contract deploy `
  --wasm target/wasm32v1-none/release/hello_world.wasm `
  --source-account alice `
  --network testnet `
  --alias hello_world
```

---

## 🔗 Deployed Smart Contract

**Contract ID:** CAPNQNOQIJZH3AFDMBFR4CTDDNYEGGGSKPG7FYT6SHEDHPNHY7IQ4SPJ

👉 Replace `CAPNQNOQIJZH3AFDMBFR4CTDDNYEGGGSKPG7FYT6SHEDHPNHY7IQ4SPJ` after deployment with your actual contract ID
Example:

```
https://stellar.expert/explorer/testnet/contract/CAPNQNOQIJZH3AFDMBFR4CTDDNYEGGGSKPG7FYT6SHEDHPNHY7IQ4SPJ
```

---

## 🧪 Example Usage

* Upload a file reference
* Fetch all stored files
* Filter files by owner address

---

## 🔮 Future Improvements

* 🗑 File deletion feature
* 🔒 Private/Public file access control
* 👥 File sharing permissions
* 🌍 Frontend (React + Wallet Integration)
* 📊 Pagination for large datasets

---

## 📜 License

This project is licensed under the MIT License.

---

## 👨‍💻 Author

Aditya Gupta
# Decentralized-File-Storage
