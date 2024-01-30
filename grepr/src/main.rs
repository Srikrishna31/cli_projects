fn main() {
    if let Err(e) = grepr::get_args().and_then(grepr::run) {
        println!("{e}");
        std::process::exit(1);
    }
}
