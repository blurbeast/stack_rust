use std::env;
use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use diesel::prelude::PgConnection;
use dotenvy::dotenv;
use crate::model::{NewPost, Post};

mod schema;
mod model;

pub struct AppConfiguration {
    pub database_url: String,
    pub pool: PgConnection,
}

// type

pub fn run_database_connection() -> PgConnection {
    dotenv().ok();

    PgConnection::establish(
        &env::var("DATABASE_URL"
        ).expect("could not load environment variable"))
        .expect("could not connect to database")
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

    let mut post_title = String::new();
    let mut post_body = String::new();

    println!("enter the title");
    std::io::stdin().read_line(&mut post_title).expect("could not read line");

    println!("enter the body");
    std::io::stdin().read_line(&mut post_body).expect("could not read line");

    let new_post = create_new_post(pool, &post_title, &post_body);

    println!("{:?}", new_post);


    println!("Hello, world!");
}
