use crate::utils;
use directories::ProjectDirs;
use serde::*;
use std::{fs, io::prelude::*, path::*};

#[derive(Serialize, Deserialize, Debug)]
/// Base configuration settings for the app
pub struct Config {
    /// Environment Variables
    pub rpc_url: String,
    /// Environment Variables
    pub chain_id: String,
    /// Environment Variables
    pub pk: String,
}

/// The Configuration is setup through a TOML file that should
/// be setup automatically in your project directory.
pub fn get_config() -> std::io::Result<Config> {
    let original_config = convert()?;
    let rpc_validation = utils::get_rpc_provider(&original_config.rpc_url);
    match rpc_validation {
        Ok(_) => {}
        Err(_) => change_rpc()?,
    }
    if original_config.pk.len() != 64 {
        change_pk()?;
    }
    let config = convert()?;
    Ok(config)
}

fn config_path() -> std::io::Result<PathBuf> {
    let path = if let Some(proj_dirs) = ProjectDirs::from("dev", "perp", "curie_cli") {
        let project_dir = proj_dirs.config_dir();
        if !project_dir.try_exists()? {
            fs::DirBuilder::new().recursive(true).create(project_dir)?;
        }
        project_dir.join("perpcli_config.toml")
    } else {
        Path::new("perpcli_config.toml").to_path_buf()
    };
    Ok(path)
}

fn edit(config: Config) -> std::io::Result<()> {
    let toml = toml::to_string(&config).unwrap();
    let path = config_path()?;
    fs::remove_file(&path)?;
    let mut config_file = fs::File::create(&path)?;
    config_file.write_all(toml.as_bytes())?;
    Ok(())
}

/// Changes the key for the config
pub fn change_pk() -> std::io::Result<()> {
    let mut response = String::new();
    println!("Would you like to change your key? (y/n)");
    std::io::stdin().read_line(&mut response).unwrap();
    if response.trim() != "y" {
        return Ok(());
    }
    response = String::new();
    println!("Please provide the 64-character private key: (Without the 0x prefix)");
    std::io::stdin().read_line(&mut response).unwrap();
    let usr_reply = response.trim();
    if usr_reply.len() != 64 {
        println!("Please enter a valid key.");
    }
    let mut config: Config = convert()?;
    config.pk = usr_reply.to_string();
    edit(config)?;
    println!("Key updated");
    Ok(())
}

/// Changes the Chain ID of the config
pub fn change_chain_id() -> std::io::Result<()> {
    let mut config: Config = convert()?;
    let mut response = String::new();
    println!("Your Chain ID is set to: {}", config.chain_id);
    println!("Would you like to change your Chain ID? (y/n)");
    std::io::stdin().read_line(&mut response).unwrap();
    if response.trim() != "y" {
        return Ok(());
    }
    response = String::new();
    println!("Please provide a Chain ID: (Mainnet = 10)");
    std::io::stdin().read_line(&mut response).unwrap();
    let usr_reply = response.trim();
    // let new_id = usr_reply.parse::<usize>().unwrap();
    config.chain_id = usr_reply.to_string();
    edit(config)?;
    println!("Chain ID has been changed to {}", usr_reply);
    Ok(())
}

/// Changing RPC URL in the config file
pub fn change_rpc() -> std::io::Result<()> {
    let mut response = String::new();
    println!("Would you like to change your RPC URL? (y/n)");
    std::io::stdin().read_line(&mut response).unwrap();
    if response.trim() != "y" {
        return Ok(());
    }
    response = String::new();
    println!("Please provide an RPC URL:");
    std::io::stdin().read_line(&mut response).unwrap();
    let usr_reply = response.trim();
    let provider_request = utils::get_rpc_provider(usr_reply);
    match provider_request {
        Ok(_) => {
            println!("New RPC URL has been validated and works. Saving to config file now...");
            let mut config: Config = convert()?;
            config.rpc_url = usr_reply.to_string();
            edit(config)?;
        }
        Err(_) => {
            panic!("URL failed validation. Please check your URL and try again.")
        }
    }
    change_chain_id()?;
    Ok(())
}

fn convert() -> std::io::Result<Config> {
    let config_file = read_config()?;
    let config: Config = toml::from_str(&config_file)?;
    Ok(config)
}

// Read and/or Create Configurations for the Perp CLI App
fn read_config() -> std::io::Result<String> {
    let config_path = config_path()?;
    if !config_path.as_path().try_exists()? {
        create(&config_path)?;
    }
    let config_file = fs::read_to_string(config_path)?;
    Ok(config_file)
}

fn create(dir: &Path) -> std::io::Result<()> {
    let mut config_file = fs::File::create(dir)?;
    println!("Creating New Config File");
    config_file.write_all(default().as_bytes())?;
    Ok(())
}

fn default() -> &'static str {
    r#"rpc_url = "None"
chain_id = "10"
pk = "None""#
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_config() -> std::io::Result<()> {
        get_config()?;
        Ok(())
    }

    #[test]
    fn test_change_rpc() -> std::io::Result<()> {
        change_rpc()?;
        let config = convert()?;
        assert_eq!(config.rpc_url, String::from("http://127.0.0.1:8545"));
        Ok(())
    }

    #[test]
    fn test_change_chain_id() -> std::io::Result<()> {
        change_chain_id()?;
        let config = convert()?;
        assert_eq!(config.chain_id, String::from("10"));
        Ok(())
    }

    #[test]
    fn test_change_pk() -> std::io::Result<()> {
        change_pk()?;
        let config = convert()?;
        assert_eq!(config.pk.len(), 64);
        Ok(())
    }
}
