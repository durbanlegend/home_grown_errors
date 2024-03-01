use std::error::Error;
use std::fmt::{self, Display};

#[derive(Debug)]
enum MyError {
    FileReadError(std::io::Error),
    InvalidDataError(String),
}

// Implement `Display` for `MyError`.
impl Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let desc = match self {
            MyError::InvalidDataError(ref _e) => "InvalidDataError".to_string(),
            MyError::FileReadError(e) => format!("FileReadError({})", &e.to_string()),
        };
        write!(f, "{desc}")
    }
}

impl Error for MyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            // The cause is the underlying implementation error type. Is implicitly
            // cast to the trait object `&error::Error`. This works because the
            // underlying type already implements the `Error` trait.
            MyError::InvalidDataError(ref _e) => Some(self),
            MyError::FileReadError(ref e) => Some(e),
        }
    }
}

fn main() -> Result<(), MyError> {
    // Read a file and process its content
    let data = std::fs::read_to_string("data.txt").map_err(MyError::FileReadError)?;

    // Process the data and extract the desired String
    let processed_data = process_data(&data)?;
    println!("processed_data={processed_data}");

    // Return the processed data as a success
    Ok(())
}

// Example function to process data and potentially return an error
fn process_data(data: &str) -> Result<String, MyError> {
    // ... some logic to process data ...
    let data = data.trim();
    if data.is_empty() {
        return Err(MyError::InvalidDataError("Data is empty".to_string()));
    }

    // Return the processed String
    Ok(String::from(data))
}
