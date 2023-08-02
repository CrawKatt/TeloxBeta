use crate::dependencies::*;

/// # Errors
pub async fn no_username_found(bot: Bot, msg: Message, username: &str) -> ResponseResult<()> {

    let err = bot
        .send_message(msg.chat.id, format!("{NOT_USERNAME_FOUND_404} {username}"))
        .await?;

    tokio::spawn(async move {
        sleep(Duration::from_secs(60)).await;
        bot.delete_message(msg.chat.id, err.id).await.unwrap_or_default();
        bot.delete_message(msg.chat.id, msg.id).await.unwrap_or_default();
    });

    Ok(())
}

/// # Errors
pub async fn no_arguments(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_message(
        msg.chat.id,
        NOT_ID_PROVIDED_404,
    )
        .await?;

    bot.delete_message(msg.chat.id, msg.id).await?;

    println!("{NOT_ID_PROVIDED_404} {msg:#?}");

    Ok(())
}

/// # Errors
pub async fn permissions_denied(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, PERMISSIONS_DENIED)
        .await?;

    tokio::spawn(async move {
        sleep(Duration::from_secs(5)).await;
        bot.delete_message(msg.chat.id, msg.id).await.unwrap_or_default();
    });

    Ok(())
}

/// # Errors
pub async fn id_or_username_not_valid(bot: Bot, msg: Message) -> ResponseResult<()> {
    let err = bot
        .send_message(
            msg.chat.id,
            ID_OR_USERNAME_NOT_VALID,
        )
        .await?;

    tokio::spawn(async move {
        sleep(Duration::from_secs(60)).await;
        bot.delete_message(msg.chat.id, err.id).await.unwrap_or_default();
        bot.delete_message(msg.chat.id, msg.id).await.unwrap_or_default();
    });

    Ok(())
}

/// # Errors
pub async fn user_is_not_muted(bot: Bot, msg: Message, username: &str, user_id: u64) -> ResponseResult<()> {

    let err = bot
        .send_message(
            msg.chat.id,
            format!("❌ @{username} [<code>{user_id}</code>] No está silenciado"),
        )
        .await?;

    tokio::spawn(async move {
        sleep(Duration::from_secs(60)).await;
        bot.delete_message(msg.chat.id, err.id).await.unwrap_or_default();
        bot.delete_message(msg.chat.id, msg.id).await.unwrap_or_default();
    });

    Ok(())
}

/// # Errors
pub async fn already_banned(
    bot: Bot,
    msg: Message,
    user_id: u64,
    username: String,
) -> ResponseResult<()> {

    let err = bot
        .send_message(
            msg.chat.id,
            format!("❌ @{username} [<code>{user_id}</code>] {ALREADY_BANNED}"),
        )
        .await?;

    tokio::spawn(async move {
        sleep(Duration::from_secs(60)).await;
        bot.delete_message(msg.chat.id, err.id).await.unwrap_or_default();
        bot.delete_message(msg.chat.id, msg.id).await.unwrap_or_default();
    });

    Ok(())
}