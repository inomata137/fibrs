fn main() {
    let mut args = std::env::args();
    let cmd = args.next().unwrap();
    let n: u64 = args
        .next()
        .expect(&format!("Usage:\n$ {} <positive_integer>\n", cmd))
        .parse()
        .expect(&format!("Usage:\n$ {} <positive_integer>\n", cmd));
    println!("{}", fibrs::fibonacci(n));
}
