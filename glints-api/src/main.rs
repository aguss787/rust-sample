use crate::schema::Query;
use actix_web::web::Data;
use actix_web::{guard, web, App, HttpServer};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use glints_managed_talent::{HubberService, ManagedTalentModule, PgConnectionPool};
use shaku::module;

#[cfg(feature = "graphql-playground")]
use actix_web::HttpResponse;
#[cfg(feature = "graphql-playground")]
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};

mod schema;

module! {
    APIModule {
        components = [],
        providers = [],

        use ManagedTalentModule {
            components = [ HubberService<PgConnectionPool> ],
            providers = [],
        }
    }
}

fn build_module() -> APIModule {
    APIModule::builder(glints_managed_talent::build_module().into()).build()
}

async fn index(
    schema: Data<Schema<Query, EmptyMutation, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[cfg(feature = "graphql-playground")]
async fn graphql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(build_module())
        .finish();

    println!("listening in 0.0.0.0:8080");

    HttpServer::new(move || {
        let app = App::new()
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index));

        #[cfg(feature = "graphql-playground")]
        let app = app.service(
            web::resource("/")
                .guard(guard::Get())
                .to(graphql_playground),
        );

        app
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
