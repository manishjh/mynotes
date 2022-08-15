use crate::model::note::Note;
use rusqlite::Connection;

pub trait Repository<T> {
    fn new() -> Self;
    fn create_db(&mut self) -> Result<(), rusqlite::Error>;
    fn get(&self, key: u64) -> Option<T>;
    fn insert(&self, val: T) -> Option<T>;
    fn get_paged(&self, page_num: i32, page_size: i32) -> Option<Vec<T>>;
    fn remove(&self, key: u64) -> Option<u64>;
}

pub struct NotesRepository {
    pub connection: Option<Connection>,
}

impl Repository<Note> for NotesRepository {
    fn new() -> Self {
        NotesRepository { connection: None }
    }

    fn create_db(&mut self) -> Result<(), rusqlite::Error> {
        let conn = Connection::open("notes.db")?;

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

        self.connection = Some(conn);

        Ok(())
    }

    fn get(&self, key: u64) -> Option<Note> {
        match &self.connection {
            Some(con) => {
                let mut stmt = con
                    .prepare(format!("select * from notes where id = {key}").as_str())
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
                            return Some(note);
                        }
                        _ => return None,
                    }
                }
                None
            }
            None => None,
        }
    }

    fn insert(&self, note: Note) -> Option<Note> {
        match &self.connection {
            Some(con) => {
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
                                .prepare(
                                    "select * from notes where id = (SELECT last_insert_rowid())",
                                )
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
                                        return Some(note);
                                    }
                                    _ => return None,
                                }
                            }
                        }
                        return None;
                    }
                    _ => {}
                }
            }
            None => {}
        }
        None
    }

    fn get_paged(&self, page_num: i32, page_size: i32) -> Option<Vec<Note>> {
        match &self.connection {
            Some(con) => {
                let start = (&page_num - 1) * &page_size;
                let end = &start + &page_size - 1;
                let mut stmt = con
                    .prepare(
                        format!("select * from notes where id >= {start} and id <= {end}").as_str(),
                    )
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
                Some(notes_vec)
            }
            None => None,
        }
    }

    fn remove(&self, key: u64) -> Option<u64> {
        todo!()
    }
}
