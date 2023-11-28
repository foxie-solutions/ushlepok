use axum::{
    routing::{get, post},
    Router, Server,
};
use axum_template::engine::Engine;
use container::Container;
use templates::AppEngine;

mod container;
mod handlers;
mod templates;

const PATH: &str = "ushlepok.json";
const IMAGE_PATH: &str = "image.jpeg";

#[derive(Clone)]
pub struct AppState {
    engine: AppEngine,
    container: Container,
}

#[tokio::main]
async fn main() {
    let hbs = templates::init_handlebars().unwrap();
    let container = Container::new(std::path::Path::new(PATH), std::path::Path::new(IMAGE_PATH));
    if let Err(e) = container.load() {
        eprintln!("container load error: {}", e);
    }

    let app = Router::new()
        .route("/", get(handlers::root::get))
        .route("/image", get(handlers::image::get))
        .route("/add", get(handlers::add::get))
        .route("/add", post(handlers::add::post))
        .route("/rules", get(handlers::rules::get))
        .fallback(get(handlers::assets::get))
        .with_state(AppState {
            engine: Engine::from(hbs),
            container,
        });

    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
