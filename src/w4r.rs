use serenity::{
    //collector::EventCollector,
    model::{channel::Message, prelude::{ReactionType, MessageReaction}},
    prelude::*, utils::CustomMessage,
};
use std::{time::Duration, ops::Deref};
use tokio::sync::Mutex;
use std::sync::Arc;
async fn react(ctx: Arc<Mutex<Context>>, msg: Arc<Mutex<Message>>, reaction: Arc<Mutex<ReactionType>>){
    let c = &*ctx.lock().await;
    let r = &*reaction.lock().await;
    let m = &*msg.lock().await;
    m.react(&c, r.clone()).await; 
}

pub async fn handle(msg: Message, ctx: &Context) {
    // let old_ctx = ctx.clone();
    let react_msg = msg.reply(&ctx, "react with something!").await.unwrap();
    // let rm_clone = react_msg.clone();
    // let ctx = Arc::new(ctx);
    // let t_u = ReactionType::try_from("👍").unwrap();
    // let t_d = ReactionType::try_from("👎").unwrap();
    let t1 = tokio::spawn(
        react(
            Arc::new(Mutex::new(ctx.clone())), 
            Arc::new(Mutex::new(react_msg.clone())), 
            Arc::new(Mutex::new(ReactionType::try_from("👍").unwrap()))
        )
    );
    println!("t1 spawned");
    let t2 = tokio::spawn(
        react(
            Arc::new(Mutex::new(ctx.clone())), 
            Arc::new(Mutex::new(react_msg.clone())), 
            Arc::new(Mutex::new(ReactionType::try_from("👎").unwrap()))
        )
    );
    println!("t2 spawned");
    // let t1 = tokio::spawn(
    //     rm_clone.lock().await.react(ctx.clone(), ReactionType::try_from("👍").unwrap())
    // );
    // let t2 = tokio::spawn(async move {
    //     let x = Arc::clone(&react_msg).lock().await.react(ctx.clone(), ReactionType::try_from("👎").unwrap());
    //     return x;
    // });
    t1.await.and_then(|_|Ok(println!("t1 done!")));
    t2.await.and_then(|_|Ok(println!("t2 done!")));
    if let Some(reaction) = react_msg
        .await_reaction(&ctx)
        .timeout(Duration::from_secs(10))
        .author_id(msg.author.id)
        .await
    {
        let emoji = &reaction.as_inner_ref().emoji.as_data();
        react_msg
            .reply(ctx, format!("you reacted with {emoji}"))
            .await
            .expect("error with replying");
    } else {
        let _ = msg.reply(ctx, "No reaction within 10 seconds.").await;
    }
}
