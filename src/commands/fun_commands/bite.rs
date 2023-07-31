use crate::commands::dependencies::*;

pub async fn send_bite(bot: Bot, msg: Message) -> ResponseResult<()> {
    let Some(text) = msg.text() else {
        return Ok(());
    };

    let (_, username_target) = match text.find(' ') {
        Some(index) => text.split_at(index),
        None => ("", text),
    };

    let first_name_author = &msg.from().unwrap().first_name;

    let username_author = msg.from().unwrap().username.as_ref().unwrap_or(first_name_author);

    let target_data = bot.get_chat_member(msg.chat.id, msg.from().unwrap().id).await?;
    let target_user_id = target_data.user.id;
    let mention = format!("<a href=\"tg://user?id={}\">{}</a>", target_user_id, username_target);

    let url = nekosbest::get(nekosbest::Category::Bite).await.unwrap().url;
    bot.send_animation(msg.chat.id, InputFile::url(url.parse().unwrap()))
        .caption(format!("{} Ha mordido a{}", username_author, mention))
        .parse_mode(ParseMode::Html)
        .await?;

    Ok(())
}

/*
let username_author = match msg.from().as_ref() {
    Some(user) => user.username.as_ref(),
    None => None,
};

let username_author = match username_author {
    Some(username) => username,
    None => "",
};
*/
