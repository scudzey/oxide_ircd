use super::channel::Channel;
use super::client::Client;
use super::command::Command;
use rand::Rng;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, RwLock};
use tracing::instrument;

#[derive(Debug)]
pub struct ServerState {
    pub users: RwLock<HashMap<String, Arc<RwLock<Client>>>>,
    pub channels: RwLock<HashMap<String, Arc<RwLock<Channel>>>>,
}

impl ServerState {
    pub async fn add_client(&self, nickname: String, client: &Arc<RwLock<Client>>) {
        self.users.write().await.insert(nickname, client.clone());
    }
    pub async fn remove_client(&self, nickname: &str) {
        self.users.write().await.remove(nickname);
    }

    pub async fn change_nick(&self, old_nick: &str, new_nick: &str) {
        let mut users = self.users.write().await;
        if let Some(client) = users.remove(old_nick) {
            users.insert(new_nick.to_string(), client);
        }
    }
}

#[derive(Debug)]
pub struct ClientHandle {
    pub sender: mpsc::UnboundedSender<String>,
}

pub type SharedServerState = Arc<ServerState>;

#[instrument]
pub async fn run(listener: TcpListener) -> Result<(), Box<dyn std::error::Error>> {
    let server_state = Arc::new(ServerState {
        users: RwLock::new(HashMap::new()),
        channels: RwLock::new(HashMap::new()),
    });

    loop {
        let (socket, addr) = listener.accept().await.unwrap();
        tracing::info!("Accepted connection from: {}", addr);

        let state_clone = server_state.clone();

        tokio::spawn(async move {
            let _ = handle_client(socket, state_clone).await;
        });
    }
}

#[tracing::instrument(name = "Handling client connection", skip(socket))]
async fn handle_client(
    socket: TcpStream,
    server_state: SharedServerState,
) -> Result<(), Box<dyn std::error::Error>> {
    let (reader, mut writer) = socket.into_split();
    let mut reader = BufReader::new(reader).lines();

    let (client_tx, mut client_rx) = mpsc::unbounded_channel::<String>();

    let nickname = format!("guest{}", rand::thread_rng().gen_range(1..=9999));
    let session: Arc<RwLock<Client>> = Arc::new(RwLock::new(Client::new(nickname, client_tx)));
    let nickname = session.read().await.nick.clone().unwrap();
    server_state.add_client(nickname, &session).await;

    tokio::spawn(async move {
        while let Some(msg) = client_rx.recv().await {
            let _ = writer.write_all(msg.as_bytes()).await;
        }
    });

    while let Ok(Some(line)) = reader.next_line().await {
        let command = Command::parse(&line);
        command.handle(&session, &server_state).await;
    }

    Ok(())
}
