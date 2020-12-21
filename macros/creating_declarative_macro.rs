macro_rules! match_string {
    ($data: expr) => ({
        match $data.as_ref() {
            "alabama" => println!("1"),
            "georgia" => println!("0"),
            _ => println!("Invalid option")
        }   
    })
}

macro_rules! create_vector {
    ($x:expr, for $variable:ident in $col:expr, if $condition: expr) => {
        {
            let mut vector = vec![];
            for $variable in $col {
                if $condition {
                    vector.push($x.clone());
                }
            }
            vector
        }
    } 
}

fn main() {
    let value = String::from("alabama");
    match_string!(value);
    
    let value: Vec<usize> = create_vector![x*x, for x in 0..10, if x % 2 == 0];
    println!("The list comprehension is {:?}", value);
}

