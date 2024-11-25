use serde::{Serialize, Deserialize};  
use validator::{Validate};  
use chrono::NaiveDate;

#[derive(Serialize, Deserialize, Validate)]  
pub struct GameLaunch {  
    #[validate(length(min = 1))]  
    pub master_code: String,  

    #[validate(length(min = 1))]  
    pub master_token: String,  

    #[validate(length(min = 1))]  
    pub player_code: String,  

    pub game_type: String,  

    #[validate(length(min = 1))]  
    pub provider_code: String,  

    pub game_code: String,  

    #[validate(length(min = 1))]  
    pub lang: String,  

    #[validate(range(min = 0.00, max = 1000000.00))]  
    pub deposit_amount: Option<f64>,  

    #[validate(range(min = 0.00, max = 1000000.00))]  
    pub player_balance: Option<f64>,  
}  

#[derive(Serialize, Deserialize, Validate)]  
pub struct UserCreate {  
    #[validate(length(min = 1))]  
    master_code: String,  

    #[validate(length(min = 1))]  
    master_token: String,  

    #[validate(length(min = 1))]  
    player_code: String,  

    #[validate(range(min = 0.00))]  
    deposit_amount: Option<f64>,  
}  

#[derive(Serialize, Deserialize, Validate)]  
pub struct UserDeposit {  
    #[validate(length(min = 1))]  
    master_code: String,  

    #[validate(length(min = 1))]  
    master_token: String,  

    #[validate(length(min = 1))]  
    player_code: String,  

    #[validate(range(min = 0.01))]  
    amount: f64,  
}  

#[derive(Serialize, Deserialize, Validate)]  
pub struct UserWithdraw {  
    #[validate(length(min = 1))]  
    pub master_code: String,  

    #[validate(length(min = 1))]  
    pub master_token: String,  

    #[validate(length(min = 1))]  
    pub player_code: String,  

    #[validate(range(min = 0.01))]  
    pub amount: Option<f64>,  
}  

#[derive(Serialize, Deserialize, Validate)]  
pub struct UserWithdrawAll {  
    #[validate(length(min = 1))]  
    pub master_code: String,  

    #[validate(length(min = 1))]  
    pub master_token: String,  
}  

#[derive(Serialize, Deserialize, Validate)]  
pub struct Info {  
    #[validate(length(min = 1))]  
    pub master_code: String,  
    
    #[validate(length(min = 1))]  
    pub master_token: String,  

    pub player_code: Option<String>,  
}

#[derive(Serialize, Deserialize, Validate)]  
pub struct ProviderList {  
    #[validate(length(min = 1))]  
    pub master_code: String,  

    #[validate(length(min = 1))]  
    pub master_token: String,  

    pub game_type: Option<String>,  
}  

#[derive(Serialize, Deserialize, Validate)]  
pub struct GameList {  
    #[validate(length(min = 1))]  
    pub master_code: String,  

    #[validate(length(min = 1))]  
    pub master_token: String,  

    #[validate(length(min = 1))]  
    pub provider_code: String,  

    pub player_code: Option<String>,  

    pub game_type: Option<String>,  

    pub lang: Option<String>,  
}  
#[derive(Serialize, Deserialize, Validate)]  
pub struct GetDateLog {  
    #[validate(length(min = 1))]  
    pub master_code: String,  

    #[validate(length(min = 1))]  
    pub master_token: String,  

    pub player_code: Option<String>,  

    #[validate(length(min = 1))]  
    pub game_type: String,  

    pub start: NaiveDate,  
    pub end: NaiveDate,  
    pub page: i32,  

    #[validate(range(min = 0, max = 1000))]  
    pub length: i32,  

    pub search: Option<String>,  
}  
#[derive(Serialize, Deserialize, Validate)]  
pub struct GetIdLog {  
    #[validate(length(min = 1))]  
    pub master_code: String,  

    #[validate(length(min = 1))]  
    pub master_token: String,  

    pub player_code: Option<String>,  

    #[validate(length(min = 1))]  
    pub game_type: String,  

    #[validate(range(min = 0))]  
    pub last_history_id: i32,  
}  

#[derive(Serialize, Deserialize, Validate)]  
pub struct GetExchangeHistory {  
    #[validate(length(min = 1))]  
    pub master_code: String,  

    #[validate(length(min = 1))]  
    pub master_token: String,  

    pub player_code: Option<String>,  

    pub start: NaiveDate,  
    pub end: NaiveDate,  

    #[validate(range(min = 0))]  
    pub page: i32,  

    #[validate(range(min = 0, max = 1000))]  
    pub length: i32,  
}  

#[derive(Serialize, Deserialize, Validate)]  
pub struct CurrentPlayers {  
    #[validate(length(min = 1))]  
    pub master_code: String,  

    #[validate(length(min = 1))]  
    pub master_token: String,  
}  

#[derive(Serialize, Deserialize, Validate)]  
pub struct CallList {  
    #[validate(length(min = 1))]  
    pub master_code: String,  

    #[validate(length(min = 1))]  
    pub master_token: String,  

    #[validate(length(min = 1))]  
    pub provider_code: String,  

    #[validate(length(min = 1))]  
    pub game_code: String,  

    #[validate(length(min = 1))]  
    pub player_code: String,  

    #[validate(range(min = 1, max = 2))]  
    pub call_type: i32,  

    pub machine_id: Option<String>,  
}  

#[derive(Serialize, Deserialize, Validate)]  
pub struct CallApply {  
    #[validate(length(min = 1))]  
    pub master_code: String,  

    #[validate(length(min = 1))]  
    pub master_token: String,  

    #[validate(length(min = 1))]  
    pub provider_code: String,  

    #[validate(length(min = 1))]  
    pub game_code: String,  

    #[validate(length(min = 1))]  
    pub player_code: String,  

    #[validate(range(min = 0.01))]  
    pub call_rtp: f64,  

    #[validate(range(min = 1, max = 2))]  
    pub call_type: i32,  

    pub machine_id: Option<String>,  
}  

#[derive(Serialize, Deserialize, Validate)]  
pub struct CallCancel {  
    #[validate(length(min = 1))]  
    pub master_code: String,  

    #[validate(length(min = 1))]  
    pub master_token: String,  

    #[validate(range(min = 0))]  
    pub call_id: i32,  
}  

#[derive(Serialize, Deserialize, Validate)]  
pub struct CallHistory {  
    #[validate(length(min = 1))]  
    pub master_code: String,  

    #[validate(length(min = 1))]  
    pub master_token: String,  

    #[validate(range(min = 0))]  
    pub offset: i32,  

    #[validate(range(min = 1))]  
    pub limit: i32,  

    #[validate(range(min = 0))]  
    pub last_call_id: Option<i32>,  

    pub order_dir: Option<String>,  
}  

#[derive(Serialize, Deserialize, Validate)]  
pub struct AgentRtp {  
    #[validate(length(min = 1))]  
    pub master_code: String,  

    #[validate(length(min = 1))]  
    pub master_token: String,  

    #[validate(range(min = 20, max = 95))]  
    pub agent_rtp: f64,  
}  

#[derive(Serialize, Deserialize, Validate)]  
pub struct UserRtp {  
    #[validate(length(min = 1))]  
    pub master_code: String,  

    #[validate(length(min = 1))]  
    pub master_token: String,  

    #[validate(length(min = 1))]  
    pub provider_code: String,  

    pub player_code: Option<String>,  

    #[validate(range(min = 20, max = 95))]  
    pub player_rtp: f64,  
}  

#[derive(Serialize, Deserialize)]  
pub struct RequestBody {  
    pub agent_code: String,  
    pub agent_secret: String,  
    pub agent_type: String,  
    pub user_code: String,  
    pub  provider_code: String,  
    pub game_code: String,  
    pub type_: String, // `type` is a reserved keyword in Rust  
    pub txn_id: String,  
    pub agent_before_balance: f64,  
    pub agent_after_balance: f64,  
    pub user_before_balance: f64,  
    pub user_after_balance: f64,  
    pub amount: f64,  
    pub msg: String,  
}  

#[derive(Serialize)]  
pub struct GameServerRequestData {  
    pub agent_code: String,  
    pub user_code: String,  
    pub currency: String,  
    pub game_code: String,  
    pub balance: f64,  
    pub rtp: f64,  // Assuming RTP might be optional  
    pub lang: String,  
    pub jackpot_come: i32,  
    pub site_end_point: String,  
    pub is_test: bool,  
}  

#[derive(Deserialize)]  
pub struct GameServerResponse {  
    pub status: i32,  
    pub url: Option<String>,  
    pub msg: Option<String>,  
}  

#[derive(Default, Serialize, Deserialize)]  
pub struct GameLaunchResult {  
    pub status: i32,  
    pub msg: String,  
    pub launch_url: Option<String>,  
    pub master_code: String,  
    pub master_balance: f64,  
    pub master_type: String,  
    pub player_code: String,  
    pub player_balance: f64,  
    pub player_created: bool, 
    pub player_deposit: bool,  // This might need handling based on the context  
    pub currency: String,  
    pub lang: String,  
}  