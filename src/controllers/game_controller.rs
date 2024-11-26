use actix_web::{web, HttpResponse};  
use super::super::models::user::User;  

pub async fn get_users() -> HttpResponse {  
    // HttpResponse::Ok().json(vec![  
    //     User { id: 1, name: String::from("Alice") },  
    //     User { id: 2, name: String::from("Bob") },  
    // ])  
    HttpResponse::Ok().body("agent data")
}  