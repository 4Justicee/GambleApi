use serde::{Serialize, Deserialize};  


#[derive(Serialize, Deserialize)]  

pub struct UserBalance {  
    pub id: i32,  
    pub agent_code: String,  
    pub user_code: String,  
    pub user_prev_balance: Option<f64>, // Optional since allowNull is true  
    pub user_after_balance: Option<f64>, // Optional since allowNull is true  
    pub amount: Option<f64>, // Optional since allowNull is true  
    pub target: String,  
    pub direction: String,  
    pub cause: String,  
    pub comment: String,  
    pub parent_path: String,  
} 