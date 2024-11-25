use serde::{Serialize, Deserialize};  
use sqlx::FromRow;  

#[derive(Serialize, Deserialize, Debug, FromRow)]  


pub struct AgentBlock {  
    pub id: i32,  
    pub agent_code: String,  
    pub block_provider_code: String,  
    pub block_game_code: String,   
}  