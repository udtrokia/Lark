use std::io::Write;
use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;
use std::result::Result;
use std::io::Error;

fn get_file(path: &Path) -> Result<File, &str> {
    let open_res = OpenOptions::new().read(true).write(true).open(path);
    let file: File;

    // `Result::is_ok()` for nest create result;
    if open_res.is_ok() {
        file = open_res.unwrap();
    } else {
        file = File::create(path).unwrap();
    }

    Ok(file)
}

pub fn write<'storage>(path: &Path, data: &[u8]) -> Result<(), &'storage str> {
    let file = get_file(path);
    assert_eq!(file.is_ok(), true);

    let mut writer = file.unwrap();
    let mut res: Result<(), Error>;
    res = writer.write_all(&data);
    assert_eq!(res.is_ok(), true);
    res = writer.flush();
    assert_eq!(res.is_ok(), true);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::config;
    use super::*;
    #[test]
    fn it_works() {
        let path = config::Config::default().path;
        let res = write(path, &[1 as u8]);
        assert_eq!(res.is_ok(), true);
    }
}
