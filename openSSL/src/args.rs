fn get_nth_arg(n: usize) -> String {
    if (std::env::args().nth(n).is_none()) {
        match n {
            1 => panic!("Path must be first argument and not nullable field"),
            2 => panic!("rsa_bits must be second argument and not nullable field"),
            _ => panic!("Not available arg in position {}.", n - 1),
        }
    }
    return std::env::args().nth(n).unwrap();
}

#[derive(Debug)]
pub struct Args {
    pub path: String,
    pub rsa_bits: u32,
}

impl Args {
    pub fn new() -> Self {
        return Args {
            path: get_nth_arg(1),
            rsa_bits: get_nth_arg(2).parse::<u32>().unwrap(),
        };
    }
}
