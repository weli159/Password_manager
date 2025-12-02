# Rust CLI Password Manager 🦀

A simple, local command-line tool to securely manage passwords. This project was built to learn Rust, specifically focusing on file I/O, serialization, and encryption.

## 🚀 Features
* **Encrypted Storage:** Passwords are encrypted using `magic-crypt` before being saved to disk.
* **Local Database:** Uses a `passwords.json` file to store data using `serde`.
* **CLI Interface:** Built with `clap` for easy command-line interaction.
* **CRUD Operations:** Supports Adding, Getting, and Listing credentials.

## 🛠️ Built With
* **Rust** - The core programming language.
* **Clap** - For parsing command-line arguments.
* **Serde** - For serializing data to JSON.
* **Magic-Crypt** - For handling string encryption.

## 💻 Usage

**Add a password:**
```bash
cargo run -- add <service> <username> <password>
# Example: cargo run -- add google myemail@gmail.com mysecret123
