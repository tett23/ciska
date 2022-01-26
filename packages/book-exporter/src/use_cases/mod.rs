#[derive(Debug, Queryable)]
pub struct Novel {
    pub id: i32,
    pub novel_source_name: String,
    pub narou_novel_source_id: Option<i32>,
    pub novel18_novel_source_id: Option<i32>,
    pub hameln_novel_source_id: Option<i32>,
    pub kakuyomu_novel_source_id: Option<i32>,
}

#[derive(Debug, Queryable)]
pub struct Episode {
    pub id: i32,
    pub novel_id: i32,
    pub episode_source_name: String,
    pub narou_episode_source_id: Option<i32>,
    pub novel18_episode_source_id: Option<i32>,
    pub hameln_episode_source_id: Option<i32>,
    pub kakuyomu_episode_source_id: Option<i32>,
}

#[derive(Debug, Queryable)]
pub struct EpisodeSourceNarou {
    pub id: i32,
    pub episode_id: i32,
    pub nvoel_id: i32,
    pub title: String,
    pub text: String,
    pub uid: i32,
}

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use std::ops::{Generator, GeneratorState};
use std::pin::Pin;

pub fn establish_connection() -> SqliteConnection {
    // dotenv().ok();

    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
    let database_url = "/Users/tett23/.config/kindlize/development/development.db";
    SqliteConnection::establish(database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn books() -> Vec<Novel> {
    use crate::schemas;
    use crate::schemas::episode_source_hameln::dsl::*;
    use crate::schemas::episode_source_kakuyomu::dsl::*;
    use crate::schemas::episode_source_narou::dsl::*;
    use crate::schemas::episode_source_novel18::dsl::*;
    use crate::schemas::episodes::dsl::*;
    use crate::schemas::novels::dsl::*;

    let connection = establish_connection();
    let results = novels
        .limit(5)
        .load::<Novel>(&connection)
        .expect("Error loading posts");
    dbg!(&results);

    let novel = results.first().unwrap();

    // let episodes = episodes
    //     .filter(novel_id.eq(novel.id))
    //     .load::<Episode>(&connection);
    dbg!(&episodes);
    match novel.novel_source_name.as_str() {
        "narou" => {
            let aaaa = episodes
                .filter(schemas::episodes::novel_id.eq(novel.id))
                .load::<Episode>(&connection)
                .unwrap()
                .iter()
                .map(|episode| {
                    episode_source_narou
                        .filter(schemas::episode_source_narou::episode_id.eq(episode.id))
                        .first::<EpisodeSourceNarou>()
                })
                .collect::<Vec<_>>(conn);
        }
        "novel18" => {}
        "hameln" => {}
        "kakuyomu" => {}
    }

    let episode_origin = episodes
        .filter(novel_id.eq(novel.id))
        .load::<Episode>(&connection);
    dbg!(&episodes);

    vec![]
}
