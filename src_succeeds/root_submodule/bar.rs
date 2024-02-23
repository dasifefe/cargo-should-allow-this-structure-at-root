use crate::*;

mod submodule; // However, in here, this import is allowed. Hence the inconsistency. In root is not allowed, in submodules/subfolders it is allowed.
pub use self::submodule::*;

pub struct BarThing {
    pub value: SubThing,
}

impl Default for BarThing {
    fn default() -> Self {
        BarThing {
            value: SubThing::default(),
        }
    }
}