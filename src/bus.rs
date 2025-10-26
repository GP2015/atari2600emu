pub struct Bus<T: Copy> {
    value: T,
}

impl<T> Bus<T>
where
    T: Copy,
{
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u8_bus() {
        let mut bus = Bus::<u8>::new(6);
        assert_eq!(bus.get(), 6 as u8);
        bus.set(7);
        assert_eq!(bus.get(), 7 as u8);
    }

    #[test]
    fn test_bool_bus() {
        let mut bus = Bus::new(true);
        assert_eq!(bus.get(), true);
        bus.set(false);
        assert_eq!(bus.get(), false);
    }
}
