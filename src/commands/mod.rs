// Imports de archivos del bot
// Imports of bot files
pub mod admin;
pub mod comandos;
pub mod fun;
pub mod funciones;
pub mod admin_commands;

// uso de archivos del bot
// Use of bot files
pub use admin::*;
pub use comandos::*;
pub use fun::*;
pub use funciones::*;
pub use crate::commands::admin_commands::*; // Llama a la carpeta admin_commands >> calls the admin_commands folder and mod.rs into the admin_commands folder