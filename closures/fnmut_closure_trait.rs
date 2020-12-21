fn process_string<Fx>(mut closure: Fx) where Fx: FnMut() {
    closure();   
}

fn method1() {
    let mut text_with_underscore = "Th__i_s __is__ a_ m_ut_a___ble cl___osure".to_string();
    println!("With underscore: {}", text_with_underscore);
    let retain_the_string = || {
        text_with_underscore.retain(|x| x != '_');
    };
    // retain_the_string();
    process_string(retain_the_string);
    println!("Without underscore: {}", text_with_underscore);
}

fn method2() {
    let text_with_underscore = "Th__i_s __is__ a_ m_ut_a___ble cl___osure".to_string();
    let another_technique: String = text_with_underscore.chars().filter(|x| *x != '_').collect();

    println!("Another technique for removing underscore: {}", another_technique);
}

fn method3() {
    let mut another_string = String::from("A string created on heap");
    let closure = || {
        let a_ref = &mut another_string;
        a_ref.push('!');
    };

    process_string(closure);

    println!("After updating changes: {}", another_string);
}

fn return_closure<'a>(arr: &'a [u8; 4], mut sum: u8) -> impl FnMut() + 'a {
    move || {
        for element in arr {
            sum += *element
        }
        println!("Sum: {}", sum);
    }
}

fn main() {
    let array = [1,2,3,4];
    let sum = 0;
    method1();
    method2();
    method3();
    return_closure(&array, sum)();
}