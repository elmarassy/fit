#[derive(Debug, Clone)]
pub struct Parameter {
    name: String,
    fixed: bool,
}

impl Parameter {
    pub fn new(name: String, fixed: bool) -> Self {
        Self { name, fixed }
    }
}
