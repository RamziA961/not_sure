use diesel::{Insertable, Queryable, Selectable};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
pub(crate) struct Users {
    pub(crate) id: i32,
    pub(crate) display_name: String,
    pub(crate) email: String,
    pub(crate) password: String,
    pub(crate) handle: String,
    pub(crate) created_on: Option<chrono::NaiveDateTime>,
    pub(crate) profile_uri: Option<String>,
    pub(crate) photo_uri: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub(crate) struct NewUsers<'a> {
    pub(crate) display_name: &'a str,
    pub(crate) email: &'a str,
    pub(crate) password: &'a str,
    pub(crate) handle: &'a str,
    pub(crate) created_on: Option<chrono::NaiveDateTime>,
    pub(crate) profile_uri: Option<&'a str>,
    pub(crate) photo_uri: Option<&'a str>,
}
