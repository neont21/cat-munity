//! # CAT-munity
//! 디스코드 기반의 개발자 커뮤니티에서 사용할 디스코드 봇입니다. 
//! 이름이 왜 캣뮤니티냐고요? 그야 당연히 고양이는 고양이기 때문입니다.

// use for Environment Variable
use std::{collections::HashSet, env};

// use for Discord API
use serenity::{
    async_trait,
    framework::standard::{
        help_commands,
        macros::{group, help, hook},
        Args, CommandGroup, CommandResult, DispatchError, HelpOptions, StandardFramework,
    },
    http::Http,
    model::{
        channel::Message, 
        gateway::Ready, 
        id::UserId
    },
    prelude::*,
};

// modules
mod commands;
use commands::{ping::*, embed::*}; //TODO create the module files and implement the functions

struct Handler;
#[async_trait]
impl EventHandler for Handler {
    /// Called when the bot is ready.
    /// It prints the name of the bot on the console.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} 왔다요!!", ready.user.name);
    }
}

/// General user group.
#[group]
#[commands(ping)]
struct General;

/// Owner group.
#[group]
#[owners_only]
#[only_in(guilds)]
struct Owner;

/// Response to `help` command.
#[help]
#[individual_command_tip="캣뮤니티의 기능을 알아보자요!!"]
#[command_not_found_text="캣뮤니티는 `{}`라는 명령어를 가지고 있지 않다요!!"]
#[max_levenshtein_distance(3)]
#[lacking_permissions="Hide"]
#[lacking_role="Strike"]
#[lacking_ownership="Hide"]
async fn catminity_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;

    Ok(())
}

/// Called whenever before responding the command.
#[hook]
async fn before(_ctx: &Context, msg: &Message, command_name: &str) -> bool {
    println!(
        "사용자 '{}'의 명령어 '{}' 받았다요!!",
        msg.author.name,
        command_name,
    );

    true
}

/// Called whenever after responding the command.
#[hook]
async fn after(_ctx: &Context, _msg: &Message, command_name: &str, command_result: CommandResult) {
    match command_result {
        Ok(()) => println!("명령어 '{}' 성공적으로 처리했다요!!", command_name),
        Err(why) => println!("명령어 '{}' 처리하는 데 문제가 있다요!!\n 오류: {:?}", command_name, why),
    }
}

/// Called whenever the unknown command sent.
#[hook]
async fn unknown_command(_ctx: &Context, _msg: &Message, unknown_command_name: &str) {
    println!("명령어 '{}' 찾을 수 없다요!!", unknown_command_name);
}

/// Called whenever the normal message sent.
#[hook]
async fn normal_message(_ctx: &Context, msg: &Message) {
    println!("명령어가 아닌 메시지 '{}' 받았다요!!", msg.content);
}

/// Called whenever the response of the message failed.
#[hook]
async fn dispatch_error(ctx: &Context, msg: &Message, error: DispatchError, cmd: &str) {
    if let DispatchError::Ratelimited(duration) = error {
        let _ = msg
            .channel_id
            .say(
                &ctx.http,
                &format!("명령어 {} 오류 났다요!!\n{}초 후에 다시 시도해보라요!!", cmd, duration.as_secs()),
            )
            .await;
    }
}

/// Set up the bot.
#[tokio::main(flavor="current_thread")]
async fn main() {
    let token = env::var("CATMINITY_TOKEN").expect("환경변수 `CATMINITY_TOKEN` 가 설정되지 않았다요!!");
    let http = Http::new(&token);

    // Set owners and get the bot data
    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(team) = info.team {
                for team_member in team.members.iter() {
                    owners.insert(team_member.user.id);
                }
            } else {
                owners.insert(info.owner.id);
            }
            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => panic!("캣미니티 봇에 접속할 수 없다요!!: {:?}", why),
            }
        }
        Err(why) => panic!("캣미니티 정보를 불러올 수 없다요!!: {:?}", why),
    };

    // Set prefix as quotation and set the command groups.
    let framework = StandardFramework::new()
        .configure(|c| {
            c.with_whitespace(true)
                .on_mention(Some(bot_id))
                .prefix(&format!("<@{}>", bot_id)[..])
                .delimiters(vec![", ", ",", " "])
                .owners(owners)
        })
        .before(before)
        .after(after)
        .unrecognised_command(unknown_command)
        .normal_message(normal_message)
        .on_dispatch_error(dispatch_error)
        .help(&CATMINITY_HELP)
        .group(&EMBED_GROUP)
        .group(&GENERAL_GROUP)
        .group(&OWNER_GROUP);


    // Create a new instance of the client by the bot token.
    let intents = GatewayIntents::all();
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("클라이언트 생성에 실패했다요!!");

    // Start the client
    if let Err(why) = client.start().await {
        println!("클라이언트 오류가 발생했다요!!: {:?}", why);
    }
}
