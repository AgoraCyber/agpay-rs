use serde::{Deserialize, Serialize};

/// Client context data for PTLC
#[derive(Debug, Serialize, Deserialize)]
pub struct ClientPTLC {}

impl ClientPTLC {}

/// Server context data for PTLC
#[derive(Debug, Serialize, Deserialize)]
pub struct ServerPTLC {}
