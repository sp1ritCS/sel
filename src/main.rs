#[macro_use]
extern crate clap;
extern crate victoria_dom;
use clap::App;
use std::fs::File;
use std::io::BufReader;
use std::io::{self, Read};
use std::process;
use victoria_dom::DOM;
use walkdir::WalkDir;

static mut PARAM_VERBOSITY: u64 = 0;

fn v_printer(verbosity: u64, message: &'static str) {
    unsafe {
        if PARAM_VERBOSITY >= verbosity {
            println!("{}", message);
        }
    }
}

fn get_piped() -> String {
    let mut input = String::new();
    match io::stdin().lock().read_to_string(&mut input) {
        Ok(n) => {
            println!("{} bytes read", n);
            //println!("{}", input);
            return input;
        }
        Err(error) => {
            println!("error: {}", error);
            process::exit(1);
        }
    }
}

fn get_file(path: String) -> String {
    let file = File::open(&path);
    match file {
        Ok(file) => {
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();
            buf_reader.read_to_string(&mut contents);
            return contents;
        }
        Err(e) => {
            println!("file not found \n{:?}", e);
            return "".to_string();
        }
    }
}

fn get_folder(path: String) -> Vec<String> {
    let mut files = Vec::new();
    for entry in WalkDir::new(path) {
        let entry = entry.unwrap();
        files.push(get_file(entry.path().display().to_string()));
    }
    return files;
}

fn parser(html: String, selector: &str, single: bool) {
    let dom = DOM::new(html.as_str());
    if single {
        println!("{}", dom.at(selector).unwrap().to_string());
    } else {
        for node in dom.find(selector) {
            println!("{}", node.to_string())
        }
    }
}
fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    unsafe {
        PARAM_VERBOSITY = matches.occurrences_of("verbose");
    };
    let selector = matches.value_of("selector").unwrap();
    let single = matches.occurrences_of("single") > 0;
    if matches.value_of("input").is_some() {
        if matches.occurrences_of("recursive") > 0 {
            v_printer(2, "Using Folder recursivly");
            for file in get_folder(matches.value_of("input").unwrap().to_string()) {
                parser(file, selector, single);
            }
        } else {
            v_printer(2, "Using Input File");
            parser(
                get_file(matches.value_of("input").unwrap().to_string()),
                selector,
                single,
            );
        }
    } else {
        v_printer(2, "Using Piped");
        parser(get_piped(), selector, single);
    }
}
