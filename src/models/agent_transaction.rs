use serde::{Serialize, Deserialize};  
use chrono::Utc;
use chrono::DateTime;
#[derive(Serialize, Deserialize)]  

pub struct AgentTransaction {  
    pub id: i32,  
    pub operator_code: String,  
    pub parent_code: String,  
    pub agent_code: String,  
    pub charge_type: i32,  
    pub charge_amount: f64,  
    pub operator_prev_balance: f64,  
    pub operator_after_balance: f64,  
    pub parent_prev_balance: f64,  
    pub parent_after_balance: f64,  
    pub agent_prev_balance: f64,  
    pub agent_after_balance: f64,  
    pub operator_prev_total_balance: f64,  
    pub operator_after_total_balance: f64,  
    pub parent_prev_total_balance: f64,  
    pub parent_after_total_balance: f64,  
    pub agent_prev_total_balance: f64,  
    pub agent_after_total_balance: f64,  
    pub status: i32,  
    pub parent_path: String,  
    pub memo: Option<String>, // Assuming nullable  
    pub created_at: Option<DateTime<Utc>>,  // Assuming timestamp  
    pub updated_at: Option<DateTime<Utc>>,  // Assuming timestamp  
}  