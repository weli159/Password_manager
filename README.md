# Rust CLI Password Manager 🦀

A robust, local command-line tool to securely manage passwords. This project was built to learn Rust, specifically focusing on file I/O, serialization, encryption, and random data generation.

## 🚀 Features
* **Encrypted Storage:** Passwords are encrypted using `magic-crypt` (AES-256) before being saved to disk.
* **Auto-Generator:** Built-in secure password generator with customizable length and symbol options.
* **CRUD Operations:** Full support for **C**reating, **R**eading, **U**pdating, and **D**eleting credentials.
* **Unified CLI:** Intuitive commands built with `clap` that handle both manual input and automatic generation.
* **Local Database:** Uses a `passwords.json` file to store data persistently.

## 🛠️ Built With
* **Rust** - The core programming language.
* **Clap** - For parsing command-line arguments.
* **Serde** - For serializing data to JSON.
* **Magic-Crypt** - For encryption/decryption.
* **Rand** - For generating secure random passwords.

## 💻 Usage

### 1. Add a Password
You can type a password manually **OR** ask the tool to generate one for you.

**Manual:**
```bash
cargo run -- add google myemail@gmail.com mysecret123
```

**Auto-Generate (Default 16 chars):**
```bash
cargo run -- add facebook myemail@fb.com --generate
```

**Auto-Generate (Custom):**
```bash
# Generate 20 chars with symbols (!@#$)
cargo run -- add twitter myhandle --generate --length 20 --symbols
```

### 2. Retrieve a Password
Decrypts and reveals the password for a specific service.

```bash
cargo run -- get google
```

### 3. Modify a Password
Update an existing entry. Like adding, you can provide a new password or generate one.

```bash
# Manual update
cargo run -- modify google newpassword456

# Auto-update to a new random password
cargo run -- modify google --generate
```

### 4. Delete a Password
Permanently remove a service from the database.

```bash
cargo run -- delete facebook
```

### 5. List All Services
View all saved accounts (hides sensitive passwords).

```bash
cargo run -- list
```
