extern crate termion;

use termion::style;
use std::env;

use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::ClientConfig;
use twitch_irc::SecureTCPTransport;
use twitch_irc::TwitchIRCClient;

#[tokio::main]
pub async fn main() {
    // default configuration is to join chat as anonymous.
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please give channel to join");
        return;
    }

    print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
    println!("");
    let channel = &args[1];

    let config = ClientConfig::default();
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);


    // first thing you should do: start consuming incoming messages,
    // otherwise they will back up.
    let join_handle = tokio::spawn(async move {
        use twitch_irc::message::ServerMessage;

        while let Some(message) = incoming_messages.recv().await {
            match message {
                ServerMessage::Privmsg(message) => {
                    println!("{}: {}{}{}", message.sender.login, style::Bold, message.message_text, style::Reset);
                }
                _ => ()
            }
        }
    });

    // join a channel
    client.join(channel.to_owned());

    // keep the tokio executor alive.
    // If you return instead of waiting the background task will exit.
    join_handle.await.unwrap();
}
