use std::env;
use std::path::PathBuf;

use pdf::error::PdfError;
use pdf::file::File;


fn main() -> Result<(),PdfError> {

    let mut file = File::open("a.pdf").unwrap();

    

    Ok(())
}
