use std::sync::Arc;

use tokio::sync::RwLock;

use super::{
    channel::Channel,
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
                let old_nick = active_session.nick.as_ref().unwrap().clone();
                let new_nick = nick.clone();
                server_state.change_nick(&old_nick, &new_nick).await;
                tracing::debug!("Finished updating server state");

                //update the references in the channel lists
                let channels = server_state.channels.read().await;
                for channel in channels.values() {
                    let mut channel = channel.write().await;
                    if channel.users.contains_key(&old_nick) {
                        let client = channel.users.remove(&old_nick).unwrap();
                        channel.users.insert(new_nick.clone(), client);
                    }
                }

                active_session.nick = Some(nick.clone());

                let _ = active_session
                    .sender
                    .send(format!(":server 001 {} :Welcome!\r\n", nick));

                //send NICK message to all connected users
                let formatted_message = format!(":{} NICK {}\r\n", old_nick, new_nick);
                let recipient_handles = {
                    let users = server_state.users.read().await;
                    users.keys()
                        .filter_map(|user| {
                            if user != &new_nick {
                                users.get(user).map(Arc::clone)
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                };
                for handle in recipient_handles {
                    let client = handle.write().await;
                    let _ = client.sender.send(formatted_message.clone());
                }
                
            }

            Command::USER(user) => {
                session.write().await.user = Some(user.clone());
            }

            Command::JOIN(channel) => {
                let nickname = {
                    let active_session = session.read().await;
                    active_session.nick.as_ref().unwrap().clone()
                };

                tracing::debug!("User {} joining channel {}", nickname, channel);
                
                let channel_obj = {
                    let mut channels_lock = server_state.channels.write().await;
                    if let Some(channel) = channels_lock.get(channel) {
                        channel.clone()
                    } else { //channel doesn't exist, create it
                        let channel_obj = Arc::new(RwLock::new(Channel::new(channel.clone())));
                        channels_lock.insert(channel.clone(), channel_obj.clone());
                        channel_obj
                    }
                };
                tracing::debug!("found/created channel");
                {
                    channel_obj.write().await.users.insert(nickname.clone(), session.clone());
                }
                tracing::debug!("added user to channel");
                let channel_name = {
                    let channel_lock = channel_obj.read().await;
                    channel_lock.name.clone()
                };
                tracing::debug!("Finished updating server state");

                //Send user JOIN message back to the user
                tracing::debug!("Sending JOIN message to user");
                let _ = {
                    let active_session = session.read().await;
                    active_session
                    .sender
                    .send(format!(":{} JOIN {}\r\n", nickname, channel_name))
                };

                //Send Channel topic value to the user
                tracing::debug!("Sending channel topic to user");
                let params = ResponseParams::new(nickname.clone()).channel(channel_name.clone());
                let response = ResponseCode::RPL_TOPIC.message(params);
                let _ = {
                    let active_session = session.read().await;
                    active_session.sender.send(response)
                };

                //Send name list to user
                tracing::debug!("Sending name list to user");
                let user_list = {
                    let users = server_state
                        .channels
                        .read()
                        .await
                        .get(channel_name.as_str())
                        .unwrap()
                        .read()
                        .await
                        .users
                        .clone();
                    users.keys().cloned().collect::<Vec<_>>().join(" ")
                };
                tracing::debug!("User list: {}", user_list);
                let params = ResponseParams::new(nickname.clone())
                    .channel(channel_name.clone())
                    .message(user_list);
                let response = ResponseCode::RPL_NAMREPLY.message(params);
                let _ = {
                    let active_session = session.read().await;
                    active_session.sender.send(response)
                };

                let params = ResponseParams::new(nickname.clone()).channel(channel_name.clone());
                let response = ResponseCode::RPL_ENDOFNAMES.message(params);
                let _ = {
                    let active_session = session.read().await;
                    active_session.sender.send(response)
                };

                //Send join message to all connected users of channel
                tracing::debug!("Sending JOIN message to all users of channel");
                let formatted_message = format!(":{} JOIN {}\r\n", nickname, channel_name);
                let recipient_handles = {
                    let channel_lock = channel_obj.read().await;
                    let users = channel_lock.users.clone();
                    let users_lock = server_state.users.read().await;
                    users.iter()
                        .filter_map(|(user, _)| {
                            if user != &nickname {
                                users_lock.get(user).map(Arc::clone)
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                };
                for handle in recipient_handles {
                    let client = handle.write().await;
                    let _ = client.sender.send(formatted_message.clone());
                }
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
                        if let Some(channel) = channels.get(target) {
                            let users_lock = server_state.users.read().await;
                            let channel_lock = channel.read().await;
                            Some(
                                channel_lock
                                    .users
                                    .keys()
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
                    if let Some(channel) = channels.get(channel) {
                        let channel_lock = channel.read().await;
                        let users = channel_lock.users.clone();
                        let user_list = users.keys().cloned().collect::<Vec<_>>().join(" ");

                        let params =
                            ResponseParams::new(active_session.nick.as_ref().unwrap().clone())
                                .channel(channel_lock.name.clone())
                                .message(user_list.clone());

                        let response = ResponseCode::RPL_NAMREPLY.message(params);
                        let _ = active_session.sender.send(response);

                        let params =
                            ResponseParams::new(active_session.nick.as_ref().unwrap().clone())
                                .channel(channel_lock.name.clone());
                        let response = ResponseCode::RPL_ENDOFNAMES.message(params);
                        let _ = active_session.sender.send(response);
                    }
                } else {
                    let channels = server_state.channels.read().await;
                    for channel in channels.keys() {
                        let channel_obj = channels.get(channel).unwrap();
                        let channel_lock = channel_obj.read().await;
                        let user_list = channel_lock
                            .users
                            .keys()
                            .cloned()
                            .collect::<Vec<_>>()
                            .join(" ");
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
