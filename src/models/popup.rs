use serde::{Serialize, Deserialize};  

use chrono::Utc;
use chrono::DateTime;

#[derive(Serialize, Deserialize)]  

pub struct Popup {  
    pub id: i32,  
    pub content: String,  
    pub status: i32,  
    pub created_at: Option<DateTime<Utc>>,  
    pub updated_at: Option<DateTime<Utc>>,  
}  