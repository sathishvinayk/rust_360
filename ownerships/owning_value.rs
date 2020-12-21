fn example_variable1() {
    {
        let value: u8 = 8;
        println!("Value is {}", value);
    }
}

fn example_variable2() {
    { 
        let mut vector: Vec<u8> = vec![1,2,3,4];

        println!("Value of elements: {:?}", vector);

        {
            let value = 6;
            vector.push(value);
        }

        for _value in vec![1,2,3] {
            let data = 7;
            vector.push(data);
        }
        println!("Vector value: {:?}", vector);
    }
    // println!("Vector value: {:?}", vector);
}

fn example_variable3() {
    {
        let pointer = Box::new(12);
        let formatting_pointer = format!("Box value is {:?}", pointer);

        println!("{}", formatting_pointer);
    }

    println!("{}", formatting_pointer); //Error: not found in scope
}

fn main() {
    example_variable1();
    example_variable2();
    example_variable3();
}