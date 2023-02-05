use clap::Parser;

#[derive(Debug, Parser)]
struct CliArgs {
    input_string: String,
    file_path: std::path::PathBuf,
}

fn main() {
    // Read args via clap
    let CliArgs {
        input_string,
        file_path,
    } = CliArgs::parse();
    println!(
        "input_string: {}, file_path: {}",
        input_string,
        file_path
            .to_str()
            .expect("file_path should be parsable to string")
    );

    // Open the file using the file_full_name
    let file_in_string = std::fs::read_to_string(&file_path);
    let file_content = match file_in_string {
        Ok(file_content) => file_content,
        Err(error) => {
            panic!("NONONONONONO {}", error)
        }
    };
    for line in file_content.lines() {
        if line.contains(&input_string) {
            println!("{}", &line);
        }
    }
}
