use std::fs::File;
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
        .get_matches();

    let stdin = std::io::stdin();
    let mut stdin_lock = stdin.lock();

    let buf: (&str, Box<dyn BufRead>) = if matches.is_present("stdin") {
        ("stdin", Box::new(&mut stdin_lock))
    } else {
        let fname = matches.value_of("whitelist").unwrap();
        println!("file: {}", fname);
        let file = File::open(fname).unwrap();
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
        panic!();
    }
    let user_list = result.unwrap();
    let spanned_list: jsv::spanned::Array = jsv::from_str(&json).unwrap();
    let spanned_list: Vec<jsv::Spanned<_>> = spanned_list.into_inner();

    let it = user_list.iter().zip(spanned_list.iter());

    println!("start user check...");
    for (user, spanned) in it {
        print!("{}: ", user.name);
        if user.exist().await.unwrap() {
            println!("[ok]");
            continue;
        }

        let obj = spanned.as_span_object().unwrap();
        println!("{}", obj.range().start);

        use codespan_reporting::term;
        term::emit(
            &mut term::termcolor::StandardStream::stdout(term::termcolor::ColorChoice::Auto),
            &term::Config::default(),
            &files,
            &codespan_reporting::diagnostic::Diagnostic::error()
                .with_message("this user does not exist!")
                .with_labels(vec![codespan_reporting::diagnostic::Label::primary(
                    file,
                    obj.range(),
                )]),
        )
        .unwrap();

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
