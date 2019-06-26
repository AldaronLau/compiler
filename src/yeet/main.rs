fn main() {
    use std::io::Read;

    let filename = std::env::args().skip(1).next().unwrap_or_else(|| {
        eprintln!("Usage: yeet myproject.yeet");
        std::process::exit(255);
    });
    let mut file = String::new();
    std::fs::File::open(&filename).unwrap_or_else(|_| {
        eprintln!("Couldn't find file {}!", filename);
        std::process::exit(254);
    }).read_to_string(&mut file).unwrap();

    file.push('\0');

    println!("file: {}", file);

    if !file.starts_with("#!yeet 0.0\n") {
        eprintln!("Header does not match!\n");
        eprintln!("\tShould be: `#!yeet 0.0`");
        std::process::exit(253);
    }

    let file = file.as_bytes();

    let mut code = yeet::Code::new();

    code.parse(&file[11..]);

    for op in code.to_ops() {
        println!("{:?}", op);
    }
}
