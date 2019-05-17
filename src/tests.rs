#[cfg(test)]
mod tests {
    use crate::get_repository_url;
    use crate::get_settings;
    use crate::get_repository;
    use std::panic;
    use git2::RepositoryState;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn test_get_settings() {
        let file_name = ".Meiling.test.toml";
        let file_content = "repository = \"https://github.com/sinvalmendes/notes\"";
        create_test_config_file(file_content, file_name);
        let settings = get_settings(&file_name);
        println!("{:?}", settings);
        match settings.get("repository") {
            Some(x) => x,
            None => panic!(),
        };
    }

    #[test]
    fn test_get_repository_url() {
        let file_name = ".Meiling.test.toml";
        let file_content = "repository = \"https://github.com/sinvalmendes/notes\"";
        create_test_config_file(file_content, file_name);
        let repository_url = get_repository_url(&file_name);
        println!("{:?}", repository_url);
        assert_eq!("https://github.com/sinvalmendes/notes", repository_url);
    }

    #[test]
    fn test_get_repository() {
        let file_name = ".test_get_repository.test.toml";
        let file_content = "repository = \"https://github.com/sinvalmendes/notes\"";
        create_test_config_file(file_content, file_name);
        let repository_url = get_repository_url(&file_name);
        assert_eq!("https://github.com/sinvalmendes/notes", repository_url);
        let repository = get_repository(&repository_url, ".test.repositories/test01/");
        assert_eq!(Ok(false), repository.is_empty());
        assert_eq!(RepositoryState::Clean, repository.state());
    }


    #[test]
    fn test_get_non_existent_repository() {
        let file_name = ".test_get_non_existent_repository.test.toml";
        let file_content = "repository = \"https://dontexist1234567890.com/sinvalmendes/notes\"";
        create_test_config_file(file_content, file_name);
        let repository_url = get_repository_url(&file_name);

        panic::set_hook(Box::new(|_info| {
            // do nothing, this is just to set the hook for panic::catch_unwind
        }));

        let result = panic::catch_unwind(|| {
            get_repository(&repository_url, ".test.repositories/test02/");
        });

        match result {
            Ok(res) => panic!("the get_repository should have panicked!"),
            Err(_) => (),
        }
    }

    fn create_test_config_file(file_content: &str, file_name: &str) {
        let mut file = File::create(&file_name);
        match file {
            Ok(mut x) => x.write(file_content.as_bytes()),
            Err(e) => panic!(),
        };
    }
}
