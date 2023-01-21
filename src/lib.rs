use std::{error::Error, fs};

pub struct Config {
    query: String,
    file_name: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("It has not an adequate length");
        }

        let query = args.get(1).expect("query is missing.").clone();
        let contents = args.get(2).expect("contents is missing").clone();

        let config = Config {
            query,
            file_name: contents,
        };

        Ok(config)
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut items = Vec::new();

    for line in contents.lines(){
        if line.contains(query){
            items.push(line);
        }
    }

    items
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_name)?;

    let result = search(&config.query, &contents);

    print!("{:?}",result);

    Ok(())
}
#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    pub fn test() {
        let query = String::from("nobody");
        let file_name = String::from("poem.txt");

        let config = Config { query, file_name };

        let _contents = fs::read_to_string(&config.file_name).expect("no contents");
    }
}
