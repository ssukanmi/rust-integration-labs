use file_sim::file::{self, File, Read};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut my_file = File::new_with_data("my_file.txt", &[114, 117, 115, 116, 33]);

    let mut buffer = vec![];

    my_file = file::open(my_file)?;

    let my_file_len = my_file.read(&mut buffer)?;

    my_file = file::close(my_file)?;

    let txt = String::from_utf8_lossy(&buffer);

    println!("{}", my_file);
    println!("{:?}", my_file);
    println!("{} is {} bytes long", &my_file.name, my_file_len);
    println!("{}", txt);

    Ok(())
}
