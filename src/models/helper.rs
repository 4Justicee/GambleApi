use serde::{Serialize, Deserialize};  
use chrono::Utc;
use chrono::DateTime;
#[derive(Serialize, Deserialize)]  

pub struct Helper {  
    pub id: i32,  
    pub agent_code: String,  
    pub user_code: String,  
    pub provider_code: String,  
    pub game_code: String,  
    pub machine_id: Option<String>, // Nullable with default nullptr  
    pub server_call_id: i32,  
    pub bet: f64,  
    pub user_prev: Option<f64>,  
    pub user_after: Option<f64>,  
    pub agent_prev: Option<f64>,  
    pub agent_after: Option<f64>,  
    pub expect: Option<f64>,  
    pub missed: Option<f64>,  
    pub real: Option<f64>,  
    pub rtp: f64,  
    pub type_: i32, // Suffix underscore because 'type' is a reserved keyword in Rust  
    pub status: i32,  
    pub msg: Option<String>,  
    pub parent_path: String,  
    pub created_at: Option<DateTime<Utc>>,  
    pub updated_at: Option<DateTime<Utc>>,  
}  