use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};

use json_spanned_value as jsv;

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
            Arg::new("whitelist")
                .long("whitelist")
                .env("JSON_FILE")
                .takes_value(true)
                .default_value("whitelist.json")
                .help("path to whitelist.json"),
        )
        .arg(
            Arg::new("rdjson")
                .long("rdjson")
                .env("RDJSON_FILE")
                .help("output with Reviewdog Diagnostic Format")
                .takes_value(true),
        )
        .get_matches();

    let stdin = std::io::stdin();
    let mut stdin_lock = stdin.lock();

    let buf: (&str, Box<dyn BufRead>) = if matches.is_present("stdin") {
        ("stdin", Box::new(&mut stdin_lock))
    } else {
        let fname = matches.value_of("whitelist").unwrap();
        println!("file: {}", fname);
        let file = File::open(fname).unwrap_or_else(|_| panic!("could not open {}", fname));
        let buf = BufReader::new(file);
        (fname, Box::new(buf))
    };
    let (fname, mut buf) = buf;

    let json = buf2str(&mut buf).unwrap();
    let mut files = codespan_reporting::files::SimpleFiles::new();
    let file = files.add(fname, &json);

    // check json file is valid user list
    let result: Result<minecraft::UserList, _> = serde_json::from_str(&json);
    if let Err(err) = result {
        use rdfmt::*;
        let ejson = RdJson::error().with_diagnost(
            Diagnostic::error()
                .with_message(err.to_string())
                .with_location(Location {
                    path: Some(fname.to_string()),
                    range: Some(Range {
                        start: Some(Position::new(err.line(), err.column())),
                        end: None,
                    }),
                }),
        );
        println!("{}", serde_json::to_string(&ejson).unwrap());
        panic!();
    }
    let user_list = result.unwrap();
    let spanned_list: jsv::spanned::Array = jsv::from_str(&json).unwrap();
    let spanned_list: Vec<jsv::Spanned<_>> = spanned_list.into_inner();

    let it = user_list.iter().zip(spanned_list.iter());

    let mut rdjson = rdfmt::RdJson::error();
    println!("start user check...");
    let mut has_error = false;
    for (user, spanned) in it {
        print!("{}: ", user.name);
        if user.exist().await.unwrap() {
            println!("[ok]");
            continue;
        }

        let obj = spanned.as_span_object().unwrap();
        let range = obj.range();

        use codespan_reporting::term;
        term::emit(
            &mut term::termcolor::StandardStream::stdout(term::termcolor::ColorChoice::Auto),
            &term::Config::default(),
            &files,
            &codespan_reporting::diagnostic::Diagnostic::error()
                .with_message("this user does not exist!")
                .with_labels(vec![codespan_reporting::diagnostic::Label::primary(
                    file,
                    range.clone(),
                )]),
        )
        .unwrap();

        if matches.is_present("rdjson") {
            let (line, column) = get_range(&json, range.start);
            let start = rdfmt::Position::new(line, column);
            let diagnost = rdfmt::Diagnostic::error()
                .with_message("this user does not exist!".to_string())
                .with_location(rdfmt::Location {
                    path: Some(fname.to_string()),
                    range: Some(rdfmt::Range {
                        start: Some(start),
                        end: None,
                    }),
                });
            rdjson = rdjson.with_diagnost(diagnost);
        }

        has_error = true;
    }
    let rdjson = serde_json::to_string(&rdjson).expect("rdjson serialize failed!");
    if let Some(fname) = matches.value_of("rdjson") {
        println!("{}", rdjson);
        if fname.is_empty() {
            println!("warning: rdjson file name is empty. skip output.");
        } else {
            let mut file = File::create(fname)
                .unwrap_or_else(|_| panic!("could not create rdjson file: {}", fname));
            file.write_all(rdjson.as_bytes()).unwrap();
        }
    }

    if has_error {
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

fn get_range(file: &str, pos: usize) -> (usize, usize) {
    let file = file.lines();

    let mut line = 1;
    let mut p = 0;
    for l in file {
        for (column, _) in l.chars().enumerate() {
            if p == pos {
                return (line, column + 1);
            }
            p += 1;
        }

        p += 1;
        line += 1;
    }

    unreachable!();
}
