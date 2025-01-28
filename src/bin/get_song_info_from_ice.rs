use ampachedb::{get_request, get_song_info_from_ice};
use std::env::args;

fn main() {
    let ice_info = args()
        .nth(1)
        .expect("ID Required")
        .parse::<String>()
        .expect("Invalid ID");

    let (sid, info) = get_song_info_from_ice(ice_info);
    println!("{sid} {info:?}");
    if sid != 0 {
        let rq = get_request(sid);
        println!("inf {rq:?}");
    }
}
