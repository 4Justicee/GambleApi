use serde::{Serialize, Deserialize};  
use sqlx::FromRow;  

#[derive(Serialize, Deserialize, Debug, FromRow)]  

pub struct Provider {  
    pub id: i32,  
    pub code: String,  
    pub name: String,  
    pub provider_type: String, 
    pub endpoint: String,  
    pub vendor_key: String,  
    pub backoffice: String,  
    pub percent: i32,  
    pub status: Option<i32>,  // Using Option<T> since it may not be provided, and default is 1  
    pub config: String,  // representing TEXT as String  
}  
