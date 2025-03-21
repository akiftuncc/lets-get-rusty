extern crate rusty;

use rocket_db_pools::Database;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/",rocket::routes![
            rusty::rocket_routes::authorization::login,
            //
            rusty::rocket_routes::rustaceans::get_rustaceans,
            rusty::rocket_routes::rustaceans::view_rustacean,
            rusty::rocket_routes::rustaceans::create_rustacean,
            rusty::rocket_routes::rustaceans::update_rustacean,
            rusty::rocket_routes::rustaceans::delete_rustacean,
            //
            rusty::rocket_routes::crates::get_crates,
            rusty::rocket_routes::crates::view_crate,
            rusty::rocket_routes::crates::create_crate,
            rusty::rocket_routes::crates::update_crate,
            rusty::rocket_routes::crates::delete_crate,

        ])
        .attach(rusty::rocket_routes::DbConn::init())
        .launch()
        .await;
}