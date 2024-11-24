use serde::{Serialize, Deserialize}; 
use chrono::Utc;
use chrono::DateTime;
#[derive(Serialize, Deserialize)]  

pub struct Message {  
    pub id: i32,  
    pub sender_code: String,  
    pub receiver_code: String,  
    pub message_title: String,  
    pub message_content: String,  
    pub read_status: i32,  
    pub answer_content: String,  
    pub answer_status: i32,  
    pub parent_path: String,  
    pub created_at: Option<DateTime<Utc>>,  
    pub updated_at: Option<DateTime<Utc>>,  
}  