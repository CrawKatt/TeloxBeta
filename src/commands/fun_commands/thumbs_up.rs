use crate::commands::dependencies::*;

/// # Errors
/// # Panics
pub async fn send_thumbs_up(bot: Bot, msg: Message) -> ResponseResult<()> {

    let Some(text) = msg.text() else {

        return Ok(());
    };

    let (_, username_target) = text.find(' ').map_or(("", text), |index| text.split_at(index));

    let username_author = msg.from().as_ref().and_then(|user| user.username.as_ref());

    let username_author = username_author.map_or("", |username| username);

    let url = nekosbest::get(nekosbest::Category::ThumbsUp)
        .await
        .unwrap()
        .url;

    bot.send_animation(msg.chat.id, InputFile::url(url.parse().unwrap()))
        .caption(format!("@{} Apoya a{}", username_author, username_target))
        .parse_mode(ParseMode::Html)
        .await?;

    Ok(())
}
