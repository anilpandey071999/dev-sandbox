use std::collections::HashMap;
use raft::prelude::*;

#[tokio::main]
async fn start_node() {
    let config = Config { id: 1, ..Default::default() } ;
    // let store
    // raft::RawNode::new(&config, store, logger)
}
