use rand::{distributions::Alphanumeric, Rng};

pub fn generate_random_string(length: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();
    let random_string: String = (0..length)
        .map(|_| {
            let index = rng.gen_range(0..CHARSET.len());
            CHARSET[index] as char
        })
        .collect();
    random_string
}

pub fn generate_secure_random_string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let random_string: String = rng
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();
    random_string
}

pub fn generate_secure_random_string_lowercase(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let random_string: String = rng
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .map(|c| c.to_ascii_lowercase())
        .collect();
    random_string
}


#[test]
fn test_generate_random_string() {
    let random_string = generate_random_string(10);
    assert_eq!(random_string.len(), 10);
}