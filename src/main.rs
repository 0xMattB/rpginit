/***************************************************************************************************
* RPG Initiative Tracker
* Version 0.3.0 (08/04/23)
* main.rs
* 0xMattB
***************************************************************************************************/
mod initiative;

use std::env;
use serenity::{
	async_trait,
	model::{channel::Message, gateway::Ready},
	prelude::*
};
use initiative::entities::Entities;
use initiative::options::Options;
use initiative::parse;
use initiative::command;

struct Handler;
static mut ENTITIES: Entities = Entities::new();
static mut OPTIONS: Options = Options::new();

#[async_trait]
impl EventHandler for Handler {
	async fn message(&self, ctx: Context, msg: Message) {
		if msg.content.starts_with("!") {
			let content = msg.content.clone();
			let mut user = msg.author.name.clone();
			let mut parse = parse::parse(&content);
			let mut response = String::new();
			
			unsafe {
				command::execute(&mut ENTITIES, &mut OPTIONS, &mut parse, &mut user, &mut response);
			}
			
			response.insert_str(0, "```");
			response.push_str("```");
			
			if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
				println!("Error sending message: {:?}", why);
			}
		}
	}
	
	async fn ready(&self, _: Context, ready: Ready) {
		println!("{} is connected!", ready.user.name);
	}
}

#[tokio::main]
async fn main() {
	let token = env::var("DISCORD_TOKEN")
		.expect("Expected a token in the environment");
	
	let mut client = Client::builder(&token, GatewayIntents::all())
		.event_handler(Handler)
		.await
		.expect("Error creating client");
	
	if let Err(why) = client.start().await {
		println!("Client error: {:?}", why);
	}
}