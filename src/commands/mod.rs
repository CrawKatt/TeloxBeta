// Imports de archivos del bot
// Imports of bot files
pub mod testing;
pub mod handler;
pub mod funciones;
pub mod admin_commands;
pub mod fun_commands;

// uso de archivos del bot
// Use of bot files
pub use testing::*;
pub use handler::*;
pub use funciones::*;
pub use admin_commands::*; // Llama a la carpeta admin_commands >> calls the admin_commands folder and mod.rs into the admin_commands folder
pub use fun_commands::*; // Llama a la carpeta fun_commands >> calls the fun_commands folder and mod.rs into the fun_commands folder