use serde::{Serialize, Deserialize};  

use chrono::Utc;
use chrono::DateTime;

#[derive(Serialize, Deserialize)]  

pub struct Plan {  
    pub id: i32,  
    pub name: String,  
    pub amount: String,  // Retained as string based on Sequelize model use  
    pub bonus_percent: i32,  
    pub allow_ggr: i32,  
    pub memo: String,  
    pub created_at: Option<DateTime<Utc>>,  
    pub updated_at: Option<DateTime<Utc>>,  
} 