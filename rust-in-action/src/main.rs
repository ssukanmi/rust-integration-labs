use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let ptr = 42 as *const Vec<String>;
    unsafe {
        let new_addr = ptr.offset(4);
        println!("{:p} -> {:p}", ptr, new_addr);
    }
    Ok(())
}
