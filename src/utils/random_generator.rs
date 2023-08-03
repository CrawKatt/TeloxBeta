use crate::dependencies::*;

/// # Errors
pub async fn ban_animation_generator(bot: Bot, msg: Message) -> ResponseResult<()> {
    println!("Test Generator");

    let Some(text) = msg.text() else {
        println!("❌ No se pudo obtener el texto del mensaje {msg:#?}");

        return Ok(())
    };

    let (_, username) = text
        .find('@')
        .map_or(("", text), |index| text.split_at(index));

    if username.contains('@') {
        let contents = read_database_file()?;

        let user_data_vec: Vec<UserData> = serde_json::from_str(&contents).map_or_else(
            |_| {
                eprintln!("❌ No se pudo leer el archivo de base de datos");

                Vec::new()
            },
            |vec| vec,
        );

        let user_id = user_data_vec.iter().find_map(|data| {
            data.username.as_ref().and_then(|name| {
                if name == username {
                    Some(data.id.to_string())
                } else {
                    None
                }
            })
        });

        if let Some(user_id) = user_id {
            let user_id = match user_id.parse::<u64>() {
                Ok(id) => id,
                Err(e) => {
                    eprintln!("❌ No se pudo obtener el ID del usuario: {e}");

                    return Ok(())
                }
            };

            let Some(text) = msg.text() else {
                println!("❌ No se pudo obtener el texto del mensaje {msg:#?}");

                return Ok(())
            };

            // get the arguments after the command trigger
            let (_, arguments) = text
                .find(' ')
                .map_or(("", text), |index| text.split_at(index));

            // check if the arguments are empty
            if arguments.is_empty() {
                bot.send_message(msg.chat.id, NOT_ID_PROVIDED_404).await?;

                bot.delete_message(msg.chat.id, msg.id).await?;

                println!("❌ No has especificado un ID para obtener el usuario {msg:#?}");

                return Ok(())
            }

            let mut rng: StdRng = SeedableRng::from_entropy();

            // generate a random number from 0 to 14
            let random_number = rng.gen_range(0..=14);

            // List of ban animations
            let file_names = [
                "1.gif", "2.gif", "3.gif", "4.gif", "5.gif", "6.gif", "7.gif", "8.gif",
                "9.gif", "10.gif", "11.gif", "12.mp4", "13.mp4", "14.mp4",
            ];

            // Get the file name from the list
            let get_file_name = |index: usize| -> &'static str {
                file_names.get(index).map_or_else(
                    || file_names.last().map_or("", |last_file| last_file),
                    |file_name| file_name,
                )
            };

            // Send the ban animation and match the file extension to send the correct
            // type of file.
            let file_path = format!("./assets/ban/{}", get_file_name(random_number));

            if Path::new(&file_path)
                .extension()
                .map_or(false, |ext| ext == "gif")
            {
                let gif = bot
                    .send_animation(msg.chat.id, InputFile::file(file_path))
                    .caption(format!(
                        "{username} [<code>{}</code>] baneado",
                        UserId(user_id)
                    ))
                    .parse_mode(ParseMode::Html)
                    .await?;

                sleep(Duration::from_secs(60)).await;

                bot.delete_message(msg.chat.id, gif.id).await?;

                bot.delete_message(msg.chat.id, msg.id).await?;
            } else {
                let video = bot
                    .send_video(msg.chat.id, InputFile::file(file_path))
                    .caption(format!("{username} [<code>{user_id}</code>] baneado"))
                    .parse_mode(ParseMode::Html)
                    .await?;

                sleep(Duration::from_secs(60)).await;

                bot.delete_message(msg.chat.id, video.id).await?;

                bot.delete_message(msg.chat.id, msg.id).await?;
            };
        }
    }

    Ok(())
}

/// # Errors
pub async fn mute_animation_generator(bot: Bot, msg: Message) -> ResponseResult<()> {
    let Some(text) = msg.text() else {
        return Ok(())
    };

    let (_, username) = text
        .find('@')
        .map_or(("", text), |index| text.split_at(index));

    if username.contains('@') {
        let contents = read_database_file()?;

        let user_data_vec: Vec<UserData> = serde_json::from_str(&contents).map_or_else(
            |_| {
                eprintln!("❌ No se pudo leer el archivo de base de datos");

                Vec::new()
            },
            |vec| vec,
        );

        let user_id = user_data_vec.iter().find_map(|data| {
            data.username.as_ref().and_then(|name| {
                if name == username {
                    Some(data.id.to_string())
                } else {
                    None
                }
            })
        });

        if let Some(user_id) = user_id {
            let user_id = match user_id.parse::<u64>() {
                Ok(id) => id,
                Err(e) => {
                    eprintln!("❌ No se pudo obtener el ID del usuario: {e}");

                    return Ok(())
                }
            };

            let mut rng: StdRng = SeedableRng::from_entropy();

            let file_names = ["1.gif", "2.gif", "3.gif", "4.gif", "5.jpg"];

            let random_number = rng.gen_range(0..file_names.len());

            let file_path = format!("./assets/mute/{}", file_names[random_number]);

            let file_extension = file_path.split('.').last().unwrap_or("");

            match file_extension {
                "gif" => {
                    let ok = bot
                        .send_animation(msg.chat.id, InputFile::file(file_path))
                        .caption(format!(
                            "{username} [<code>{}</code>] ha sido silenciado",
                            UserId(user_id)
                        ))
                        .parse_mode(ParseMode::Html)
                        .reply_to_message_id(msg.id)
                        .await?;

                    sleep(Duration::from_secs(60)).await;

                    bot.delete_message(msg.chat.id, ok.id).await?;
                }

                "jpg" => {
                    let ok = bot
                        .send_photo(msg.chat.id, InputFile::file(file_path))
                        .caption(format!(
                            "{username} [<code>{}</code>] ha sido silenciado",
                            UserId(user_id)
                        ))
                        .parse_mode(ParseMode::Html)
                        .reply_to_message_id(msg.id)
                        .await?;

                    sleep(Duration::from_secs(60)).await;

                    bot.delete_message(msg.chat.id, ok.id).await?;
                }

                _ => {
                    let err = bot
                        .send_message(msg.chat.id, "❌ No se pudo enviar el archivo")
                        .await?;

                    sleep(Duration::from_secs(60)).await;

                    bot.delete_message(msg.chat.id, err.id).await?;

                    bot.delete_message(msg.chat.id, msg.id).await?;
                }
            };
        }
    }

    Ok(())
}

/// # Errors
pub async fn send_random_meme_generator(bot: Bot, msg: Message) -> ResponseResult<()> {
    let file_names = [
        "1.mp4", "2.mp4", "3.mp4", "4.mp4", "5.mp4", "6.jpg", "7.jpg", "8.jpg", "9.jpg",
        "10.jpg", "11.jpg", "12.jpg", "13.jpg", "14.jpg", "15.jpg", "16.jpg", "17.jpg",
        "18.jpg", "19.jpg", "20.jpg", "21.jpg", "22.jpg", "23.jpg", "24.jpg", "25.jpg",
        "26.jpg", "27.jpg", "28.jpg", "29.jpg",
    ];

    // Esta es una función anónima, también conocida como "lambda" (linea 64).
    // Esta función toma un argumento de tipo `usize` y devuelve una cadena de
    // caracteres 'static str'
    let get_file_name = |index: usize| -> &'static str {
        file_names.get(index).map_or_else(
            || file_names.last().map_or("", |last_file| last_file),
            |file_name| file_name,
        )
    };

    // Español/Spanish

    // generamos un numero aleatorio

    // Inglés/English

    // generate a random number
    let mut rng: StdRng = SeedableRng::from_entropy();

    let random_number = rng.gen_range(0..file_names.len());

    // Inglés/English

    // imprimimos el resultado en consola con fines de debug
    // print the result in the console for debugging purposes
    println!("Resultado : {random_number}");

    // borramos el mensaje original
    // bot.delete_message(msg.chat.id, msg.id).await?;

    // Español/Spanish

    // Utiliza la función `format!` para crear una cadena de caracteres que contiene
    // una ruta de archivo. La ruta de archivo se compone de la cadena
    // "./assets/memes/" seguida del nombre de archivo obtenido de la función
    // `get_file_name`. El argumento proporcionado a `get_file_name` es
    // `random_number`, que es un número aleatorio generado previamente.

    // Inglés/English

    // Use the `format`! function to create a string that contains a file path.
    // The file path consists of the string "./assets/memes/" followed by the file
    // name obtained from the `get_file_name` function. The argument provided to
    // `get_file_name` is `random_number`, which is a randomly generated number
    // previously.
    let file_path = format!("./assets/memes/{}", get_file_name(random_number));

    // Español/Spanish

    // Utiliza la función `Path::new` para crear un objeto `Path` a partir de la
    // cadena de caracteres `file_path`. Luego, se llama al método `is_file` del
    // objeto `Path` para verificar si la ruta de archivo apunta a un archivo
    // existente.

    // Inglés/English

    // Use the `Path::new` function to create a `Path` object from the string
    // `file_path`. Then, call the `is_file` method of the `Path` object to
    // check if the file path points to an existing file.

    if Path::new(&file_path).is_file() {
        // If the file path points to an existing file, check if the file name ends with
        // ".mp4". Si la ruta de archivo apunta a un archivo existente, se
        // verifica si el nombre de archivo termina con ".mp4".
        if Path::new(&file_path)
            .extension()
            .map_or(false, |ext| ext.eq_ignore_ascii_case("mp4"))
        {
            // If the file name ends with ".mp4", send the video file to the chat using
            // the `send_video` method of the `bot` object. Si el nombre de
            // archivo termina con ".mp4", se envía el archivo de video al
            // chat utilizando el método `send_video` del objeto `bot`. El
            // argumento proporcionado a `send_video` es un objeto `InputFile`
            // que se crea a partir de la ruta de archivo.
            bot.send_video(msg.chat.id, InputFile::file(&file_path))
                .caption("Aquí tienes un meme de programación")
                .reply_to_message_id(msg.id)
                .await?; // The argument provided to `send_video` is an
                         // `InputFile` object that is created from the file
                         // path.
        } else {
            // If the file name does not end with ".mp4", assume it is an image file and
            // send it to the chat using the `send_photo` method of the `bot` object.
            // Si el nombre de archivo no termina con ".mp4", se asume que es un archivo
            // de imagen y se envía al chat utilizando el método `send_photo`
            // del objeto `bot`. El argumento proporcionado a `send_photo` es
            // un objeto `InputFile` que se crea a partir de la ruta de
            // archivo.
            bot.send_photo(msg.chat.id, InputFile::file(&file_path))
                .caption("Aquí tienes un meme de programación") // Use .caption() for send a message with the photo
                .reply_to_message_id(msg.id)
                .await?; // The argument provided to `send_photo` is an
                         // `InputFile` object that is created from the file
                         // path.
        }
    }

    Ok(())
}
