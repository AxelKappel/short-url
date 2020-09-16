use url::Url;

pub fn is_valid(input: &str) -> bool {
    match Url::parse(input) {
        Ok(_) => return true,
        Err(_) => return false,
    };
}
