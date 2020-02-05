use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use smxdasm::headers::SMXHeader;

use structopt::StructOpt;


#[derive(StructOpt)]
struct Cli {
    #[structopt(name = "file", parse(from_os_str))]
    /// Path to the SMX file
    pub file: PathBuf,
}

fn main() -> Result<(), std::io::Error> {
    let cli = Cli::from_args();

    let mut file = File::open(cli.file)?;

    let mut data = Vec::new();

    file.read_to_end(&mut data)?;

    let o = SMXHeader::new(data);

    let header = match o {
        Err(e) => {
            println!("{}", e);

            return Ok(());
        }
        Ok(ce) => ce,
    };

    for byte in header.data {
        print!("{:X?}", byte);
    }

    Ok(())
}
