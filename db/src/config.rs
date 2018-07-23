use std::path::Path;

pub struct Config<'config> {
    pub path: &'config Path,
}

impl<'config> Config<'config> {
    pub fn default() -> Config<'config> {
        Config {
            path: &Path::new("/Users/mercury/tmp/pandora.azad"),
        }
    }
}

#[test]
fn it_works() {
    let the_path = Config::default().path;
    println!("path: {:?}", the_path);
}
