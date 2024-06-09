use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Write};
use crate::TFile;

impl TFile {
    pub fn write_to_file(data: &str) -> io::Result<()> {
        let filename = "userdata";
        // Open the file for writing, creating it if it doesn't exist, and truncating it if it does
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(filename)?;

        // Create a buffered writer to improve I/O performance
        let mut writer = BufWriter::new(file);

        // Write data to the file
        writer.write_all(data.as_bytes())?;

        // Flush the buffered writer to ensure all data is written to the file
        writer.flush()?;

        // Close the file
        Ok(())
    }
    pub fn read_from_file() -> io::Result<String> {
        let filename = "userdata";
        // Open the file for reading
        let file = File::open(filename)?;

        // Create a buffered reader for efficient reading
        let reader = BufReader::new(file);

        // Read all lines from the file and concatenate them into a single string
        let mut content = String::new();
        for line in reader.lines() {
            content.push_str(&line?);
            content.push('\n'); // Add newline character between lines
        }
        
        // Return the read content
        Ok(content)
    }
}