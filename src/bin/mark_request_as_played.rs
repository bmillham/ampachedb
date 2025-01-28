use ampachedb::mark_request_as_played;
use std::env::args;

fn main() {
    let request_id = args()
        .nth(1)
        .expect("ID Required")
        .parse::<u32>()
        .expect("Invalid ID");

    let r = mark_request_as_played(request_id);
    println!("{r:?}");
}
