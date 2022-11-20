use random_number::random;
pub trait IsEmpty {
    fn is_empty(&self) -> bool;
}

impl IsEmpty for Option<String> {
    fn is_empty(&self) -> bool {
        return match self {
            Some(s) => s.is_empty(),
            _ => true,
        };
    }
}

pub fn random_code() -> String {
    let mut code = String::new();
    for i in 0..4{
        let mut rng = random_number::rand::thread_rng();
        let n: u8 = random!(..=9, rng);
        code = format!("{}{}", code,n)
    }
    code
}