use std::io::Write;


pub fn write_data(filename: &str, data: Vec<u8>) -> Result<(), std::io::Error> {

    let mut file = if std::path::Path::new(&filename).exists() {
        std::fs::OpenOptions::new().write(true).open(filename)?
    } else {
        std::fs::OpenOptions::new().write(true).create_new(true).open(filename)?
    };

    file.write_all(&data)?;

    Ok(())
}


pub fn write_bytes(filename: &str, data: &[u8]) -> Result<(), std::io::Error> {

    let mut file = if std::path::Path::new(&filename).exists() {
        std::fs::OpenOptions::new().write(true).open(filename)?
    } else {
        std::fs::OpenOptions::new().write(true).create_new(true).open(filename)?
    };

    file.write_all(data)?;

    Ok(())
}