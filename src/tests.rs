#[cfg(test)]
mod tests {
    use crate::main;
    use crate::get_settings;
    use crate::get_repository_url;
    use crate::open_note_editor;
    use std::io::prelude::*;
    use std::fs::File;

    #[test]
    fn test_get_settings() {
        let file_name = ".Meiling.test.toml";
        let file_content = "repository = \"https://github.com/sinvalmendes/notes\"";
        create_test_config_file(file_content, file_name);
        let settings = get_settings(&file_name);
        println!("{:?}", settings);
        let result = match settings.get("repository") {
            Some(x) => x,
            None => panic!()
        };
    }

    #[test]
    fn test_get_repository_url() {
        let url = get_repository_url();
        println!("{:?}", url);
        assert_eq!("https://github.com/sinvalmendes/notes", url);
    }

    #[test]
    fn test_open_note_editor() {
        open_note_editor("file");
    }


    fn create_test_config_file(file_content: &str, file_name: &str) {
        let mut file = File::create(&file_name);
        match file {
            Ok(mut x) => {
                x.write(file_content.as_bytes());
            },
            Err(e) => {
                panic!();
            }
        }
    }
}
