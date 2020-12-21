trait Animal {
    fn lifespan(&self) -> u8;
 }
 
 trait Species: Animal {
    fn class(&self) -> String;
 }
 
 struct Bird {
    name: String,
    age: u8
 }
 
 impl Animal for Bird {
    fn lifespan(&self) -> u8 {
       match self.name.as_ref() {
          "cockatoo" => 12,
          "macaw" => 50,
          "owl" => 25,
          _ => 0
       }
    }
 }
 
 impl Species for Bird {
    fn class(&self) -> String {
       format!("The {} is a {} class", self.name, "Aves".to_string())
    }
 }
 
 fn supertrait_call<R: Species>(value: R) {
    println!("Calling it from inside function: {}",value.class());
    println!("Calling it from inside function: {}",value.lifespan());
 }
 
 fn main() {
    let bird = Bird {
       name: "cockatoo".to_string(),
       age: 7
    };
    println!("Class: {}", bird.class());
    println!("Lifespan: {}", bird.lifespan());
 
    println!("\n");
    supertrait_call(bird);
 }