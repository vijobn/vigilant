use std::process::{Command, exit};
use std::io::{self, Write};
use chrono::Utc;
use std::vec::Vec;

/// Struct to store information about a command execution.
pub struct CmdOutput {
    pub cmdline: String,          // The command line that was executed
    pub timestamp: String,        // The timestamp when the command was executed
    pub output: Vec<String>,      // The output of the command, stored in a Vec
}

impl CmdOutput {
    /// Creates a new CmdOutput instance with the given command line.
    /// It executes the command, captures the output, and stores it in the struct.
    pub fn new(cmdline: &str) -> Result<Self, io::Error> {
        // Split the cmdline string into arguments
        let args: Vec<&str> = cmdline.split_whitespace().collect();

        // Ensure that there is at least one argument (the command itself)
        if args.is_empty() {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "No command provided"));
        }

        // Execute the command
        let output = Command::new(args[0])
            .args(&args[1..])
            .output(); // Execute the command and capture output

        match output {
            Ok(output) => {
                if !output.status.success() {
                    // If the command failed, return an error
                    return Err(io::Error::new(io::ErrorKind::Other, "Command failed"));
                }

                // Capture the standard output as a string and split it into lines
                let output_lines = String::from_utf8_lossy(&output.stdout)
                    .lines()
                    .map(|line| line.to_string())
                    .collect::<Vec<String>>();

                // Create and return the CmdOutput struct
                Ok(CmdOutput {
                    cmdline: cmdline.to_string(),
                    timestamp: Utc::now().to_rfc3339(), // Timestamp when the command was executed
                    output: output_lines,
                })
            }
            Err(e) => {
                // If there was an error with the command execution itself
                Err(e)
            }
        }
    }

    //pub fn set_cmdline(mut self, cmdline: String) -> Result<T, E> {
    //    self.cmdline = cmdline;

    //    Ok(())
   // }
}

/// Executes a given command line string and returns a CmdOutput struct.
pub fn execute(cmdline: &str) -> Result<CmdOutput, io::Error> {
    // Split the cmdline string into arguments
    let args: Vec<&str> = cmdline.split_whitespace().collect();

    // Ensure that there is at least one argument (the command itself)
    if args.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "No command provided"));
    }

    // Execute the command
    let output = Command::new(args[0])
        .args(&args[1..])
        .output();

    match output {
        Ok(output) => {
            if !output.status.success() {
                // If the command failed, return an error
                return Err(io::Error::new(io::ErrorKind::Other, "Command failed"));
            }

            // Capture the standard output as a string and split it into lines
            let output_lines = String::from_utf8_lossy(&output.stdout)
                .lines()
                .map(|line| line.to_string())
                .collect::<Vec<String>>();

            // Create and return the CmdOutput struct
            CmdOutput::new(cmdline)
        }
        Err(e) => {
            // If there was an error with the command execution itself
            Err(e)
        }
    }
}


