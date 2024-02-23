use crate::*;

pub struct FooThing {
    pub value: u32,
}

impl Default for FooThing {
    fn default() -> Self {
        FooThing {
            value: 0,
        }
    }
}