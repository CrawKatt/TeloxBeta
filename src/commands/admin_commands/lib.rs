// Import rand modules for generating random numbers
pub use rand::{
    SeedableRng,
    Rng,
    rngs::StdRng,
};

// Import Teloxide modules for interacting with the Telegram API
pub use teloxide::{
    prelude::*,
    types::*,
    adaptors::DefaultParseMode,
    utils::command::BotCommands,
};

// Import Teloxide_core modules for interacting with the Telegram API
pub use teloxide_core::{
    prelude::UserId,
    types::{
        ChatMemberStatus,
        Message,
        ParseMode::MarkdownV2,
    }
};

pub use serde::{Serialize, Deserialize};

// type Bot for using the DefaultParseMode adapter
pub type Bot = DefaultParseMode<teloxide::Bot>;

// Import the dotenv module for loading environment variables from a .env file
pub use dotenv::dotenv;

// Import standard library modules for working with files
pub use std::path::Path;
pub use std::fs::OpenOptions;
pub use std::io::prelude::*;
pub use std::error::Error;
pub use std::time::Duration;
pub use std::fs;
pub use std::io::{self, Write};

// Import Tokio modules for asynchronous programming
pub use tokio::time::sleep;