use actix_web::{web, error, HttpResponse, http::StatusCode};  
use crate::models::agent::Agent; // Assuming Agent is a model in models/agent.rs  
use crate::models::provider::Provider; // Assuming Agent is a model in models/agent.rs  
use crate::models::user::User; // Assuming Agent is a model in models/agent.rs  
use crate::defines::game_launch::GameLaunch;  
use crate::defines::game_launch::UserCreate;  
use crate::defines::game_launch::UserDeposit;  
use crate::defines::game_launch::UserWithdraw;  
use crate::defines::game_launch::UserWithdrawAll;  
use crate::defines::game_launch::Info;  
use crate::defines::game_launch::ProviderList;  
use crate::defines::game_launch::GameList;  
use crate::defines::game_launch::GetDateLog;  
use crate::defines::game_launch::GetIdLog;  
use crate::defines::game_launch::GetExchangeHistory;  

use sqlx::postgres::PgPoolOptions;  
use sqlx::{PgPool, FromRow, Error as SqlxError};  

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

    let agent_result = sqlx::query_as::<_, Agent>(  
        "SELECT * FROM agents WHERE agent_code = 1"  
    )  
    .bind(&body.master_code)  
    .fetch_optional(pool.as_ref())  
    .await;  

    let agent = match agent_result {  
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

    let user_created = false;  
    let userBalance = 0;
    let userApiType = 0;

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
