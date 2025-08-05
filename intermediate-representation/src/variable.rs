#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Variable {
    pub name: String,
    pub fixed: bool,
}
