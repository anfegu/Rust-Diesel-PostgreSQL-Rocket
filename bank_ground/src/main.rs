#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

use diesel::prelude::*;

pub mod error;
pub mod session;
use crate::error::BankWebErr;
use std::path::PathBuf;

use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket::response::{NamedFile, Responder};
use rocket::State;

use maud::{html, DOCTYPE};

use db_bank::{models::*, schema::*};

#[get("/")] //Root Path
fn root() -> Result<impl Responder<'static>, failure::Error> {
    NamedFile::open("site/static/index.html").map_err(|e| e.into())
}

#[get("/<path..>")]
fn static_file(path: PathBuf) -> Result<impl Responder<'static>, BankWebErr> {
    let path = PathBuf::from("site/static").join(path);
    NamedFile::open(path).map_err(|e| e.into())
}

#[derive(FromForm)]
pub struct LoginData {
    name: String,
    pass: String,
}

#[database("db_bankbase")]
pub struct DPool(diesel::pg::PgConnection);

#[post("/login", data = "<dt>")]
fn login(
    dt: Form<LoginData>,
    db: DPool,
    ss: State<session::Session>,
    mut cookies: Cookies,
) -> Result<impl Responder<'static>, BankWebErr> {
    let ldform = dt.into_inner();
    let vals = users::table
        .filter(users::name.eq(ldform.name))
        .load::<User>(&db.0)?;

    let user = vals.iter().next().ok_or(BankWebErr::UserDoesNotExistErr)?;

    if !user.verify_pass(&ldform.pass) {
        return Err(BankWebErr::PasswordErr);
    }

    let sess_id = ss.put(user.clone());
    cookies.add(Cookie::new("login", sess_id.to_string()));

    Ok(html! {
        (DOCTYPE)
        head {
            meta charset = "utf-8";
        }
        body {
            h1 { "Welcome " (user.name)}
            h2 { "Create Account"}
            div style="border:1px solid black;" {
                    form action ="create" method="POST" {
                        br;
                        "Initial Amount: " input type="text" name="amount";
                        br;br;
                        "id_Account " input type="text" name="num";
                        br;br;
                        input type="submit" value="Create";
                    }
            }
        }
    })
}

#[derive(FromForm)]
pub struct CreateForm {
    num: String,
    amount: i32,
}

#[post("/create", data = "<dt>")]
pub fn create(
    dt: Form<CreateForm>,
    st: State<session::Session>,
    cookies: Cookies,
    db: DPool,
) -> Result<impl Responder<'static>, BankWebErr> {
    let login = cookies.get("login").ok_or(BankWebErr::NoCookie)?.value();
    let user = st
        .get(login.parse().map_err(|_| BankWebErr::NoCookie)?)
        .ok_or(BankWebErr::NoSession)?;

    let account = NewAccount {
        num: &dt.num,
        amount: &dt.amount,
    };

    let p_added: Account = diesel::insert_into(accounts::table)
        .values(&account)
        .get_result(&db.0)?;

    Ok(html! {
        (DOCTYPE)
        head {
            meta charset = "utf-8";
        }
        body {
            h1 {"Account Created Succesfully"}
            h2 { "Initial Amount :" (p_added.amount) } br;

        }
    })
}

fn main() {
    let sess = session::Session::new();
    rocket::ignite()
        .mount("/", routes![root, static_file, login, create])
        .attach(DPool::fairing())
        .manage(sess)
        .launch();
}
