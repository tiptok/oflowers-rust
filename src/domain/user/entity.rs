use diesel::{prelude::*, Identifiable, QueryResult, Queryable};
use serde_derive::{Deserialize, Serialize};

use crate::pkg::database::Conn;
use crate::schema::users::dsl::*;

#[derive(Identifiable, Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i64,
    pub client_type: String,
    pub name: String,
    pub avatar: String,
    pub phone: String,
}

pub trait UserRepository
where
    Self: Clone + Copy,
{
    fn find_one_by_phone(&self, conn: &mut Conn, keyword: &str) -> QueryResult<User>;
    fn find_one(&self, conn: &mut Conn, id: &i64) -> QueryResult<User>;
    // fn find(conn: &mut Conn) -> QueryResult<User>;
}

#[derive(Clone, Copy)]
pub struct PgUserRepository {}

//
impl UserRepository for PgUserRepository {
    fn find_one_by_phone(&self, conn: &mut Conn, keyword: &str) -> QueryResult<User> {
        users.filter(phone.eq(keyword)).get_result::<User>(conn)
    }
    fn find_one(&self, conn: &mut Conn, user_id: &i64) -> QueryResult<User> {
        users.filter(id.eq(user_id)).get_result::<User>(conn)
    }
}
