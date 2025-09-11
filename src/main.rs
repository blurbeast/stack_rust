use std::env;
use diesel::{Connection, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper, TextExpressionMethods};
use diesel::prelude::PgConnection;
use dotenvy::dotenv;
use crate::model::{NewPost, NewUser, Post, PostTextBody, User};
use crate::schema::users::dsl::users;
use crate::schema::users::{email, name};

mod schema;
mod model;

pub struct AppConfiguration {
    pub database_url: String,
    pub pool: PgConnection,
}

// type

pub fn run_database_connection() -> PgConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("could not load environment variable");

    PgConnection::establish(&db_url).expect("could not connect to database")
}

pub fn create_new_post(db_connection: &mut PgConnection, title: &str, body: &str) -> Post {
    use crate::schema::posts;

    let new_post = NewPost { title, body };
    diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_select())
        .get_result(db_connection)
        .expect("could not create new post")
}

fn main() {


    let pool = &mut run_database_connection();

    // let mut post_title = String::new();
    // let mut post_body = String::new();
    //
    // println!("enter the title");
    // std::io::stdin().read_line(&mut post_title).expect("could not read line");
    //
    // println!("enter the body");
    // std::io::stdin().read_line(&mut post_body).expect("could not read line");
    //
    // let new_post = create_new_post(pool, &post_title, &post_body);

    // println!("{:?}", new_post);

    use crate::schema::posts::dsl::*;

    //to update a db
    diesel::update(posts.find(&1))
        .set(published.eq(true))
        .returning(Post::as_returning())
        .get_result(pool)
        .unwrap();

    let r = posts.filter(published.eq(true))
        .limit(10)
        .select(PostTextBody::as_select())
        .load(pool)
        .unwrap();

    println!("{:#?}", r);

    let rr = posts
        .find(&1)
        .select(Post::as_returning())
        .first(pool)
        .optional();


    posts
        .filter(title.eq("third"))
        .select(PostTextBody::as_select())
        .get_result(pool).optional();

    println!("{:#?}", rr);

    diesel::delete(posts.filter(title.like(""))).execute(pool).unwrap();


    println!("Hello, world!");
    let new_user = NewUser { name: "test".to_string(), email: "emailsiio".to_string() };

    // diesel::insert_into(users).values(&new_user).returning(User::as_select()).execute(pool).unwrap();

    diesel::update(users).filter(email.eq("")).set((name.eq("jp"), email.eq("kpo"))).execute(pool).unwrap();
}
