use actix_web::{web, HttpResponse};  
use super::super::controllers::game_controller;  

pub fn config(cfg: &mut web::ServiceConfig) {  
    cfg.service(  
        web::resource("/users").route(web::get().to(game_controller::get_users)),  
    );  
}  