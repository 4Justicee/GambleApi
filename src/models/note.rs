use serde::{Serialize, Deserialize};  

use chrono::Utc;
use chrono::DateTime;
#[derive(Serialize, Deserialize)]  

pub struct Note {  
    pub id: i32,  
    pub sender_code: String,  
    pub receiver_code: String,  
    pub note_title: String,  
    pub note_content: String,  
    pub read_status: i32,  
    pub parent_path: String,  
    pub created_at: Option<DateTime<Utc>>,  
    pub updated_at: Option<DateTime<Utc>>,  
} 