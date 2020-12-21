trait GenericTrait {
    type T;
    type U;
    fn has(&self, value: Self::T) -> bool;
    fn count(&self) -> Self::U;
 }
 
 struct Company {
    id: u8,
    name: String 
 }
 
 struct Record {
    array: Vec<Company>
 }
 
 impl GenericTrait for Record {
    type T = u8;
    type U = u16;
    fn has(&self, value: Self::T) -> bool {
       for record in self.array.iter() {
          if record.id == value.into() {
             return true
          }
       }
       return false
    }
    fn count(&self) -> Self::U {
       self.array.len() as u16
    }
 }
 
 fn find_record<H: GenericTrait>(array: &H, value: <H as GenericTrait>::T) -> bool {
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
    record.has(1);
    find_record(&record, 4);
 }