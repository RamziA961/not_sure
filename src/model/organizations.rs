use diesel::{Insertable, Queryable, Selectable};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::organizations)]
pub(crate) struct Organizations {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) founded: chrono::NaiveDate,
    pub(crate) fk_user: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::organizations)]
pub(crate) struct NewOrganizations<'a> {
    pub(crate) name: &'a str,
    pub(crate) founded: chrono::NaiveDate,
    pub(crate) fk_user: i32,
}
