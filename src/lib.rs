use self::models::{Album, Artist, RequestList, Song};
use self::schema::artist::dsl::*;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
pub mod models;
pub mod schema;

// Prefixes for Artist/Album
const PREFIXES: &[&str; 7] = &["The", "A", "An", "Die", "La", "Le", "Les"];

pub fn establish_connection(url: Option<String>) -> MysqlConnection {
    dotenv().ok();

    //let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let database_url = match url {
        Some(u) => u,
        _ => env::var("DATABASE_URL").expect("DATABASE_URL not set"),
    };
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Unable to connect to {}", database_url))
}

pub fn get_artist_by_id(connection: &mut MysqlConnection, aid: u32) -> String {
    use self::models::Artist;
    use self::schema::artist::dsl::*;

    let art = &artist
        .find(aid)
        .limit(1)
        .load::<Artist>(connection)
        .expect("error")[0];
    art.name.clone().expect("")
}

fn get_prefix(value: &str) -> (&str, &str) {
    for p in PREFIXES {
        let x = value.split_once(" ");

        if x.is_none() {
            continue;
        }
        let (pre, n) = x.unwrap();
        if pre == *p {
            return (pre, n);
        }
    }
    ("", value)
}

pub fn get_song_info_from_ice(ice_title: String) -> (u32, String) {
    let connection = &mut establish_connection(None);

    let parts: Vec<&str> = ice_title.split(" - ").collect();
    if parts.len() != 3 {
        return (0, format!("Bad Format: {ice_title}"));
    }

    let (pre, art) = get_prefix(parts[0]);

    let results: Vec<Artist> = match pre.is_empty() {
        true => artist
            .filter(
                crate::schema::artist::name
                    .eq(art)
                    .and(crate::schema::artist::prefix.is_null()),
            )
            .load::<Artist>(connection)
            .expect("not fount"),
        false => artist
            .filter(
                crate::schema::artist::name
                    .eq(art)
                    .and(crate::schema::artist::prefix.eq(pre)),
            )
            .load::<Artist>(connection)
            .expect("not fount"),
    };
    //println!("Artist {:?}", results);
    let song_res = Song::belonging_to(&results)
        .filter(crate::schema::song::title.eq(parts[1]))
        .select(Song::as_select())
        .load(connection)
        .expect("error");
    //println!("Song: {:?}", song_res);
    for s in song_res {
        use self::schema::album::dsl::*;
        let (pre, alb) = get_prefix(parts[2]);
        let alb_res: Vec<Album> = match pre.is_empty() {
            true => album
                .find(s.album)
                .filter(
                    crate::schema::album::name
                        .eq(alb)
                        .and(crate::schema::album::prefix.is_null()),
                )
                .load::<Album>(connection)
                .expect("error"),
            false => album
                .find(s.album)
                .filter(
                    crate::schema::album::name
                        .eq(alb)
                        .and(crate::schema::album::prefix.eq(pre)),
                )
                .load::<Album>(connection)
                .expect("error"),
        };

        if !alb_res.is_empty() {
            let seconds = s.time % 60;
            let minutes = (s.time / 60) % 60;
            let mut com_title: Vec<String> = Vec::new();

            com_title.push(ice_title.clone());

            if let Some(d) = alb_res[0].disk {
                if d > 0 {
                    com_title.push(format!("[Disc {d}]"));
                };
            };

            if s.year > 0 {
                com_title.push(format!("({})", s.year));
            };

            com_title.push(format!("[{minutes:0>2}:{seconds:0>2}]"));

            return (s.id, com_title.join(" "));
        }
    }
    (0, format!("Not found: {ice_title}"))
}

pub fn get_request(sid: u32) -> (u32, String) {
    use self::schema::requestlist::dsl::*;

    let connection = &mut establish_connection(None);
    let results = requestlist
        .select(RequestList::as_select())
        .filter(songID.eq(sid).and(status.eq("playing")))
        .load::<RequestList>(connection)
        .expect("error");
    if results.is_empty() {
        (0, "".to_string())
    } else {
        (
            results[0].id,
            format!(
                "(Requested by: {} @ {} GMT)",
                results[0].name.clone().expect("NONE"),
                results[0].t_stamp.clone()
            ),
        )
    }
}

pub fn get_playing_requests() -> Vec<RequestList> {
    use self::schema::requestlist::dsl::*;

    let connection = &mut establish_connection(None);
    requestlist
        //.select(RequestList::as_select())
        .filter(status.eq("playing"))
        .load::<RequestList>(connection)
        .expect("error")
}

pub fn mark_request_as_played(rid: u32, eta: NaiveDateTime) -> usize {
    use self::schema::requestlist::dsl::*;

    let connection = &mut establish_connection(None);
    diesel::update(requestlist.find(rid))
        .set((status.eq("played"), ETA.eq(eta)))
        .execute(connection)
        .unwrap()
}

pub fn mark_all_requests_as_played() -> usize {
    use self::schema::requestlist::dsl::*;

    let connection = &mut establish_connection(None);
    let playing_requests = get_playing_requests();
    println!("Marking {} requests as played", playing_requests.len());
    for r in &playing_requests {
        let x = diesel::update(requestlist.find(r.id))
            .set((status.eq("played"), ETA.eq(Utc::now().naive_utc())))
            .execute(connection)
            .unwrap();
        println!("Updated {} {x}", r.id);
    }
    playing_requests.len()
}
