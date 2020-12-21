struct Base<'h, T>(&'h mut Vec<T>);

impl<'h, T> Base<'h, T> {
   fn swapping_method<'f>(&mut self, vector: &'f mut Vec<T>) -> &'f mut Vec<T> {
      let pop_vector1 = self.0.pop().unwrap();
      let pop_vector2 = vector.pop().unwrap();

      self.0.push(pop_vector2);
      vector.push(pop_vector1);

      vector
   }
   fn split_vector<'j, X>(&mut self, vector: &'j mut Vec<X>) -> (&'j mut [X], &'j mut [X]) {
      let (left, right) = vector
      .split_at_mut(3);

      (left, right)
   }
}

fn main() {
   let mut vector1 = vec!["emp", "shockwave", "adv"];
   let mut vector_to_split = vec![2,2,4,6,84,2,1];
   {
      let mut base = Base(&mut vector1);
      {
         let mut vector2 = vec!["drone", "binocular", "shield"];
         base.swapping_method(&mut vector2);
         println!("Vector2 after changing: {:?}", vector2);
      }
      {
         let value = base.split_vector(&mut vector_to_split);
         println!("The value of vector: {:?}", value);
      }
   }
   println!("Vector1 {:?}", vector1);
}