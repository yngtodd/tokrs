use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

use pyo3::prelude::*;
//use pyo3::wrap_pyfunction;

/// Vocabulary for NLP applications
///
/// This is a mapping from tokenized 
/// vocabulary terms to integer tokens.
#[pyclass]
pub struct Vocab {
    /// Mapping from tokens to integers
    map: HashMap<String, i32>,
}

//#[pymethods]
impl Vocab {
    /// Create a Vocabulary
    /// 
    /// # Arguments 
    /// 
    /// * `path` - Path to a raw text file to be parsed
    pub fn new(fpath: &str) -> Result<Vocab, std::io::Error> {
        let mut map = HashMap::new();
        let contents = Vocab::read_file(fpath);
        let tokens = Vocab::tokenize(contents);

        let mut tok = 0;
        for term in &tokens {
            if !map.contains_key(term) {
                map.insert(term.to_owned(), tok);
                tok += 1;
            }
        }

        Ok(Vocab {map})
    }

    /// Read in a file
    pub fn read_file(fpath: &str) -> String {
        let mut file = File::open(fpath).expect("Cannot open file!");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Cannot read file!");

        contents
    } 

    /// Tokenize raw text
    ///
    /// Strip whitespace, lowercase terms, and remove punctuation. 
    /// We then return a vector of token Strings.
    ///
    /// # Arguments
    /// 
    /// * `text` - raw text String from which a vocabulary is built
    pub fn tokenize(text: String) -> Vec<String> {
        let tokens: Vec<String> = text.split(|c: char| !(c.is_alphanumeric() || c == '\''))
                                      .filter(|s| !s.is_empty())
                                      .map(|s| s.to_string())
                                      .map(|s| s.to_lowercase())
                                      .collect();

        tokens
    }

    /// Load a previously built vocabulary from disk
    ///
    /// # Arguments
    /// 
    /// * `path` - Path to a saved vocabulary
    pub fn load(fpath: &str) -> Result<Vocab, std::io::Error> {
        let mut map = HashMap::new(); 
        let contents = Vocab::read_file(fpath);

        for line in contents.lines() {
            let mut chunks = line.splitn(2, '\t');
            let voc = chunks.next().expect("No vocab term!");
            let tok = chunks.next().expect("No token!").parse().unwrap();
            map.insert(voc.to_owned(), tok);
        }

        Ok(Vocab {map})
    } 

    /// Write the vocabulary to disk
    /// 
    /// Saved as a `.tsv` file, where each line is in the following format:
    ///
    /// ```
    /// term    token 
    /// term    token
    /// ...
    /// ```
    ///
    /// # Arguments 
    /// 
    /// * `path` - path to save the vocabulary tsv file 
    pub fn write(&self, fpath: &str) -> std::io::Result<()> {
        let mut contents = String::new();
        for (voc, tok) in &self.map {
            contents.push_str(voc);
            contents.push('\t');
            contents.push_str(&tok.to_string());
            contents.push('\n');
        }

        std::fs::write(fpath, contents)
    }

    /// Get the number of vocabulary terms
    pub fn size(&self) -> usize {
        self.map.len()
    }

    /// Print the contents of the database
    pub fn show(&self) {
        for (voc, tok) in &self.map {
            println!("  KEY: {}, VALUE: {}", voc, tok);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
