use super::schema::words;

#[derive(Insertable)]
#[table_name="words"]
pub struct NewWord<'a> {
    pub word: &'a str,
    pub counter: i32,
}

#[derive(Queryable)]
pub struct Word {
    pub id: i32,
    pub word: String,
    pub counter: i32,
}