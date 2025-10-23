pub struct Node {
    pub id: String,
    pub leader_address: Option<String>,
    pub follower_address: Vec<String>,
    pub log_checksum: String,
}

impl Node {
    pub fn new(is_leader: bool, leader_address: String) -> Self {
        let id: String = uuid::Uuid::new_v4().into();
        if is_leader {
            return Self {
                id,
                leader_address: Some(leader_address),
                follower_address: Vec::new(),
                log_checksum: String::new(),
            };
        }
        Self {
            id,
            leader_address: None,
            follower_address: Vec::new(),
            log_checksum: String::new(),
        }
    }
}
