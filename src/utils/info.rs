use crate::dependencies::*;

pub async fn get_chat_member(bot: Bot, msg: Message) -> ResponseResult<()> {
    match msg.reply_to_message() {
        Some(replied) => {

            let user = if let Some(from) = replied.from() {
                from
            } else {
                // Send an error message and delete it after 5 seconds.
                let error_msg = bot.send_message(msg.chat.id, "❌ No se pudo obtener el usuario").reply_to_message_id(msg.id).await?;
                let error_msg_id = error_msg.id;

                sleep(Duration::from_secs(5)).await;
                bot.delete_message(msg.chat.id, error_msg_id).await?;
                bot.delete_message(msg.chat.id, msg.id).await?;

                return Ok(());
            };
/*
            let username_user = match user.clone().username {
                Some(username) => username,
                None => String::new(),
            };
            */

            let id_usuario = user.id;
            let first_name = user.first_name.clone();
            let username_user = user.clone().username.unwrap_or(first_name.clone());

            if username_user == first_name {
                bot.send_message(msg.chat.id, format!(
                    r#"
                <b>Nombre:</b> {}
                <b>Username:</b>@{} <b>
                ID: </b><code>{}</code>
            "#, first_name.clone(), username_user, id_usuario)).parse_mode(ParseMode::Html).await?;
            }

            bot.send_message(msg.chat.id, format!(
                r#"
                <b>Nombre:</b> {}
                <b>Username:</b>{} <b>
                ID: </b><code>{}</code>
            "#, first_name.clone(), username_user, id_usuario)).parse_mode(ParseMode::Html).await?;
        }

        None => {
            bot.send_message(msg.chat.id, "❌ No se ha respondido a ningún mensaje").await?;
        }
    }

    Ok(())
}