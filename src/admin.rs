//use std::time::Duration;
use crate::comandos;
use crate::ResponseResult;

use rand::rngs::StdRng;
use rand::SeedableRng;
use rand::Rng;

use teloxide::types::InputFile;
use teloxide::types::ChatPermissions;
use teloxide::prelude::Requester;
use teloxide_core::prelude::{UserId};
use teloxide_core::types::{ChatMemberStatus, Message};


// Banear a un usuario respondiendo un mensaje
pub async fn ban_user(bot: comandos::MyBot, msg: Message) -> ResponseResult<()> {
    let user = msg.reply_to_message().and_then(|replied| replied.from()).unwrap();
    println!("Usuario a banear: {}", user.id);

    let chat_id = msg.chat.id;
    println!("Chat id: {}", chat_id);

    let username_user = user.username.clone().unwrap_or_default();
    println!("Username: {}", username_user);

    let chat_member = bot.get_chat_member(chat_id, msg.from().unwrap().id).await?;

    let is_admin_or_owner = chat_member.status().is_administrator() || chat_member.status().is_owner();
    println!("Es admin o owner: {}", is_admin_or_owner);

    if is_admin_or_owner {
        bot.delete_message(chat_id, msg.id).await?;
        bot.ban_chat_member(chat_id, user.id).await?;
        bot.send_message(chat_id, format!("@{} ha sido baneado", username_user), ).await?;

        let mut rng: StdRng = SeedableRng::from_entropy();
        let random_number = rng.gen_range(0..=14);

        let file_names = [
            "1.gif", "2.gif", "3.gif", "4.gif", "5.gif",
            "6.gif", "7.gif", "8.gif", "9.gif", "10.gif",
            "11.gif", "12.mp4", "13.mp4", "14.mp4", "15.mp4",
        ];

        let get_file_name = |index: usize| -> &'static str {
            file_names.get(index).unwrap_or_else(|| file_names.last().unwrap())
        };

        let file_path = format!("./assets/ban/{}", get_file_name(random_number));

        if file_path.ends_with(".gif") {
            bot.send_animation(chat_id, InputFile::file(file_path)).await?;
        } else {
            bot.send_video(chat_id, InputFile::file(file_path)).await?;
        }
    } else {
        bot.delete_message(chat_id, msg.id).await?;
        bot.send_message(chat_id, "No tienes permisos para usar este comando",).await?;
    }
    Ok(())
}

pub async fn unban_user(bot: comandos::MyBot, msg: Message) -> ResponseResult<()> {
    match msg.reply_to_message() {

        Some(replied) => {

            let user = replied.from().unwrap();
            println!("Usuario a desbanear: {}", user.id);

            let chat_id = msg.chat.id;
            println!("Chat id: {}", chat_id);

            let username_user = user.username.clone().unwrap_or_default();
            println!("Username: {}", username_user);

            let chat_member = bot.get_chat_member(chat_id, msg.from().unwrap().id).await?;

            let is_admin_or_owner = chat_member.status().is_administrator() || chat_member.status().is_owner();
            println!("Es admin u owner: {} \n", is_admin_or_owner);


            if is_admin_or_owner {
                bot.delete_message(chat_id, msg.id).await?;
                bot.unban_chat_member(chat_id, user.id).await?;
                bot.send_message(chat_id, format!("@{} ha sido desbaneado", username_user)).await?;
                bot.send_video(chat_id, InputFile::file("./assets/unban/1.mp4")).await?;

            } else {
                bot.delete_message(chat_id, msg.id).await?;
                bot.send_message(chat_id, "No tienes permisos para remover el ban a un usuario.").await?;
            }

        }

        None => {
            bot.delete_message(msg.chat.id, msg.id).await?;
            bot.send_message(msg.chat.id, "Usa este comando respondiendo a otro mensaje").await?;
        }

    }

    Ok(())

}

// silenciar a un usuario por tiempo indeterminado pero solo si el usuario que emplea el comando es administrador
pub async fn mute_user_admin(bot: comandos::MyBot, msg: Message) -> ResponseResult<()> {
    let user = msg.reply_to_message().unwrap().from().unwrap();

    let chat_id = msg.chat.id;
    println!("Chat ID : {:?}", chat_id);

    let username_user = user.username.clone().unwrap_or_default();
    println!("Username : {:?}", username_user);

    let chat_member = bot.get_chat_member(chat_id, msg.from().unwrap().id).await?;

    let is_admin_or_owner = chat_member.status().is_administrator() || chat_member.status().is_owner();
    println!("Es admin o owner: {} \n", is_admin_or_owner);

    if is_admin_or_owner {
        bot.delete_message(chat_id, msg.id).await?;
        bot.restrict_chat_member(chat_id, user.id, ChatPermissions::empty()).await?;
        bot.send_message(chat_id, format!("@{} ha sido silenciado", username_user)).await?;

        let mut rng: StdRng = SeedableRng::from_entropy();

        let file_names = [
            "1.gif", "2.gif", "3.gif", "4.gif", "5.jpg",
        ];

        let random_number = rng.gen_range(0..=file_names.len() - 1);
        let file_path = format!("./assets/mute/{}", file_names[random_number]);
        let file_extension = file_path.split(".").last().unwrap_or("");

        match file_extension {
            "gif" => bot.send_animation(chat_id, InputFile::file(file_path)).await?,
            "jpg" => bot.send_photo(chat_id, InputFile::file(file_path)).await?,
            _ => bot.send_message(chat_id, "No se pudo enviar el archivo").await?,
        };

    } else {
        bot.delete_message(chat_id, msg.id).await?;
        bot.send_message(chat_id, "No tienes permisos para silenciar a un usuario").await?;
    };

    Ok(())
}

// remover silencio a un usuario
pub async fn unmute_user(bot: comandos::MyBot, msg: Message) -> ResponseResult<()> {

    let user = msg.reply_to_message().unwrap().from().unwrap();
    println!("Info : {:?}", user);

    let chat = msg.chat.id;
    println!("Chat ID : {:?}", chat);

    let usuario = user.clone().first_name;
    println!("Nombre : {:?}", usuario);

    let username_user = user.clone().username;
    println!("@username : {:?}", username_user);

    let chat_member = bot.get_chat_member(msg.chat.id, msg.from().unwrap().id).await?;

    let user_id = chat_member.user.id;
    println!("ID usuario : {:?}", user_id);

    if chat_member.status() == ChatMemberStatus::Administrator || chat_member.status() == ChatMemberStatus::Owner || chat_member.status() == ChatMemberStatus::Administrator {
        bot.delete_message(msg.chat.id, msg.id).await?;
        bot.restrict_chat_member(msg.chat.id, user.id, ChatPermissions::all()).await?;
        bot.send_message(msg.chat.id, format!("@{} ya no está silenciado", username_user.unwrap())).await?;
        println!("Silencio Removido : {}", user.first_name);
        bot.send_video(msg.chat.id, InputFile::file("./assets/unmute/unmute.mp4")).await?;

    } else {
        bot.delete_message(msg.chat.id, msg.id).await?;
        bot.send_message(msg.chat.id, "No tienes permisos para remover el silencio a un usuario").await?;
    };

    Ok(())

}

pub async fn get_chat_member(bot: comandos::MyBot, msg: Message) -> ResponseResult<()> {
    let usuario = msg.reply_to_message().unwrap().from().unwrap();
    println!("Info : {:?}", usuario);

    let user_id = usuario.id;
    println!("ID usuario : {:?}", user_id);

    bot.send_message(msg.chat.id, format!("ID usuario : \\-{user_id}")).await?;

    Ok(())
}

// banear a un usuario mediante su @username
pub async fn ban_user_id(bot: comandos::MyBot, msg: Message, user_id: i64) -> ResponseResult<()> {

    let chat_id = msg.chat.id;
    let chat_member = bot.get_chat_member(chat_id, msg.from().unwrap().id).await?;

    if chat_member.status() == ChatMemberStatus::Administrator || chat_member.status() == ChatMemberStatus::Owner || chat_member.status() == ChatMemberStatus::Administrator {
        bot.ban_chat_member(chat_id, UserId(user_id as u64)).await?;
        bot.delete_message(msg.chat.id, msg.id).await?;
        bot.send_message(msg.chat.id, "El usuario ha sido baneado del grupo").await?;
    } else {
        bot.send_message(msg.chat.id, "No tienes permisos para banear a un usuario").await?;
    }

    Ok(())
}

