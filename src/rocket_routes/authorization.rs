use crate::{rocket_routes::{DbConn, server_error},repositories::UserRepository};
use argon2::{PasswordHash,PasswordVerifier};
use rocket::serde::json::{Json,json,Value};
use rocket::response::status::Custom;
use rocket_db_pools::Connection;

#[derive(serde::Deserialize)]
pub struct Credentials {
    pub username:String,
    pub password:String
}


#[rocket::post("/login",format="json", data="<credentials>")]
pub async fn login(mut db: Connection<DbConn>, credentials: Json<Credentials>) -> Result<Value, Custom<Value>>{
    UserRepository::find_by_username(&mut db, &credentials.username).await
        .map(|user|{
            let argon2 = argon2::Argon2::default();
            match PasswordHash::new(&user.password) {
                Ok(db_hash) => {
                    let result = argon2.verify_password(credentials.password.as_bytes(), &db_hash);
                    if result.is_ok() {
                        json!("Success")
                    } else {
                        json!("Unauthorized")
                    }
                },
                Err(_) => json!("Unauthorized")
            }
        })
        .map_err(|e| server_error(e.into()))
}  

