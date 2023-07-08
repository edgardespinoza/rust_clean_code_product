use actix_cors::Cors;
use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use config::{create_pool, Config};
use crud_product_actix::{
    application::{self, openapi::API_DOC},
    state::state_factory,
};
use utoipa_swagger_ui::SwaggerUi;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));

    let config = Config::from_env();

    let pool = create_pool(config.database_url(), config.max_connection)
        .await
        .unwrap();

    HttpServer::new(move || {
        println!("Starting HTTP server");
        App::new()
            .app_data(Data::new(state_factory(pool.clone())))
            // .wrap(ErrorHandlers::new().default_handler(error::add_error_header))
            .wrap(
                Cors::default()
                    .send_wildcard()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    //.allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    //.allowed_header(header::CONTENT_TYPE)
                    .allow_any_header()
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(Logger::default())
            .service(
                SwaggerUi::new("/api/v1/swagger-ui/{_:.*}")
                    .url("/api/v1/api-doc/openapi.json", API_DOC.clone()),
            )
            .configure(application::router::config)
    })
    .bind((config.server, config.port))?
    .run()
    .await
}
