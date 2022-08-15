use std::sync::{Arc, Mutex};

use actix_web::{
    get,
    http::StatusCode,
    post,
    web::{self, Data},
    Error, HttpRequest, HttpResponse, Responder, Result,
};
use serde::Deserialize;

use crate::{
    model::note::Note,
    repository::notes_repo::{NotesRepository, Repository},
};

#[derive(Deserialize)]
pub struct GetNotesRequest {
    page_num: i32,
    page_size: i32,
}

#[get("/")]
pub async fn home() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text.html; charset=utf-8")
        .body(format!("welcome to myNotes api")))
}

#[get("/api/notes")]
pub async fn get_notes(
    req: HttpRequest,
    get_params: web::Query<GetNotesRequest>,
) -> Result<impl Responder> {
    let data = req.app_data::<Data<Arc<Mutex<NotesRepository>>>>();

    match data {
        Some(d) => {
            let repo = d.lock().unwrap();

            let notes_vec = repo.get_paged(get_params.page_num, get_params.page_size);
            match notes_vec {
                Some(notes) => {
                    return Ok(web::Json(notes));
                }
                None => {
                    return Ok(web::Json(Vec::new()));
                }
            }
        }
        None => return Ok(web::Json(Vec::new())),
    }
}

#[get("/api/notes/{id}")]
pub async fn get_note_by_id(req: HttpRequest, note_id: web::Path<u64>) -> Result<impl Responder> {
    let data = req.app_data::<Data<Arc<Mutex<NotesRepository>>>>();

    match data {
        Some(d) => {
            let repo = d.lock().unwrap();

            let note = repo.get(note_id.into_inner());
            match note {
                Some(n) => {
                    return Ok(web::Json(n));
                }
                None => {
                    return Ok(web::Json(Note::new()));
                }
            }
        }
        None => return Ok(web::Json(Note::new())),
    }
}

#[post("/api/notes/add")]
pub async fn add_note(req: HttpRequest, new_note: web::Json<Note>) -> impl Responder {
    let data = req.app_data::<Data<Arc<Mutex<NotesRepository>>>>();

    match data {
        Some(d) => {
            let repo = d.lock().unwrap();

            let note = repo.insert(new_note.into_inner());
            match note {
                Some(n) => {
                    return Ok::<web::Json<Note>, Error>(web::Json(n));
                }
                None => {
                    return Ok(web::Json(Note::new()));
                }
            }
        }
        None => return Ok(web::Json(Note::new())),
    }
}
