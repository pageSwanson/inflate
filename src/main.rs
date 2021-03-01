use argh::FromArgs;
use std::path::PathBuf;
use std::fs::File;

#[derive(FromArgs)]
/// Inflate files.
struct Inflate {
    /// data size in bytes (required)
    #[argh(option, short = 's')]
    size: u64,

    /// unit for size of data.
    /// options are 'B' byte, 'k' kilobyte, 'M' megabyte
    /// (defaults to 'B')
    #[argh(option, short = 'u', default = "'B'")]
    units: char,

    /// new file path (creates 'balloon.txt' in working dir by default)
    #[argh(option, short = 'p', default = "PathBuf::from(r\"balloon.txt\")")]
    path: PathBuf,
}

fn main() {
    let inflate: Inflate = argh::from_env();

    println!("inflate will create a file with data size {}{}!", inflate.size, inflate.units);
    println!("File will be created at {:#?}!", &inflate.path);

    let size_bytes: u64 = match inflate.units {
        'B' => { inflate.size },
        'k' => { inflate.size * 1000 },
        'M' => { inflate.size * 1000 * 1000 },
         _  => { panic!("Unsupported unit {:#?}.\nUse '--help' for supported units list.", inflate.units) }
    };

    let file = File::create(inflate.path).unwrap();
    file.set_len(size_bytes)
        .expect("could not write to file");
}
