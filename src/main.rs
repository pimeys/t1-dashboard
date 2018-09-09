#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;
#[macro_use] extern crate diesel_derives;
#[macro_use] extern crate diesel;

extern crate warp;
extern crate tokio;
extern crate futures;
extern crate serde;
extern crate serde_json;
extern crate chrono;
extern crate pretty_env_logger;
extern crate r2d2_diesel;
extern crate r2d2;
extern crate dotenv;

mod schema;
mod models;

use dotenv::dotenv;
use warp::Filter;
use std::{env, sync::Arc};
use diesel::{PgConnection, RunQueryDsl};
use r2d2_diesel::ConnectionManager;
use r2d2::Pool;
use models::Entry;
use schema::{
    entries::{self, dsl::*},
};

type Database = Pool<ConnectionManager<PgConnection>>;

fn main() {
    dotenv().ok();

    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "t1-dashboard=info");
    }

    pretty_env_logger::init();

    let entries_path = warp::path("entries");
    let entries_index = entries_path.and(warp::path::index());
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let pool = Arc::new(
        Pool::builder()
            .max_size(15)
            .build(manager)
            .expect("Failed to create a pool.")
    );

    let db = warp::any().map(move || pool.clone());

    let list = warp::get2()
        .and(entries_index)
        .and(db.clone())
        .map(|db: Arc<Database>| {
            "listing entries here"
        });

    let create = warp::post2()
        .and(entries_index)
        .and(warp::body::json())
        .and(db.clone())
        .map(|new_entries: Vec<Entry>, db: Arc<Database>| {
            let conn = db.get().unwrap();

            diesel::insert_into(entries::table)
                .values(&new_entries)
                .execute(&*conn)
                .expect("OH NO");

            "creating new entries here"
        });

    let api = list.or(create).with(warp::log("entries"));

    warp::serve(api).run(([0, 0, 0, 0], 1337));
}
