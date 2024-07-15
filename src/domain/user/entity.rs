use diesel::{prelude::*, Identifiable, QueryResult, Queryable};
use serde_derive::{Deserialize, Serialize};

use crate::pkg::database::Conn;
use crate::schema::users::dsl::*;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use diesel::internal::derives::multiconnection::chrono;
use diesel::query_dsl::methods::OrderDsl;
use diesel::sql_types::BigInt;
use futures::future::ok;
use crate::pkg::pagination::{Paginate, Paginated};

#[derive(Identifiable, Queryable, Selectable, Deserialize, Serialize,AsChangeset,Insertable,Default)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i64,
    pub name: String,
    pub avatar: String,
    pub phone: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,
}
#[derive( Deserialize, Serialize,AsChangeset,Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct UserDTO {
    pub name: String,
    pub avatar: String,
    pub phone: String,
    pub created_at: i64,
    pub updated_at: i64,
}




pub trait UserRepository
where
    Self: Clone + Copy,
{
    fn insert(&self,conn: &mut Conn,dm: UserDTO)->QueryResult<User>;
    fn update(&self,conn:&mut Conn,dm: User)->QueryResult<User>;
    fn delete(&self,conn:&mut Conn,id: i64)->QueryResult<User>;
    fn find_one_by_phone(&self, conn: &mut Conn, keyword: &str) -> QueryResult<User>;
    fn find_one(&self, conn: &mut Conn, id: i64) -> QueryResult<User>;
    fn find_one_unscoped(&self, conn: &mut Conn, id: i64) -> QueryResult<User>;
    fn find(&self, conn: &mut Conn,queryOptions: UserQueryOptions)->QueryResult<Vec<User>>;
    fn find_paginate(&self, conn: &mut Conn,queryOptions: UserQueryOptions)->QueryResult<(Vec<User>,i64)>;
}

#[derive(Clone, Copy)]
pub struct PgUserRepository {}


impl UserRepository for PgUserRepository {
    fn find_one_by_phone(&self, conn: &mut Conn, keyword: &str) -> QueryResult<User> {
        users.filter(phone.eq(keyword)).get_result::<User>(conn)
    }
    fn insert(&self,conn: &mut Conn,dm: UserDTO)->QueryResult<User>{
        let inserted_id: i64 = diesel::insert_into(users).values(&dm).returning(id).get_result(conn)?;
        Ok(User{
            id:inserted_id,
            name:dm.name,
            phone:dm.phone,
            avatar:dm.avatar,
            ..Default::default()
        })
    }
    fn update(&self,conn:&mut Conn,dm: User)->QueryResult<User>{
        let other_id : i64 = dm.id;
        diesel::update(users.find(other_id))
            // .set(name.eq(dm.name))
            // .set(phone.eq(dm.phone))
            // .set(avatar.eq(dm.avatar))
            .set(dm)
            .execute(conn)?;
        let updated = users.find(other_id).first(conn)?;
        Ok(updated)
    }
    fn delete(&self,conn:&mut Conn,other_id: i64)->QueryResult<User>{
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs() as i64;
        diesel::update(users.find(other_id))
            .set(deleted_at.eq(Some(current_time)))
            .execute(conn)?;
        let deleted = users.find(other_id).first(conn)?;
        Ok(deleted)
    }
    fn find_one(&self, conn: &mut Conn, user_id: i64) -> QueryResult<User> {
        users.filter(id.eq(user_id)).filter(deleted_at.is_null()).get_result::<User>(conn)
    }
    fn find_one_unscoped(&self, conn: &mut Conn, user_id: i64) -> QueryResult<User> {
        users.filter(id.eq(user_id)).get_result::<User>(conn)
    }
    fn find(&self, conn: &mut Conn,queryOptions: UserQueryOptions)->QueryResult<Vec<User>>{
        let mut query = users.into_boxed();
        query = query.filter(deleted_at.is_null());
        if let Some(username)=queryOptions.name{
            query = query.filter(name.like(format!("%{}%", username)));
        }
        if let Some(userphone)=queryOptions.phone{
            query = query.filter(phone.like(format!("%{}%", userphone)));
        }
        if let Some(page) = queryOptions.page{
            if let Some(size) = queryOptions.size{
                query = query.offset((page-1)*size);
                query = query.limit(size);
            }
        }
        query = OrderDsl::order(query, id.asc());
        query.load::<User>(conn)
    }
    fn find_paginate(&self, conn: &mut Conn,queryOptions: UserQueryOptions)->QueryResult<(Vec<User>,i64)>{
        let mut query = users.into_boxed();
        query = query.filter(deleted_at.is_null());
        if let Some(username)=queryOptions.name{
            query = query.filter(name.like(format!("%{}%", username)));
        }
        if let Some(userphone)=queryOptions.phone{
            query = query.filter(phone.like(format!("%{}%", userphone)));
        }
        query
            .paginate(queryOptions.page)
            .page_size(queryOptions.size)
            .sort(queryOptions.sort_by,queryOptions.sort_direction)
            .load_and_count_pages::<User>(conn)
    }
}
#[derive(Default)]
pub struct UserQueryOptions {
    pub name: Option<String>,
    pub gender: Option<String>,
    pub age: Option<i32>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub sort_by: Option<String>,
    pub sort_direction: Option<String>,
    pub page: Option<i64>,
    pub size: Option<i64>,
}