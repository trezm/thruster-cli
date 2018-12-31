use crate::schema::{{ snek_case }}s;

#[derive(Debug, Deserialize, Serialize, Queryable)]
pub struct {{ name }} {
  pub id: i32,
  pub some_field: Option<String>
}

#[derive(Insertable, Debug, Deserialize, Serialize)]
#[table_name="{{ snek_case }}s"]
pub struct New{{ name }}<'a> {
  pub some_field: &'a str
}
