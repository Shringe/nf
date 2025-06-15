mod processer;

use std::env;

use processer::Processer;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let _ = args.remove(0);
    dbg!(&args);

    let processer = Processer::new(args);
}
