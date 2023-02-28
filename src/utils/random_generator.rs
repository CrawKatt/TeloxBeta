use crate::admin_commands::*;

pub async fn ban_animation_generator(bot: Bot, msg: Message) -> ResponseResult<()> {
    // Random number generator
    let mut rng: StdRng = SeedableRng::from_entropy();

    // generate a random number from 0 to 14
    let random_number = rng.gen_range(0..=14);

    // List of ban animations
    let file_names = [
        "1.gif", "2.gif", "3.gif", "4.gif", "5.gif", "6.gif", "7.gif", "8.gif",
        "9.gif", "10.gif", "11.gif", "12.mp4", "13.mp4", "14.mp4", "15.mp4",
    ];

    // Get the file name from the list
    let get_file_name = |index: usize| -> &'static str {
        file_names.get(index).unwrap_or_else(|| file_names.last().unwrap())
    };

    // Send the ban animation and match the file extension to send the correct type of file.
    let file_path = format!("./assets/ban/{}", get_file_name(random_number));
    match file_path.ends_with(".gif") {

        // If the file is a GIF, send it as an animation.
        true => {
            let gif = bot.send_animation(msg.chat.id, InputFile::file(file_path)).await?;
            sleep(Duration::from_secs(60)).await;
            bot.delete_message(msg.chat.id, gif.id).await?;
        }

        // Else, send it as a video.
        false => {
            let video = bot.send_video(msg.chat.id, InputFile::file(file_path)).await?;
            sleep(Duration::from_secs(60)).await;
            bot.delete_message(msg.chat.id, video.id).await?;
        }

    };

    Ok(())
}

pub async fn mute_animation_generator(bot: Bot, msg: Message) -> ResponseResult<()> {

    let chat_id = msg.chat.id;

    let mut rng: StdRng = SeedableRng::from_entropy();
    let file_names = ["1.gif", "2.gif", "3.gif", "4.gif", "5.jpg"];
    let random_number = rng.gen_range(0..=file_names.len() - 1);

    let file_path = format!("./assets/mute/{}", file_names[random_number]);
    let file_extension = file_path.split('.').last().unwrap_or("");

    match file_extension {

        "gif" => {
            let gif = bot.send_animation(chat_id, InputFile::file(file_path)).await?;
            sleep(Duration::from_secs(60)).await;
            bot.delete_message(chat_id, gif.id).await?;
        },

        "jpg" => {
            let jpg = bot.send_photo(chat_id, InputFile::file(file_path)).await?;
            sleep(Duration::from_secs(60)).await;
            bot.delete_message(chat_id, jpg.id).await?;
        },

        _ => {
            let err = bot.send_message(chat_id, "❌ No se pudo enviar el archivo").await?;
            sleep(Duration::from_secs(60)).await;
            bot.delete_message(chat_id, err.id).await?;
        }

    };

    Ok(())
}

pub async fn send_random_meme_generator(bot: Bot, msg: Message) -> ResponseResult<()> {
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

    /*Español/Spanish*/
    // generamos un numero aleatorio

    /*Inglés/English*/
    // generate a random number
    let mut rng: StdRng = SeedableRng::from_entropy();
    let random_number = rng.gen_range(0..=file_names.len() - 1);

    /*Inglés/English*/
    // imprimimos el resultado en consola con fines de debug
    // print the result in the console for debugging purposes
    println!("Resultado : {}", random_number);

    // borramos el mensaje original
    //bot.delete_message(msg.chat.id, msg.id).await?;

    /*Español/Spanish*/
    // Utiliza la función `format!` para crear una cadena de caracteres que contiene una ruta de archivo.
    // La ruta de archivo se compone de la cadena "./assets/memes/" seguida del nombre de archivo obtenido de la función `get_file_name`.
    // El argumento proporcionado a `get_file_name` es `random_number`, que es un número aleatorio generado previamente.

    /*Inglés/English*/
    // Use the `format`! function to create a string that contains a file path.
    // The file path consists of the string "./assets/memes/" followed by the file name obtained from the `get_file_name` function.
    // The argument provided to `get_file_name` is `random_number`, which is a randomly generated number previously.
    let file_path = format!("./assets/memes/{}", get_file_name(random_number));

    /*Español/Spanish*/
    // Utiliza la función `Path::new` para crear un objeto `Path` a partir de la cadena de caracteres `file_path`.
    // Luego, se llama al método `is_file` del objeto `Path` para verificar si la ruta de archivo apunta a un archivo existente.

    /*Inglés/English*/
    // Use the `Path::new` function to create a `Path` object from the string `file_path`.
    // Then, call the `is_file` method of the `Path` object to check if the file path points to an existing file.

    if Path::new(&file_path).is_file() { // If the file path points to an existing file, check if the file name ends with ".mp4".
        // Si la ruta de archivo apunta a un archivo existente, se verifica si el nombre de archivo termina con ".mp4".
        if file_path.ends_with(".mp4") { // If the file name ends with ".mp4", send the video file to the chat using the `send_video` method of the `bot` object.
            // Si el nombre de archivo termina con ".mp4", se envía el archivo de video al chat utilizando el método `send_video` del objeto `bot`.
            // El argumento proporcionado a `send_video` es un objeto `InputFile` que se crea a partir de la ruta de archivo.
            bot.send_video(msg.chat.id, InputFile::file(&file_path)).reply_to_message_id(msg.id).await?; // The argument provided to `send_video` is an `InputFile` object that is created from the file path.
        } else { // If the file name does not end with ".mp4", assume it is an image file and send it to the chat using the `send_photo` method of the `bot` object.
            // Si el nombre de archivo no termina con ".mp4", se asume que es un archivo de imagen y se envía al chat utilizando el método `send_photo` del objeto `bot`.
            // El argumento proporcionado a `send_photo` es un objeto `InputFile` que se crea a partir de la ruta de archivo.
            bot.send_photo(msg.chat.id, InputFile::file(&file_path)).reply_to_message_id(msg.id).await?; // The argument provided to `send_photo` is an `InputFile` object that is created from the file path.
        }
    }

    Ok(())
}

pub async fn random_pat(bot: Bot, msg: Message) ->ResponseResult<()> {
    let file_names = [
        "1.gif", "2.gif", "3.gif", "4.gif", "5.gif", "6.gif", "7.gif", "8.gif", "9.gif", "10.gif",
        "11.gif", "12.gif", "13.gif", "14.gif", "15.gif", "16.gif", "17.gif", "18.gif", "19.gif",
        "20.gif", "21.gif", "22.gif", "23.gif", "24.gif", "25.gif", "26.gif", "27.gif", "28.gif",
        "29.gif",
    ];

    let get_file_name = |index: usize| -> &'static str { file_names.get(index).unwrap_or(&file_names[0]) };

    let mut rng: StdRng = SeedableRng::from_entropy();
    let random_number = rng.gen_range(0..=file_names.len() - 1);
    println!("Resultado : {}", random_number);

    bot.delete_message(msg.chat.id, msg.id).await?;

    let file_path = format!("./assets/pat/{}", get_file_name(random_number));

    if Path::new(&file_path).is_file() {
        bot.send_animation(msg.chat.id, InputFile::file(&file_path)).await?;
    }

    Ok(())
}