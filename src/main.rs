use std::fs::File;
use std::io::{BufRead, BufReader};

#[cfg(test)]
pub mod test;

pub mod minecraft;
pub mod mojang;

fn main() {
    use clap::Arg;

    let matches = clap::App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("stdin")
                .short("s")
                .long("stdin")
                .help("read from stdin")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("JSON_FILE")
                .help("path to whitelist.json")
                //.required(true)
                .index(1),
        )
        .get_matches();

    let stdin = std::io::stdin();
    let mut stdin_lock = stdin.lock();

    let mut buf: Box<dyn BufRead> = if let Some(file) = matches.value_of("JSON_FILE") {
        println!("file: {}", file);
        let file = File::open(file).unwrap();
        let buf = BufReader::new(file);
        Box::new(buf)
    } else {
        // stdin?
        if !matches.is_present("stdin") {
            panic!("")
        }

        Box::new(&mut stdin_lock)
    };

    let json = buf2str(&mut buf).unwrap();
    println!("{}", json);

    let _whitelist: minecraft::UserList = serde_json::from_str(&json).unwrap();
}

fn buf2str(stream: &mut impl BufRead) -> Result<String, ()> {
    let mut s = String::new();
    loop {
        match stream.read_line(&mut s) {
            Ok(0) => return Ok(s), // EOF
            Ok(_) => continue,
            _ => panic!("hoge"),
        }
    }
}
