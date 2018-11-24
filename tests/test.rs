#[macro_use]
extern crate bitflags;

#[macro_use(flags)]
extern crate flags_macro;

mod ponydom {
    bitflags! {
        pub struct Flags: u32 {
            const Winged = 0b01;
            const Horned = 0b10;
        }
    }
}

#[test]
fn deeper_path() {
    let alicorn = flags![ponydom::Flags::{Winged | Horned}];
    assert_eq!(alicorn, ponydom::Flags::Winged | ponydom::Flags::Horned);
}
