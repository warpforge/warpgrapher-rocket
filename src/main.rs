#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use juniper_rocket::GraphQLResponse;
use rocket::State;
use rocket::config::Environment;
use rocket::http::{Status};
use rocket_contrib::json::Json;
use std::collections::HashMap;
use std::process::exit;
use warpgrapher::{Engine};
use warpgrapher::juniper::http::GraphQLRequest;

mod app;

#[get("/graphiql")]
fn graphiql() -> rocket::response::content::Html<String> {
    juniper_rocket::playground_source("/graphql")
}

#[post("/graphql", data = "<request>")]
fn graphql(
    engine: State<Engine<(), ()>>,
    request: Json<GraphQLRequest>
) -> GraphQLResponse {
    let metadata: HashMap<String, String> = HashMap::new();
    let resp = engine.execute(&request.into_inner(), &metadata);
    GraphQLResponse::custom(Status::Ok, resp.unwrap())
}

fn run_rocket_server(port: u16, engine: Engine<(), ()>) {
    println!("Running warpgrapher on http://localhost:{}/graphiql", port);
    let config = rocket::config::Config::build(Environment::Staging)
        .log_level(rocket::config::LoggingLevel::Off)
        .address("0.0.0.0")
        .port(port)
        .finalize()
        .unwrap();
    rocket::custom(config)
        .manage(engine)
        .mount("/", routes![graphql, graphiql])
        .attach(rocket_cors::CorsOptions::default().to_cors().unwrap())
        .launch();
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let engine = match app::create_app_engine().await {
        Ok(v) => v,
        Err(e) => {
            println!("[ERROR] {:#?}", e);
            exit(1);
        }
    };
    run_rocket_server(8888, engine);
}