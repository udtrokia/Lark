extern crate db;

use db::azad::Azad;

fn main() {
    let ts = Azad::init();
    let res = ts.set("hello", &[2]);
    assert_eq!(res.is_ok(), true);
}
