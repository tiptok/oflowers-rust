#![allow(unused_imports)]

use diesel::{PgConnection, QueryId, QueryResult, RunQueryDsl};
use diesel::pg::Pg;
use diesel::query_builder::{AstPass, Query, QueryFragment};
use diesel::query_dsl::methods::LoadQuery;
use diesel::sql_types::BigInt;

pub trait Paginate: Sized{
    fn paginate(self,page: Option<i64>)->Paginated<Self>;
}

impl<T> Paginate for T {
    fn paginate(self, page: Option<i64>) -> Paginated<Self> {
        let p = page.unwrap_or(1);
        Paginated{
            query:self,
            page: p,
            size: DEFAULT_PER_PAGE,
            offset:(p -1)*DEFAULT_PER_PAGE,
            sort_by :"".to_string(),
            sort_direction:"".to_string()
        }
    }
}

const DEFAULT_PER_PAGE: i64 = 10;

#[derive(Debug,Clone,QueryId)]
pub struct Paginated<T>{
    query: T,
    page: i64,
    size: i64,
    offset: i64,
    sort_by: String,
    sort_direction: String,
}

impl<T> Paginated<T>  {
    pub fn page_size(self,size:Option<i64>)->Self{
        let s = size.unwrap_or(DEFAULT_PER_PAGE);
        Paginated{
            size:s,
            offset: (self.page -1)*s,
            ..self
        }
    }
    pub fn sort(self,sort_by: Option<String>,sort_direction: Option<String>)->Self{
        Paginated{
            sort_by:sort_by.unwrap_or("".to_string()),
            sort_direction: sort_direction.unwrap_or("".to_string()),
            ..self
        }
    }

    pub fn load_and_count_pages<'a,U>(self,conn: &mut PgConnection) -> QueryResult<(Vec<U>, i64)>
    where
        Self: LoadQuery<'a,PgConnection,(U,i64)>
    {
        // let page = self.page;
        // let size = self.size;
        let results = self.load::<(U,i64)>(conn)?;
        let total = results.first().map(|x| x.1).unwrap_or(0);
        let records = results.into_iter().map(|x|x.0).collect();
        Ok((records,total))
    }
}



impl <T> QueryFragment<Pg> for Paginated<T>
where
    T: QueryFragment<Pg>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, Pg>) -> QueryResult<()> {
            out.push_sql("SELECT *, COUNT(*) OVER () FROM (");
            self.query.walk_ast(out.reborrow())?;
            out.push_sql(") t LIMIT ");
            out.push_bind_param::<BigInt,_>(&self.size)?;
            out.push_sql(" OFFSET ");
            out.push_bind_param::<BigInt,_>(&self.offset)?;
            Ok(())
        }
}

impl <T: Query> Query for Paginated<T>
where
    T: Query,
{
    type SqlType = (T::SqlType,BigInt);
}

impl<T> RunQueryDsl<PgConnection> for Paginated<T> {}