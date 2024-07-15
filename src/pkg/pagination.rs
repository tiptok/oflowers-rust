#![allow(unused_imports)]

use diesel::{PgConnection, QueryId, QueryResult, RunQueryDsl};
use diesel::pg::Pg;
use diesel::query_builder::{AstPass, Query, QueryFragment};
use diesel::query_dsl::methods::LoadQuery;
use diesel::sql_types::BigInt;

pub trait Paginate: Sized{
    fn paginate(self,page: i64)->Paginated<Self>;
}

impl<T> Paginate for T {
    fn paginate(self, page: i64) -> Paginated<Self> {
        Paginated{
            query:self,
            page,
            size: DEFAULT_PER_PAGE,
            offset:(page -1)*DEFAULT_PER_PAGE,
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
    pub fn page_size(self,size:i64)->Self{
        Paginated{
            size,
            offset: (self.page -1)*size,
            ..self
        }
    }
    pub fn sort(self,sort_by: String,sort_direction: String)->Self{
        Paginated{
            sort_by,
            sort_direction,
            ..self
        }
    }

    pub fn load_and_count_pages<'a,U>(self,conn: &mut PgConnection) -> QueryResult<(Vec<U>, i64)>
    where
        Self: LoadQuery<'a,PgConnection,(U,i64)>
    {
        let size = self.size;
        let results = self.load::<(U,i64)>(conn)?;
        let total = results.first().map(|x| x.1).unwrap_or(0);
        let records = results.into_iter().map(|x|x.0).collect();
        let total_pages = (total as f64 / size as f64).ceil() as i64;
        Ok((records,total_pages))
    }
}

impl <T: Query> Query for Paginated<T>{
    type SqlType = (T::SqlType,BigInt);
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