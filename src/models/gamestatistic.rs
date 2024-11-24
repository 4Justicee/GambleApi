use serde::{Serialize, Deserialize};  

#[derive(Serialize, Deserialize)]  

pub struct GameStatistics {  
    pub id: i32,  
    pub agent_code: String,  
    pub parent_path: String,  
    pub user_code: String,  
    pub provider_code: String,  
    pub game_code: String,  
    pub currency: String,  
    pub bet_count: i32,  
    pub bet_amount: f64, // using f64 to represent DOUBLE  
    pub win_count: i32,  
    pub win_amount: f64, // using f64 to represent DOUBLE  
    pub spending_amount: f64, // using f64 to represent DOUBLE  
    pub free_count: i32,  
    pub free_bet_amount: f64, // using f64 to represent DOUBLE  
    pub free_win_amount: f64, // using f64 to represent DOUBLE  
    pub buy_count: i32,  
    pub buy_bet_amount: f64, // using f64 to represent DOUBLE  
    pub buy_win_amount: f64, // using f64 to represent DOUBLE  
    pub call_count: i32,  
    pub call_bet_amount: f64, // using f64 to represent DOUBLE  
    pub call_win_amount: f64, // using f64 to represent DOUBLE  
} 