use failure_derive::Fail;
use rocket::http::{ContentType, Status};
use rocket::response::{Responder, Response};
use rocket::Request;
use std::io::Cursor;

#[derive(Fail, Debug)]
pub enum BankWebErr {
    #[fail(display = "IO Error {}", 0)]
    IOErr(std::io::Error),
    #[fail(display = "Database Error {}", 0)]
    DatabaseError(diesel::result::Error),
    #[fail(display = "User does not exist")]
    UserDoesNotExistErr,
    #[fail(display = "Password Incorrect")]
    PasswordErr,
    #[fail(display = "No Login Cookie Provided")]
    NoCookie,
    #[fail(display = "Session Not Active")]
    NoSession,
}

impl From<std::io::Error> for BankWebErr {
    fn from(e: std::io::Error) -> Self {
        BankWebErr::IOErr(e)
    }
}

impl From<diesel::result::Error> for BankWebErr {
    fn from(e: diesel::result::Error) -> Self {
        BankWebErr::DatabaseError(e)
    }
}

impl<'r> Responder<'r> for BankWebErr {
    fn respond_to(self, _: &Request) -> rocket::response::Result<'r> {
        let res = Response::build()
            .status(Status::InternalServerError)
            .header(ContentType::Plain)
            .sized_body(Cursor::new(format!("Error doing loading page: {}", self)))
            .finalize();
        Ok(res)
    }
}
