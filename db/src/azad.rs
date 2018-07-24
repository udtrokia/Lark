// db.rs
use bincode::{serialize, deserialize};
use config::Config;
use io::{read, write};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Azad<'storage> {
    key: &'storage str,
    value: &'storage [u8]
}

impl<'storage> Azad<'storage> {
    pub fn init() -> Self {
        Azad { key: "hello", value: &[1]}
    }

    pub fn get(&self, k: &str, callback: fn(azad: &mut Azad)) {
        let path = Config::default().path;
        assert_eq!(read(path).is_ok(), true);
        println!("k: {:?}", k);
        let res = read(path).unwrap();
        let mut deser: Azad = deserialize(&res).unwrap();
        callback(&mut deser);
    }
    
    pub fn set(&self, k: &'storage str, v: &'storage [u8]) -> Result<(), &'storage str> {
        // Get default path.
        let path = Config::default().path;
        let data = Azad{ key: k, value: v};
        // Test
        assert_eq!(serialize(&data).is_ok(), true);
        write(path.clone(), &serialize(&data).unwrap())
    }
    
}

#[test]
fn it_works() {
    let item = Azad::init();
    let res = item.set("hello", &[1 as u8]);
    assert_eq!(res.is_ok(), true);
}
