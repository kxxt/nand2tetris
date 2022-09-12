pub struct TranslationState {
    comparison_counter: u16,
    name: String,
}

impl TranslationState {
    pub fn new(name: &str) -> Self {
        TranslationState {
            comparison_counter: 0,
            name: name.to_string(),
        }
    }

    pub fn advance_comparison_counter(&mut self) -> u16 {
        self.comparison_counter += 1;
        self.comparison_counter
    }
}
