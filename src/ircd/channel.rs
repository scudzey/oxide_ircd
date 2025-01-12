use std::{collections::{HashMap, HashSet}, sync::Arc};

use tokio::sync::RwLock;

use super::client::Client;

#[derive(Debug)]
pub struct Channel {
    pub name: String,
    pub topic: String,
    pub users: HashMap<String, Arc<RwLock<Client>>>,
    pub modes: HashSet<String>,
}

impl Channel {
    pub fn new(name: String) -> Self {
        Self {
            name,
            topic: String::new(),
            users: HashMap::new(),
            modes: HashSet::new(),
        }
    }
}
