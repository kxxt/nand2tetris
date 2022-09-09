pub struct TranslationState {
    comparison_counter: u16,
}

impl TranslationState {
    pub fn new() -> Self {
        TranslationState {
            comparison_counter: 0,
        }
    }

    pub fn advance_comparison_counter(&mut self) -> u16 {
        self.comparison_counter += 1;
        self.comparison_counter
    }
}
