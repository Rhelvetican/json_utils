#[cfg(test)]
mod file_test {
    use crate::{file::write_json, prelude::read_json};

    use serde::{Deserialize, Serialize};
    use serde_json::json;

    #[derive(Debug, Serialize, Deserialize)]
    struct OptionSerde {
        option: Option<TestStruct>,
        maybe: Option<i32>,
    }

    impl OptionSerde {
        fn new(option: Option<TestStruct>, maybe: Option<i32>) -> Self {
            Self { option, maybe }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct TestStruct {
        a: i32,
        b: String,
    }

    impl TestStruct {
        fn new(a: i32, b: &str) -> Self {
            Self { a, b: b.to_string() }
        }
    }

    #[test]
    fn test_write_json() {
        let test = TestStruct::new(42, "hello");
        let path = "./test/test.json";
        let result = write_json(path, test);
        assert!(result.is_ok());
    }

    #[test]
    fn test_write_json_2() {
        let test = json!({
            "obj": {
                "a": 42,
                "b": "hello"
            },
            "arr": [1, 2, 3],
            "int": 42,
            "float": 42.069,
            "str": "hello",
            "bool": true,
        });
        let path = "./test/test2.json";
        let result = write_json(path, test);
        assert!(result.is_ok());
    }

    #[test]
    fn test_read_json() {
        let test = read_json::<_, TestStruct>("./test/test.json").unwrap();
        assert_eq!(test.a, 42);
        assert_eq!(test.b, "hello".to_string());
    }

    #[test]
    fn skt() {
        let mut collector = Vec::new();
        let test = TestStruct::new(42, "hello");
        let test2 = TestStruct::new(69, "world");
        let test3 = TestStruct::new(420, "world");

        let option1 = OptionSerde::new(Some(test), Some(42));
        let option2 = OptionSerde::new(None, Some(69));
        let option3 = OptionSerde::new(Some(test3), None);
        let option4 = OptionSerde::new(Some(test2), Some(999));
        collector.push(option1);
        collector.push(option2);
        collector.push(option3);
        collector.push(option4);

        write_json("./test/option.json", collector).unwrap();
    }
}
