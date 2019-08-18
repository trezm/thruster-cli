use uuid::Uuid;

use crate::schema::{{ snek_case_plural }};

#[derive(Debug, Deserialize, Serialize, Queryable)]
pub struct {{ name }} {
  pub id: Uuid,
  pub some_field: Option<String>
}

#[derive(Insertable, Debug, Deserialize, Serialize)]
#[table_name="{{ snek_case_plural }}"]
pub struct New{{ name }}<'a> {
  pub some_field: &'a str
}
