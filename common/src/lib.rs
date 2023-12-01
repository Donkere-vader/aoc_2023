use std::{fs, io, path::Path};

#[derive(Debug)]
pub enum InputFileError {
    FileNotFound(String),
    IoError(io::Error),
}

impl From<io::Error> for InputFileError {
    fn from(value: io::Error) -> Self {
        InputFileError::IoError(value)
    }
}

pub fn get_input_file(day: usize, file: &str) -> Result<String, InputFileError> {
    let path_string = format!("input/{file}");
    let path_string_2 = format!("day_{day}/input/{file}");

    let mut path = Path::new(&path_string);

    if !path.exists() {
        path = Path::new(&path_string_2);
    }
    if !path.exists() {
        return Err(InputFileError::FileNotFound(format!(
            "Path '{}' doesn't exist",
            path_string
        )));
    }

    Ok(fs::read_to_string(path)?)
}
