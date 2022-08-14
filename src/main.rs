mod api;
use std::sync::{Arc, Mutex};

use api::notes::*;

use actix_web::{middleware::Logger, web, App, HttpServer};
use rusqlite::{Connection, Error, Result};

async fn create_db() -> Result<Connection, Error> {
    let conn = Connection::open("notes.db").unwrap();

    conn.execute(
        "create table if not exists users (
            id integer primary key,
            name text not null unique
        )",
        [],
    )?;

    conn.execute(
        "create table if not exists notes (
            id integer primary key,
            title text not null,
            data text,
            created_on DATE DEFAULT (datetime('now','localtime')),
            updated_on DATE DEFAULT (datetime('now','localtime')),
            user_id integer not null references users(id)
        )",
        [],
    )?;

    Ok(conn)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let conn = Arc::new(Mutex::new(create_db().await.unwrap()));

    let my_data = web::Data::new(conn);

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .app_data(my_data.clone())
            .wrap(logger)
            .service(home)
            .service(get_notes)
            .service(add_note)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
