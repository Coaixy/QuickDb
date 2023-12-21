pub fn app_path() -> String {
    let mut path = std::env::current_dir().unwrap();
    path.pop();
    path.to_str().unwrap().to_string()
}