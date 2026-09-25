use std::{
    env, fs,
    io::{self, BufReader, Read, Write},
    process, result,
};

fn main() {
    let mut file_path = String::new();
    match fetch_filepath() {
        Ok(fp) => file_path.push_str(&fp),
        Err(error) => {
            eprintln!("{error}");
            process::exit(1);
        }
    }

    match validate_filepath(&file_path) {
        Ok(_) => {}
        Err(error) => {
            eprintln!("{file_path} {error}");
            process::exit(1);
        }
    }

    match read_file_contents(&file_path) {
        Ok(fc) => {
            let stdout = io::stdout();
            let mut handle = stdout.lock();

            if let Err(error) = handle.write_all(&fc) {
                eprintln!("ERROR: couldn't write file contents to stdout. {}", error);
                process::exit(1)
            }
        }
        Err(error) => {
            eprintln!("{error}");
            process::exit(1);
        }
    }
}

fn read_file_contents(file_path: &String) -> result::Result<Vec<u8>, io::Error> {
    let file_object = fs::File::open(file_path)?;

    let mut buffer_reader = BufReader::new(file_object);
    let mut contents: Vec<u8> = Vec::new();
    buffer_reader.read_to_end(&mut contents)?;

    Ok(contents)
}

fn validate_filepath(file_path: &str) -> result::Result<(), io::Error> {
    if !fs::exists(file_path)? {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "File Does Not Exist!",
        ));
    }

    let metadata = fs::metadata(file_path)?;
    if metadata.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Is A Directory!",
        ));
    }

    Ok(())
}

fn fetch_filepath() -> result::Result<String, String> {
    let argv: Vec<String> = env::args().collect();

    if argv.len() <= 1 {
        return Err(String::from("Please Provide A File Path."));
    } else if argv.len() > 2 {
        return Err(String::from(
            "File Path Cannot Contain Spaces, Include The Path In Double Quotes If Spaces Are Needed.",
        ));
    }

    Ok(argv[1].clone())
}
