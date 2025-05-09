use std::{fs, io, path::Path};

pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

pub fn snake_case_to_camel_case(s: String) -> String {
    s.split('_')
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut c = s.chars();

            let first = match c.next() {
                Some(f) => f.to_uppercase().collect::<String>(),
                None => String::new(),
            };
            let rest = c.collect::<String>();

            first + &rest
        })
        .collect()
}

mod tests {
    #[test]
    fn test_snake_case_to_camel_case() {
        assert_eq!(
            super::snake_case_to_camel_case("hello_world".to_string()),
            "HelloWorld"
        );
        assert_eq!(
            super::snake_case_to_camel_case("hello_world_123".to_string()),
            "HelloWorld123"
        );
        assert_eq!(
            super::snake_case_to_camel_case("hello_world_123_hello_world".to_string()),
            "HelloWorld123HelloWorld"
        );
        assert_eq!(
            super::snake_case_to_camel_case("hello_world__hello_world".to_string()),
            "HelloWorldHelloWorld"
        );
        assert_eq!(
            super::snake_case_to_camel_case("_hello_world_hello_world_".to_string()),
            "HelloWorldHelloWorld"
        );
    }
}
