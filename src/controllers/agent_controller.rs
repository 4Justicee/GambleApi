use actix_web::{web, error, HttpResponse};  
use crate::models::agent::Agent; // Assuming Agent is a model in models/agent.rs  
use crate::models::provider::Provider; // Assuming Agent is a model in models/agent.rs  
use crate::models::user::User; // Assuming Agent is a model in models/agent.rs  
use crate::defines::types::GameLaunch;  
use crate::defines::types::UserCreate;  
use crate::defines::types::UserDeposit;  
use crate::defines::types::UserWithdraw;  
use crate::defines::types::UserWithdrawAll;  
use crate::defines::types::Info;  
use crate::defines::types::ProviderList;  
use crate::defines::types::GameList;  
use crate::defines::types::GetDateLog;  
use crate::defines::types::GetIdLog;  
use crate::defines::types::GetExchangeHistory;  
use crate::defines::types::RequestBody;  

use serde_json::json;  
use sqlx::{PgPool};  
use reqwest::Client;  

use sqlx::query_as;

fn sqlx_error_to_actix_error(e: sqlx::Error) -> actix_web::Error {  
    match &e {  
        sqlx::Error::RowNotFound => {  
            error::ErrorNotFound("Resource not found")  
        }  
        _ => {  
            error::ErrorInternalServerError("Internal Server Error")  
        }  
    }  
}  

pub async fn game_launch(pool: web::Data<PgPool>, body: web::Json<GameLaunch>) -> Result<HttpResponse, actix_web::Error> {                   
    let game_type = body.game_type.as_str();
    let provider_code = body.provider_code.as_str();
    let agent_code = body.master_code.as_str();
    let user_code = body.player_code.as_str();
    let player_balance = &body.player_balance;

    let agent_result = sqlx::query_as::<_, Agent>(  
        "SELECT * FROM agents WHERE agent_code = 1"  
    )  
    .bind(&body.master_code)  
    .fetch_optional(pool.as_ref())  
    .await;  

    let mut agent = match agent_result {  
        Ok(Some(agent)) => agent,  
        Ok(None) => return Ok(HttpResponse::NotFound().finish()),  
        Err(e) => return Err(sqlx_error_to_actix_error(e)),  
    };  

    let provider = if game_type == "sports" {  
        sqlx::query_as::<_, Provider>("SELECT * FROM providers WHERE provider_type = $1")  
            .bind("sports")  
            .fetch_one(pool.get_ref())  
            .await  
    } else {  
        sqlx::query_as::<_, Provider>("SELECT * FROM providers WHERE code = $1")  
            .bind(provider_code)  
            .fetch_one(pool.get_ref())  
            .await  
    };  

    let user_result = query_as::<_, User>(  
        "SELECT * FROM users WHERE user_code = $1 AND agent_code = $2"  
    )  
    .bind(agent_code)
    .bind(user_code)
    .fetch_optional(pool.get_ref())  
    .await;  

    let mut user_created = false;  
    let mut userBalance: f64 = 0.0;
    let mut userApiType = 1;

    let user = match user_result {  
        Ok(Some(user)) => user,  
        Ok(None) => {  
            let new_user = User {  
                id: 0, // Auto-generated  
                user_code: user_code.to_string(),  
                agent_code: agent_code.to_string(),  
                parent_path: format!("{}.{}", agent.parent_path, agent.id),  
                target_rtp: agent.rtp,  
                balance: 0.0,  
                status: 1,  
                api_type: 1,  
                game_stop: 0,  
                real_rtp: 0.0,  
                total_credit: 0.0,  
                total_debit: 0.0,  
                total_play_count: 0,  
            };  
            let insert_result = sqlx::query("INSERT INTO users (user_code, agent_code, parent_path, target_rtp, balance, status, api_type, game_stop, real_rtp, total_credit, total_debit, total_play_count) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)")  
                .bind(&new_user.user_code)  
                .bind(&new_user.agent_code)  
                .bind(&new_user.parent_path)  
                .bind(new_user.target_rtp)  
                .bind(new_user.balance)  
                .bind(new_user.status)  
                .bind(new_user.api_type)  
                .bind(new_user.game_stop)  
                .bind(new_user.real_rtp)  
                .bind(new_user.total_credit)  
                .bind(new_user.total_debit)  
                .bind(new_user.total_play_count)  
                .execute(pool.as_ref())  
                .await;  

            match insert_result {  
                Ok(_) => new_user,  
                Err(e) => return Err(sqlx_error_to_actix_error(e)),  
            }  
        },  
        Err(e) => return Err(sqlx_error_to_actix_error(e)),  
    };  

    if agent.api_type == 0 {  
        if let Some(player_balance) = player_balance {  
            if player_balance.is_nan() {  
                return  Ok(HttpResponse::BadRequest().json(json!({"status": 0, "msg": "Seamless agent user balance required"})));  
            }  
            // Update user balance  
            let result = sqlx::query("UPDATE users SET balance = $1, api_type = 0 WHERE user_code = $2")  
                .bind(player_balance)  // Bind the first placeholder (\$1) to 'player_balance'  
                .bind(user_code)       // Bind the second placeholder (\$2) to 'user_code'  
                .execute(pool.as_ref())  
                .await;  
        
            match result {  
                Ok(_) => (),
                Err(e) => return Err(sqlx_error_to_actix_error(e)), 
            } 
        } 
        userApiType = 0;
    }
    else {
        if let Some(amount) = body.deposit_amount {  
            let agentPrevBalance = agent.balance;
            let userPrevBalance = user.balance;

            let mut tx = pool.begin().await.map_err(|e| {  
                actix_web::error::ErrorInternalServerError(e)  
            })?;  
          

            // Check if the agent has enough balance  
            if agent.balance < amount {  
                tx.rollback().await.map_err(|e| {  
                    actix_web::error::ErrorInternalServerError(e)  
                })?;  
                
                return Ok(HttpResponse::BadRequest().body("Not enough balance"));  
            }  

            // Decrement the agent's balance  
            agent.balance -= amount;  
            sqlx::query("UPDATE agents SET balance = $1 WHERE agent_code = $2")  
                .bind(agent.balance)  
                .bind(&agent.agent_code)  
                .execute(&mut tx).await.map_err(|e| {  
                    actix_web::error::ErrorInternalServerError(e)  
                })?;  
                
            userBalance = userPrevBalance + amount;
            
            sqlx::query("INSERT INTO user_transactions (agent_code, user_code, charge_amount, agent_prev_balance, agent_after_balance, user_prev_balance, user_after_balance, charge_type, status, parent_path) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)")  
                .bind(&agent.agent_code) // agent_code  
                .bind(&user.user_code) // user_code  
                .bind(amount) // charge_amount  
                .bind(agentPrevBalance) // agent_prev_balance  
                .bind(agent.balance) // agent_after_balance  
                .bind(userPrevBalance) // user_prev_balance  
                .bind(userBalance) // user_after_balance  
                .bind(1) // charge_type  
                .bind(1) // status  
                .bind(&user.parent_path) // parent_path  
                .execute(&mut tx).await.map_err(|e| {  
                    actix_web::error::ErrorInternalServerError(e)  
                })?;     
            
            sqlx::query("INSERT INTO user_balances (agent_code, user_code, user_prev_balance, user_after_balance, amount, target, cause, direction, comment, parent_path) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)")  
                .bind(&agent.agent_code)  
                .bind(&user.user_code)  
                .bind(user.balance) // User previous balance  
                .bind(userBalance)  
                .bind(amount)  
                .bind(&agent.agent_code) // Target is the agent's code  
                .bind("API | USER DEPOSIT") // Cause  
                .bind("Increase") // Direction  
                .bind("") // Comment  
                .bind(&user.parent_path)  
                .execute(&mut tx).await.map_err(|e| {  
                    actix_web::error::ErrorInternalServerError(e)  
                })?;       
            
            sqlx::query("INSERT INTO agent_balance_progress (agent_code, agent_prev_balance, agent_after_balance, agent_prev_total_balance, agent_after_total_balance, currency, api_type, amount, target, cause, direction, comment, parent_path) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)")  
                .bind(&agent.agent_code)  
                .bind(agent.balance) // Agent previous balance  
                .bind(agent.balance)  
                .bind(agent.total_balance) // previous total balance  
                .bind(agent.total_balance) // after total balance, assuming no change  
                .bind(&agent.currency)  
                .bind(&agent.api_type)  
                .bind(amount)  
                .bind(&user.user_code) // Target is the user's code  
                .bind("API | USER DEPOSIT") // Cause  
                .bind("Decrease") // Direction  
                .bind("") // Comment  
                .bind(&agent.parent_path)  
                .execute(&mut tx).await.map_err(|e| {  
                    actix_web::error::ErrorInternalServerError(e)  
                })?;         
            
            sqlx::query("UPDATE users SET balance = $1, api_type = $2 WHERE user_code = $3")  
                .bind(userBalance)  
                .bind(userApiType)  
                .bind(user_code)  
                .execute(&mut tx)  
                .await.map_err(|e| {  
                    actix_web::error::ErrorInternalServerError(e)  
                })?;           

            // Commit transaction  
            tx.commit().await.map_err(|e| {  
                actix_web::error::ErrorInternalServerError(e)  
            })?;  
            
            if !agent.site_end_point.trim().is_empty() { 
                let client = Client::new();  
                let url = format!("{}/callback_api/money_callback", agent.site_end_point.trim());  

                let req_body = RequestBody {  
                    agent_code: agent_code.to_string(),  
                    agent_secret: agent.secret_key.to_string(),  
                    agent_type: match agent.api_type {  
                        1 => "Transfer".to_string(),  
                        _ => "Seamless".to_string()
                    },  
                    user_code: user_code.to_string(),  
                    provider_code: "".to_string(),  
                    game_code: "".to_string(),  
                    type_: "deposit".to_string(),  
                    txn_id: "".to_string(),  
                    agent_before_balance: agentPrevBalance,  
                    agent_after_balance: agent.balance,  
                    user_before_balance: userPrevBalance,  
                    user_after_balance: userBalance,  
                    amount,  
                    msg: "Paying user money with the game launch API.".to_string(),  
                }; 

                let response = client.post(url)  
                    .json(&req_body)  
                    .send()  
                    .await.map_err(|e| {  
                        actix_web::error::ErrorInternalServerError(e)  
                    })?;

            } else {  
                return Ok(HttpResponse::BadRequest().body("Have to input site endpoint"));  
            } 
        }   
        else {
            userBalance = user.balance;
        }
    }

    
    Ok(HttpResponse::Ok().body("agent data"))
} 

pub async fn user_create(body: web::Json<UserCreate>) -> HttpResponse {  
    HttpResponse::Ok().body("agent data")
}

pub async fn user_deposit(body: web::Json<UserDeposit>) -> HttpResponse {  
    HttpResponse::Ok().body("agent data")
}

pub async fn user_withdraw(body: web::Json<UserWithdraw>) -> HttpResponse {  
    HttpResponse::Ok().body("agent data")
}

pub async fn user_withdraw_all(body: web::Json<UserWithdrawAll>) -> HttpResponse {  
    HttpResponse::Ok().body("agent data")
}

pub async fn info(body: web::Json<Info>) -> HttpResponse {  
    HttpResponse::Ok().body("agent data")
}

pub async fn provider_list(body: web::Json<ProviderList>) -> HttpResponse {  
    HttpResponse::Ok().body("agent data")
}

pub async fn game_list(body: web::Json<GameList>) -> HttpResponse {  
    HttpResponse::Ok().body("agent data")
}

pub async fn get_date_log(body: web::Json<GetDateLog>) -> HttpResponse {  
    HttpResponse::Ok().body("agent data")
}

pub async fn get_id_log(body: web::Json<GetIdLog>) -> HttpResponse {  
    HttpResponse::Ok().body("agent data")
}

pub async fn get_exchange_history(body: web::Json<GetExchangeHistory>) -> HttpResponse {  
    HttpResponse::Ok().body("agent data")
}
