use std::{collections::HashSet, sync::Arc};

use tokio::sync::RwLock;

use super::{
    client::Client,
    ircd::SharedServerState,
    response::{ResponseCode, ResponseParams},
};

#[derive(Debug)]
pub enum Command {
    CapLs,
    CapReq(Vec<String>),
    CapEnd,
    NICK(String),
    USER(String),
    JOIN(String),
    PING(String),
    PRIVMSG(String, String),
    NAMES(Option<String>),
    QUIT,
    Unknown(String),
}

impl Command {
    pub fn parse(input: &str) -> Self {
        let parts: Vec<&str> = input.split_whitespace().collect();

        match parts.first().map(|s| s.to_ascii_uppercase()) {
            Some(cmd) if cmd == "CAP" => match parts.get(1).map(|s| s.to_ascii_uppercase()) {
                Some(ref sub_cmd) if sub_cmd == "LS" => Command::CapLs,
                Some(ref sub_cmd) if sub_cmd == "END" => Command::CapEnd,
                Some(ref sub_cmd) if sub_cmd == "REQ" => {
                    let caps = parts.iter().skip(2).map(|s| s.to_string()).collect();
                    Command::CapReq(caps)
                }
                _ => Command::Unknown(input.to_string()),
            },

            Some(cmd) if cmd == "NICK" => {
                if let Some(nick) = parts.get(1) {
                    Command::NICK(nick.to_string())
                } else {
                    Command::Unknown(input.to_string())
                }
            }

            Some(cmd) if cmd == "PING" => {
                if let Some(token) = parts.get(1) {
                    Command::PING(token.to_string())
                } else {
                    Command::Unknown(input.to_string())
                }
            }

            Some(cmd) if cmd == "USER" => {
                if let Some(user) = parts.get(1) {
                    Command::USER(user.to_string())
                } else {
                    Command::Unknown(input.to_string())
                }
            }

            Some(cmd) if cmd == "JOIN" => {
                if let Some(channel) = parts.get(1) {
                    Command::JOIN(channel.to_string())
                } else {
                    Command::Unknown(input.to_string())
                }
            }

            Some(cmd) if cmd == "PRIVMSG" => match (parts.get(1), parts.len() > 2) {
                (Some(target), true) => {
                    let msg = parts[2..]
                        .join(" ")
                        .trim_start_matches(":")
                        .trim()
                        .to_string();
                    if !msg.is_empty() {
                        Command::PRIVMSG(target.to_string(), msg.to_string())
                    } else {
                        Command::Unknown(input.to_string())
                    }
                }
                _ => Command::Unknown(input.to_string()),
            },

            Some(cmd) if cmd == "NAMES" => {
                if let Some(channel) = parts.get(1) {
                    Command::NAMES(Some(channel.to_string()))
                } else {
                    Command::NAMES(None)
                }
            }

            Some(cmd) if cmd == "QUIT" => Command::QUIT,

            _ => Command::Unknown(input.to_string()),
        }
    }

    #[tracing::instrument(name = "Handling command operation")]
    pub async fn handle(&self, session: &Arc<RwLock<Client>>, server_state: &SharedServerState) {
        match self {
            Command::CapLs | Command::CapReq(_) | Command::CapEnd => {
                let mut session = session.write().await;
                if let Some(response) = session.handle_cap_command(self) {
                    tracing::debug!("Sending CAP response: {}", response);
                    let _ = session.sender.send(response);
                }
            }

            Command::NICK(nick) => {
                tracing::debug!("Changing nickname to: {}", nick);
                let mut active_session = session.write().await;
                let old_nick = active_session.nick.as_ref().unwrap();
                let new_nick = nick.clone();
                server_state.change_nick(old_nick, &new_nick).await;
                tracing::debug!("Finished updating server state");
                active_session.nick = Some(nick.clone());

                let _ = active_session
                    .sender
                    .send(format!(":server 001 {} :Welcome!\r\n", nick));
            }

            Command::USER(user) => {
                session.write().await.user = Some(user.clone());
            }

            Command::JOIN(channel) => {
                let nickname = {
                    let active_session = session.read().await;
                    active_session.nick.as_ref().unwrap().clone()
                };
                let mut channels = server_state.channels.write().await;
                let users = channels.entry(channel.clone()).or_insert_with(HashSet::new);
                users.insert(nickname.clone());
            }

            Command::PRIVMSG(target, message) => {
                let nickname = {
                    let active_session = session.read().await;
                    active_session.nick.as_ref().unwrap().clone()
                };

                let formatted_message =
                    format!(":{} PRIVMSG {} :{}\r\n", nickname, target, message);

                if target.starts_with("#") {
                    tracing::debug!("Sending message to channel: {}", target);
                    let recipient_handles: Option<Vec<Arc<RwLock<Client>>>> = {
                        let channels = server_state.channels.read().await;
                        if let Some(users) = channels.get(target) {
                            let users_lock = server_state.users.read().await;
                            Some(
                                users
                                    .iter()
                                    .filter(|user| *user != &nickname)
                                    .filter_map(|user| users_lock.get(user).map(Arc::clone))
                                    .collect::<Vec<_>>(),
                            )
                        } else {
                            Some(vec![])
                        }
                    };

                    for handle in recipient_handles.unwrap() {
                        let client = handle.write().await;
                        let _ = client.sender.send(formatted_message.clone());
                    }
                } else if let Some(recipient) = {
                    let users = server_state.users.read().await;
                    users.get(target).map(Arc::clone)
                } {
                    let client = recipient.write().await;
                    let _ = client.sender.send(formatted_message);
                }
            }

            Command::PING(token) => {
                let _ = session
                    .write()
                    .await
                    .sender
                    .send(format!("PONG server {}\r\n", token));
            }

            Command::NAMES(channel) => {
                let active_session = session.write().await;
                if let Some(channel) = channel {
                    let channels = server_state.channels.read().await;
                    if let Some(users) = channels.get(channel) {
                        let user_list = users.iter().cloned().collect::<Vec<_>>().join(" ");

                        let params =
                            ResponseParams::new(active_session.nick.as_ref().unwrap().clone())
                                .channel(channel.clone())
                                .message(user_list.clone());

                        let response = ResponseCode::RPL_NAMREPLY.message(params);
                        let _ = active_session.sender.send(response);

                        let params =
                            ResponseParams::new(active_session.nick.as_ref().unwrap().clone())
                                .channel(channel.clone());
                        let response = ResponseCode::RPL_ENDOFNAMES.message(params);
                        let _ = active_session.sender.send(response);
                    }
                } else {
                    let channels = server_state.channels.read().await;
                    for channel in channels.keys() {
                        let users = channels.get(channel).unwrap();
                        let user_list = users.iter().cloned().collect::<Vec<_>>().join(" ");
                        let params =
                            ResponseParams::new(active_session.nick.as_ref().unwrap().clone())
                                .channel(channel.clone())
                                .message(user_list.clone());

                        let response = ResponseCode::RPL_NAMREPLY.message(params);
                        let _ = active_session.sender.send(response);
                    }

                    let params = ResponseParams::new(active_session.nick.as_ref().unwrap().clone())
                        .channel("*".to_string());
                    let response = ResponseCode::RPL_ENDOFNAMES.message(params);
                    let _ = active_session.sender.send(response);
                }
            }

            Command::QUIT => {
                let active_session = session.write().await;
                let nickname = active_session.nick.as_ref().unwrap();
                let _ = session
                    .write()
                    .await
                    .sender
                    .send(format!(":{} QUIT\r\n", nickname));
            }

            Command::Unknown(cmd) => {
                tracing::info!("Unknown command: {}", cmd);
            }
        }
    }
}
