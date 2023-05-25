use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub fn gen_str(size: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(size)
        .map(char::from)
        .collect()
}

pub fn str_to_vec(obj: String) -> Vec<String> {
    obj.to_string()
        .split(",")
        .map(String::from)
        .collect::<Vec<_>>()
}
