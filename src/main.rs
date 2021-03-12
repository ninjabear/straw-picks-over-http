#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;
extern crate base64;

mod decision;
mod decision_repository;
mod picker;
mod signature;
mod ticket;

use decision::Decision;
use decision_repository::DecisionRepository;
use rocket::http::RawStr;
use rocket::State;

use signature::*;
use std::sync::Mutex;
use ticket::{write, Ticket};

struct AppState {
    decision_repository: Mutex<DecisionRepository>,
}

#[get("/")]
fn try_pool() -> String {
    format!(
        "Try `/?choices=A,B`\n\npsst. My ed25519 public key is:\n{}",
        signature::public_key()
    )
}

#[get("/?<choices>")]
fn index(choices: &RawStr, s: State<AppState>) -> Option<String> {
    let possibles: Vec<&str> = choices
        .as_str()
        .split(",")
        .filter(|i| i.trim() != "")
        .collect();

    if possibles.len() == 0 || possibles.len() > 15 {
        return None;
    }

    let name = picker::pick_random(&possibles);

    let d = Decision::new(name, &possibles);

    let shared_data: &AppState = s.inner();
    shared_data
        .decision_repository
        .lock()
        .unwrap()
        .put(d.clone());

    let ticket = Ticket::new(
        d,
        shared_data
            .decision_repository
            .lock()
            .unwrap()
            .all()
            .to_vec(),
    );

    return Some(with_appended_sig(&write(&ticket)));
}

fn main() {
    rocket::ignite()
        .manage(AppState {
            decision_repository: Mutex::new(DecisionRepository::new()),
        })
        .mount("/", routes![index, try_pool])
        .launch();
}
