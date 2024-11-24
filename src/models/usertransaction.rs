use serde::{Serialize, Deserialize};  


#[derive(Serialize, Deserialize)]  

pub struct UserTransaction {  
    pub id: i32,  
    pub operator_code: String,  
    pub agent_code: String,  
    pub user_code: String,  
    pub charge_type: Option<i32>, // chargeType has optional comments, suggesting it might be optional  
    pub charge_method: Option<String>, // chargeMethod as optional following above reasoning  
    pub charge_amount: f64,  
    pub agent_prev_balance: f64,  
    pub agent_after_balance: f64,  
    pub agent_prev_total_balance: f64,  
    pub agent_after_total_balance: f64,  
    pub user_prev_balance: f64,  
    pub user_after_balance: f64,  
    pub status: Option<i32>, // status is marked without allowNull, implying it might be nullable  
    pub parent_path: String,  
}  