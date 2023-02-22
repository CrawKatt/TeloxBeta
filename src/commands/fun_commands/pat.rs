use crate::fun_commands::*;

pub async fn send_pat(bot: Bot, msg: Message) -> ResponseResult<()> {
    let file_names = [
        "1.gif", "2.gif", "3.gif", "4.gif", "5.gif", "6.gif", "7.gif", "8.gif", "9.gif", "10.gif",
        "11.gif", "12.gif", "13.gif", "14.gif", "15.gif", "16.gif", "17.gif", "18.gif", "19.gif",
        "20.gif", "21.gif", "22.gif", "23.gif", "24.gif", "25.gif", "26.gif", "27.gif", "28.gif",
        "29.gif",
    ];

    let get_file_name =
        |index: usize| -> &'static str { file_names.get(index).unwrap_or(&file_names[0]) };

    let mut rng: StdRng = SeedableRng::from_entropy();
    let random_number = rng.gen_range(0..=file_names.len() - 1);
    println!("Resultado : {}", random_number);

    bot.delete_message(msg.chat.id, msg.id).await?;

    let file_path = format!("./assets/pat/{}", get_file_name(random_number));

    if Path::new(&file_path).is_file() {
        bot.send_animation(msg.chat.id, InputFile::file(&file_path)).await?;
    }

    Ok(())
}