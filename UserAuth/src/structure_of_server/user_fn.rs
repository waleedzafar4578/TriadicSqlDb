use std::{fmt, io};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::time::{SystemTime, UNIX_EPOCH};
use bcrypt::{hash, verify};
use sha2::{Digest, Sha256};
use crate::User;

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "UserToken:{}\nUsername:{}\nPassword:{}\nUniqueId:{}\nSelected Database:{}\nTimestamp:{}\nExpirationTime:{}",
            self.token, self.username, self.password, self.unique_id, self.selected_database,self.timestamp,self.expiration_time
        )
    }
}

impl Default for User {
    fn default() -> Self {
        Self {
            token: "".to_string(),
            username: "".to_string(),
            password: "".to_string(),
            unique_id: "".to_string(),
            selected_database: "".to_string(),
            timestamp: 0,
            expiration_time: 0,
        }
    }
}

impl Clone for User {
    fn clone(&self) -> Self {
        Self {
            token: self.token.to_string(),
            username: self.username.to_string(),
            password: self.password.to_string(),
            unique_id: self.unique_id.to_string(),
            selected_database: self.selected_database.to_string(),
            timestamp: self.timestamp,
            expiration_time: self.expiration_time,
        }
    }
}

impl User {
    // Function to validate username
    pub fn validate_username(username: &str) -> Result<String, String> {
        // Trim leading and trailing whitespace
        let trimmed_username = username.trim();

        // Check if the username is empty after trimming whitespace
        if trimmed_username.is_empty() {
            return Err("Username cannot be empty.".to_string());
        }

        // Check if the username length is less than 8 characters
        if trimmed_username.len() < 8 {
            return Err("Username must be at least 8 characters long.".to_string());
        }

        // Check if the username starts with a digit
        if trimmed_username.chars().next().unwrap().is_ascii_digit() {
            return Err("Username cannot start with a digit.".to_string());
        }

        // Additional checks (such as special characters, profanity filter, etc.) can be added here

        // If all checks pass, return the validated username
        Ok(trimmed_username.to_string())
    }

    pub fn validate_password(password: &str) -> Result<String, String> {
        // Trim leading and trailing whitespace
        let trimmed_password = password.trim();

        // Check if the password is empty after trimming whitespace
        if trimmed_password.is_empty() {
            return Err("Password cannot be empty.".to_string());
        }

        // Check if the password length is less than 8 characters
        if trimmed_password.len() < 8 {
            return Err("Password must be at least 8 characters long.".to_string());
        }

        // Check if the password contains at least one symbol
        if !trimmed_password.chars().any(|c| c.is_ascii_punctuation()) {
            return Err("Password must contain at least one symbol.".to_string());
        }

        // Check if the password contains at least one uppercase letter
        if !trimmed_password.chars().any(|c| c.is_ascii_uppercase()) {
            return Err("Password must contain at least one uppercase letter.".to_string());
        }

        // Check if the password contains at least one number
        if !trimmed_password.chars().any(|c| c.is_ascii_digit()) {
            return Err("Password must contain at least one number.".to_string());
        }

        // Hash the password
        let hashed_password = match hash(trimmed_password, 10u32) {
            Ok(hashed_password) => Ok(hashed_password),
            Err(err) => Err(format!("Error hashing password: {:?}", err)),
        }
            .expect("TODO: panic message");
        // If all checks pass, return the hashed password
        Ok(hashed_password)
    }

    pub fn generate_token(&mut self) {
        // Get current timestamp
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("SystemTime before UNIX EPOCH!")
            .as_secs();

        // Set timestamp
        self.timestamp = current_time;

        // Set expiration time (10 minutes from current time)
        self.expiration_time = current_time + 600; // 600 seconds = 10 minutes

        // Concatenate user properties and timestamp to create input for hashing
        let input = format!(
            "{}:{}:{}:{}:{}:{}:{}", // Added {} for expiration time
            self.username,
            self.password,
            self.unique_id,
            self.selected_database,
            self.token,
            self.timestamp,       // Added timestamp
            self.expiration_time  // Added expiration time
        );

        // Calculate hash using the SHA-256 algorithm
        let mut hasher = Sha256::new();
        hasher.update(input);
        let result = hasher.finalize();

        // Convert hash bytes to hexadecimal string
        self.token = result
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect();
    }
    // Function to verify password
    pub fn verify_password(&self, password: &str) -> bool {
        // Verify the password against the hashed password
        if let Ok(result) = verify(password, &self.password) {
            result
        } else {
            false
        }
    }
}

//public function
impl User {
    pub fn set(&mut self, username: &str, password: &str,confirm: &str) -> String {
        match Self::validate_username(username) {
            Ok(_vname) => {
                self.username = _vname;
            }
            Err(_ename) => {
                return _ename;
            }
        };
        if password !=confirm{
            return "Password and Confirm password not equal".to_string();
        }
        match Self::validate_password(password) {
            Ok(_vpass) => {
                self.password = _vpass;
            }
            Err(_epass) => {
                return _epass;
            }
        }
        self.unique_id = "../../AllUserData/".to_owned() + &*self.username.clone()+"/";
        "Account Created".to_string()
    }
    pub fn compare_tokens(&self, input_token: &str) -> bool {
        // Check if the input token matches the user's token
        if input_token == self.token {
            // Get current time
            let current_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("SystemTime before UNIX EPOCH!")
                .as_secs();

            // Check if the token has not expired
            current_time <= self.expiration_time
        } else {
            false
        }
    }
    pub fn cloned_username(&self) -> String {
        self.username.clone()
    }
    pub fn cloned_path(&self) -> String {
        self.unique_id.clone()
    }
    pub fn cloned_token(&self) -> String {
        self.token.clone()
    }
    pub fn set_database(&mut self,db:&str)->bool{
        self.selected_database= db.to_string();
        true
    }
}
//for file input & output

impl User {
    pub fn write_to_file(filename: &str, data: &str) -> io::Result<()> {
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
    pub fn read_from_file(filename: &str) -> io::Result<String> {
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