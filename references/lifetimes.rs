fn main() {
    {
        let uninitialized;
        let initiated = 5;
        
        {
            uninitialized = initiated;
        }
        println!("{}", uninitialized);
    }
}