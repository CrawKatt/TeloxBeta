use crate::comandos;
use crate::Message;
use crate::ResponseResult;

use rand::rngs::StdRng;
use rand::SeedableRng;
use rand::Rng;

use teloxide::types::InputFile;
use teloxide::types::ChatPermissions;
use teloxide::prelude::Requester;


// Banear a un usuario respondiendo un mensaje
pub async fn ban_user(bot: comandos::MyBot, msg: Message) -> ResponseResult<()> {
    match msg.reply_to_message() {

        Some(replied) => {
            let user = msg.reply_to_message().unwrap().from().unwrap();
            println!("{:?}", user);

            bot.delete_message(msg.chat.id, msg.id).await?;
            bot.ban_chat_member(msg.chat.id, replied.from().unwrap().id).await?;
            bot.send_message(msg.chat.id, format!("{} ha sido baneado", user.first_name)).await?;
            println!("user : {} ha sido baneado", user.first_name);

            let mut rng : StdRng = SeedableRng::from_entropy();
            let random_number = rng.gen_range(0..=15);
            println!("Resultado : {}", random_number);

            match random_number {
                0  => bot.send_animation(msg.chat.id, InputFile::file("./assets/ban/1.gif" )).await?,
                1  => bot.send_animation(msg.chat.id, InputFile::file("./assets/ban/2.gif" )).await?,
                2  => bot.send_animation(msg.chat.id, InputFile::file("./assets/ban/3.gif" )).await?,
                3  => bot.send_animation(msg.chat.id, InputFile::file("./assets/ban/4.gif" )).await?,
                4  => bot.send_animation(msg.chat.id, InputFile::file("./assets/ban/5.gif" )).await?,
                5  => bot.send_animation(msg.chat.id, InputFile::file("./assets/ban/6.gif" )).await?,
                6  => bot.send_animation(msg.chat.id, InputFile::file("./assets/ban/7.gif" )).await?,
                7  => bot.send_animation(msg.chat.id, InputFile::file("./assets/ban/8.gif" )).await?,
                8  => bot.send_animation(msg.chat.id, InputFile::file("./assets/ban/9.gif" )).await?,
                9  => bot.send_animation(msg.chat.id, InputFile::file("./assets/ban/10.gif")).await?,
                10 => bot.send_animation(msg.chat.id, InputFile::file("./assets/ban/11.gif")).await?,
                11 => bot.send_video(msg.chat.id, InputFile::file("./assets/ban/12.mp4")).await?,
                12 => bot.send_video(msg.chat.id, InputFile::file("./assets/ban/13.mp4")).await?,
                13  => bot.send_video(msg.chat.id, InputFile::file("./assets/ban/14.mp4")).await?,
                _  => bot.send_video(msg.chat.id, InputFile::file("./assets/ban/15.mp4")).await?,
            };
        }

        None => {
            bot.delete_message(msg.chat.id, msg.id).await?;
            bot.send_message(msg.chat.id, "Usa este comando respondiendo a otro mensaje").await?;
        }

    }
    Ok(())
}

pub async fn unban_user(bot: comandos::MyBot, msg: Message) -> ResponseResult<()> {
    match msg.reply_to_message() {

        Some(replied) => {
            let user = msg.reply_to_message().unwrap().from().unwrap();
            println!("Usuario desbaneado {:?}", user);

            bot.delete_message(msg.chat.id, msg.id).await?;
            bot.unban_chat_member(msg.chat.id, replied.from().unwrap().id).await?;
            bot.send_message(msg.chat.id, format!("{} ha sido desbaneado", user.first_name)).await?;
            bot.send_video(msg.chat.id, InputFile::file("./assets/unban/1.mp4")).await?;
        }

        None => {
            bot.delete_message(msg.chat.id, msg.id).await?;
            bot.send_message(msg.chat.id, "Usa este comando respondiendo a otro mensaje").await?;
        }

    }
    Ok(())
}

// silenciar a un usuario por tiempo indeterminado
pub async fn mute_user(bot: comandos::MyBot, msg: Message) -> ResponseResult<()> {

    let user = msg.reply_to_message().unwrap().from().unwrap();
    println!("Info : {:?}", user);

    let chat = msg.chat.id;
    println!("Chat ID : {:?}", chat);

    let usuario = user.clone().first_name;
    println!("usuario : {:?}", usuario);

    let username_user = user.clone().username;
    println!("username_user : {:?}", username_user);

    bot.delete_message(msg.chat.id, msg.id).await?;
    bot.restrict_chat_member(msg.chat.id, user.id, ChatPermissions::empty()).await?;

    bot.send_message(msg.chat.id, format!("{} ha sido silenciado", user.first_name)).await?;
    println!("Silenciado : {}", user.first_name);

    let mut rng: StdRng = SeedableRng::from_entropy();
    let random_number = rng.gen_range(0..=6);
    println!("random_number : {:?}", random_number);

    match random_number {
        0 => bot.send_animation(msg.chat.id, InputFile::file("./assets/mute/1.gif")).await?,
        1 => bot.send_animation(msg.chat.id, InputFile::file("./assets/mute/2.gif")).await?,
        2 => bot.send_animation(msg.chat.id, InputFile::file("./assets/mute/3.gif")).await?,
        3 => bot.send_animation(msg.chat.id, InputFile::file("./assets/mute/4.gif")).await?,
        _ => bot.send_photo(msg.chat.id, InputFile::file("./assets/mute/5.jpg")).await?,
    };

    Ok(())
}


// des silenciar a un usuario por tiempo indeterminado
pub async fn unmute_user(bot: comandos::MyBot, msg: Message) -> ResponseResult<()> {

    let user = msg.reply_to_message().unwrap().from().unwrap();
    println!("Info : {:?}", user);

    bot.delete_message(msg.chat.id, msg.id).await?;
    bot.restrict_chat_member(msg.chat.id, user.id, ChatPermissions::all()).await?;
    bot.send_message(msg.chat.id, format!("{} ya no está silenciado", user.first_name)).await?;

    bot.send_video(msg.chat.id, InputFile::file("./assets/unmute/unmute.mp4")).await?;

    Ok(())
}
