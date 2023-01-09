//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
// ✨λ s1_pg_sqlx1.rs  Playing with Rust Book ι✧22․07․23✦12․43․42․   🌎η ✧22․07․28․ ✧22․07․23․    
#![allow(dead_code)]
#![allow(unused)]
use std::error::Error;
use sqlx::Postgres; 
use sqlx::postgres::{PgPoolOptions, PgRow, PgArguments};
use sqlx::{FromRow, Row};
use sqlx::query::QueryAs;
use alps::{type_of, pr_type_of};

//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//  Constants § Types

const C_LL: &str = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";

//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//λ unit test mod 


//λ test_system
#[cfg(test)]
mod test_system {

    #[test]
    #[should_panic]
    fn fail_test_system() {
        let ok: bool = false;
        assert!(ok);
    }
}


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//λ util mod  
mod alps {

    //λ type_of returns a description (string) of the type of an object 
    pub fn type_of<T>(_: &T) -> String {
        format!("{}", std::any::type_name::<T>())
    }


    pub fn pr_type_of<T>(label: &str, _: &T) {
        print!("type of {label} is <{}>\n", std::any::type_name::<T>())
    }
}


//λ unit tests ─ test_traits
// #[cfg(test)]

// cargo watch -q -c -x run        ‡ -q: quite, -c: clear, -x: ???   
// cargo watch -q -c -x run 




///λ check_mod is the module suppervisor (a sub-main program); it can also serve as integration tester
pub fn check_mod() -> Result<(), Box<dyn Error>> {

//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//λ check_mod
#[tokio::main]
pub async fn check_mod() -> Result<(), Box<dyn Error>> {
    print!("{}🎡𐡋 Testing: a1_format.rs \n\n", C_LL);

    print!("1. Test pr_type_of function \n\n");
    let i1: i32 = 12; alps::pr_type_of("i1", &i1);


    print!("{}2. create a connection pool § create tables: compose and exec \n\n", C_LL);
    // create a connection pool 
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://sys7:sys7@localhost/sys7")         // connects to sys7 DB
        .await?;

    // create tables 
    let sql_stm = sqlx::query(                                  // preps SQL query 
        r#"create table if not exists dev_team2 (
            id              bigserial,
            name_id         text,
            official_name   text,
            product_owner   text,
            tech_lead       text); "#,);
    sql_stm.execute(&pool).await?;                              // execs query

    // drop tables if they exist 
    let drop_q = sqlx::query( r#"drop table if exists dev_team3;"#, );
    pr_type_of("drop_q", &drop_q);                  // type of drop_q is <sqlx_core::query::Query<sqlx_core::postgres::database::Postgres, sqlx_core::postgres::arguments::PgArguments>>
    drop_q.execute(&pool).await?;

    // insert records 
    print!("{}3. insert records \n\n", C_LL);
    let ins_q = sqlx::query_as( "insert into dev_team 
            (name_id, official_name)
            values ($1, $2)
            returning id");
    pr_type_of("ins_q", &ins_q);                    // type of ins_q is <sqlx_core::query_as::QueryAs<sqlx_core::postgres::database::Postgres, (i64,), sqlx_core::postgres::arguments::PgArguments>>
    let res: (i64, ) = ins_q.bind("team_06")
        .bind("Official Team 06")
        .fetch_one(&pool)
        .await?;
    pr_type_of("res", &res);                        // type of res is <(i64,)>

    // selecting records 
    let sel_q = sqlx::query( r#"select * from dev_team;"#, );
    pr_type_of("sel_q", &sel_q);                    // type of sel_q is <sqlx_core::query::Query<sqlx_core::postgres::database::Postgres, sqlx_core::postgres::arguments::PgArguments>>
    let rows = sel_q.fetch_all(&pool).await?;
    pr_type_of("rows", &rows);                      // type of rows is <alloc::vec::Vec<sqlx_core::postgres::row::PgRow>> 


    let str_rows = rows.iter()
        .map(|rr | format!("{} - {}", rr.get::<i64, _>("id"), rr.get::<String, _>("name_id")))
        .collect::<Vec<String>>();
    pr_type_of("str_rows", &str_rows);              // type of str_rows is <alloc::vec::Vec<alloc::string::String>>

    //      .join("\n");

    //for row in str_rows {
    //    print!("    {rows}\n");                      // {:?}` (or {:#?} for pretty-print
    //}

    // let ins_q = sqlx::query_as::<DB, O>( "INSERT INTO dev_team 
    //    

    // let ins_q: QueryAs<'_, _, i64, _> = sqlx::query_as( "INSERT INTO dev_team 
    
    // let q_res = ins_q.fetch_one(&pool).await?;
    // let q_res = ins_q.fetch_one(&pool).await?;


    print!("5. doing an s-query with map() (build ticket manually)\n\n");


    print!("6. doing an s-query with query_as (using derive FromRow)\n\n");



    print!("done!\n\n");

    Ok(())
    // panic!("for: No Reason");
}




//λ The Code Pit
/*
   sqlx  error[E0412]: cannot find type `O` in this scope
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
Connecting to DB Servers and DBs 
        .connect("postgres://sys7:sys7@localhost/sys7")                 // connects to sys7 DB
        .connect("postgres://sys7:sys7@localhost/postgres")             // connects to postgress DB

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    print!("{}2. create tables \n\n", C_LL);
    print!("2.1 composing sql statement \n\n");
    let sql_stm = sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS dev_team3 (
            id              bigserial,
            name_id         text,
            official_name   text,
            product_owner   text,
            tech_lead       text); "#,);

    print!("2.2 executing the create table1 statement \n\n");
    sql_stm.execute(&pool).await?;

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    print!("{}2. Drop tables \n\n", C_LL);
    print!("2.1 composing sql statement \n\n");
    let sql_stm = sqlx::query( r#"DROP TABLE IF EXISTS dev_team3;"#,);

    print!("2.2 executing the Drop table3 statement \n\n");
    sql_stm.execute(&pool).await?;

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    // insert records 
    print!("{}3. insert records \n\n", C_LL);
    print!("3.2 insert record 5\n\n");
    let ins_q: (i64,) = sqlx::query_as( "INSERT INTO dev_team 
            (name_id, official_name)
            VALUES ($1, $2)
            RETURNING id")
        .bind("team_05")
        .bind("Official Team 05")
        .fetch_one(&pool)
        .await?;

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    print!("{}2. create tables \n\n", C_LL);
    print!("2.1 composing sql statement \n\n");
    let sql_stm = sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS dev_team (
            id              bigserial,
            name_id         text,
            official_name   text,
            product_owner   text,
            tech_lead       text); "#,);

    
        .execute(&pool)
        .await?;

    print!("2.1 create a table1 \n\n");

    print!("{}3. insert records \n\n", C_LL);
    print!("3.2 insert record 2\n\n");
    let ins_q: (i64,) = sqlx::query_as( "INSERT INTO dev_team 
            (name_id, official_name)
            VALUES ($1, $2)
            RETURNING id")
        .bind("team_01")
        .bind("Official Team 01")
        .fetch_one(&pool)
        .await?;




    print!("4.1 selecting records #1 \n\n");
    let sel_rows


•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
FIND THE TYPE OF ins_q !!

    let ins_q: QueryAs<_, Postgres, std::result::Result<[type error], [type error]>, PgArguments> = sqlx::query_as( "INSERT INTO dev_team 
            (name_id, official_name)
            VALUES ($1, $2)
            RETURNING id")
        .bind("team_05")
        .bind("Official Team 05");
    
*/
// End Of The Code Pit

