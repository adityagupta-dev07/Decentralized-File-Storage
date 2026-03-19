# 📦 Decentralized File Storage (Soroban Smart Contract)

## 🚀 Project Description

This project is a decentralized file storage system built using **Soroban smart contracts** on the Stellar network. It enables users to securely store file metadata (such as file hash and file name) on-chain, ensuring transparency, ownership, and immutability.

Instead of storing full files on-chain (which is costly), this system stores **file references (hashes)**, making it efficient, scalable, and suitable for real-world decentralized applications.

---

## ⚙️ What It Does

* Upload file metadata (file hash + file name)
* Link files to a user’s wallet address
* Retrieve all stored files
* Retrieve files owned by a specific user

---

## ✨ Features

* 🔐 **Ownership Verification**: Only the owner can upload files
* 📁 **On-chain Metadata Storage**
* 🔍 **Data Retrieval**:

  * Get all files
  * Get files by owner
* ⚡ **Efficient Storage**: Only hashes stored (low cost)
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

**Contract ID:**

```
CAPNQNOQIIZH3AFDMBFR4CTDDNYEGGGSKPG7FYT6SHEDHPNHY7IQ4SPJ
```

**Explorer Link:**
https://lab.stellar.org/r/testnet/contract/CAPNQNOQIIZH3AFDMBFR4CTDDNYEGGGSKPG7FYT6SHEDHPNHY7IQ4SPJ

---

## 🧪 Example Use Cases

* Store file references securely
* Build decentralized Google Drive–like apps
* Integrate with IPFS for full decentralized storage

---

## 🔮 Future Improvements

* 🗑 File deletion feature
* 🔒 Private/Public access control
* 👥 File sharing permissions
* 🌍 Frontend (React + Wallet Integration)
* 📊 Pagination for large datasets

---

## 📜 License

MIT License

---

## 👨‍💻 Author

Aditya Gupta
