use crate::admin_commands::*;

pub async fn ban_user(bot: MyBot, msg: Message) -> ResponseResult<()> {
    match msg.reply_to_message() {
        Some(replied) => {
            let user = replied.from().unwrap();
            println!("Usuario a banear: {}", user.id);

            let chat_id = msg.chat.id;
            println!("Chat id: {}", chat_id);

            let id_usuario = user.id;
            println!("Id usuario: {}", id_usuario);

            let username_user = user.username.clone().unwrap_or_default();
            println!("Username: {}", username_user);

            let chat_member = bot.get_chat_member(chat_id, msg.from().unwrap().id).await?;

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

                let mut rng: StdRng = SeedableRng::from_entropy();
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
                bot.delete_message(chat_id, msg.id).await?;
                bot.send_message(chat_id, "❌ No tienes permisos para usar este comando")
                    .await?;
            };
        }
        None => {
            let text = &msg.text().unwrap();
            let (_, arguments) = text.split_at(text.find(' ').unwrap_or(text.len()));
            let user_id = arguments.trim().parse::<i64>().unwrap();
            let chat_id = msg.chat.id;
            let chat_member = bot.get_chat_member(chat_id, msg.from().unwrap().id).await?;
            let is_admin_or_owner = chat_member.status() == ChatMemberStatus::Administrator
                || chat_member.status() == ChatMemberStatus::Owner;

            if is_admin_or_owner {
                bot.ban_chat_member(chat_id, UserId(user_id as u64)).await?;
                bot.delete_message(msg.chat.id, msg.id).await?;
                bot.send_message(msg.chat.id, format!("✅ Baneado \\[`{}`\\]", user_id))
                    .await?;

                let mut rng: StdRng = SeedableRng::from_entropy();
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
                bot.delete_message(msg.chat.id, msg.id).await?;
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