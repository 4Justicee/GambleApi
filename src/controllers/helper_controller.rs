use actix_web::{web, HttpResponse, Responder};  
use crate::models::agent::Agent; // Assuming Agent is a model in models/agent.rs  

use crate::defines::types::CurrentPlayers;  
use crate::defines::types::CallList;  
use crate::defines::types::CallApply;  
use crate::defines::types::CallCancel;  
use crate::defines::types::CallHistory;  
use crate::defines::types::AgentRtp;  
use crate::defines::types::UserRtp;  

pub async fn current_players(body: web::Json<CurrentPlayers>) -> HttpResponse {  
    HttpResponse::Ok().body("agent data")
}

pub async fn call_list(body: web::Json<CallList>) -> HttpResponse {  
    HttpResponse::Ok().body("agent data")
}

pub async fn call_apply(body: web::Json<CallApply>) -> HttpResponse {  
    HttpResponse::Ok().body("agent data")
}

pub async fn call_cancel(body: web::Json<CallCancel>) -> HttpResponse {  
    HttpResponse::Ok().body("agent data")
}

pub async fn call_history(body: web::Json<CallHistory>) -> HttpResponse {  
    HttpResponse::Ok().body("agent data")
}

pub async fn agent_rtp(body: web::Json<AgentRtp>) -> HttpResponse {  
    HttpResponse::Ok().body("agent data")
}

pub async fn user_rtp(body: web::Json<UserRtp>) -> HttpResponse {  
    HttpResponse::Ok().body("agent data")
}

