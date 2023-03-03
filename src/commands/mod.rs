// Imports de archivos del bot
// Imports of bot files
pub mod testing;
pub mod handler;
pub mod funciones;
pub mod admin_commands;
pub mod fun_commands;

// Uso de admin_commands en el bot
// Use of admin_commands in the bot
pub use admin_commands::*; // Llama a la carpeta admin_commands >> calls the admin_commands folder and mod.rs into the admin_commands folder