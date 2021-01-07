use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::string::String;

pub fn read_file(path: &str) -> Result<Vec<String>, Error> {
    let file = File::open(path)?;

    let br = BufReader::new(file);
    let mut v = Vec::new();
    for line in br.lines() {
        let line = line?;
        let n = line.trim().to_owned();
        v.push(n);
    }
    Ok(v)
}

pub fn read_file_ints(path: &str) -> Result<Vec<i64>, Error> {
    let lines = read_file(path)?;
    let mut v = Vec::new();
    for line in lines {
        let n = line
            .parse()
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        v.push(n);
    }
    Ok(v)
}

pub fn read_file_delim_ints(path: &str, delimiter: &str) -> Result<Vec<usize>, Error> {
    let lines = read_file(path)?;
    let mut v = Vec::new();
    for line in lines {
        let chars: Vec<String> = line.split(delimiter).map(|s| s.to_string()).collect();
        for c in chars {
            let integer = c
                .parse()
                .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
            v.push(integer);
        }
    }
    Ok(v)
}

pub fn read_file_delim_strings(path: &str, delimiter: &str) -> Result<Vec<String>, Error> {
    let lines = read_file(path)?;
    let mut v = Vec::new();
    for line in lines {
        let entries: Vec<String> = line.split(delimiter).map(|s| s.to_string()).collect();
        for e in entries {
            v.push(e);
        }
    }
    Ok(v)
}
