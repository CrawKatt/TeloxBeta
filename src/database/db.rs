use std::env;
use std::error::Error;
use mongodb::Client;
use mongodb::options::ClientOptions;

pub async fn conectar() -> Result<(), Box<dyn Error>> {
    let client_uri =
        env::var("MONGODB_URI").expect("¡Debes establecer la variable de entorno MONGODB_URI!");

    let options = ClientOptions::parse(&client_uri).await?;
    let client = Client::with_options(options)?;

    println!("Bases de Datos Disponibles:");
    for name in client.list_database_names(None, None).await? {
        println!("- {}", name);
    };

    Ok(())
}