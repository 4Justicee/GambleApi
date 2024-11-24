use serde::{Serialize, Deserialize};  
use chrono::Utc;
use chrono::DateTime;

#[derive(Serialize, Deserialize)]  

pub struct AgentApiHistory {  
    pub id: i32,  
    pub agent_code: String,  
    pub parent_path: Option<String>,  
    pub type_: Option<String>,  // `type_` since `type` is a reserved keyword in Rust  
    pub method: Option<String>,  
    pub url: Option<String>,  
    pub ip: Option<String>,  
    pub req_body: Option<String>,  
    pub req_size: Option<f64>,  
    pub req_time: Option<String>,  // Consider using `DateTime<Utc>` if storing timestamp  
    pub res_body: Option<String>,  
    pub res_size: Option<f64>,  
    pub res_time: Option<String>,  // Consider using `DateTime<Utc>` if storing timestamp  
    pub delay_time: Option<f64>,  
    pub created_at: Option<DateTime<Utc>>,  
    pub updated_at: Option<DateTime<Utc>>,  
} 