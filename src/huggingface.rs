use std::fs::File;
use std::io::Read;
use std::path::Path;

use reqwest::blocking::Client;

pub fn download_huggingface_model(url: &str) -> Vec<u8> {
    let client = Client::new();
    let mut response = client.get(&url).send().unwrap();
    let mut buffer = Vec::new();
    response.read_to_end(&mut buffer).unwrap();
    buffer
}
