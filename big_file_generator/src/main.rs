use std::process;

fn main() {
    let config = match big_file_generator::get_args() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    };

    if let Err(e) = big_file_generator::run(config) {
        eprintln!("{e}");
        process::exit(1);
    }
}
