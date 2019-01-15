use diesel;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::result::Error;

use crate::util::db;
use crate::models::{{ snek_case }}s::{ {{ name }}, New{{ name }} };
use crate::schema::{{ snek_case }}s;
use crate::schema::{{ snek_case }}s::dsl::*;

pub fn create_{{ snek_case }}(new_{{ snek_case }}: New{{ name }}) -> Result<{{ name }}, Error> {
  let conn = db::establish_connection();

  let {{ snek_case }} = diesel::insert_into({{ snek_case }}s::table)
    .values(&new_{{ snek_case }})
    .get_result(&conn);

  {{ snek_case }}
}

pub fn get_{{ snek_case }}(identifier: i32) -> Result<{{ name }}, Error> {
  let conn = db::establish_connection();

  let {{ snek_case }} = {{ snek_case }}s.find(identifier)
    .first(&conn);

  {{ snek_case }}
}
