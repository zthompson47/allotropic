#![feature(slice_concat_trait)]

use std::ffi::{OsStr, OsString};
use std::slice::Join;
use std::rc::Rc;

fn main() {
    let path: [OsString; 4] = [
        "home".into(),
        "zach".into(),
        "allotropic".into(),
        "rust".into(),
    ];

    let joined = Join::join(&Rc::new(path), "/");
}
