use url::Url;

pub fn is_valid(input: &str) -> bool {
    Url::parse(input).is_ok()
}
