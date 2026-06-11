pub async fn hostname() -> String {
    match hostname::get() {
        Ok(name) => name.into_string().unwrap_or_else(|_| "Unknown".to_string()),
        Err(_) => "Unknown".to_string(),
    }
}