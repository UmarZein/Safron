use serenity::{
    //collector::EventCollector,
    model::channel::Message,
    prelude::*, utils::MessageBuilder,
};
use logmat::*;
pub async fn handle(msg: Message, ctx: &Context, input: &str) {
    if let Ok(parsed) = parser::parse(input){
        let text = format!(r#"```
{parsed}
{}
```"#,parsed.truth_table());
        dbg!(text.clone());
        println!("text = \n{text}");    
        if let Err(e) = msg.reply(&ctx, text).await{
            println!("error: {e}");
        } 
    }
}
