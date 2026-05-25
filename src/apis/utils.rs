/// Helper function to build query string from optional parameters
pub fn build_query_string(params: &[(&str, Option<String>)]) -> String {
    let query_parts: Vec<String> = params
        .iter()
        .filter_map(|(key, value)| {
            value.as_ref().map(|v| format!("{}={}", key, urlencoding::encode(v)))
        })
        .collect();
    query_parts.join("&")
}

/// Helper function to build query string with numeric parameters
pub fn build_query_string_with_numbers(params: &[(&str, Option<u32>)]) -> String {
    let query_parts: Vec<String> = params
        .iter()
        .filter_map(|(key, value)| {
            value.map(|v| format!("{}={}", key, v))
        })
        .collect();
    query_parts.join("&")
}

