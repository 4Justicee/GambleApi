use actix_web::{web, HttpResponse, App, HttpServer, Responder};  

use crate::controllers::{agent_controller, helper_controller};  

pub fn config(cfg: &mut web::ServiceConfig) {  
    cfg.service(  
        web::scope("/api")   
            .service(  
                web::resource("/gameRun")  
                    .route(web::post().to(agent_controller::game_launch))  
            )  
            .service(  
                web::resource("/playerCreate")  
                    .route(web::post().to(agent_controller::user_create))  
            )  
            .service(  
                web::resource("/playerDeposit")  
                    .route(web::post().to(agent_controller::user_deposit))  
            )  
            .service(  
                web::resource("/playerWithdraw")  
                    .route(web::post().to(agent_controller::user_withdraw))  
            )  
            .service(  
                web::resource("/playerWithdrawAll")  
                    .route(web::post().to(agent_controller::user_withdraw_all))  
            )  
            .service(  
                web::resource("/info")  
                    .route(web::post().to(agent_controller::info))  
            )  
            .service(  
                web::resource("/providerList")  
                    .route(web::post().to(agent_controller::provider_list))  
            )  
            .service(  
                web::resource("/gameList")  
                    .route(web::post().to(agent_controller::game_list))  
            )  
            .service(  
                web::resource("/getLogByDate")  
                    .route(web::post().to(agent_controller::get_date_log))  
            )  
            .service(  
                web::resource("/getLogById")  
                    .route(web::post().to(agent_controller::get_id_log))  
            )  
            .service(  
                web::resource("/getExchangeHistory")  
                    .route(web::post().to(agent_controller::get_exchange_history))  
            )  
            .service(  
                web::resource("/currentPlayers")  
                    .route(web::post().to(helper_controller::current_players)),  
            )  
            .service(  
                web::resource("/callList")  
                    .route(web::post().to(helper_controller::call_list)),  
            ) 
            .service(  
                web::resource("/callApply")  
                    .route(web::post().to(helper_controller::call_apply)),  
            )  
            .service(  
                web::resource("/callHistory")  
                    .route(web::post().to(helper_controller::call_history)),  
            )  
            .service(  
                web::resource("/callCancel")  
                    .route(web::post().to(helper_controller::call_cancel)),  
            )  
            .service(  
                web::resource("/masterRtp")  
                    .route(web::post().to(helper_controller::agent_rtp)),  
            )  
            .service(  
                web::resource("/playerRtp")  
                    .route(web::post().to(helper_controller::user_rtp)),  
            )   
            // Other callcheck routes...  
    );  
}  
