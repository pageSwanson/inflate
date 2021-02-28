use argh::FromArgs;
use std::path::PathBuf;
use std::fs::File;

#[derive(FromArgs)]
/// Create lengthy files.
struct Inflate {
    /// data size in bytes (required)
    #[argh(option, short = 's')]
    size: u64,

    //// unit for size of data
    //#[argh(option, short = 'u')]
    //units: char,

    /// new file path (creates 'balloon.txt' in working dir by default)
    #[argh(option, short = 'p', default = "PathBuf::from(r\"balloon.txt\")")]
    path: PathBuf,
}

fn main() {
    let inflate: Inflate = argh::from_env();

    println!("inflate will create a file with data size {} in bytes!", inflate.size);
    println!("File will be created as {:#?}!", &inflate.path);

    let file = File::create(inflate.path).expect("could not create file");
    file.set_len(inflate.size)
        .expect("could not write to file");
}
