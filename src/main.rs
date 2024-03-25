mod data_structs;
mod utils;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use data_structs::Connection;
use poise::serenity_prelude::User;
use std::{collections::HashMap, env};
use utils::insert_twitch;

async fn index(req: web::Query<HashMap<String, String>>) -> impl Responder {
    if let Some(code) = req.get("code") {
        let client_id = env::var("CLIENT_ID").expect("CLIENT_ID not set");
        let client_secret = env::var("CLIENT_SECRET").expect("CLIENT_SECRET not set");
        let client_uri = env::var("URL_URI").expect("URL_URI not set");
        let redirect_uri = format!("{}", client_uri);
        let mut params = HashMap::new();
        params.insert("client_id".to_string(), client_id);
        params.insert("client_secret".to_string(), client_secret);
        params.insert("grant_type".to_string(), "authorization_code".to_string());
        params.insert("code".to_string(), code.clone());
        params.insert("redirect_uri".to_string(), redirect_uri.to_string());

        match get_oauth_token(&params).await {
            Ok(output) => {
                if let Some(access) = output.get("access_token").and_then(|v| v.as_str()) {
                    match get_user_connections(&access).await {
                        Ok(connections) => {
                            if let Some(twitch_connection) = connections
                                .iter()
                                .find(|conn| conn.connection_type == "twitch")
                            {
                                match get_user_data(&access).await {
                                    Ok(user) => {
                                        match insert_twitch(
                                            user.id.into(),
                                            twitch_connection.name.to_string(),
                                        ) {
                                            Ok(response) => {
                                                let answer = response.success_description;
                                                println!("Success redirect {:?}", answer);
                                                return HttpResponse::Ok().body(format!(
                                                    "User verificado: {:#?}",
                                                    answer
                                                ));
                                            }
                                            Err(e) => {
                                                eprintln!("Error handling redirect: {}", e);
                                                return HttpResponse::Ok()
                                                    .body("Error insertando user");
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        eprintln!("Error getting user data: {}", e);
                                        return HttpResponse::Ok()
                                            .body("Error obteniendo data del usuario");
                                    }
                                }
                            } else {
                                println!("No Twitch connection found");
                                return HttpResponse::Ok()
                                    .body("No se encontro conexion con twitch");
                            }
                        }
                        Err(e) => {
                            eprintln!("Error getting user connections: {}", e);
                            return HttpResponse::Ok().body(format!(
                                "Error al chequear las conexiones del usuario: {}",
                                e
                            ));
                        }
                    }
                } else {
                    return HttpResponse::Ok().body("No access");
                }
            }
            Err(e) => {
                eprintln!("Error getting OAuth token: {}", e);
                return HttpResponse::Ok().body(format!("Error en oauth_token: {}", e));
            }
        }
    } else {
        return HttpResponse::Ok().body("No hay codigo disponible");
    }
}

async fn get_oauth_token(
    params: &HashMap<String, String>,
) -> Result<serde_json::Value, reqwest::Error> {
    let client = reqwest::Client::new();
    client
        .post("https://discord.com/api/oauth2/token")
        .form(params)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await
}

async fn get_user_connections(access_token: &str) -> Result<Vec<Connection>, reqwest::Error> {
    let client = reqwest::Client::new();
    client
        .get("https://discord.com/api/v10/users/@me/connections")
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?
        .json::<Vec<Connection>>()
        .await
}

async fn get_user_data(access_token: &str) -> Result<User, reqwest::Error> {
    let client = reqwest::Client::new();
    client
        .get("https://discord.com/api/v10/users/@me")
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?
        .json::<User>()
        .await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT").unwrap_or_else(|_| "80".to_string());
    let port = port.parse::<u16>().expect("Invalid port number");
    println!("Hi port: {}", port);

    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
