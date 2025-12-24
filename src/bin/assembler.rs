use std::env;
use std::fs::{self, File};
use std::io::{self, BufReader, BufWriter, Read, Write};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <sliced_folder> <output_file>", args[0]);
        std::process::exit(1);
    }

    let sliced_folder = &args[1];
    let output_file = &args[2];

    let mut entries: Vec<_> = fs::read_dir(sliced_folder)?
        .filter_map(|e| e.ok())
        .collect();

    entries.sort_by_key(|entry| {
        entry
            .path()
            .file_stem()
            .and_then(|s| s.to_str())
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(usize::MAX)
    });

    let output = File::create(output_file)?;
    let mut writer = BufWriter::new(output);

    for entry in entries {
        let path = entry.path();

        if path.extension().and_then(|e| e.to_str()) != Some("bin") {
            continue;
        }

        let file = File::open(&path)?;
        let mut reader = BufReader::new(file);

        let mut buffer = [0u8; 8192];
        loop {
            let bytes_read = reader.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }

            writer.write_all(&buffer[..bytes_read])?;
        }
    }

    writer.flush()?;
    println!("Successfully assembled into '{}'", output_file);

    Ok(())
}
