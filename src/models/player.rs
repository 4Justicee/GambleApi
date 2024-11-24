use serde::{Serialize, Deserialize};  

use chrono::Utc;
use chrono::DateTime;

#[derive(Serialize, Deserialize)]  

pub struct Player {  
    pub id: i32,  
    pub user_id: String,  
    pub agent_code: String,  
    pub parent_path: String,  
    pub user_code: String,  
    pub provider_code: String,  
    pub game_code: String,  
    pub machine_id: Option<String>, // Can be null  
    pub last_bet: Option<f64>,  
    pub last_win: Option<f64>,  
    pub status: Option<String>,  
    pub created_at: Option<DateTime<Utc>>,  
    pub updated_at: Option<DateTime<Utc>>,  
}  