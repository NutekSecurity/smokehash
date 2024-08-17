use sha256::try_digest;
use std::path::Path;

#[cfg(test)]
mod tests {
    use std::io::ErrorKind;

    use super::*;

    #[test]
    fn try_digest_succeeds() {
        let write_file = std::fs::write("test", "test").is_ok();
        assert!(write_file, "creating temporary file with content");
        let input = Path::new("./test");
        let val = try_digest(input).unwrap();
        let del_file = std::fs::remove_file("test").is_ok();
        assert!(del_file, "deleteing temporary file");
        assert_eq!(
            val,
            "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08"
        );
    }

    #[test]
    #[should_panic(expected = "Error: NotFound")]
    fn try_digest_fails() {
        let input = Path::new("nonexistent.file");
        let val = try_digest(input);
        match val {
            Ok(_) => {
                panic!("should not be Ok")
            }
            Err(error) => {
                let e = error.kind();
                if e == ErrorKind::NotFound {
                    panic!("Error: NotFound");
                } else {
                    panic!("Not looking for error: {}", e);
                }
            }
        }
    }
}
