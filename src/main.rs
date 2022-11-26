use chrono::TimeZone;
use serenity::{
    async_trait,
    //collector::EventCollector,
    model::{channel::Message, gateway::Ready, prelude::ReactionType, Timestamp},
    prelude::*, utils::MessageBuilder, futures::TryFutureExt,
};
use dotenv;
use std::{env, time::Duration};
use tokio;
use reqwest;
use soup;
use serde_json::{Result, Value};
use json;

mod w4r;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("!debug"){
            if let Err(why) = msg.channel_id.say(&ctx.http, format!("{:#?}",msg)).await {
                println!("Error sending message: {:?}", why);
            }
        }
        if msg.content == "!ping" {
            let x = msg.timestamp.to_string();
            if let Ok(then)= chrono::DateTime::parse_from_rfc3339(&x){
                if let Ok(now) = chrono::DateTime::parse_from_rfc3339(&Timestamp::now().to_string()){
                    let delta = (now-then).num_milliseconds(); 
                    let message = format!("ping: {delta}ms");
                    if let Err(why) = msg.reply(&ctx.http, message).await {
                        println!("Error sending message: {:?}", why);
                    }
                }
            } else {
                if let Err(why) = msg.reply(&ctx.http, "something went wrong").await {
                    println!("Error sending message: {:?}", why);
                }
            }
        }
        else if let Some(content) = msg.content.strip_prefix("!repeat: "){
            if let Err(why) = msg.channel_id.say(&ctx.http, content).await {
                println!("Error sending message: {:?}", why);
            }
        }
        else if msg.content == "!w4r"{
            w4r::handle(msg,&ctx).await;
        }
        else if let Some(content) = msg.content.strip_prefix("!weather: "){
            let resp = reqwest::get(format!("https://weatherdbi.herokuapp.com/data/weather/{}", content)).await.unwrap().text().await.unwrap();
            if let Err(why) = json::parse(&resp){
                println!("json parse error: {why}");
                println!("resp = {resp}");
                if let Err(why2) = msg.reply(&ctx, "sorry. it seems that service is currently unavailable").await{
                    println!("Error sending message: {why2:?}");
                }
            } else if let Ok(jsonv) = json::parse(&resp){
                if !jsonv.has_key("status"){
                    if let Err(why) = msg.channel_id.say(&ctx.http, jsonv["currentConditions"]["temp"]["c"].to_string()+"°C").await {
                        println!("Error sending message: {:?}", why);
                    }
                } else {
                    if let Err(why) = msg.channel_id.say(&ctx.http, "failure...").await {
                        println!("Error sending message: {:?}", why);
                    }
                }
            }
        }
        else if msg.content == "!bored"{
            let resp = reqwest::get(ACTY).await.unwrap().text().await.unwrap();
            let jsonv = json::parse(&resp).unwrap();
            if let Err(why) = msg.channel_id.say(&ctx.http, jsonv["activity"].to_string().as_str()).await {
                println!("Error sending message: {:?}", why);
            }
        }
        else if msg.content == "!everyone"{
            if let Err(why) = msg.channel_id.say(&ctx.http, "@everyone").await {
                println!("Error sending message: {:?}", why);
            }
        }
        else if msg.content.starts_with("!wait:"){
            let content = msg.content.split(":");
            if content.clone().count() != 3{
                if let Err(why) = msg.channel_id.say(&ctx.http, "invalid format").await {
                    println!("Error sending message: {:?}", why);
                }
            } else {
                let content: Vec<&str> = content.map(|x|x.trim()).collect();
                if let Ok(v) = content[1].parse::<f64>(){
                    let rpl = msg.reply(&ctx, "timer set!").await;
                    if let Err(why) =rpl{
                        println!("Error sending message: {:?}", why);
                    } else {
                        tokio::time::sleep(Duration::from_secs_f64(v)).await;
                        let message = MessageBuilder::new()
                            .mention(&msg.author.id)
                            .push(" ")
                            .push(content[2])
                            .build();
                        let rpl = msg.reply(&ctx, message).await;
                        if let Err(why) = rpl{
                            println!("Error sending message: {why:?}");
                        }
                    }
                }
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        // println!("info:\n{:#?}",ready);
    }
}


const ACTY: &'static str = "https://www.boredapi.com/api/activity";

#[tokio::main]
async fn main() {
    println!("starting process...");
    dotenv::dotenv().expect("there must be .env with TOKEN");
    let token = env::var("TOKEN").expect("where is bot token?");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGE_REACTIONS;

    let mut client = Client::builder(token.as_str(), intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
