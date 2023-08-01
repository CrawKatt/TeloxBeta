use crate::dependencies::*;

/// # Errors
pub async fn get_chat_member(bot: Bot, msg: Message) -> ResponseResult<()> {
    match msg.reply_to_message() {
        Some(replied) => {
            let Some(user) = replied.from() else {
                // Send an error message and delete it after 5 seconds.
                let error_msg = bot
                    .send_message(msg.chat.id, "❌ No se pudo obtener el usuario")
                    .reply_to_message_id(msg.id)
                    .await?;
                let error_msg_id = error_msg.id;

                sleep(Duration::from_secs(5)).await;
                bot.delete_message(msg.chat.id, error_msg_id).await?;
                bot.delete_message(msg.chat.id, msg.id).await?;

                return Ok(());
            };

            let user_copy = user.clone();
            let id_usuario = user.id;
            let first_name = user.first_name.clone();
            let first_name_copy = first_name.clone();
            let username_user = user_copy.username.unwrap_or(first_name);

            if username_user == first_name_copy {
                bot.send_message(
                    msg.chat.id,
                    format!(
                        r#"
                <b>Nombre:</b> {first_name_copy}
                <b>Username:</b>@{username_user} <b>
                ID: </b><code>{id_usuario}</code>
            "#
                    ),
                )
                .await?;
            }

            bot.send_message(
                msg.chat.id,
                format!(
                    r#"
                <b>Nombre:</b> {first_name_copy}
                <b>Username:</b>{username_user} <b>
                ID: </b><code>{id_usuario}</code>
            "#
                ),
            )
            .parse_mode(ParseMode::Html)
            .await?;
        }

        None => {
            bot.send_message(msg.chat.id, "❌ No se ha respondido a ningún mensaje").await?;
        }
    }

    Ok(())
}
