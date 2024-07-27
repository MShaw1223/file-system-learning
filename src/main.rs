use std::fs;
use std::fs::File;
use std::io::{self, Result, Write};

fn main() {
    loop {
        println!(
            "
            Choose one of the following:
                1. Create a new file - 'C'
                2. Delete a file - 'D'
                3. Append a file - 'A'

                press 'Q' to quit
            "
        );
        let choice: String = get_input();
        let fname: String;
        if choice == "C" || choice == "c" {
            // add filename
            println!(
                "
            Enter the name of the new file
            "
            );
            fname = get_input();
            match create_file(&fname) {
                Ok(()) => println!("success"),
                Err(e) => println!("failure: {}", e),
            }
        } else if choice == "D" || choice == "d" {
            println!(
                "
            Enter the file name, ie foobar.bam, foo.txt etc
            "
            );
            fname = get_input();
            match delete_file(&fname) {
                Ok(()) => println!("success"),
                Err(e) => println!("failure: {}", e),
            }
        } else if choice == "A" || choice == "a" {
            println!(
                "
            Enter the file name, ie foobar.bam, foo.txt etc
            "
            );
            fname = get_input();
            match edit_file(&fname) {
                Ok(()) => println!("success"),
                Err(e) => println!("failure: {}", e),
            }
        } else if choice == "Q" || choice == "q" {
            println!(
                "

                ...Exiting...

                "
            );
            break;
        } else {
            println!(
                "

                ...Incorrect input...

                "
            )
        }
    }
}

fn edit_file(name: &str) -> Result<()> {
    let file: String = format!("{}", name);
    println!(
        "
        Enter the new text below:
        "
    );
    let contents: String = get_input();
    fs::write(file, contents)?;
    Ok(())
}

fn delete_file(name: &str) -> Result<()> {
    fs::remove_file(name)?;
    Ok(())
}
fn create_file(name: &str) -> Result<()> {
    // Define the file path
    let file_path = format!("{}.txt", name);

    // Create a new file (this will overwrite the file if it already exists)
    let mut file = File::create(file_path)?;

    // Data to write into the file
    let data = "Hello, world!";

    // Write data to the file
    file.write_all(data.as_bytes())?;

    // Print a success message
    println!("File created and data written successfully.");

    Ok(())
}
fn get_input() -> String {
    let mut cli_input = String::new();

    // print macro instead of println so no newline is at the end leading to prompt on one line and input on another
    print!("Enter input here: ");

    // Flush stdout to ensure the prompt is displayed before input
    io::stdout().flush().expect("Failed to flush stdout");

    io::stdin()
        .read_line(&mut cli_input)
        .expect("Failed to read input");

    return cli_input.trim().to_string();
}
