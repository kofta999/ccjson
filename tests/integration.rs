use std::fs;

use ccjson::run;

#[test]
fn test_valid_json() {
    let dir = fs::read_dir("./tests/final").unwrap();
    for path in dir {
        let path = path.unwrap().path();
        if path.is_file()
            && path.extension().unwrap() == "json"
            && path
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .starts_with("pass")
        {
            let content = fs::read_to_string(&path).unwrap();
            assert!(run(&content).is_ok(), "Failed on file: {:?}", path);
        }
    }

    let file = fs::read_to_string("./tests/final/sample.json").unwrap();
    assert!(run(&file).is_ok());
}

#[test]
fn test_invalid_json() {
    let dir = fs::read_dir("./tests/final").unwrap();
    for path in dir {
        let path = path.unwrap().path();
        if path.is_file()
            && path.extension().unwrap() == "json"
            && path
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .starts_with("fail")
        {
            let content = fs::read_to_string(&path).unwrap();
            assert!(run(&content).is_err(), "Didn't fail on file: {:?}", path);
        }
    }
}
