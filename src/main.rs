extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate juniper;

use std::io;
use std::sync::Arc;

use actix_web::{get, web, App,Error, HttpResponse, HttpServer, Responder};
use futures::{future::Future, TryFutureExt};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

mod schema;

use crate::schema::{create_schema, Schema};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = std::sync::Arc::new(create_schema());

    HttpServer::new(move || {
        App::new()
            .app_data(schema.clone())
            .service(web::resource("/graphql").route(web::post().to(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello graphql world!!!")
}

fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let res = data.execute(&st, &());
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .map_err(Error::from)
    .and_then(|user| {
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(user))
    })
}


fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://localhost:8080/graphql", Some("ws://localhost:8080/subscriptions"));
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
