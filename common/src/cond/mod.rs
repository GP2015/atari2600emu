pub mod base;
pub mod check;

use crate::cond::base::BaseCondition;

pub trait IsCondition {
    fn as_cond(&self) -> BaseCondition;
}
