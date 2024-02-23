use crate::*;

pub struct SubThing {
    pub value: u32,
}

impl Default for SubThing {
    fn default() -> Self {
        SubThing {
            value: 0,
        }
    }
}
