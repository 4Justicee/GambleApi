use serde::{Serialize, Deserialize};  
use sqlx::FromRow;  

#[derive(Serialize, Deserialize, Debug, FromRow)]  

pub struct Agent {  
    pub id: i32,  
    pub agent_code: String,  
    pub agent_name: String,  
    pub password: String,  
    pub api_type: i32,  
    pub agent_type: i32,  
    pub rtp: i32,  
    pub balance: f64,  
    pub total_balance: f64,  
    pub total_credit: f64,  
    pub total_debit: f64,  
    pub percent: f64,  
    pub memo: Option<String>,  
    pub admin_memo: Option<String>,  
    pub token: String,  
    pub secret_key: String,  
    pub site_end_point: String,  
    pub parent_id: Option<i32>,  
    pub status: i32,      
    pub depth: i32,  
    pub role: i32,  
    pub ip_address: String,      
    pub parent_path: String,  
    pub providers: String,  
    pub zero_setting: String,  
    pub zero_array: Option<String>,  
    pub cur_index: i32,      
    pub jackpot_come: i32,  
    pub lang: String,  
    pub currency: String,  
    pub open_call_api: i32,      
    pub open_pachinko_api: i32,  
    pub open_reel_api: i32,  
    pub allow_manage_call_and_rtp: i32,  
    pub allow_billing: i32,  
    pub billing_address: String,  
}  