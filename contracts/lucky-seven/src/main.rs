#![cfg_attr(not(any(feature = "library", test)), no_std)]
#![cfg_attr(not(test), no_main)]

#[cfg(any(feature = "library", test))]
extern crate alloc;

#[cfg(not(any(feature = "library", test)))]
ckb_std::entry!(program_entry);
#[cfg(not(any(feature = "library", test)))]
ckb_std::default_alloc!(16384, 1258306, 64);

use ckb_std::{debug, high_level::load_script};

const LUCKY_BYTE: u8 = 0x07;

const ERR_NO_ARGS: i8 = 5;
const ERR_NOT_LUCKY: i8 = 7;

pub fn program_entry() -> i8 {
    let script = match load_script() {
        Ok(s) => s,
        Err(_) => return 1,
    };
    let args = script.args().raw_data();

    debug!("lucky-seven: args len = {}", args.len());

    if args.is_empty() {
        return ERR_NO_ARGS;
    }

    if args.iter().any(|b| *b == LUCKY_BYTE) {
        debug!("lucky-seven: 7 found, unlocking");
        0
    } else {
        debug!("lucky-seven: no 7 in args, rejecting");
        ERR_NOT_LUCKY
    }
}
