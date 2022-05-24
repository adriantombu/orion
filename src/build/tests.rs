#[cfg(test)]
mod tests {
    use crate::build::get_html_file_path;
    use std::path::PathBuf;

    #[test]
    fn test_get_html_file_path() {
        let tests = vec![
            ("", "../blog/public/"),
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

            assert_eq!(get_html_file_path(&path), expect);
        }
    }
}
