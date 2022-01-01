use std::fs::File;
use std::io::{BufRead, BufReader};

#[cfg(test)]
pub mod test;

pub mod minecraft;
pub mod mojang;

#[tokio::main]
async fn main() {
    use clap::Arg;

    let matches = clap::App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("stdin")
                .short('s')
                .long("stdin")
                .help("read from stdin")
                .takes_value(false),
        )
        .arg(
            Arg::new("JSON_FILE")
                .help("path to whitelist.json")
                //.required(true)
                .index(1),
        )
        .get_matches();

    let stdin = std::io::stdin();
    let mut stdin_lock = stdin.lock();

    let buf: (&str, Box<dyn BufRead>) = if matches.is_present("stdin") {
        ("stdin", Box::new(&mut stdin_lock))
    } else {
        let fname = if let Some(fname) = matches.value_of("JSON_FILE") {
            fname
        } else {
            "whitelist.json"
        };
        println!("file: {}", fname);
        let file = File::open(fname).unwrap();
        let buf = BufReader::new(file);
        (fname, Box::new(buf))
    };
    let (fname, mut buf) = buf;

    let json = buf2str(&mut buf).unwrap();

    let result: Result<minecraft::UserList, _> = serde_json::from_str(&json);
    if let Err(err) = result {
        use rdfmt::*;
        let ejson =
            RdJson::error().diagnost(Diagnostic::error().message(err.to_string()).location(
                Location {
                    path: Some(fname.to_string()),
                    range: Some(Range {
                        start: Some(Position::new(err.line(), err.column())),
                        end: None,
                    }),
                },
            ));
        println!("{}", serde_json::to_string(&ejson).unwrap());
    }
    let whitelist: minecraft::UserList = serde_json::from_str(&json).unwrap();

    println!("start user check...");
    for user in whitelist {
        print!("{}: ", user.name);
        if user.exist().await.unwrap() {
            println!("[ok]");
            continue;
        }

        println!("does not exist!");
        std::process::exit(1);
    }
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
