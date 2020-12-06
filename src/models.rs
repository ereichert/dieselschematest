use super::schema::somethings;
use serde::Deserialize;

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "somethings"]
pub struct NewSomething {
    pub fielda: String,
    pub fieldb: String,
}
