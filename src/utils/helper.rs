pub fn convert_string_to_bool(origin: String) -> bool {
    match origin.as_str() {
        "true" | "TRUE" | "1" => true,
        "false" | "FALSE" | "0" => false,
        _ => false,
    }
}