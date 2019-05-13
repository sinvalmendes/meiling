#[cfg(test)]
mod tests {
    use crate::main;
    use crate::get_settings;
    use crate::get_repository_url;
    use crate::open_note_editor;

    #[test]
    fn test_get_settings() {
        let settings = get_settings(".Meiling.toml");
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

}
