use std::default::Default;

mod root_submodule; // We create and import this "dummy" root submodule, and import side-by-side with `lib.rs`, because that way it works.
pub use self::root_submodule::*;

pub struct LibThing {
    pub foo: FooThing,
    pub bar: BarThing,
}

impl Default for LibThing {
    fn default() -> Self {
        LibThing {
            foo: FooThing::default(),
            bar: BarThing::default(),
        }
    }
}
