use serde::{Serialize, Deserialize};  
use chrono::Utc;
use chrono::DateTime;
#[derive(Serialize, Deserialize)]  

pub struct AgentBalanceProgress {  
    pub id: i32,  
    pub agent_code: String,  
    pub agent_prev_balance: Option<f64>,  
    pub agent_after_balance: Option<f64>,  
    pub agent_prev_total_balance: Option<f64>,  
    pub agent_after_total_balance: Option<f64>,  
    pub amount: Option<f64>,  
    pub currency: String,  
    pub api_type: i32,  
    pub target: String,  
    pub direction: String,  
    pub cause: String,  
    pub comment: String,  
    pub parent_path: String,  
    pub created_at: Option<DateTime<Utc>>,  // assuming a timestamp column exists for creation  
    pub updated_at: Option<DateTime<Utc>>,  // assuming a timestamp column exists for updates  
}  