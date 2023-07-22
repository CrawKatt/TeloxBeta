use crate::commands::admin_commands::*;

pub async fn send_sad(bot: Bot, msg: Message) -> ResponseResult<()> {
    let username_author = match msg.from().as_ref() {
        Some(user) => user.username.as_ref(),
        None => None,
    };

    let username_author = match username_author {
        Some(username) => username,
        None => "",
    };

    let random_pat = nekosbest::get(nekosbest::Category::Cry).await.unwrap().url;
    bot.send_animation(msg.chat.id, InputFile::url(random_pat.parse().unwrap()))
        .caption(format!("@{} Está triste", username_author))
        .parse_mode(ParseMode::Html)
        .await?;

    Ok(())
}