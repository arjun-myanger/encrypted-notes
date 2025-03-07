# 🔒 Encrypted Notes App

A secure **Rust-based encrypted notes** application that allows users to create **self-destructing notes**. Once a note is read, it is **deleted permanently** from storage. Built using **Warp (Web API)** and **CLI (Command Line Interface)**.

---

## 🚀 Features

✅ **AES-256-GCM Encryption** - All notes are securely encrypted before storage.  
✅ **One-Time Read (Self-Destructing Notes)** - Once a note is read, it is permanently deleted.  
✅ **CLI & API Support** - Use it via command line or integrate it into web applications.  
✅ **Rust-powered Security** - Built using Rust’s strong type safety and performance benefits.  
✅ **Local-Only Storage** - Messages are currently stored **only on the same machine**.  

---

## ⚠️ **Current Limitations: Local-Only Usage**
Currently, this app **only works on the same machine**. This means:
- The **API is not exposed to the internet** (`127.0.0.1:3030` is only accessible locally).
- Notes **cannot be shared** with someone on another device or location.
- **All stored messages exist only on your local system.**

🔹 **To make it work globally, we would need to:**
1. **Deploy the API online** (e.g., AWS, DigitalOcean, Railway.app).
2. **Use a remote database** instead of local storage (`db/` folder).
3. **Expose a real URL** so anyone can create & read notes remotely.

📌 **This version is great for personal use or local secure notes!** 🔒

---

## 🛠 Setup & Installation

### 📌 **Clone the Repository**
```sh
git clone https://github.com/YOUR-USERNAME/encrypted-notes.git
cd encrypted-notes
```

### 📌 **Build the Project**
```sh
cargo build
```

### 📌 **Run the Server** (For API Usage)
```sh
cargo run
```

### 📌 **Run in CLI Mode** (For Local Usage)
```sh
cargo run -- create "Your secret message!"
```

---

## 🔑 Important: Replace the Encryption Key
For **better security**, replace the **hardcoded encryption key** inside `main.rs`.

### ✅ **Generate a Secure 32-Byte Key**
Use OpenSSL or any secure method to generate a **random** key:
```sh
openssl rand -base64 32
```

### ✅ **Modify `main.rs`**
Replace the existing key in `main.rs`:
```rust
let key: [u8; 32] = *b"this_is_a_strong_32_byte_key12!!"; // 🔴 Replace this!
```
**With:**
```rust
let key: [u8; 32] = *b"your_new_random_32_byte_key_here";
```
📌 **Do not hardcode your key in production. Store it securely using environment variables or a secrets manager.**

---

## 📌 Usage

### 📝 **Create an Encrypted Note**
```sh
cargo run -- create "This is a secret note!"
```
✅ **Output:**
```sh
Note created! Access it using:
http://127.0.0.1:3030/read/<note_id>
```

### 🔍 **Read a Note (First Time)**
```sh
cargo run -- read <note_id>
```
✅ **Expected Output:**
```sh
Note: This is a secret note!
```

### ❌ **Try Reading Again (Note is Deleted)**
```sh
cargo run -- read <same_note_id>
```
🚨 **Expected Output:**
```sh
❌ This note has already been read and is now deleted!
```

---

## 📜 License
This project is licensed under the **MIT License** - feel free to use and modify it as needed.

---
