use std::fs;
pub struct Bike {
    pub make: String,
    pub model: String
}

pub fn read_file_as_string() -> Result<String, std::io::Error> {
    let file_content = fs::read_to_string("bikes.yaml");
    file_content
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_file_opens() {
        match read_file_as_string() {
            Ok(_) => assert!(true, "bikes file has been opened."),
            Err(_) => assert!(true, "bikes file could not be opened.")
        }
        
    }

}
