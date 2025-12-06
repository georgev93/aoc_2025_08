use std::fs::{File, read_to_string};
use std::io::{BufRead, BufReader, Read};

pub struct FileParser {
    file: File,
    contents: String,
}

impl FileParser {
    pub fn new(path: &str) -> Self {
        let file = File::open(path).unwrap_or_else(|_| {
            panic!("Could not find file \"{path}\"");
        });

        let mut ret_val = Self {
            file,
            contents: "".to_string(),
        };

        let mut reader = BufReader::new(&ret_val.file);
        reader.read_to_string(&mut ret_val.contents).unwrap();

        ret_val
    }
    //
    // fn parse_lines(&self) -> Vec<String> {
    //     let mut lines: Vec<String> = Vec::new();
    //     for line in BufReader::new(&self.file).lines() {
    //         lines.push(line.unwrap().clone());
    //     }
    //     lines
    // }
    //
    // fn parse_delimeted(&self) -> Vec<String> {
    //     let mut items: Vec<String> = Vec::new();
    //     for item in BufReader::new(&self.file).split(b',') {
    //         items.push(String::from_utf8(item.unwrap().trim_ascii().to_vec()).unwrap());
    //     }
    //     items
    // }
    //
    // fn parse_grid(&self) -> Vec<Vec<char>> {
    //     let mut ret_vec: Vec<Vec<char>> = Vec::new();
    //     for line in BufReader::new(&self.file).lines() {
    //         let unwrapped_line = line.unwrap();
    //         let line_arr = unwrapped_line.as_bytes();
    //         let char_vec = line_arr.iter().map(|b| *b as char).collect::<Vec<char>>();
    //
    //         ret_vec.push(char_vec);
    //     }
    //     ret_vec
    // }

    pub fn get_str(&self) -> &str {
        self.contents.as_str()
    }

    // fn parse_grid_strings(&self) -> Vec<Vec<String>> {
    //     let mut ret_vec: Vec<Vec<String>> = Vec::new();
    //     for line in BufReader::new(&self.file).lines() {
    //         let unwrapped_line = line.unwrap();
    //
    //         let line_arr = unwrapped_line
    //             .trim_ascii()
    //             .split_whitespace()
    //             .map(|x| x.to_string())
    //             .collect();
    //
    //         ret_vec.push(line_arr);
    //     }
    //     ret_vec
    // }
}
