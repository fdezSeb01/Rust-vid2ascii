fn get_nth_arg(n: usize) -> String {
    match std::env::args().nth(n) {
        Some(d) => d,
        None => panic!("Couldn't get argument {}", n),
    }
}

#[derive(Debug)]
pub struct Args {
    pub path: String,
}

impl Args {
    pub fn new() -> Self {
        Args {
            path: get_nth_arg(1),
        }
    }
}
