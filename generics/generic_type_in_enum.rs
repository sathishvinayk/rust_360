enum Base<'a, T> {
    Data(&'a mut Vec<T>)
 }
 
 impl<'a, T> Base<'a, T> {
    fn process_vector<'f>(&'f mut self, vector: &'f mut Vec<T>) -> &'f mut Vec<T>{
       match self {
          Base::Data(val) => {
             let pop_value1 = val.pop().unwrap();
             let pop_value2 = vector.pop().unwrap();
 
             val.push(pop_value2);
             vector.push(pop_value1);
 
             vector
          }
       }
    }
    fn split_function<'h, X>(&'h mut self, vector: &'h mut Vec<X>) -> (&'h mut [X], &'h mut [X]) {
       let (left, right) = vector.split_at_mut(3);
       (left, right)
    }
 }
 
 fn main() {
    let mut vector1 = vec!["char", "char2"];
    let mut base = Base::Data(&mut vector1);
    {
       let mut vector2 = vec!["char3", "char4"];
       base.process_vector(&mut vector2);
       println!("Vector2 value is {:?}", vector2);
    }
    {
       let mut vector3 = vec!['f','a','s','s','h','b','z'];
       let value = base.split_function(&mut vector3);
 
       println!("Vector3 value is {:?}", value);
    }
 
    println!("Vector1 value is {:?}", vector1);
 }