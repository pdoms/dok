use rand::prelude::*;

use crate::error::DokError;

pub fn gen_id() -> Result<String, DokError> {
    let rng = rand::thread_rng();
    let mut id = String::new();
    for _ in 0..6 {
        match do_digit() {
            true => {
                let d = rand_digit(rng.clone());
                id.push_str(d.as_str())
            }
            false => {
                let d = rand_char(rng.clone())?;
                id.push_str(d.as_str())
            }
        }
    }
    Ok(id)
}

fn do_digit() -> bool {
    rand::random()
}

fn rand_digit(mut r: rand::rngs::ThreadRng) -> String {
    let x: u8 = r.gen_range(0..10);
    x.to_string()
}

fn rand_char(mut r: rand::rngs::ThreadRng) -> Result<String, DokError> {
    let mut cap: Vec<u8> = (65..91).collect();
    for i in 97..123 {
        cap.push(i)
    }

    let length = cap.len();
    let idx = r.gen_range(0..length);
    match std::str::from_utf8(&[cap[idx]]) {
        Ok(v) => Ok(v.to_string()),
        Err(e) => Err(DokError::ErrGenId),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rand_id() {
        for _ in 0..500 {
            gen_id().unwrap();
        }
    }
}
