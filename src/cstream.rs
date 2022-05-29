use std::fs;

pub struct CStream {
    contents: Option<String>,
    char_pos: usize,
}

impl CStream {
    pub fn new(file: &str) -> CStream {
        let contents = fs::read_to_string(file);
        //check if the file is empty or not then contruct the CStream 
        if let Ok(contents) = contents {
            CStream {
                contents: Some(contents),
                char_pos: 0,
            }
        } else {
            CStream {
                contents: None,
                char_pos: 0,
            }
        }
    }

    pub fn read(&mut self) -> Option<char> { //read char then move to next
        let res = self.contents.as_ref().unwrap().chars().nth(self.char_pos);
        self.char_pos += 1;
        return res
    }

    pub fn look_curr(&self) -> Option<char> { //just read dont move
        self.contents.as_ref().unwrap().chars().nth(self.char_pos)
    }
}
