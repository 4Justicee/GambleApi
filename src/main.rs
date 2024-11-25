// src/main.rs  
use actix_web::{App, HttpServer};  
use sqlx::PgPool;  
use dotenv::dotenv;  

mod controllers;  
mod models;  
mod routes;  
mod defines;

pub async fn setup_database(pool: &PgPool) -> Result<(), sqlx::Error> {  
    sqlx::query!(r#"  
    CREATE TABLE IF NOT EXISTS providers (  
        id SERIAL PRIMARY KEY,  
        code VARCHAR(255) NOT NULL,  
        name VARCHAR(255) NOT NULL,  
        provider_type VARCHAR(255) NOT NULL,  
        endpoint VARCHAR(255) NOT NULL,  
        vendor_key VARCHAR(255),  
        backoffice VARCHAR(255),  
        percent FLOAT,  
        status INT NOT NULL,  
        config VARCHAR(255)
    );  
    "#)  
    .execute(pool)  
    .await?; 

    sqlx::query!(r#"  
        CREATE TABLE IF NOT EXISTS games (  
            id SERIAL PRIMARY KEY,  
            provider_code VARCHAR(255) NOT NULL,  
            game_type VARCHAR(255) NOT NULL,  
            en_name VARCHAR(255) NOT NULL,  
            game_code VARCHAR(255) NOT NULL,  
            banner VARCHAR(255),  
            status INT NOT NULL,  
            created_at TIMESTAMP WITH TIME ZONE,  
            updated_at TIMESTAMP WITH TIME ZONE  
        );  
    "#)  
    .execute(pool)  
    .await?;  

    Ok(())  
}  

#[actix_web::main]  
async fn main() -> std::io::Result<()> {  
    dotenv().ok(); // Load .env file if exists  
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"); 

    let pool = PgPool::connect(&database_url)  
        .await  
        .expect("Failed to create pool");  

    setup_database(&pool).await.expect("Failed to setup database");

    HttpServer::new(move || {  
        App::new()  
        .configure(routes::game::config)  
        .configure(routes::agent::config) 
    })  
    .bind("127.0.0.1:8080")?  
    .run()  
    .await  
}  