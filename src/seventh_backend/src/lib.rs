use dotenvy::dotenv;
use std::env::{self, VarError};
// use candid::CandidType;

#[derive(Debug)]
pub enum ConfigError {
    MissingVariable(VarError),
    DotenvyError(dotenvy::Error),
}

#[ic_cdk::query]
pub fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[ic_cdk::query]
pub fn get_data() -> Result<[String; 11], ConfigError> {
    dotenv().map_err(ConfigError::DotenvyError)?;

    let pre_minted_tokens = env::var("PRE_MINTED_TOKENS").map_err(ConfigError::MissingVariable)?;
    let default = env::var("DEFAULT").map_err(ConfigError::MissingVariable)?;
    let transfer_fee = env::var("TRANSFER_FEE").map_err(ConfigError::MissingVariable)?;
    let archive_controller = env::var("ARCHIVE_CONTROLLER").map_err(ConfigError::MissingVariable)?;
    let trigger_threshold = env::var("TRIGGER_THRESHOLD").map_err(ConfigError::MissingVariable)?;
    let cycle_for_archive_creation = env::var("CYCLE_FOR_ARCHIVE_CREATION").map_err(ConfigError::MissingVariable)?;
    let num_of_block_to_archive = env::var("NUM_OF_BLOCK_TO_ARCHIVE").map_err(ConfigError::MissingVariable)?;
    let token_name = env::var("TOKEN_NAME").map_err(ConfigError::MissingVariable)?;
    let token_symbol = env::var("TOKEN_SYMBOL").map_err(ConfigError::MissingVariable)?;
    let minter = env::var("MINTER").map_err(ConfigError::MissingVariable)?;
    let feature_flags = env::var("FEATURE_FLAGS").map_err(ConfigError::MissingVariable)?;

    Ok([pre_minted_tokens, default, transfer_fee, archive_controller, trigger_threshold, cycle_for_archive_creation, num_of_block_to_archive, token_name, token_symbol, minter, feature_flags])
}

// pub fn main() -> Result<(), Box<dyn Error>> {
//     // Load environment variables from .env file.
//     // Fails if .env file not found, not readable or invalid.
//     dotenvy::dotenv()?;

//     for (key, value) in env::vars() {
//         println!("{key}: {value}");
//     }

//     Ok(())
// }

// #[ic_cdk::query]
// pub fn get_data() -> [String; 11] {
//     dotenv().ok();

//     let pre_minted_tokens = env::var("PRE_MINTED_TOKENS").expect("PRE_MINTED_TOKENS not found");
//     let default = env::var("DEFAULT").expect("DEFAULT not found");
//     let transfer_fee = env::var("TRANSFER_FEE").expect("TRANSFER_FEE not found");
//     let archive_controller = env::var("ARCHIVE_CONTROLLER").expect("ARCHIVE_CONTROLLER not found");
//     let trigger_threshold = env::var("TRIGGER_THRESHOLD").expect("TRIGGER_THRESHOLD not found");
//     let cycle_for_archive_creation = env::var("CYCLE_FOR_ARCHIVE_CREATION").expect("CYCLE_FOR_ARCHIVE_CONTROLLER not found");
//     let num_of_block_to_archive = env::var("NUM_OF_BLOCK_TO_ARCHIVE").expect("NUM_OF_BLOCK_TO_ARCHIVE not found");
//     let token_name = env::var("TOKEN_NAME").expect("TOKEN_NAME not found");
//     let token_symbol = env::var("TOKEN_SYMBOL").expect("TOKEN_SYMBOL not found");
//     let minter = env::var("MINTER").expect("MINTER not found");
//     let feature_flags = env::var("FEATURE_FLAGS").expect("FEATURE_FLAGS not found");

//     return [pre_minted_tokens, default, transfer_fee, archive_controller, trigger_threshold, cycle_for_archive_creation, num_of_block_to_archive, token_name, token_symbol, minter, feature_flags];
// }
