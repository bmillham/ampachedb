use ampachedb::get_playing_requests;

fn main() {
    let playing = get_playing_requests();
    for p in &playing {
        println!("{}", p.id);
    }
}
