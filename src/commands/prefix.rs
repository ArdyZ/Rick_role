pub struct Prefix {
    pub a: String,
}

impl Prefix {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Default for Prefix {
    fn default() -> Self {
        Prefix { a: "!".to_string() }
    }
}
