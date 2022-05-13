#[allow(dead_code)]
mod sudoku;
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
