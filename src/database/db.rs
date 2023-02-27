use crate::database::*;

pub async fn conectar() -> Result<(), Box<dyn Error>> {
    let client_uri = env::var("MONGODB_URI").expect("\nERROR AL CONECTAR A LA BASE DE DATOS DE MONGODB\n¡Debes establecer la variable de entorno MONGODB_URI!");

    let options = ClientOptions::parse(&client_uri).await?;
    let client = Client::with_options(options)?;

    println!("
╔═════════════════════════════════════════════════════╗
║                                                     ║
║          Conectando a la DB de MongoDB...           ║
║                                                     ║
╚═════════════════════════════════════════════════════╝
");
    for name in client.list_database_names(None, None).await? {
        println!("
╔═════════════════════════════════════════════════════╗
║ Conectado:                                          ║
║ -{name}                                              ║
║                                                     ║
╚═════════════════════════════════════════════════════╝
        ");
    };

    Ok(())
}