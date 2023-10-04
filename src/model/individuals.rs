use diesel::{Insertable, Queryable, Selectable};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::individuals)]
pub(crate) struct Individuals {
    pub(crate) id: i32,
    pub(crate) first_name: String,
    pub(crate) last_name: String,
    pub(crate) dob: chrono::NaiveDate,
    pub(crate) fk_user: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::individuals)]
pub(crate) struct NewIndividual<'a> {
    pub(crate) first_name: &'a str,
    pub(crate) last_name: &'a str,
    pub(crate) dob: chrono::NaiveDate,
    pub(crate) fk_user: i32,
}
