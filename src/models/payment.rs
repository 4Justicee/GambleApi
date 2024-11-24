use serde::{Serialize, Deserialize};  

use chrono::Utc;
use chrono::DateTime;
#[derive(Serialize, Deserialize)]  

pub struct Payment {  
    pub id: i32,  
    pub agent_code: Option<String>,  
    pub parent_id: Option<i32>,  
    pub parent_path: Option<String>,  
    pub key: Option<String>,  
    pub order_id: Option<String>,  
    pub invoice_id: Option<String>,  
    pub purchase_id: Option<String>,  
    pub order_description: Option<String>,  
    pub outcome_amount: Option<String>,  
    pub outcome_currency: Option<String>,  
    pub pay_address: Option<String>,  
    pub pay_amount: Option<String>,  
    pub pay_currency: Option<String>,  
    pub payment_id: Option<String>,  
    pub payment_status: Option<String>,  
    pub price_amount: Option<String>,  
    pub price_currency: Option<String>,  
    pub actually_paid: Option<String>,  
    pub status: Option<String>,  
    pub deposit_amount: Option<String>,  
    pub agent_before_balance: Option<String>,  
    pub agent_after_balance: Option<String>,   
    pub created_at: Option<DateTime<Utc>>,  
    pub updated_at: Option<DateTime<Utc>>,  
}  