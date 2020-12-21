fn main() {
    let u8_maximum_value = u8::MAX;
    let u16_maximum_value = u16::MAX;
    let u32_maximum_value = u32::MAX;
    let u64_maximum_value = u64::MAX;
    let u128_maximum_value = u128::MAX;
    let size_arch = usize::MAX;

    let u64_min = u64::MIN;

    let i8_signed_integer_max = i8::MAX;

    // let wrong_i8: i8 = 127;  Error: Exceeds the i8 range

    let floating_type_32 = f32::MAX;
    let floating_type_64 = f64::MAX;

    println!("Printing max u8 {}", u8_maximum_value);
    println!("Printing max u16 {}", u16_maximum_value);
    println!("Printing max u32 {}", u32_maximum_value);
    println!("Printing max u64 {}", u64_maximum_value);
    println!("Printing max u128 {}", u128_maximum_value);
    println!("Printing max usize {}", size_arch);
    println!("Printing min u64 {}", u64_min);

    println!("Printing i8_signed_maximum_value: {}", i8_signed_integer_max);
    // println!("Incorrect i8 signed: {}", wrong_i8);  //Uncomment this and wrong_i8 variable to see the error

    println!("Float value: {}, {}", floating_type_32, floating_type_64);
}