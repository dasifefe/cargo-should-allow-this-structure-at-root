use std::default::Default;

mod bar; // Cargo cannot find this module, because it expects it to be side-by-side with `lib.rs` or be a folder containing `mod.rs`.
pub use self::bar::*;

mod foo; // Cargo cannot find this module, because it expects it to be side-by-side with `lib.rs` or be a folder containing `mod.rs`.
pub use self::foo::*;

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
