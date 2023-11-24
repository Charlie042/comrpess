use std::io::{Write, Read, self};
use std::fs::File;
use std::error::Error;
use std::path::{Path,PathBuf};
use zip::{ZipWriter, CompressionMethod,write::FileOptions};

fn main()-> Result<(), Box<dyn Error>> {

   let zip_file_path = Path::new("little.zip");

   let zip_file = File::create(&zip_file_path)?;

   let mut zip = ZipWriter::new(&zip_file);

   // Define the file you want to compress

   let zip_to_comp :Vec<PathBuf> = vec![PathBuf::from("little.pdf")];

   // set compression option
let options = FileOptions::default().compression_method(CompressionMethod::DEFLATE);

//iterate through the files 

for file_path in &zip_to_comp {

let file = File::open(file_path)?;

let file_name = file_path.file_name().unwrap().to_str().unwrap();

// adding file to the Zip archive.
zip.start_file(file_name, options)?;

let mut buffer= Vec::new();
io::copy(&mut file.take(u64::MAX), &mut buffer)?;

zip.write_all(&buffer)?;





}
zip.finish()?;


println!("file has been compressed to {:?}", zip_file_path);


   Ok(())
    
}