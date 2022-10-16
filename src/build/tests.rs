#[cfg(test)]
mod build_tests {
    use crate::build::get_html_file_path;
    use std::path::Path;

    #[test]
    fn test_get_html_file_path_empty() {
        assert!(get_html_file_path(Path::new(""), Path::new("")).is_err());
    }

    #[test]
    fn test_get_html_file_path() {
        let tests = vec![
            (
                "articles/2019-12-31-markdown-demo.md",
                "./doc/2019-12-31-markdown-demo.html",
            ),
            (
                "articles/2019-12-31-markdown-demo.txt",
                "./doc/2019-12-31-markdown-demo.txt",
            ),
        ];

        tests.into_iter().for_each(|(value, expect)| {
            let config_path = Path::new("./doc");
            let path = Path::new(value);

            assert_eq!(
                get_html_file_path(config_path, path).expect("could not retrieve filename"),
                expect
            );
        });
    }
}
