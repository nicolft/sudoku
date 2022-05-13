#[allow(dead_code)] // TODO remove
mod sudoku;
#[allow(dead_code)] // TODO remove
mod webserver;

use rocket::routes;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _ = rocket::build()
        .mount("/", routes![webserver::index])
        .launch()
        .await?;

    Ok(())
}
