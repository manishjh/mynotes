use std::sync::{Arc, Mutex};

use actix_web::{
    get,
    http::StatusCode,
    post,
    web::{self, Data},
    Error, HttpRequest, HttpResponse, Responder,
};
use chrono::NaiveDateTime;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Note {
    id: u64,
    title: String,
    data: String,
    created_on: NaiveDateTime,
    updated_on: NaiveDateTime,
    user_id: u64,
}

#[get("/")]
pub async fn home() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text.html; charset=utf-8")
        .body(include_str!("../templates/index.html")))
}
#[get("/api/notes")]
pub async fn get_notes(req: HttpRequest) -> impl Responder {
    let data = req.app_data::<Data<Arc<Mutex<Connection>>>>().unwrap();

    let con = data.lock().unwrap();

    let mut stmt = con.prepare("select * from notes").unwrap();

    let notes = stmt
        .query_map([], |row| {
            Ok(Note {
                id: row.get(0)?,
                title: row.get(1)?,
                data: row.get(2)?,
                created_on: row.get(3)?,
                updated_on: row.get(4)?,
                user_id: row.get(5)?,
            })
        })
        .unwrap();

    let mut str = String::new();

    let mut notes_vec: Vec<Note> = Vec::new();

    for n in notes {
        match n {
            Ok(note) => {
                notes_vec.push(note.clone());
            }
            _ => str.push_str("..."),
        }
    }
    return web::Json(notes_vec);
}

#[post("/api/notes/add")]
pub async fn add_note(req: HttpRequest, new_note: web::Json<Note>) -> impl Responder {
    let data = req.app_data::<Data<Arc<Mutex<Connection>>>>().unwrap();

    let con = data.lock().unwrap();

    match new_note.into_inner() {
        note => {
            let mut stmt = con
                .prepare(
                    format!(
                        "INSERT INTO notes (title, data, user_id) values (\"{}\",\"{}\",\"{}\")",
                        &note.title, &note.data, &note.user_id
                    )
                    .as_str(),
                )
                .unwrap();

            match stmt.execute([]) {
                Ok(val) => {
                    if val > 0 {
                        let mut stmt = con
                            .prepare("select * from notes where id = (SELECT last_insert_rowid())")
                            .unwrap();

                        let notes = stmt
                            .query_map([], |row| {
                                Ok(Note {
                                    id: row.get(0)?,
                                    title: row.get(1)?,
                                    data: row.get(2)?,
                                    created_on: row.get(3)?,
                                    updated_on: row.get(4)?,
                                    user_id: row.get(5)?,
                                })
                            })
                            .unwrap();
                        for n in notes {
                            match n {
                                Ok(note) => {
                                    return web::Json(note);
                                }
                                _ => {}
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
    return web::Json(Note {
        id: 0,
        title: "null".to_string(),
        data: "null".to_string(),
        created_on: NaiveDateTime::MIN,
        updated_on: NaiveDateTime::MIN,
        user_id: 0,
    });
}
