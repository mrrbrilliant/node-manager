use std::fs::{File, OpenOptions};
use std::str::FromStr;

use reqwest::{
    header::{CONTENT_LENGTH, RANGE, HeaderValue},
    StatusCode,
};

pub fn continue_file(source_file: &str) -> File {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(source_file)
        .unwrap()
}

const CHUNK_SIZE: u32 = 409599; // Download Chunk size

pub struct PartialRangeIter {
    pub start: u64,
    pub end: u64,
    pub buffer_size: u32,
}

impl PartialRangeIter {
    pub fn new(start: u64, end: u64, buffer_size: u32) -> reqwest::Result<Self> {
        if buffer_size == 0 {
            panic!("invalid buffer_size, give a value greater than zero.");
        }
        Ok(PartialRangeIter {
            start,
            end,
            buffer_size,
        })
    }
}

impl Iterator for PartialRangeIter {
    type Item = HeaderValue;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start > self.end {
            None
        } else {
            let prev_start = self.start;
            self.start += std::cmp::min(self.buffer_size as u64, self.end - self.start + 1);
            Some(
                HeaderValue::from_str(&format!("bytes={}-{}", prev_start, self.start - 1))
                    .expect("string provided by format!"),
            )
        }
    }
}

pub fn download_file(download_link: &str, mut output_file: File) -> Result<(), String> {
    let client = reqwest::blocking::Client::new();
    let response = match client.head(download_link).send() {
        Ok(response) => Ok(response),
        Err(err) => Err(err.to_string()),
    }?;
    let length = match response.headers().get(CONTENT_LENGTH) {
        Some(length) => Ok(length),
        None => Err(String::from("No Content Length")),
    }?;
    let length = u64::from_str(length.to_str().unwrap()).unwrap();
    let startingpoint = output_file.metadata().unwrap().len();
    let whole_range = match PartialRangeIter::new(startingpoint, length - 1, CHUNK_SIZE) {
        Ok(whole_range) => Ok(whole_range),
        Err(err) => Err(err.to_string()),
    }?;

    for range in whole_range {
        let mut response = match client.get(download_link).header(RANGE, &range).send() {
            Ok(response) => Ok(response),
            Err(err) => Err(err.to_string()),
        }?;

        let status = response.status();
        match status == StatusCode::OK || status == StatusCode::PARTIAL_CONTENT {
            true => Ok(()),
            false => Err("error_status_code".to_string()),
        }?;

        match std::io::copy(&mut response, &mut output_file) {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string()),
        }?
    }

    Ok(())
}