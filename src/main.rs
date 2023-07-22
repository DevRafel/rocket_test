#![feature(proc_macro_hygiene, decl_macro)]

mod looprs;

#[macro_use] extern crate rocket;

use rocket::response::NamedFile;
use std::path::{Path, PathBuf};

#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open("static/index.html").ok()
}

#[get("/<file..>")]
fn static_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn main() {

    rocket::ignite()
        .mount("/", routes![index, static_files])
        .launch();
}


