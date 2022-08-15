mod api;
mod model;
mod repository;
use std::sync::{Arc, Mutex};

use api::notes::*;
use repository::notes_repo::*;

use actix_web::{middleware::Logger, web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let mut repo = NotesRepository::new();

    repo.create_db();

    let share_repo = Arc::new(Mutex::new(repo));

    let my_data = web::Data::new(share_repo);

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .app_data(my_data.clone())
            .wrap(logger)
            .service(home)
            .service(get_notes)
            .service(get_note_by_id)
            .service(add_note)
            .service(remove_note_by_id)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
