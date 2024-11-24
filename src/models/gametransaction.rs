use serde::{Serialize, Deserialize};  

#[derive(Serialize, Deserialize)]  

pub struct GameTransaction {  
    pub id: i32,  
    pub agent_code: String,  
    pub user_code: String,  
    pub provider_code: String,  
    pub game_code: String,  
    pub round_id: Option<String>, // Matches Sequelize's `roundId: { type: Sequelize.STRING }`  
    pub game_name: Option<String>, // Matches Sequelize's `allowNull: true`  
    pub game_category: Option<String>,  
    pub transaction_type: String, // `type` is a reserved word in Rust, hence `transaction_type`  
    pub bet: f64,  
    pub win: f64,  
    pub currency: String,  
    pub txn_id: String,  
    pub txn_type: String,  
    pub agent_start_balance: f64,  
    pub agent_end_balance: f64,  
    pub user_start_balance: f64,  
    pub user_end_balance: f64,  
    pub parent_path: String,  
    pub is_buy: i16,    // TINYINT maps to i16 or i8 in Rust, depending on needed range.  
    pub is_call: i16,   // As above.  
    pub pattern_id: Option<String>,  
    pub machine_id: Option<String>,  
    pub total_debit: f64,  
    pub total_credit: f64,  
    pub real_rtp: f64,  
}  