use crate::admin_commands::*;

pub async fn mute_user_admin(bot: MyBot, msg: Message) -> ResponseResult<()> {
    match msg.reply_to_message() {
        Some(replied) => {
            let user = replied.from().unwrap();
            println!("Usuario a Remover Mute: {}", &user.id);

            let chat_id = msg.chat.id;
            println!("Chat ID : {:?}", chat_id);

            let username_user = user.username.clone().unwrap_or_default();
            println!("Username : {:?}", username_user);

            let chat_member = bot.get_chat_member(chat_id, msg.from().unwrap().id).await?;

            let is_admin_or_owner =
                chat_member.status().is_administrator() || chat_member.status().is_owner();
            println!("Es admin o owner: {} \n", is_admin_or_owner);

            if is_admin_or_owner {
                bot.delete_message(chat_id, msg.id).await?;
                bot.restrict_chat_member(chat_id, user.id, ChatPermissions::empty())
                    .await?;
                bot.send_message(chat_id, format!("✅ @{} ha sido silenciado", username_user))
                    .await?;

                let mut rng: StdRng = SeedableRng::from_entropy();

                let file_names = ["1.gif", "2.gif", "3.gif", "4.gif", "5.jpg"];

                let random_number = rng.gen_range(0..=file_names.len() - 1);
                let file_path = format!("./assets/mute/{}", file_names[random_number]);
                let file_extension = file_path.split(".").last().unwrap_or("");

                match file_extension {
                    "gif" => {
                        bot.send_animation(chat_id, InputFile::file(file_path))
                            .await?
                    }
                    "jpg" => bot.send_photo(chat_id, InputFile::file(file_path)).await?,
                    _ => {
                        bot.send_message(chat_id, "❌ No se pudo enviar el archivo")
                            .await?
                    }
                };
            } else {
                bot.delete_message(chat_id, msg.id).await?;
                bot.send_message(chat_id, "❌ No tienes permisos para silenciar a un usuario")
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
                bot.restrict_chat_member(chat_id, UserId(user_id as u64), ChatPermissions::empty())
                    .await?;
                bot.delete_message(msg.chat.id, msg.id).await?;
                bot.send_message(msg.chat.id, format!("Silenciado \\[`{}`\\]", user_id))
                    .await?;

                let mut rng: StdRng = SeedableRng::from_entropy();
                let random_number = rng.gen_range(0..=6);

                let file_names = ["1.gif", "2.gif", "3.gif", "4.gif", "5.jpg"];

                let get_file_name = |index: usize| -> &'static str {
                    file_names
                        .get(index)
                        .unwrap_or_else(|| file_names.last().unwrap())
                };

                let file_path = format!("./assets/mute/{}", get_file_name(random_number));

                if file_path.ends_with(".gif") {
                    bot.send_animation(chat_id, InputFile::file(file_path))
                        .await?;
                } else {
                    bot.send_photo(chat_id, InputFile::file(file_path)).await?;
                }
            } else {
                bot.send_message(
                    msg.chat.id,
                    "❌ No tienes permisos para silenciar a un usuario",
                )
                    .await?;
            }
        }
    }

    Ok(())
}