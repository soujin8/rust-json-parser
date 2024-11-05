use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let entries = fs::read_dir("json")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    // println!("{:?}", entries);

    for entry in entries {
        println!("{:?}", entry);

        let contents = fs::read_to_string(entry)?;

        println!("{:?}", contents);
    }

    Ok(())
}
