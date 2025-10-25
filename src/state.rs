pub struct State {
    active: bool,
}

impl State {
    pub fn new() -> Self {
        Self { active: true }
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }
}
