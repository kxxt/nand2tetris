pub struct TranslationState {
    comparison_counter: u16,
    name: String,
    function: Option<String>,
    ret_counter: u16
}

impl TranslationState {
    pub fn new(name: &str) -> Self {
        TranslationState {
            comparison_counter: 0,
            ret_counter: 0,
            name: name.to_string(),
            function: None,
        }
    }

    pub fn advance_ret_counter(&mut self) -> u16 {
        self.ret_counter += 1;
        self.ret_counter
    }

    pub fn advance_comparison_counter(&mut self) -> u16 {
        self.comparison_counter += 1;
        self.comparison_counter
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn leave_func(&mut self) -> Option<String> {
        self.function.take()
    }

    pub fn enter_func(&mut self, function: &str) -> bool {
        if self.function == None {
            self.function = Some(function.to_string());
            return true;
        }
        false
    }

    pub fn func(&self) -> Option<&String> {
        self.function.as_ref()
    }
}
