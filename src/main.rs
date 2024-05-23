use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        panic!("File name was not provided as run argument");
    }

    let file_to_read = File::open(&args[1]);
    let file = match file_to_read {
        Ok(file) => file,
        Err(error) => panic!("An Error accured {:?}", error),
    };

    let reader = BufReader::new(file);
    let mut line_counter = 1;
    for line in reader.lines() {
        match line {
            Ok(line) => {
                println!("Line {} from file {}", line_counter, line);
                line_counter += 1;
            }
            Err(error) => {
                panic!(
                    "An error {} accured while reading line number {}",
                    error, line_counter
                );
            }
        };
    }
}
