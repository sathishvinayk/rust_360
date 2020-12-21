fn for_loop() {
    let value = vec![1,2,3,4,5];

    for x in value {
        println!("Values: {}", x);
    }
    for a in 0..10 {
        println!("Range values: {}", a);
    }

    for ax in 0..5 {
        if ax == 2 {
            println!("Value matches so exit: {}", ax);
            break;
        } else {
            println!("Value doesn't match: {}", ax)
        }
    }
}
fn loop_fn() {
    let intro: &str = "Hi there";
    'firstloop: loop {
        if intro.chars().nth(1) == Some('i') {
            println!("From Loop fn: {}", true);
            
            loop {
                println!("Hi access is granted!");
                break 'firstloop
            }
        }
    }
}
fn while_let() {
    let intro: &str = "Hi there";
    while let Some('i') | Some('t') | Some('e') = intro.chars().nth(1) {
        println!("From while let: {}", true);
        return
    }
}

fn while_block() {
    let intro: &str = "Hi there";
    while intro.chars().nth(1) == Some('i') {
        println!("From while: {}", true);
        return
    }
}
fn main(){
    // while_block();
    // while_let();
    // loop_fn();
    for_loop();
} 