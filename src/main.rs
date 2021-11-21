use serde::{Deserialize, Serialize};
use twitch_irc::login::{CredentialsPair, StaticLoginCredentials};
use twitch_irc::{ClientConfig, SecureTCPTransport, TwitchIRCClient};

fn channel_to_join() -> Result<String, Box<dyn std::error::Error>> {
    let channel = get_env_var("TWITCH_SEND_CHANNEL")?;
    Ok(channel)
}

fn get_env_var(key: &str) -> Result<String, Box<dyn std::error::Error>> {
    let my_var = std::env::var(key)?;
    Ok(my_var)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UsernameUpdate {
    date: String,
    comment: String,
    pub content: Vec<String>,
}

#[tokio::main]
async fn main() {
    let dl = get_env_var("TWITCH_BAN_LIST").unwrap();
    let twitch_name = get_env_var("TWITCH_SEND_NAME").unwrap();
    let twitch_token = get_env_var("TWITCH_SEND_TOKEN")
        .unwrap()
        .replacen("oauth:", "", 1);
    let channel_to_join = channel_to_join().unwrap();

    let alt_dl: String = ureq::get(&dl).call().unwrap().into_string().unwrap();

    let username_list: Vec<UsernameUpdate> = serde_yaml::from_str(&alt_dl).unwrap();

    // default configuration is to join chat as anonymous.
    let config = ClientConfig {
        login_credentials: StaticLoginCredentials {
            credentials: CredentialsPair {
                login: twitch_name.clone(),
                token: Some(twitch_token),
            },
        },
        ..ClientConfig::default()
    };

    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    let cloned_client = client.clone();

    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            println!("Received message: {:?}", message);
        }
    });

    // join a channel
    cloned_client.join(channel_to_join.clone());

    let usernames = username_list
        .iter()
        .map(|x| x.content.to_owned())
        .flatten()
        .collect::<Vec<_>>();

    let reason = std::env::args().skip(1).collect::<Vec<_>>().join(" ");

    for username in usernames {
        let clean_username = username.replacen('"', "", 2);
        println!("Sending ban command for {:?} ...", clean_username);
        cloned_client
            .ban(twitch_name.clone(), &clean_username, Some(&reason))
            .await
            .unwrap();
    }

    join_handle.await.unwrap();

    drop(client);
    drop(cloned_client);
}
