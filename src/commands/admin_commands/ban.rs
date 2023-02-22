use crate::admin_commands::*;

pub async fn ban_user(bot: Bot, msg: Message) -> ResponseResult<()> {
    // The function takes a bot and a message object, and returns a Result.
    match msg.reply_to_message() {
        // Check if the message is a reply to another message.
        Some(replied) => {
            // If it is, extract the user from the replied message.
            // If the user cannot be extracted, send an error message.
            let user = if let Some(from) = replied.from() {
                from
            } else {
                // Manejar el caso de que `from()` devuelva `None`
                println!("No se pudo obtener el usuario {:?}", replied.new_chat_members());
                bot.send_message(msg.chat.id, "❌ No se pudo obtener el usuario").await?;
                bot.delete_message(msg.chat.id, msg.id).await?;
                return Ok(());
            };

            // Get the chat ID and user ID, and check if the person using the command is an admin or owner.
            let chat_id = msg.chat.id;
            println!("Chat id: {}", chat_id);
            log::info!("Chat id: {}", chat_id);

            let id_usuario = user.id;
            println!("Id usuario: {}", id_usuario);

            let username_user = user.username.clone().unwrap_or_default();
            println!("Username: {}", username_user);

            let chat_member = bot.get_chat_member(chat_id, msg.from().unwrap().id).await?;

            // If the user is an admin or owner, ban the user and send a message to the chat.
            // Also send a random GIF or MP4 file from the "./assets/ban/" folder.
            let is_admin_or_owner =
                chat_member.status().is_administrator() || chat_member.status().is_owner();
            println!("Es admin o owner: {}", is_admin_or_owner);

            if is_admin_or_owner {
                bot.delete_message(chat_id, msg.id).await?;
                bot.ban_chat_member(chat_id, user.id).await?;
                bot.send_message(
                    chat_id,
                    format!("✅ @{} \\[`{}`\\] baneado", username_user, id_usuario),
                )
                    .await?;

                // Choose a random ban animation to send.
                let mut rng: StdRng = SeedableRng::from_entropy();

                // generate a random number from 0 to 14
                let random_number = rng.gen_range(0..=14);

                let file_names = [
                    "1.gif", "2.gif", "3.gif", "4.gif", "5.gif", "6.gif", "7.gif", "8.gif",
                    "9.gif", "10.gif", "11.gif", "12.mp4", "13.mp4", "14.mp4", "15.mp4",
                ];

                let get_file_name = |index: usize| -> &'static str {
                    file_names
                        .get(index)
                        .unwrap_or_else(|| file_names.last().unwrap())
                };

                let file_path = format!("./assets/ban/{}", get_file_name(random_number));

                match file_path.ends_with(".gif") {
                    true => {
                        bot.send_animation(chat_id, InputFile::file(file_path))
                            .await?
                    }
                    false => bot.send_video(chat_id, InputFile::file(file_path)).await?,
                };
            } else {
                // If the user is not an admin or owner, send an error message.
                bot.delete_message(chat_id, msg.id).await?;
                bot.send_message(chat_id, "❌ No tienes permisos para usar este comando").await?;
            };
        }
        None => {
            // If the message is not a reply, extract the user ID from the command's arguments.
            // Check if the person using the command is an admin or owner.

            // extract the text content of the message
            let text = &msg.text().unwrap();

            // get the arguments after the command trigger
            let (_, arguments) = text.split_at(text.find(' ').unwrap_or(text.len()));

            // extract the user ID from the arguments
            let user_id = arguments.trim().parse::<i64>().unwrap();

            // get the ID of the chat where the message was sent
            let chat_id = msg.chat.id;

            // get information about the user who sent the message
            let chat_member = bot.get_chat_member(chat_id, msg.from().unwrap().id).await?;

            // check if the user is an admin or owner of the chat
            let is_admin_or_owner = chat_member.status() == ChatMemberStatus::Administrator || chat_member.status() == ChatMemberStatus::Owner;

            // If the user is an admin or owner, ban the target user and send a ban message.
            if is_admin_or_owner {
                // ban the target user
                bot.ban_chat_member(chat_id, UserId(user_id as u64)).await?;

                // delete the command message
                bot.delete_message(msg.chat.id, msg.id).await?;

                // send a ban message with the target user's ID
                bot.send_message(msg.chat.id, format!("✅ Baneado \\[`{}`\\]", user_id)).await?;

                // Choose a random ban animation to send.
                let mut rng: StdRng = SeedableRng::from_entropy();

                // generate a random number from 0 to 14
                let random_number = rng.gen_range(0..=14);

                // list of possible ban animation file names
                let file_names = [
                    "1.gif", "2.gif", "3.gif", "4.gif", "5.gif", "6.gif", "7.gif", "8.gif",
                    "9.gif", "10.gif", "11.gif", "12.mp4", "13.mp4", "14.mp4", "15.mp4",
                ];

                // helper function to get the file name at a given index
                let get_file_name = |index: usize| -> &'static str {
                    file_names
                        .get(index)
                        .unwrap_or_else(|| file_names.last().unwrap())
                };

                // choose a ban animation file based on the random number
                let file_path = format!("./assets/ban/{}", get_file_name(random_number));

                // check if the file is a GIF or video
                match file_path.ends_with(".gif") {
                    true  => bot.send_animation(chat_id, InputFile::file(file_path)).await?,
                    false => bot.send_video(chat_id, InputFile::file(file_path)).await?,
                };
                // If the user is not an admin or owner, delete the command message and send an error message.
            } else {
                // delete the command message
                bot.delete_message(msg.chat.id, msg.id).await?;

                // send an error message
                bot.send_message(msg.chat.id, "❌ No tienes permisos para banear a un usuario", ).await?;
            };
        }
    }

    Ok(())
}

/*
╔═════════════════════════════════════════════════════╗
║    || - || Desarrollado por @CrawKatt || - ||       ║
║    --| https://github.com/CrawKatt/TeloxBeta |--    ║
╚═════════════════════════════════════════════════════╝
*/