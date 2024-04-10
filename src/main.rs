use object::{File, Object, ObjectSection};
use std::env::args;
use std::error::Error;
use std::fs;

fn get_string_manganis(file: &File) -> Option<String> {
    for section in file.sections() {
        match section.name() {
            Ok(n) if n == "manganis" => {
                let bytes = section.uncompressed_data().ok()?;
                return Some(std::str::from_utf8(&bytes).ok()?.into());
            }
            _ => {}
        }
    }
    None
}

fn main() -> Result<(), Box<dyn Error>> {
    let path: &str = &args()
        .into_iter()
        .nth(1)
        .expect("provide path of rust binary");

    println!("{}", path);

    let binary_data = fs::read(path)?;
    let file = object::File::parse(&*binary_data)?;

    if let Some(content) = get_string_manganis(&file) {
        println!("I have found this data for manganis:");
        println!("{}", content);
    }

    Ok(())
}
