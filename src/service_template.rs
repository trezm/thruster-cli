use utils::SnekCase;

pub fn create(name: &str) -> String {
  templatify! { "use diesel;
use diesel::QueryDsl;
use schema::"; &name.to_snek_case() ;"s;
use schema::"; &name.to_snek_case() ;"s::dsl::*;
use diesel::RunQueryDsl;
use diesel::result::Error;
use util::db;
use models::"; &name.to_snek_case() ;"s::{"; name ;", New"; name ;"};
use std::io::Read;
use dotenv::dotenv;
use std::env;

pub fn create_"; &name.to_snek_case() ;"(new_"; &name.to_snek_case() ;": New"; name ;") -> Result<"; name ;", Error> {
  let conn = db::establish_connection();

  let "; &name.to_snek_case() ;" = diesel::insert_into("; &name.to_snek_case() ;"s::table)
    .values(&new_"; &name.to_snek_case() ;")
    .get_result(&conn);

  "; &name.to_snek_case() ;"
}

pub fn get_"; &name.to_snek_case() ;"(identifier: i32) -> Result<"; name ;", Error> {
  let conn = db::establish_connection();

  let "; &name.to_snek_case() ;" = "; &name.to_snek_case() ;"s.find(identifier)
    .first(&conn);

  "; &name.to_snek_case() ;"
}
" }
}
