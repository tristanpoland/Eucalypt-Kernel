use framebuffer::println;
use fat12::{fat12_file_exists, fat12_read_file};

fn parse_elf(filename: &str) {
    if !fat12_file_exists(filename) {
        println!("File does not exist: {}", filename);
        return;
    }

    match fat12_read_file(filename) {
        Ok(contents) => {
            if contents.len() >= 4 && &contents[0..4] == b"\x7fELF" {
                println!("{} is a valid ELF file!", filename);
            } else {
                println!("{} is not an ELF file.", filename);
            }
        }
        Err(e) => {
            println!("Failed to read file {}: {}", filename, e);
        }
    }

    
}
