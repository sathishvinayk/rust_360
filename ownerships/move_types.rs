fn move1(x: String) -> String {
    println!("Hi the value has moved here from move1: {}", x);
    String::from(x)
}

fn move2(x: String) -> String {
    println!("Hi the value has moved here from move2: {}", x);
    String::from(x)
}

fn while_fn(data: Vec<u8>) {
    println!("Vector value is {:?}", data);
}

fn move_value_example() {
    let mut addon = vec![1,2,3,4,5,6,7];
    
    let pop_method = addon.pop();

    let swap_method = addon.swap_remove(2);

    let mem_replace_method = std::mem::replace(&mut addon[0], 9);

    assert_eq!(Some(7), pop_method);

    println!("Pop method showing option {:?}", pop_method);
    println!("Swap method showing value {:?}", swap_method);
    println!("Mem replace method showing value {:?}", mem_replace_method);
    println!("Vector value: {:?}", addon);
}

fn main() {
    let name = String::from("bob");

    if true {
        let value = move1(name);

        println!("Value: {}", value);
    } else {
        let value = move2(name);

        println!("Value: {}", value);
    }

    let mut data = vec![1,2,3,4,5,6,7];
    while data.contains(&5) {
        while_fn(data);

        data = Vec::new();
    }

    // println!("{}", name);
    println!("Data from main {:?}", data);

    move_value_example();
}