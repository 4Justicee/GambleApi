use serde::{Serialize, Deserialize};  
use sqlx::FromRow;  
#[derive(Serialize, Deserialize, Debug, FromRow)]  

pub struct Game {  
    pub id: i32,  
    pub provider_code: String,  
    pub game_type: String,  
    pub en_name: String,  
    pub game_code: String,  
    pub banner: Option<String>, // Assuming banner is nullable because no allowNull set to false or defaultValue.  
    pub status: i32,  // Default value in SQLx is handled differently, typically set in the SQL schema.  
}  