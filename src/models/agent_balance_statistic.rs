use serde::{Serialize, Deserialize};  
use chrono::NaiveDateTime; // For DateTime fields if any  

#[derive(Serialize, Deserialize)]  

pub struct AgentBalanceStatistics {  
    pub id: i32,  
    pub agent_code: String,  
    pub agent_name: String,  
    pub api_type: i32,  
    pub agent_type: i32,  
    pub parent_id: Option<i32>,  
    pub status: i32,  
    pub depth: i32,  
    pub role: i32,  
    pub parent_path: String,  
    pub agent_balance: f64,  
    pub direct_user_balance_sum: f64,  
    pub child_agent_balance_sum: f64,  
    pub child_user_balance_sum: f64,  
    pub direct_user_count: i32,  
    pub child_agent_count: i32,  
    pub child_user_count: i32,  
    pub total_debit: Option<f64>,  
    pub total_credit: Option<f64>,  
    pub created_at: Option<NaiveDateTime>,  
    pub updated_at: Option<NaiveDateTime>,  
}  
