use serde::{Serialize, Deserialize};  
use chrono::Utc;
use chrono::DateTime;
#[derive(Serialize, Deserialize)]  

pub struct Check {  
    pub id: i32,  
    pub content: String,  
    pub status: i32,  // Assuming status as an integer (e.g., 0: Maintenancing, 1: Normal)  
    pub start_time: DateTime<Utc>, // Adjust these if they actually need to be stored as strings  
    pub end_time: DateTime<Utc>,  
    pub created_at: Option<DateTime<Utc>>,  
    pub updated_at: Option<DateTime<Utc>>,  
}  