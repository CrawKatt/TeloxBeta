use crate::dependencies::*;

/// # Errors
/// # Panics
pub async fn ban_user(bot: Bot, msg: Message) -> ResponseResult<()> {
    match msg.reply_to_message() {
        Some(replied) => {
            let Some(user) = replied.from() else {
                let err = bot
                    .send_message(msg.chat.id, USER_NOT_FOUND)
                    .reply_to_message_id(msg.id)
                    .await?;
                delete_message_timer(bot, msg.clone(), err.id, msg.id, 5);

                return Ok(())
            };

            let user_id = user.id;

            if let Some(from) = msg.from() {
                let username_user = user
                    .clone()
                    .username
                    .map_or_else(String::new, |username| username);

                let is_admin_or_owner = bot
                    .get_chat_member(msg.chat.id, from.id)
                    .await?
                    .is_admin_or_owner();

                let chat_member = bot.get_chat_member(msg.chat.id, user.id).await?;

                if is_admin_or_owner {
                    if chat_member.status().is_banned() {
                        already_banned(bot.clone(), msg.clone(), user_id, username_user)
                            .await?;

                        return Ok(())
                    }

                    bot.ban_chat_member(msg.chat.id, user.id).await?;

                    let mut rng: StdRng = SeedableRng::from_entropy();

                    let random_number = rng.gen_range(0..=14);

                    let file_names = [
                        "1.gif", "2.gif", "3.gif", "4.gif", "5.gif", "6.gif", "7.gif",
                        "8.gif", "9.gif", "10.gif", "11.gif", "12.mp4", "13.mp4",
                        "14.mp4",
                    ];

                    let get_file_name = |index: usize| -> &'static str {
                        file_names
                            .get(index)
                            .unwrap_or_else(|| file_names.last().unwrap())
                    };

                    let file_path =
                        format!("./assets/ban/{}", get_file_name(random_number));

                    if Path::new(&file_path)
                        .extension()
                        .map_or(false, |ext| ext.eq_ignore_ascii_case("gif"))
                    {
                        bot.send_animation(msg.chat.id, InputFile::file(file_path))
                            .caption(format!(
                                "✅ @{username_user} [<code>{user_id}</code>] baneado",
                            ))
                            .reply_to_message_id(msg.id)
                            .await?;
                    } else {
                        bot.send_video(msg.chat.id, InputFile::file(file_path))
                            .caption(format!(
                                "✅ @{username_user} <code>[{user_id}</code>] baneado",
                            ))
                            .reply_to_message_id(msg.id)
                            .await?;
                    }
                } else {
                    permissions_denied(bot, msg.clone()).await?;
                };
            }
        }
        None => {
            get_user_id_by_arguments(bot, msg).await?;
        }
    }

    Ok(())
}

// ╔═════════════════════════════════════════════════════╗
// ║    || - || Desarrollado por @CrawKatt || - ||       ║
// ║    --| https://github.com/CrawKatt/TeloxBeta |--    ║
// ╚═════════════════════════════════════════════════════╝
