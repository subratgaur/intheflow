use actix_web::{web, App, HttpServer};
use async_graphql::{Schema, Context};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use crate::schema::create_schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = create_schema();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .service(web::resource("/graphql").route(web::post().to(graphql_handler)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn graphql_handler(schema: web::Data<Schema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}