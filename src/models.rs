use chrono::{DateTime, Utc};

use schema::entries;

#[derive(Deserialize, Serialize, Debug, Queryable, Insertable)]
#[table_name="entries"]
pub struct Entry {
    #[serde(rename="dateString")]
    pub date: DateTime<Utc>,

    #[serde(rename="type")]
    pub entry_type: String,

    pub sgv: Option<f64>,
    pub direction: String,
    pub noise: Option<f64>,
    pub filtered: Option<f64>,
    pub unfiltered: Option<f64>,
    pub rssi: Option<f64>,
}

