#[derive(Debug, Clone, Default)]
pub struct Module {
    pub items: Vec<Item>,
}

#[derive(Debug, Clone)]
pub enum Item {
    // TODO: functions, lets, structs, etc.
}

impl Module {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}
