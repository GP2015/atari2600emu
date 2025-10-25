pub struct Bus<T: Copy> {
    value: T,
}

impl<T: Copy> Bus<T> {
    pub fn new(initial_value: T) -> Self {
        Self {
            value: initial_value,
        }
    }

    pub fn get(&self) -> T {
        self.value
    }

    pub fn set(&mut self, new_value: T) {
        self.value = new_value;
    }
}
