use crate::*;
use anyhow::{Context, Result};
use colored::Colorize;
use directories::ProjectDirs;
use keyring::Entry;
use rpassword::prompt_password;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::exit;
use std::{env, fs};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    username: String,
}

const KEYRING_SERVICE_NAME: &str = "ssh-grades-354";

pub fn get_user() -> Result<User> {
    let args: Vec<String> = env::args().collect();

    if args.get(1).map_or(false, |arg| arg == "reset") {
        match reset_user() {
            Ok(_) => {
                println!("✅ User reset successfully. Rerun the program to set new credentials.")
            }
            Err(e) => eprintln!("{} {}", "Error during reset:".bright_red(), e),
        }
        exit(0);
    }

    if let Some(proj_dirs) = ProjectDirs::from("dev", "ldutrie", "ssh-grades") {
        let config_dir = proj_dirs.config_dir();
        let config_path = config_dir.join("config.toml");

        if config_path.exists() {
            println!("Welcome back!");
            println!("Check {} for your authentification", "DUO".green());
            load_existing_user(&config_path)
        } else {
            println!("Welcome! Please set up your credentials.");
            initialize_new_user(config_dir, &config_path)
        }
    } else {
        Err(anyhow::anyhow!(
            "Could not find a valid configuration directory."
        ))
    }
}

fn initialize_new_user(config_dir: &std::path::Path, config_path: &PathBuf) -> Result<User> {
    loop {
        print!("Enter your username: ");
        io::stdout().flush()?;
        let mut username = String::new();
        io::stdin().read_line(&mut username)?;
        let username = username.trim().to_string();

        let password = prompt_password("Enter your password: ")?;
        let confirm_password = prompt_password("Confirm your password: ")?;
        if password != confirm_password {
            continue;
        }
        let password = password + ",PUSH";

        print!("Is this correct? (y/n): ");
        io::stdout().flush()?;
        let mut confirmation = String::new();
        io::stdin().read_line(&mut confirmation)?;

        match confirmation.trim().to_lowercase().as_str() {
            "y" | "yes" => {
                fs::create_dir_all(config_dir)?;

                let config = Config {
                    username: username.clone(),
                };
                let toml_string = toml::to_string(&config)?;
                fs::write(config_path, toml_string)?;

                let entry = Entry::new(KEYRING_SERVICE_NAME, &username)?;
                entry.set_password(&password)?;

                println!("{}", "✅ Configuration saved successfully!".bright_green());

                return Ok(User {
                    user_name: username,
                    password,
                });
            }
            "n" | "no" => {
                println!("\nLet's try that again.");
                continue;
            }
            _ => {
                println!(
                    "\nInvalid input. Please enter 'y' for yes or 'n' for no. Let's start over."
                );
                continue;
            }
        }
    }
}

/// Loads user info from the config file and system keyring.
fn load_existing_user(config_path: &PathBuf) -> Result<User> {
    let toml_string = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&toml_string)?;
    let username = config.username;

    let entry = Entry::new(KEYRING_SERVICE_NAME, &username)?;
    let password = entry.get_password()?;

    Ok(User {
        user_name: username,
        password,
    })
}

fn reset_user() -> Result<()> {
    if let Some(proj_dirs) = ProjectDirs::from("dev", "ldutrie", "ssh-grades") {
        let config_path = proj_dirs.config_dir().join("config.toml");

        if config_path.exists() {
            let toml_string =
                fs::read_to_string(&config_path).context("Could not read config file to reset.")?;
            let config: Config = toml::from_str(&toml_string)?;

            match Entry::new(KEYRING_SERVICE_NAME, &config.username)?.delete_password() {
                Ok(_) | Err(keyring::Error::NoEntry) => (),
                Err(e) => return Err(e.into()),
            }

            fs::remove_file(config_path)?;
        }
    }
    Ok(())
}
