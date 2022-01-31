extern crate rocket;

use rocket::http::{ContentType, Status};
use rocket::response::{content, Responder, Response};
use rocket::serde::json::Json;
use rocket::Request;
use rocket::{get, post, routes, State};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::io::Cursor;
use warpgrapher::engine::config::Configuration;
use warpgrapher::engine::context::RequestContext;
use warpgrapher::engine::database::neo4j::Neo4jEndpoint;
use warpgrapher::engine::database::DatabaseEndpoint;
use warpgrapher::juniper::http::playground::playground_source;
use warpgrapher::Engine;

static CONFIG: &str = "
version: 1
model:
  - name: User
    props:
      - name: name
        type: String
  - name: Team
    props: 
      - name: name
        type: String
";

#[derive(Clone, Debug)]
struct Rctx {}

impl RequestContext for Rctx {
    type DBEndpointType = Neo4jEndpoint;

    fn new() -> Self {
        Rctx {}
    }
}

#[derive(Clone, Debug, Deserialize)]
struct GraphqlRequest {
    pub query: String,
    pub variables: Option<Value>,
}

pub struct GraphQLResponse(Status, String);

impl<'r, 'o: 'r> Responder<'r, 'o> for GraphQLResponse {
    fn respond_to(self, _: &'r Request) -> Result<Response<'o>, Status> {
        let GraphQLResponse(status, body) = self;

        Ok(Response::build()
            .header(ContentType::new("application", "json"))
            .status(status)
            .sized_body(body.len(), Cursor::new(body))
            .finalize())
    }
}

#[get("/playground")]
fn playground() -> rocket::response::content::Html<String> {
    content::Html(playground_source("/graphql", None))
}

#[post("/graphql", data = "<request>")]
async fn graphql(engine: &State<Engine<Rctx>>, request: Json<GraphqlRequest>) -> GraphQLResponse {
    let metadata: HashMap<String, String> = HashMap::new();
    let resp = engine
        .execute(
            request.query.to_string(),
            request.variables.clone(),
            metadata,
        )
        .await;
    GraphQLResponse(Status::Ok, resp.unwrap().to_string())
}

#[tokio::main]
async fn main() {
    // parse config
    let config = Configuration::try_from(CONFIG.to_string()).expect("Expected to parse config.");

    // define database endpoint
    let db = Neo4jEndpoint::from_env()
        .expect("Expected to read environment variables")
        .pool()
        .await
        .expect("Expected to create database.");

    // create warpgrapher engine
    let engine: Engine<Rctx> = Engine::<Rctx>::new(config, db)
        .build()
        .expect("Expected engine build.");
    println!("Running warpgrapher on http://localhost:8000/graphql");
    rocket::build()
        .manage(engine)
        .mount("/", routes![graphql, playground])
        .launch()
        .await
        .expect("Expected successfull rocket launch")
}
