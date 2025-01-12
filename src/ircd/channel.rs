use std::collections::HashSet;

#[derive(Debug)]
pub struct Channel {
    pub name: String,
    pub topic: String,
    pub users: HashSet<String>,
    pub modes: HashSet<String>,
}

impl Channel {
    pub fn new(name: String) -> Self {
        Self {
            name,
            topic: String::new(),
            users: HashSet::new(),
            modes: HashSet::new(),
        }
    }
}
