use crate::commands::dependencies::*;

/// # Errors
/// # Panics
pub async fn send_blush(bot: Bot, msg: Message) -> ResponseResult<()> {
    let username_author = msg.from().as_ref().and_then(|user| user.username.as_ref());
    let username_author = username_author.map_or("", |username| username);

    let url = nekosbest::get(nekosbest::Category::Blush).await.unwrap().url;
    bot.send_animation(msg.chat.id, InputFile::url(url.parse().unwrap()))
        .caption(format!("@{username_author} Se sonroja"))
        .parse_mode(ParseMode::Html)
        .await?;

    Ok(())
}
