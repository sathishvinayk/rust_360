use std::io::{BufWriter, Write};
use std::fs::OpenOptions;

use csv;

#[derive(Debug)]
struct Service;

impl Service {
    fn new(&self, filename: &str) {
        println!("Service name: {}", filename);
        let file = OpenOptions::new()
            .read(true).write(true).create(true).open(filename).expect("Unable to create file!");

        let buffer = BufWriter::new(&file);

        self.process(buffer).expect("Failed to do something!");
    }
    fn process<W: Write>(&self, data: W) -> csv::Result<()> {
        let mut value = csv::Writer::from_writer(data);

        value.write_record(&["Type", "Strength"])?;
        value.write_record(&["Engineering", "5"])?;
        value.write_record(&["Architecture", "6"])?;

        value.flush()?;

        Ok(())
    }
}

fn main() {
    let _service = Service.new("random.csv");
}
