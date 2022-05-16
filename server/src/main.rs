#[allow(dead_code)] // TODO remove
mod db;
#[allow(dead_code)] // TODO remove
mod game;
#[allow(dead_code)] // TODO remove
mod sudoku;
#[allow(dead_code)] // TODO remove
mod webserver;

use crate::db::Db;

use std::sync::Mutex;

use rocket::fs::FileServer;
use rocket::routes;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let db = Mutex::new(Db::new());

    let _ = rocket::build()
        .mount("/", FileServer::from("frontend/public"))
        .mount("/create", routes![webserver::create])
        .manage(db)
        .launch()
        .await?;

    Ok(())
}
