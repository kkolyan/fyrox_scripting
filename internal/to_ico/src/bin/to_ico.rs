use std::env::args;
use to_ico::process_icon;

fn main() {
    let args = args().collect::<Vec<_>>();
    process_icon(args[1].as_str(), args[2].as_str());
}
