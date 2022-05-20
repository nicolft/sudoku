#[allow(dead_code)] // TODO remove
mod db;
#[allow(dead_code)] // TODO remove
mod game;
#[allow(dead_code)] // TODO remove
mod sudoku;
#[allow(dead_code)] // TODO remove
mod webserver;

use crate::db::Db;

use rocket::fs::FileServer;
use rocket::routes;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _ = rocket::build()
        .mount("/", routes![webserver::create, webserver::play])
        .mount("/", FileServer::from("frontend/public"))
        .manage(Db::new())
        .launch()
        .await?;

    Ok(())
}
