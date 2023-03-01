use crate::admin_commands::*;
use crate::commands::{get_user_id_by_arguments};

pub async fn unban_user(bot: Bot, msg: Message) -> ResponseResult<()> {
    match msg.reply_to_message() {
        Some(replied) => {
            let user = if let Some(from) = replied.from() {
                from
            } else {
                let error_msg = bot.send_message(msg.chat.id, "❌ No se pudo obtener el usuario").await?;
                let error_msg_id = error_msg.id;

                sleep(Duration::from_secs(5)).await;
                bot.delete_message(msg.chat.id, error_msg_id).await?;
                bot.delete_message(msg.chat.id, msg.id).await?;

                return Ok(());
            };

            if let Some(from) = msg.from() {

                let username_user = match user.clone().username {
                    Some(username) => username,
                    None => String::new(),
                };

                let chat_member = bot.get_chat_member(msg.chat.id, from.id).await?;
                let is_admin_or_owner = chat_member.status().is_administrator() || chat_member.status().is_owner();

                if is_admin_or_owner {

                    bot.unban_chat_member(msg.chat.id, user.id).await?;
                    let ok_unban = bot.send_message(msg.chat.id, format!("✅ @{} desbaneado", username_user)).await?;

                    bot.send_video(msg.chat.id, InputFile::file("./assets/unban/1.mp4")).await?;

                    sleep(Duration::from_secs(5)).await;
                    bot.delete_message(msg.chat.id, ok_unban.id).await?;
                    bot.delete_message(msg.chat.id, msg.id).await?;

                } else {
                    let err = bot.send_message(msg.chat.id, "❌ No tienes permisos para remover el ban a un usuario.").await?;
                    bot.delete_message(msg.chat.id, err.id).await?;
                }

            }

        }

        None => {
            get_user_id_by_arguments(bot, msg).await?;
        }

    }

    Ok(())
}