trait GenericTrait<T, U> {
   fn has(&self, value: T) -> bool;
   fn count(&self) -> U;
}

struct Company {
   id: u8,
   name: String
}

struct Record {
   array: Vec<Company>
}

impl GenericTrait<u8, u16> for Record {
   fn has(&self, value: u8) -> bool {
       for record in self.array.iter() {
           if record.id == value {
               return true
           }
       }
       return false
   }
   fn count(&self) -> u16 {
       self.array.len() as u16
   }
}

fn find_record<H: GenericTrait<u8, u16>>(array: &H, value: u8) -> bool {
   array.has(value)
}

fn main() {
   let c1 = Company {
       id: 1,
       name: "Bob".to_string()
   };

   let c2 = Company {
       id: 2,
       name: "Shelly".to_string()
   };

   let record = Record {
       array: vec![c1, c2]
   };
   println!("{}", record.has(1));
   println!("{}", find_record(&record, 4));
   println!("Count is {}", record.count());
}