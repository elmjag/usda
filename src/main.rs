use std::env;
use usd::Stage;

fn parse_args() -> Result<Vec<String>, String> {
    let mut args = env::args();
    let prog = args.next().unwrap();
    let files: Vec<String> = args.collect();

    if files.len() == 0 {
        return Err(format!("usage: {prog} file1 [fileN]*"));
    }
    Ok(files)
}

fn dump_file(file_path: &str) {
    let stage = Stage::open(file_path);

    for prim in stage.traverse() {
        println!("prim: '{}'", prim.name());
    }
}

fn main() {
    let args = parse_args();

    if let Err(error_msg) = args {
        eprintln!("{error_msg}");
        return;
    }

    for file in args.unwrap() {
        dump_file(file.as_str());
    }
}
