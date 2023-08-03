use crate::dependencies::*;

/// # Errors
/// # Panics
pub async fn unban_user(bot: Bot, msg: Message) -> ResponseResult<()> {
    let bot_copy = bot.clone();
    let msg_copy = msg.clone();

    let Some(replied) = msg_copy.reply_to_message() else {
        Box::pin(get_user_id_by_username(bot, msg)).await?;
        return Ok(())
    };

    let Some(user) = replied.from() else {
        user_not_found(bot, msg).await?;
        return Ok(())
    };

    let Some(from) = msg.from() else {
        return Ok(())
    };

    let username_user = user
        .username
        .clone()
        .map_or_else(String::new, |username| username);

    let username_user_copy = username_user.clone();

    let is_admin_or_owner = bot
        .get_chat_member(msg.chat.id, from.id)
        .await?
        .is_admin_or_owner();

    if !is_admin_or_owner {
        permissions_denied(bot, msg).await?;
        return Ok(())
    }

    let chat_member = bot.get_chat_member(msg.chat.id, user.id).await?;

    if !chat_member.status().is_banned() {
        not_banned(bot, msg, username_user).await?;
        return Ok(())
    }

    bot_copy
        .unban_chat_member(msg_copy.chat.id, user.id)
        .await?;

    let ok = bot_copy
        .send_video(msg_copy.chat.id, InputFile::file("./assets/unban/1.mp4"))
        .caption(format!("✅ @{username_user_copy} desbaneado",))
        .reply_to_message_id(msg_copy.id)
        .await?;

    delete_message_timer(bot_copy, msg_copy.clone(), ok.id, msg_copy.id, 60);

    Ok(())
}

// match msg_copy.reply_to_message() {
// Some(replied) => {
// let Some(user) = replied.from() else {
// user_not_found(bot, msg).await?;
// return Ok(())
// };
//
// let Some(from) = msg.from() else {
// return Ok(())
// };
//
// let username_user = user
// .username
// .clone()
// .map_or_else(String::new, |username| username);
//
// let username_user_copy = username_user.clone();
//
// let is_admin_or_owner = bot
// .get_chat_member(msg.chat.id, from.id)
// .await?
// .is_admin_or_owner();
//
// if !is_admin_or_owner {
// permissions_denied(bot, msg).await?;
// return Ok(())
// }
//
// let chat_member = bot.get_chat_member(msg.chat.id, user.id).await?;
//
// if !chat_member.status().is_banned() {
// not_banned(bot, msg, username_user).await?;
// return Ok(())
// }
//
// bot_copy
// .unban_chat_member(msg_copy.chat.id, user.id)
// .await?;
//
// let ok = bot_copy
// .send_video(msg_copy.chat.id, InputFile::file("./assets/unban/1.mp4"))
// .caption(format!("✅ @{username_user_copy} desbaneado",))
// .reply_to_message_id(msg_copy.id)
// .await?;
//
// delete_message_timer(bot_copy, msg_copy.clone(), ok.id, msg_copy.id, 60);
//
// }
//
// None => {
// Box::pin(get_user_id_by_arguments_for_unban(bot, msg)).await?;
// }
// }
//
// Ok(())
// }

/// # Errors
async fn get_user_id_by_arguments_for_unban(
    bot: Bot,
    msg: Message,
) -> ResponseResult<()> {
    let bot_copy = bot.clone();
    let msg_copy = msg.clone();

    let text = msg.text().unwrap_or_default();

    // let Some(text) = msg.text() else {
    // return Ok(())
    // };

    // get the arguments after the command trigger
    let (_, arguments) = text
        .find(' ')
        .map_or(("", text), |index| text.split_at(index));

    // check if the arguments are empty
    if !arguments.is_empty() {
        if !arguments.contains('@') {
            let Ok(user_id) = arguments.trim().parse::<u64>() else {
                id_or_username_not_valid(bot, msg).await?;
                return Ok(())
            };

            let Some(from) = msg.from() else {
                println!(
                    "❌ No se pudo obtener el usuario que envió el mensaje {msg:#?}"
                );
                return Ok(())
            };

            let is_admin_or_owner = bot
                .get_chat_member(msg.chat.id, from.id)
                .await?
                .is_admin_or_owner();

            let false = !is_admin_or_owner else {
                permissions_denied(bot, msg).await?;
                return Ok(())
            };

            let chat_member = bot.get_chat_member(msg.chat.id, UserId(user_id)).await?;

            let username = chat_member
                .user
                .username
                .clone()
                .unwrap_or_else(|| "no username".to_string());

            let ChatMemberStatus::Banned { .. } = chat_member.status() else {
                not_banned(bot, msg, username).await?;
                return Ok(())
            };

            bot.unban_chat_member(msg.chat.id, UserId(user_id)).await?;

            let unban_ok = bot
                .send_message(
                    msg.chat.id,
                    format!("✅ @{username} [<code>{user_id}</code>] Ya no está Baneado"),
                )
                .await?;

            ban_animation_generator(bot.clone(), msg.clone()).await?;

            tokio::spawn(async move {
                sleep(Duration::from_secs(5)).await;
                bot_copy
                    .delete_message(msg_copy.chat.id, unban_ok.id)
                    .await
                    .unwrap_or_default();
                bot_copy
                    .delete_message(msg_copy.chat.id, msg_copy.id)
                    .await
                    .unwrap_or_default();
            });

            Box::pin(get_user_id_by_username(bot.clone(), msg)).await?;
        }

        let Some(from) = msg_copy.from() else {
            return Ok(())
        };

        let is_admin_or_owner = bot
            .get_chat_member(msg_copy.clone().chat.id, from.id)
            .await?
            .is_admin_or_owner();

        let true = is_admin_or_owner else {
            permissions_denied(bot, msg_copy.clone()).await?;
            return Ok(())
        };

        return Ok(())
    }
    no_arguments(bot, msg).await?;

    Ok(())
}
