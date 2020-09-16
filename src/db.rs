use redis::Commands;

pub fn add(key: &str, value: &str) -> redis::RedisResult<bool> {
    let client = redis::Client::open("redis://192.168.0.102:6379/")?;
    let mut con = client.get_connection()?;

    if con.exists(key).unwrap() {
        return Ok(false);
    }

    con.set_nx(key, value)?;
    Ok(true)
}

pub fn get(key: &str) -> redis::RedisResult<String> {
    let client = redis::Client::open("redis://192.168.0.102:6379/")?;
    let mut con = client.get_connection()?;
    let url: String = con.get(key)?;
    Ok(url)
}