use clap::{Parser, Subcommand};
use magic_crypt::{new_magic_crypt, MagicCryptTrait}; // Import encryption tools
use serde::{Deserialize, Serialize};
use std::fs; // Used to handle files (read/write)

// 1. The Blueprint
#[derive(Serialize, Deserialize, Debug)]
struct PasswordEntry {
    service: String,
    username: String,
    encrypted_pass: String,
}

// 2. The CLI Setup
#[derive(Parser)]
#[command(name = "mymanager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        service: String,
        username: String,
        password: String,
    },
    Get {
        service: String,
    },
    List,
}

fn main() {
    let args = Cli::parse();
    let file_path = "passwords.json";

    // ⚠️ SECURITY NOTE: In a real app, never hardcode this key!
    // Use an environment variable or ask the user to type it every time.
    let mc = new_magic_crypt!("my_super_secret_master_key_123");

    match args.command {
        Commands::Add {
            service,
            username,
            password,
        } => {
            // 1. Load existing file (or create empty list)
            let mut passwords = load_passwords(file_path);

            // 2. Encrypt the password
            let encrypted_pass = mc.encrypt_str_to_base64(&password);

            // 3. Add to the list
            passwords.push(PasswordEntry {
                service: service.clone(), // Clone to own the string
                username,
                encrypted_pass,
            });

            // 4. Save back to file
            save_passwords(file_path, &passwords);
            println!("✅ Saved encrypted password for {}!", service);
        }
        Commands::Get { service } => {
            let passwords = load_passwords(file_path);

            // Find the entry that matches the service name
            let entry = passwords.iter().find(|p| p.service == service);

            match entry {
                Some(p) => {
                    // Try to decrypt
                    match mc.decrypt_base64_to_string(&p.encrypted_pass) {
                        Ok(decrypted) => {
                            println!("--------------------------------");
                            println!("🔑 Service:  {}", p.service);
                            println!("👤 Username: {}", p.username);
                            println!("🔓 Password: {}", decrypted);
                            println!("--------------------------------");
                        }
                        Err(_) => println!("❌ Error: Wrong master key or corrupted data."),
                    }
                }
                None => println!("❌ Service '{}' not found.", service),
            }
        }
        Commands::List => {
            let passwords = load_passwords(file_path);
            println!("Saved Services:");
            for p in passwords {
                println!(" - {}", p.service);
            }
        }
    }
}

// --- Helper Functions ---

fn load_passwords(path: &str) -> Vec<PasswordEntry> {
    // Try to read the file. If it fails (file doesn't exist), return an empty list.
    if let Ok(data) = fs::read_to_string(path) {
        serde_json::from_str(&data).unwrap_or(Vec::new())
    } else {
        Vec::new()
    }
}

fn save_passwords(path: &str, passwords: &Vec<PasswordEntry>) {
    // Convert our list to "Pretty" JSON text
    let json = serde_json::to_string_pretty(passwords).expect("Failed to serialize");
    // Write to disk
    fs::write(path, json).expect("Failed to write file");
}
