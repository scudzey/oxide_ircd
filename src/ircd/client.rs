use std::collections::HashSet;
use tokio::sync::mpsc::UnboundedSender;

use super::command::Command;


#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub enum Capability {
    MultiPrefix,
    SASL,
    EchoMessage,
}

#[derive(Debug)]
pub struct Client {
    pub nick: Option<String>,
    pub user: Option<String>,
    pub capabilities: HashSet<Capability>,
    pub state: ClientState,
    pub sender: UnboundedSender<String>,

}

#[derive(Debug)]
pub enum ClientState {
    Unregistered,
    Registered,
    Authenticated,
}


impl Client {
    pub fn new(nickname: String, sender: UnboundedSender<String> ) -> Self {
        Self {
            nick: Some(nickname.clone()),
            user: Some(nickname.clone()),
            capabilities: HashSet::new(),
            state: ClientState::Unregistered,
            sender
        }
    }

    #[tracing::instrument(
        name = "Handling cap commands",
    )]
    pub fn handle_cap_command(&mut self, cmd: &Command) -> Option<String> {
        match cmd {
            Command::CapLs => {
                let caps = vec!["multi-prefix", "sasl", "echo-message"].join(" ");
                tracing::debug!("Sending CAP * LS response");
                Some(format!("CAP * LS :{}\r\n", caps))
            }
        

            Command::CapReq(requested_caps) => {
                let supported_caps = vec![":multi-prefix", ":sasl", ":echo-message"];
                let mut ack_caps = vec![];

                for cap in requested_caps {
                    if supported_caps.contains(&cap.as_str()) {
                        match cap.as_str() {
                            ":multi-prefix" => {
                                self.capabilities.insert(Capability::MultiPrefix);
                            },
                            ":sasl" => {
                                self.capabilities.insert(Capability::SASL);
                            },
                            ":echo-message" => {
                                self.capabilities.insert(Capability::EchoMessage);
                            },
                            _ => {}
                        }
                        ack_caps.push(cap.clone());
                    }
                }

                if !ack_caps.is_empty() {
                    Some(format!("CAP * ACK :{}\r\n", ack_caps.join(" ")))
                } else {
                    Some("CAP * NAK :No valid capabilities\r\n".to_string())
                }
            }

            Command::CapEnd => {
                self.state = ClientState::Registered;
                None
            }
            
            _ => {
                Some("CAP * NAK :Invalid command\r\n".to_string())
            }
        }
    }
}