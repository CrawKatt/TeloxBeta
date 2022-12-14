use teloxide::types::InputFile;
use teloxide::prelude::Requester;

use rand::rngs::StdRng;
use rand::SeedableRng;
use rand::Rng;

use crate::comandos;
use crate::Message;
use crate::ResponseResult;


pub async fn send_pat(bot: comandos::MyBot, msg: Message) -> ResponseResult<()> {
    let mut rng : StdRng = SeedableRng::from_entropy();
    let random_number = rng.gen_range(0..=21);
    println!("Resultado : {}", random_number);

    bot.delete_message(msg.chat.id, msg.id).await?;

    match random_number {
        0 => bot.send_animation(msg.chat.id, InputFile::file("./assets/pat/1.gif")).await?,
        1 => bot.send_animation(msg.chat.id, InputFile::file("./assets/pat/2.gif")).await?,
        2 => bot.send_animation(msg.chat.id, InputFile::file("./assets/pat/3.gif")).await?,
        3 => bot.send_animation(msg.chat.id, InputFile::file("./assets/pat/4.gif")).await?,
        4 => bot.send_animation(msg.chat.id, InputFile::file("./assets/pat/5.gif")).await?,
        5 => bot.send_animation(msg.chat.id, InputFile::file("./assets/pat/6.gif")).await?,
        6 => bot.send_animation(msg.chat.id, InputFile::file("./assets/pat/7.gif")).await?,
        7 => bot.send_animation(msg.chat.id, InputFile::file("./assets/pat/8.gif")).await?,
        8 => bot.send_animation(msg.chat.id, InputFile::file("./assets/pat/9.gif")).await?,
        9 => bot.send_animation(msg.chat.id, InputFile::file("./assets/pat/10.gif")).await?,
        10 => bot.send_animation(msg.chat.id, InputFile::file("./assets/pat/11.gif")).await?,
        11 => bot.send_animation(msg.chat.id, InputFile::file("./assets/pat/12.gif")).await?,
        12 => bot.send_animation(msg.chat.id, InputFile::file("./assets/pat/13.gif")).await?,
        13 => bot.send_animation(msg.chat.id, InputFile::file("./assets/pat/14.gif")).await?,
        14 => bot.send_animation(msg.chat.id, InputFile::file("./assets/pat/15.gif")).await?,
        15 => bot.send_animation(msg.chat.id, InputFile::file("./assets/pat/16.gif")).await?,
        16 => bot.send_animation(msg.chat.id, InputFile::file("./assets/pat/17.gif")).await?,
        17 => bot.send_animation(msg.chat.id, InputFile::file("./assets/pat/18.gif")).await?,
        18 => bot.send_animation(msg.chat.id, InputFile::file("./assets/pat/19.gif")).await?,
        19 => bot.send_animation(msg.chat.id, InputFile::file("./assets/pat/20.gif")).await?,
        20 => bot.send_animation(msg.chat.id, InputFile::file("./assets/pat/21.gif")).await?,
        _ => bot.send_animation(msg.chat.id, InputFile::file("./assets/pat/21.gif")).await?,
    };
    Ok(())
}

pub async fn send_random_meme(bot: comandos::MyBot, msg: Message) -> ResponseResult<()> {
    let mut rng : StdRng = SeedableRng::from_entropy();
    let random_number = rng.gen_range(0..=28);
    println!("Resultado : {}", random_number);

    bot.delete_message(msg.chat.id, msg.id).await?;

    match random_number {
        0 => bot.send_video(msg.chat.id, InputFile::file("./assets/memes/1.mp4")).await?,
        1 => bot.send_video(msg.chat.id, InputFile::file("./assets/memes/2.mp4")).await?,
        2 => bot.send_video(msg.chat.id, InputFile::file("./assets/memes/3.mp4")).await?,
        3 => bot.send_video(msg.chat.id, InputFile::file("./assets/memes/4.mp4")).await?,
        4 => bot.send_video(msg.chat.id, InputFile::file("./assets/memes/5.mp4")).await?,
        5 => bot.send_photo(msg.chat.id, InputFile::file("./assets/memes/6.jpg")).await?,
        6 => bot.send_photo(msg.chat.id, InputFile::file("./assets/memes/7.jpg")).await?,
        7 => bot.send_photo(msg.chat.id, InputFile::file("./assets/memes/8.jpg")).await?,
        8 => bot.send_photo(msg.chat.id, InputFile::file("./assets/memes/9.jpg")).await?,
        9 => bot.send_photo(msg.chat.id, InputFile::file("./assets/memes/10.jpg")).await?,
        10 => bot.send_photo(msg.chat.id, InputFile::file("./assets/memes/11.jpg")).await?,
        11 => bot.send_photo(msg.chat.id, InputFile::file("./assets/memes/12.jpg")).await?,
        12 => bot.send_photo(msg.chat.id, InputFile::file("./assets/memes/13.jpg")).await?,
        13 => bot.send_photo(msg.chat.id, InputFile::file("./assets/memes/14.jpg")).await?,
        14 => bot.send_photo(msg.chat.id, InputFile::file("./assets/memes/15.jpg")).await?,
        15 => bot.send_photo(msg.chat.id, InputFile::file("./assets/memes/16.jpg")).await?,
        16 => bot.send_photo(msg.chat.id, InputFile::file("./assets/memes/17.jpg")).await?,
        17 => bot.send_photo(msg.chat.id, InputFile::file("./assets/memes/18.jpg")).await?,
        18 => bot.send_photo(msg.chat.id, InputFile::file("./assets/memes/19.jpg")).await?,
        19 => bot.send_photo(msg.chat.id, InputFile::file("./assets/memes/20.jpg")).await?,
        20 => bot.send_photo(msg.chat.id, InputFile::file("./assets/memes/21.jpg")).await?,
        21 => bot.send_photo(msg.chat.id, InputFile::file("./assets/memes/22.jpg")).await?,
        22 => bot.send_photo(msg.chat.id, InputFile::file("./assets/memes/23.jpg")).await?,
        23 => bot.send_photo(msg.chat.id, InputFile::file("./assets/memes/24.jpg")).await?,
        24 => bot.send_photo(msg.chat.id, InputFile::file("./assets/memes/25.jpg")).await?,
        25 => bot.send_photo(msg.chat.id, InputFile::file("./assets/memes/26.jpg")).await?,
        26 => bot.send_photo(msg.chat.id, InputFile::file("./assets/memes/27.jpg")).await?,
        27 => bot.send_photo(msg.chat.id, InputFile::file("./assets/memes/28.jpg")).await?,
        _ => bot.send_photo(msg.chat.id, InputFile::file("./assets/memes/29.jpg")).await?,
    };
    Ok(())
}