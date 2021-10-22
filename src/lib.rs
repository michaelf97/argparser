use std::collections::HashMap;
use std::env;

#[derive(Debug)]
pub struct ArgParse {
    description: String,
    arguments: HashMap<String, Option<String>>,
}

impl ArgParse {
    pub fn new(desc: Option<String>) -> ArgParse {
        ArgParse {
            description: desc.unwrap_or(String::from("Default Description")),
            arguments: HashMap::new(),
        }
    }

    pub fn add_argument(&mut self, name: &str) {
        self.arguments.insert(String::from(name), None);
    }

    pub fn parse(&mut self) {
        let args: Vec<String> = env::args().collect();
        
        args.iter().enumerate().for_each(|(i, arg)| {
            if let Some(x) = self.arguments.get_mut(arg) {
                *x = args.get(i + 1);
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::ArgParse;

    #[test]
    fn description() {
        assert_eq!(
            {
                let argparse = ArgParse::new(Some(String::from("My Description")));
                argparse.description
            },
            "My Description"
        )
    }

    #[test]
    fn description_default() {
        assert_eq!(
            {
                let argparse = ArgParse::new(None);
                argparse.description
            },
            "Default Description"
        )
    }

    #[test]
    fn main() {
        let argparse = ArgParse::new(None);
        argparse.parse();
    }
}
