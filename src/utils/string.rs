use random::Source;
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
    let mut source = random::default(9);

    for i in 0..=3 {
        code = format!("code{}",source.read::<u8>());
    }
    code
}