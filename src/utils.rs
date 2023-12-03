use std::fs::File;
use std::io;
use std::path::Path;

// referenced from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
pub fn read_lines<P>(filename: P) -> io::Result<io::BufReader<File>>
where
    P: AsRef<Path>,
{
    let file: File = File::open(filename)?;
    Ok(io::BufReader::new(file))
}

#[allow(dead_code)]
pub fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for item in row.iter() {
            print!("{}", item);
        }
        println!();
    }
}
