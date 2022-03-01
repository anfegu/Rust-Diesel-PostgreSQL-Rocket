#[macro_use]
extern crate diesel;
pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn create_connection() -> Result<PgConnection, failure::Error> {
    dotenv::dotenv().ok();
    Ok(PgConnection::establish(&std::env::var("DATABASE_URL")?)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{models::*, schema::*};
    #[test]
    fn add_user() {
        let conn = create_connection().unwrap();
        let user = new_user("Arisha Barron", "12345");
        let added: User = diesel::insert_into(users::table)
            .values(&user)
            .get_result(&conn)
            .unwrap();
        println!("New user added = {:?}", added);
    }
}
