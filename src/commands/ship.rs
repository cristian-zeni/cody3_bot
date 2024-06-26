use rand::Rng;
use serenity::model::channel::Message;
use serenity::builder::{CreateEmbed, CreateMessage};
use serenity::prelude::*;
use rand::rngs::OsRng;

use super::file::opfile;


pub async fn ship_maker(ctx: &Context, msg: &Message){
    let mut lista:Vec<String> = Vec::new();
    
    let data = match opfile("./json/ship.json".to_owned(), ctx, msg).await {
        Ok(dati)=> dati,
        Err(_err)=>return,
    };

    let mut i = 0;
    while !(data[i].is_null()){
        lista.push(data[i]["nome"].to_string().replace('"', ""));
        i+=1;
    }

    let mut ship: Vec<(String, String, bool)> = Vec::new();
    //let mut rng = rand::thread_rng();
    let j: i8 = OsRng.gen_range(2..4);
    while ship.len() < j as usize{
        let n = OsRng.gen_range(0..lista.len());
        let tmp = (lista[n].to_owned(), " ".to_owned(), true);
        let mut ctr = false;
        for dati in &ship{
            if *dati == tmp{
                ctr = true;
            }
        }
        if !ctr{
            ship.push(tmp);
        }

    }

    let embed = CreateEmbed::new()
        .title("Ship")
        .color(0xba21c1)
        .fields(ship);

        let builder = CreateMessage::new()
        .embed(embed);

    if let Err(why) = msg.channel_id.send_message(&ctx.http, builder).await{
        println!("Error sending message: {why:?}");
    }

}