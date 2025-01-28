use self::models::*;
use ampachedb::*;
use diesel::prelude::*;
use std::env::args;

fn main() {
    use self::schema::song::dsl::*;

    let song_id = args()
        .nth(1)
        .expect("ID Required")
        .parse::<u32>()
        .expect("Invalid ID");

    let connection = &mut establish_connection();

    let results = match song_id {
        0 => song.load::<Song>(connection).expect("Unable to read song"),
        x => song
            .find(x)
            .load::<Song>(connection)
            .expect("Unable to read song"),
    };

    println!("Found {} songs", results.len());

    for s in &results {
        /*let art = &artist
            .find(s.artist)
            .limit(1)
            .load::<Artist>(connection)
        .expect("error")[0];*/
        let x = get_artist_by_id(connection, s.artist);

        println!(
            "{} {} {} {} {:?} {}",
            s.id,
            s.title.as_ref().unwrap(),
            s.file.as_ref().unwrap(),
            s.track.unwrap(),
            x,
            s.album,
        );
    }

    if song_id > 0 {
        let result = RequestList::belonging_to(&results)
            .select(RequestList::as_select())
            .load(connection)
            .expect("error");

        println!("Requests {}", result.len());
        for req in result {
            println!("{} {}", req.name.unwrap(), req.msg.unwrap());
        }
    }
}
