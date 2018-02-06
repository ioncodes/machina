#[derive(Hash)]
pub struct OptimizedCache {
    pub name: String,
    pub value: usize,
}

impl Eq for OptimizedCache {}

impl PartialEq for OptimizedCache {
    fn eq(&self, other: &OptimizedCache) -> bool {
        self.name == other.name && self.value == other.value
    }
}