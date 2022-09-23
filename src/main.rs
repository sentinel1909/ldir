use std::{fs, io};

use console::style;

fn main() -> io::Result<()> {
    let mut entries = fs::read_dir(".")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    
    entries.sort();

    for entry in entries {
        if entry.is_dir() {
            println!{"{}", entry.display()};
            println!("{}", style("directory").green());
        } else {
            println!("{}", entry.display());
            println!("{}", style("file").blue());
        }
    }
    
    Ok(())
}
