#[cfg(test)]
mod build_tests {
    use crate::build::get_html_file_path;
    use std::path::PathBuf;

    #[test]
    fn test_get_html_file_path_empty() {
        let mut path = PathBuf::new();
        path.push("");

        assert!(get_html_file_path(&path).is_err());
    }

    #[test]
    fn test_get_html_file_path() {
        let tests = vec![
            (
                "articles/2019-12-31-markdown-demo.md",
                "../blog/public/2019-12-31-markdown-demo.html",
            ),
            (
                "articles/2019-12-31-markdown-demo.txt",
                "../blog/public/2019-12-31-markdown-demo.txt",
            ),
        ];

        for (value, expect) in tests {
            let mut path = PathBuf::new();
            path.push(value);

            assert_eq!(
                get_html_file_path(&path).expect("couldn't retrieve html value"),
                expect
            );
        }
    }
}
