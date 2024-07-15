#[cfg(test)]
mod file_test {
    use crate::file::write_json;

    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    struct TestStruct {
        a: i32,
        b: String,
    }

    impl TestStruct {
        fn new(a: i32, b: &str) -> Self {
            Self {
                a,
                b: b.to_string(),
            }
        }
    }

    #[test]
    fn test_write_json() {
        let test = TestStruct::new(42, "hello");
        let path = "./test/test.json";
        let result = write_json(path, test);
        assert!(result.is_ok());
    }
}
