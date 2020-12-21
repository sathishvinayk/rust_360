use std::fs::read_to_string;
use std::env;

type A = Vec<String>;

mod module1 {
    pub(self) fn read_contents(filename: &str) -> Result<String, std::io::Error> {
        let contents = super::read_to_string(filename)?;
    
        Ok(contents)
    }

    pub fn read_args() {
        let args: super::A = super::env::args().collect();
        let filename = &args[1];

        let value = self::read_contents(filename).unwrap();

        println!("{:?}", value);
    }
}

fn main(){
    module1::read_args();
    // module1::read_contents("lorem.txt");
}