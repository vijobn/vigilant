use std::process::{Command, exit};
use std::io::{self, Write};
use chrono::Utc;
use std::vec::Vec;

/// Struct to store information about command execution.
#[derive(Debug, Clone)]
pub struct CmdOutput {
    pub cmdline: String,
    pub timestamp: String,        // The timestamp when the command was executed
    pub output: Vec<String>,      // The output of the command, stored in a Vec
        cmdname: String,
        //cmdargs: Vec<str>,
        current: usize,
        headers: Option<Vec<String>>,
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
                    headers: None,
                    cmdname: args[0].to_string(),
                    //cmdargs: args[1..].to_vec()
                    current: 0
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

    /// Executes a given command line string and returns a CmdOutput struct.
    pub fn execute(&mut self, cmdline: &str) -> Result<Vec<String>, io::Error> {
        // Ensure that there is at least one argument (the command itself)
        if self.cmdname.is_empty() {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "No command provided"));
        }

        // Execute the command
        let output = Command::new(self.cmdname.clone())
            //.args(&args[1..])
            .output();

        self.current = 0;
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
                //CmdOutput::new(cmdline)
                Ok(output_lines)
            }
            Err(e) => {
                // If there was an error with the command execution itself
                Err(e)
            }
        }
    }

    pub fn get_headers(&mut self) -> Result<Vec<String>, io::Error> {
        // Ensure that there is at least one argument (the command itself)
        if self.cmdname.is_empty() {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "No command provided"));
        }
        if self.output.len() == 0 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "No output from cmdline"));
        }
        let hdrs = self.output[0].clone();
        let is_all_uppercase = hdrs.chars().all(|c| !c.is_alphabetic() || c.is_uppercase());
        if is_all_uppercase {
            self.headers = Some(hdrs.split_whitespace().map(String::from).collect::<Vec<String>>());
            return Ok(self.headers.clone().expect("Headers bad"));
        }
        Err(io::Error::new(io::ErrorKind::InvalidData, "Headers are not all uppercase"))
    }

    pub fn update_lines(&mut self, oplines: Vec<String>) -> Result<Vec<usize>, String> {
        let mut changed =  Vec::new();

        for oi in 0..self.output.len() {
            if self.output[oi] != oplines[oi] {
                println!("Found a new update on line numbered {} is {}", oi, oplines[oi]);
                println!("Found a old update on line numbered {} is {}", oi, self.output[oi]);
                self.output[oi] = oplines[oi].clone();
                println!("Now line is {}", self.output[oi]);
                changed.push(oi);
            }
        }
        self.current = 0;

        Ok(changed)
    }

    pub fn get_output_line(mut self, num: usize) -> Option<String> {
        if num < self.output.len() {
            return Some(self.output[num].clone());
        }
        None
    }

    pub fn next(&mut self) -> Option<String> {
        // Check if the current index is within bounds
        println!("Next with current: {}", self.current);
        if self.current < self.output.len() {
            // Return the next output line and increment the index
            let value = self.output[self.current].clone();
            self.current += 1;
            Some(value)
        } else {
            // Return None when we've exhausted all elements in the output Vec
            None
        }
    }
}

