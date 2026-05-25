use monoova_rs_sdk::apis::utils;

#[test]
fn test_build_query_string() {
    let params = vec![
        ("param1", Some("value1".to_string())),
        ("param2", Some("value2".to_string())),
        ("param3", None),
    ];
    let query = utils::build_query_string(&params);
    assert!(query.contains("param1=value1"));
    assert!(query.contains("param2=value2"));
    assert!(!query.contains("param3"));
}

#[test]
fn test_build_query_string_with_numbers() {
    let params = vec![
        ("page", Some(1)),
        ("page_size", Some(50)),
        ("total", None),
    ];
    let query = utils::build_query_string_with_numbers(&params);
    assert!(query.contains("page=1"));
    assert!(query.contains("page_size=50"));
    assert!(!query.contains("total"));
}

#[test]
fn test_build_query_string_empty() {
    let params = vec![
        ("param1", None::<String>),
        ("param2", None::<String>),
    ];
    let query = utils::build_query_string(&params);
    assert_eq!(query, "");
}

#[test]
fn test_build_query_string_url_encoding() {
    let params = vec![
        ("param", Some("value with spaces".to_string())),
        ("special", Some("test&value=123".to_string())),
    ];
    let query = utils::build_query_string(&params);
    assert!(query.contains("param=value%20with%20spaces"));
    assert!(query.contains("special=test%26value%3D123"));
}

