use crate::commands::dependencies::*;

pub async fn send_happy(bot: Bot, msg: Message) -> ResponseResult<()> {
    let username_author = match msg.from().as_ref() {
        Some(user) => user.username.as_ref(),
        None => None,
    };

    let username_author = match username_author {
        Some(username) => username,
        None => "",
    };

    let url = nekosbest::get(nekosbest::Category::Happy).await.unwrap().url;
    bot.send_animation(msg.chat.id, InputFile::url(url.parse().unwrap()))
        .caption(format!("@{} Está Feliz", username_author))
        .parse_mode(ParseMode::Html)
        .await?;

    Ok(())
}