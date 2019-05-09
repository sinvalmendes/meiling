#[cfg(test)]
mod tests {
    use crate::main;
    use crate::get_settings;

    #[test]
    fn test_get_settings() {
        let settings = get_settings();
        println!("{:?}", settings);
        let result = match settings.get("repository") {
            Some(x) => x,
            None => panic!()
        };
        assert_eq!("https://github.com/sinvalmendes/notes", result);
    }

}
