use std::path::Path;
use teloxide::prelude::Requester;
use teloxide::types::InputFile;

use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

use crate::comandos;
use crate::Message;
use crate::ResponseResult;

pub async fn send_pat(bot: comandos::MyBot, msg: Message) -> ResponseResult<()> {
    let file_names = [
        "1.gif", "2.gif", "3.gif", "4.gif", "5.gif", "6.gif", "7.gif", "8.gif", "9.gif", "10.gif",
        "11.gif", "12.gif", "13.gif", "14.gif", "15.gif", "16.gif", "17.gif", "18.gif", "19.gif",
        "20.gif", "21.gif", "22.gif", "23.gif", "24.gif", "25.gif", "26.gif", "27.gif", "28.gif",
        "29.gif",
    ];

    let get_file_name =
        |index: usize| -> &'static str { file_names.get(index).unwrap_or(&file_names[0]) };

    let mut rng: StdRng = SeedableRng::from_entropy();
    let random_number = rng.gen_range(0..=file_names.len() - 1);
    println!("Resultado : {}", random_number);

    bot.delete_message(msg.chat.id, msg.id).await?;

    let file_path = format!("./assets/pat/{}", get_file_name(random_number));

    if Path::new(&file_path).is_file() {
        bot.send_animation(msg.chat.id, InputFile::file(&file_path))
            .await?;
    }

    Ok(())
}

pub async fn send_random_meme(bot: comandos::MyBot, msg: Message) -> ResponseResult<()> {
    // Declara un vector llamado `file_names` que almacena una lista de nombres de archivos (linea 51).
    // Los nombres de archivos son cadenas de caracteres que terminan en `.mp4` o `.jpg`.
    let file_names = [
        // Estos son nombres de archivos de video.
        "1.mp4", "2.mp4", "3.mp4", "4.mp4", "5.mp4",
        // Estos son nombres de archivos de imagen.
        "6.jpg", "7.jpg", "8.jpg", "9.jpg", "10.jpg", "11.jpg", "12.jpg", "13.jpg", "14.jpg",
        "15.jpg", "16.jpg", "17.jpg", "18.jpg", "19.jpg", "20.jpg", "21.jpg", "22.jpg", "23.jpg",
        "24.jpg", "25.jpg", "26.jpg", "27.jpg", "28.jpg", "29.jpg",
    ];

    // Esta es una función anónima, también conocida como "lambda" (linea 64).
    // Esta función toma un argumento de tipo `usize` y devuelve una cadena de caracteres 'static str'
    let get_file_name = |index: usize| -> &'static str {
        // Aquí en la linea 69, se utiliza el método `get` del tipo `Vec` para obtener un elemento en una posición específica.
        // Si el índice es válido, se devuelve el elemento en esa posición.
        // Si el índice es inválido (es decir, si el índice es mayor que el tamaño del vector), se devuelve `None`.
        file_names
            .get(index)
            // Aquí en la linea 74, se utiliza el método `unwrap_or` para desempaquetar el valor de `Option` devuelto por `get`.
            // Si el valor es `Some`, se devuelve el valor almacenado en `Some`.
            // Si el valor es `None`, se devuelve el valor proporcionado como argumento a `unwrap_or`, en este caso el primer elemento del vector `file_names`.
            .unwrap_or(&file_names[0])
    };

    // generamos un numero aleatorio
    let mut rng: StdRng = SeedableRng::from_entropy();
    let random_number = rng.gen_range(0..=file_names.len() - 1);

    // imprimimos el resultado en consola con fines de debug
    println!("Resultado : {}", random_number);

    // borramos el mensaje original
    bot.delete_message(msg.chat.id, msg.id).await?;

    // Utiliza la función `format!` para crear una cadena de caracteres que contiene una ruta de archivo.
    // La ruta de archivo se compone de la cadena "./assets/memes/" seguida del nombre de archivo obtenido de la función `get_file_name`.
    // El argumento proporcionado a `get_file_name` es `random_number`, que es un número aleatorio generado previamente.
    let file_path = format!("./assets/memes/{}", get_file_name(random_number));

    // Utiliza la función `Path::new` para crear un objeto `Path` a partir de la cadena de caracteres `file_path`.
    // Luego, se llama al método `is_file` del objeto `Path` para verificar si la ruta de archivo apunta a un archivo existente.
    if Path::new(&file_path).is_file() {
        // Si la ruta de archivo apunta a un archivo existente, se verifica si el nombre de archivo termina con ".mp4".
        if file_path.ends_with(".mp4") {
            // Si el nombre de archivo termina con ".mp4", se envía el archivo de video al chat utilizando el método `send_video` del objeto `bot`.
            // El argumento proporcionado a `send_video` es un objeto `InputFile` que se crea a partir de la ruta de archivo.
            bot.send_video(msg.chat.id, InputFile::file(&file_path))
                .await?;
        } else {
            // Si el nombre de archivo no termina con ".mp4", se asume que es un archivo de imagen y se envía al chat utilizando el método `send_photo` del objeto `bot`.
            // El argumento proporcionado a `send_photo` es un objeto `InputFile` que se crea a partir de la ruta de archivo.
            bot.send_photo(msg.chat.id, InputFile::file(&file_path))
                .await?;
        }
    }

    Ok(())
}
