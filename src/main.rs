#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;
#[macro_use] extern crate diesel_derives;

extern crate warp;
extern crate tokio;
extern crate futures;
extern crate serde;
extern crate serde_json;
extern crate chrono;
extern crate pretty_env_logger;
extern crate diesel;
extern crate r2d2_diesel;
extern crate r2d2;
extern crate dotenv;

use dotenv::dotenv;
use warp::Filter;
use chrono::{DateTime, Utc};
use std::{env, sync::Arc};
use diesel::PgConnection;
use r2d2_diesel::ConnectionManager;
use r2d2::Pool;

#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
enum EntryType {
    Sgv,
    Mbg,
    Cal,
    Etc,
}

type Database = Pool<ConnectionManager<PgConnection>>;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", tag = "type")]
enum Entry {
    Sgv {
        #[serde(rename="dateString")]
        date: DateTime<Utc>,

        sgv: f64,
        direction: String,
        noise: f64,
        filtered: f64,
        unfiltered: f64,
        rssi: f64,
    },
    Mbg {
        #[serde(rename="dateString")]
        date: DateTime<Utc>,
    },
    Cal {
        #[serde(rename="dateString")]
        date: DateTime<Utc>,
    },
    Etc {
        #[serde(rename="dateString")]
        date: DateTime<Utc>,
    },
}

fn main() {
    dotenv().ok();

    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=todos=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "t1-dashboard=info");
    }

    pretty_env_logger::init();

    let entries = warp::path("entries");
    let entries_index = entries.and(warp::path::index());
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
        .map(|entries: Vec<Entry>, db: Arc<Database>| {
            println!("{:?}", entries);
            "creating new entries here"
        });

    let api = list.or(create).with(warp::log("entries"));

    warp::serve(api).run(([0, 0, 0, 0], 1337));
}
