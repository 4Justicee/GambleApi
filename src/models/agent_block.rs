use serde::{Serialize, Deserialize};  
use chrono::Utc;
use chrono::DateTime;
#[derive(Serialize, Deserialize)]  

pub struct AgentBlock {  
    pub id: i32,  
    pub agent_code: String,  
    pub block_provider_code: Option<String>,  
    pub block_game_code: Option<String>,  
    pub created_at: Option<DateTime<Utc>>,  
    pub updated_at: Option<DateTime<Utc>>,  
}  