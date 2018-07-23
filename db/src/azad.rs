// db.rs
use bincode::{serialize};
use config::Config;
use io::write;

#[derive(Serialize, Deserialize, Debug)]
pub struct Azad<'storage> {
    key: &'storage str,
    value: &'storage [u8]
}

impl<'storage> Azad<'storage> {     
    pub fn init() -> Self {
        Azad { key: "hello", value: &[1]}
    }
    
    pub fn set(self, k: &'storage str, v: &'storage [u8]) -> Result<(), &'storage str> {
        // Get default path.
        let path = Config::default().path;
        let data = Azad{ key: k, value: v};

        // test
        assert_eq!(serialize(&data).is_ok(), true);

        write(path, &serialize(&data).unwrap())
    }
}

#[test]
fn it_works() {
    let item = Azad::init();
    let res = item.set("hello", &[1 as u8]);
    assert_eq!(res.is_ok(), true);
}
