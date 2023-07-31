#[cfg(test)]
mod build_tests {
    use crate::build::get_html_file_path;
    use std::path::Path;

    #[test]
    fn test_get_html_file_path_empty() {
        assert!(get_html_file_path(Path::new("")).is_err());
    }

    #[test]
    fn test_get_html_file_path() {
        let tests = vec![
            (
                "posts/2019-12-31-markdown-demo.md",
                "2019-12-31-markdown-demo.html",
            ),
            (
                "posts/2019-12-31-markdown-demo.txt",
                "2019-12-31-markdown-demo.txt",
            ),
        ];

        for (value, expect) in tests {
            let path = Path::new(value);

            assert_eq!(
                get_html_file_path(path).expect("could not retrieve filename"),
                expect
            );
        }
    }
}
