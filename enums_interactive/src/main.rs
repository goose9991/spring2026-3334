
use std::process::Command;
use std::io;
use std::io::Write;

enum FileOperation {
    List(String),               // Directory path
    Display(String),            // File path
    Create(String, String),     // File path and content
    Remove(String),             // File path
    Pwd,                        // Print working directory
}

fn perform_operation(operation: FileOperation) {
    // Implement command execution based on the operation
    match operation {
        FileOperation::List(directory_path) => {
            if let Err(e) = Command::new("ls").arg(&directory_path).status() {
                println!("Failed to list directory: {}", e);
            }
        }
        FileOperation::Display(file_path) => {
            if let Err(e) = Command::new("cat").arg(&file_path).status() {
                println!("Failed to display file: {}", e);
            }
        }
        FileOperation::Create(file_path, content) => {
            let command = format!("echo '{}' > {}", content, file_path);

            match Command::new("sh")
                .arg("-c")
                .arg(&command)
                .status()
            {
                Ok(status) if status.success() => {
                    println!("File '{}' created successfully.", file_path);
                }
                Ok(_) => {
                    println!("Failed to create file '{}'.", file_path);
                }
                Err(e) => {
                    println!("Error executing command: {}", e);
                }
            }
        }
        FileOperation::Remove(file_path) => {
            match Command::new("rm").arg(&file_path).status() {
                Ok(status) if status.success() => {
                    println!("File '{}' removed successfully.", file_path);
                }
                Ok(_) => {
                    println!("Failed to remove file '{}'.", file_path);
                }
                Err(e) => {
                    println!("Error executing rm: {}", e);
                }
            }
        }
        FileOperation::Pwd => {
            print!("\nCurrent working directory: ");
            io::stdout().flush().unwrap();
            if let Err(e) = Command::new("pwd").status() {
                println!("Failed to execute pwd: {}", e);
            }
        }
    }

}

fn main() {

    loop {
        
        println!("\nWelcome to the File Operations Program!\n");
        println!("\tFile Operations Menu:");
        println!("\t1. List files in a directory");
        println!("\t2. Display file contents");
        println!("\t3. Create a new file");
        println!("\t4. Remove a file");
        println!("\t5. Print working directory");
        println!("\t0. Exit\n");

        print!("Enter your choice (0-5): ");

        io::stdout().flush().unwrap();
        let mut input_string = String::new();

        io::stdin()
            .read_line(&mut input_string)
            .expect("Failed to read input");

        let choice: u32 = match input_string.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number between 0 and 5.");
                continue;
            }
        };

        match choice {

            1 => {
                    print!("Enter the directory path: ");
                    io::stdout().flush().unwrap();

                    let mut dir_path = String::new();
                    io::stdin().read_line(&mut dir_path).expect("Failed to read input");

                    let dir = dir_path.trim();

                    if dir.is_empty() {
                        println!("Directory path cannot be empty.");
                        continue;
                    }

                    let operation = FileOperation::List(dir.to_string());
                    perform_operation(operation);
                },
            2 => {
                    print!("Enter file path: ");
                    io::stdout().flush().unwrap();

                    let mut file_path = String::new();
                    io::stdin().read_line(&mut file_path).expect("Failed to read input");

                    let f = file_path.trim();

                    if f.is_empty() {
                        println!("File path cannot be empty.");
                        continue;
                    }
                    let operation = FileOperation::Display(f.to_string());
                    perform_operation(operation);
                },
            3 => {
                    print!("Enter file name: ");
                    io::stdout().flush().unwrap();

                    let mut file_name = String::new();
                    io::stdin().read_line(&mut file_name).expect("Failed to read input");

                    let f = file_name.trim();

                    if f.is_empty() {
                        println!("File name cannot be empty.");
                        continue;
                    }

                    print!("Enter content: ");
                    io::stdout().flush().unwrap();

                    let mut content = String::new();
                    io::stdin().read_line(&mut content).expect("Failed to read input");

                    let c = content.trim();

                    if c.is_empty() {
                        println!("Content cannot be empty.");
                        continue;
                    }
    

                    let operation = FileOperation::Create(f.to_string(), c.to_string());
                    perform_operation(operation);
                },
            4 => {                
                    print!("Enter file path: ");
                    io::stdout().flush().unwrap();

                    let mut file_path = String::new();
                    io::stdin().read_line(&mut file_path).expect("Failed to read input");

                    let f = file_path.trim();

                    if f.is_empty() {
                        println!("File path cannot be empty.");
                        continue;
                    }

                    let operation = FileOperation::Remove(f.to_string());
                    perform_operation(operation);
                },
            5 => {
                    let operation = FileOperation::Pwd;
                    perform_operation(operation);
                },
            0 => {
                    println!("Goodbye!");
                    break;
                },
            _ => println!("\nInvalid choice, please try again."),

        }

    }

}
