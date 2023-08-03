use crate::dependencies::*;

/// # Errors
pub async fn no_username_found(
    bot: Bot,
    msg: Message,
    username: &str,
) -> ResponseResult<()> {
    let err = bot
        .send_message(msg.chat.id, format!("{NOT_USERNAME_FOUND_404} {username}"))
        .await?;

    delete_message_timer(bot, msg.clone(), err.id, msg.id, 5);

    Ok(())
}

/// # Errors
pub async fn no_arguments(bot: Bot, msg: Message) -> ResponseResult<()> {
    let err = bot.send_message(msg.chat.id, NOT_ID_PROVIDED_404).await?;

    delete_message_timer(bot, msg.clone(), err.id, msg.id, 5);

    println!("{NOT_ID_PROVIDED_404} {msg:#?}");

    Ok(())
}

/// # Errors
pub async fn permissions_denied(bot: Bot, msg: Message) -> ResponseResult<()> {
    let err = bot.send_message(msg.chat.id, PERMISSIONS_DENIED).await?;
    delete_message_timer(bot, msg.clone(), err.id, msg.id, 5);

    Ok(())
}

/// # Errors
pub async fn id_or_username_not_valid(bot: Bot, msg: Message) -> ResponseResult<()> {
    let err = bot
        .send_message(msg.chat.id, ID_OR_USERNAME_NOT_VALID)
        .await?;

    delete_message_timer(bot, msg.clone(), err.id, msg.id, 5);

    Ok(())
}

/// # Errors
pub async fn user_is_not_muted(
    bot: Bot,
    msg: Message,
    username: &str,
    user_id: u64,
) -> ResponseResult<()> {
    let err = bot
        .send_message(
            msg.chat.id,
            format!("❌ @{username} [<code>{user_id}</code>] No está silenciado"),
        )
        .await?;

    delete_message_timer(bot, msg.clone(), err.id, msg.id, 5);

    Ok(())
}

/// # Errors
pub async fn already_banned(
    bot: Bot,
    msg: Message,
    user_id: UserId,
    username: String,
) -> ResponseResult<()> {
    let err = bot
        .send_message(
            msg.chat.id,
            format!("❌ @{username} [<code>{user_id}</code>] {ALREADY_BANNED}"),
        )
        .await?;

    delete_message_timer(bot, msg.clone(), err.id, msg.id, 5);

    Ok(())
}

/// # Errors
pub async fn not_banned(
    bot: Bot,
    msg: Message,
    username_user: String,
) -> ResponseResult<()> {
    let err = bot
        .send_message(msg.chat.id, format!("❌ @{username_user} {NOT_BANNED}"))
        .reply_to_message_id(msg.id)
        .await?;

    delete_message_timer(bot, msg.clone(), err.id, msg.id, 5);

    Ok(())
}

/// # Errors
pub async fn user_not_found(bot: Bot, msg: Message) -> ResponseResult<()> {
    let err = bot.send_message(msg.chat.id, USER_NOT_FOUND).await?;

    delete_message_timer(bot, msg.clone(), err.id, msg.id, 5);

    Ok(())
}

pub fn delete_message_timer(
    bot: Bot,
    msg: Message,
    ok_or_err: MessageId,
    msg_id: MessageId,
    secs: u64,
) {
    tokio::spawn(async move {
        sleep(Duration::from_secs(secs)).await;
        bot.delete_message(msg.chat.id, ok_or_err)
            .await
            .unwrap_or_default();
        bot.delete_message(msg.chat.id, msg_id)
            .await
            .unwrap_or_default();
    });
}
