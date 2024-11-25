use serde::{Serialize, Deserialize};  
use sqlx::FromRow;  

#[derive(Serialize, Deserialize, Debug, FromRow)]  

pub struct User {  
    pub id: i32,  
    pub agent_code: String,  
    pub user_code: String,  
    pub target_rtp: f64,  
    pub real_rtp: f64, // f64 used to represent Sequelize's DOUBLE  
    pub balance: f64,  
    pub status: i32, // With comments noted, such as "1: OK, 2: Delete"  
    pub game_stop: i32, // Comments noted as "0: OK, 1: STOP, force stop game"  
    pub parent_path: String,  
    pub total_debit: f64,  
    pub total_credit: f64,  
    pub total_play_count: i32,  
    pub api_type: i32, // With comments noted as "0: Seamless, 1: Transfer"  
}  