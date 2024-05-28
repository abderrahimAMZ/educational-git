use std::fmt::format;
use std::fs;
use sha1::{Digest, Sha1};
use std::fs::*;
use std::io::Write;

pub struct Blob<'T> {
    file_path: &'T String,
    folder : String,
    name : String,
    file_length : usize,
}


impl<'a> Blob<'a> {
    pub fn new(file_path : &'a String) -> Blob {
        let (folder, name, file_length ) = Self::blob_from_file(file_path);
        Blob {
            folder,
            name,
            file_length,
            file_path
        }
    }

    fn blob_from_file(filename : &String)-> (String, String, usize) {
        let file = fs::read_to_string(filename).expect("Something went wrong reading the file");
        let length = file.len();
        let file_to_hash = format!("blob {}\0{}", length, file);
        let hash = Self::hash(file_to_hash);
        let folder = String::from(&hash[..2]);
        let name = String::from(&hash[2..]);
        (folder, name, length)
    }
    fn hash(content : String) -> String {
        let mut hasher = Sha1::new();
        hasher.update(content.as_bytes());
        let result = hasher.finalize();
        let hash = result.iter().map(|byte| format!("{:02x}", byte)).collect::<String>();
        hash
    }
    pub fn print_hash(&self) {
        println!("{}{}", self.folder, self.name);
    }

    pub fn write_to_objects(&self) {
        let content = fs::read_to_string(self.file_path).expect("Something went wrong reading the file");
        let content_to_compress = format!("blob {}\0{}", self.file_length, content);
        let dir_path = format!(".git/objects/{}",self.folder);
        fs::create_dir(dir_path).unwrap();
        let file_path = format!("./.git/objects/{}/{}", self.folder, self.name);
        // we need to create the file if it doens't exisit

        let compressed_content = self.compress(content_to_compress);
        fs::write(file_path, compressed_content).unwrap();
    }
    pub fn compress(&self, content : String) -> Vec<u8> {
        let mut encoder = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
        encoder.write_all(content.as_bytes()).unwrap();
        let compressed_bytes = encoder.finish().unwrap();
        compressed_bytes
    }
    pub fn hash_object(filename : String, write_to_file: bool) {
        let blob = Blob::new(&filename);
        blob.print_hash();
        if write_to_file {
            blob.write_to_objects();
        }
    }

}
