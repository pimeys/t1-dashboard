#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;

extern crate warp;
extern crate tokio;
extern crate futures;
extern crate tokio_postgres;
extern crate serde;
extern crate serde_json;
extern crate chrono;
extern crate pretty_env_logger;

use warp::Filter;
use chrono::{DateTime, Utc};
use std::env;

#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
enum EventType {
    Sgv,
    Mbg,
    Cal,
    Etc,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
struct Entry {
    #[serde(rename="type")] entry_type: EventType,
    #[serde(rename="dateString")] date: DateTime<Utc>,
    sgv: Option<f64>,
    direction: Option<String>,
    noise: Option<String>,
    filtered: Option<f64>,
    unfiltered: Option<f64>,
    rssi: Option<f64>,
}

fn main() {
    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=todos=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "t1-dashboard=info");
    }
    pretty_env_logger::init();

    info!("Moikka moi");

    let entries = warp::path("entries");

    let entries_index = entries.and(warp::path::index());

    let list = warp::get2()
        .and(entries_index)
        .map(|| {
            "listing entries here"
        });

    let create = warp::post2()
        .and(entries_index)
        .and(warp::body::json())
        .map(|entries: Vec<Entry>| {
            println!("{:?}", entries);
            "creating new entries here"
        });

    let api = list.or(create).with(warp::log("entries"));

    warp::serve(api).run(([0, 0, 0, 0], 1337));
}
