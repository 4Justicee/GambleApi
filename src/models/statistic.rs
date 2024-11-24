use serde::{Serialize, Deserialize};  


#[derive(Serialize, Deserialize)]  

pub struct SlotStatistics {  
    pub id: i32,  
    pub agent_code: String,  
    pub user_code: String,  
    pub provider_code: String,  
    pub game_code: String,  
    pub bet_count: i32,  
    pub bet_amount: f64, // Using f64 to represent Sequelize's DOUBLE  
    pub win_count: i32,  
    pub win_amount: f64,  
    pub spending_amount: f64,  
    pub free_count: i32,  
    pub free_bet_amount: f64,  
    pub free_win_amount: f64,  
    pub buy_count: i32,  
    pub buy_bet_amount: f64,  
    pub buy_win_amount: f64,  
    pub call_count: i32,  
    pub call_bet_amount: f64,  
    pub call_win_amount: f64,  
}  