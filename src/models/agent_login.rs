use serde::{Serialize, Deserialize};  
use chrono::Utc;
use chrono::DateTime;
#[derive(Serialize, Deserialize)]  

pub struct AgentLoginHistory {  
    pub id: i32,  
    pub agent_code: String,  
    pub password: String,  
    pub ip: String,  
    pub user_agent: String,  
    pub created_at: Option<DateTime<Utc>>,  
    pub updated_at: Option<DateTime<Utc>>,  
}  