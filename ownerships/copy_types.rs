// Struct component
#[derive(Debug, Copy, Clone)]
struct Component {
    id: u8
}

#[derive(Debug, Copy, Clone)]
enum Color {
    RED,
    BLUE
}

// Copy i8 function
fn copy_values_to_function(x: i8) {
    println!("Got the x value as: {}", x);
}

// Float copy function
fn float_example() {
    let x: f32 = 6.72;

    let copy_variable = x;

    println!("Value from inside float_example: {}, {}", x, copy_variable);
}

// Struct copy function
fn copy_struct(x: Component) {
    println!("Struct has a field with: {}", x.id)
}

// Struct calling function
fn struct_data(){
    let construct_struct = Component { 
        id: 12
    };
    copy_struct(construct_struct);

    println!("Printing after moved {:?}", construct_struct);
}

// Tuple, arrays, copy function
fn data_structure() {
    let x: (i8, u16, bool) = (6, 1256, true);

    let copy_x = x;

    let y: [u8; 4] = [0,1,2,3];

    let copy_array = y;

    println!("x and copy_x values from data structure is: {:?}, {:?}", x, copy_x);
    println!("y and copy_array: {:?}, {:?}", y, copy_array);
}

// Enum copy
fn enum_copy(x: Color) {
    println!("The color is {:?}", x);
}

// Enum example function
fn enum_example() {
    let color = Color::RED;

    enum_copy(color);

    println!("Enum is after moved {:?}", color);
}

fn heap_copy_example() {
    let factory: String = String::from("Papermint");

    let copy_factory = factory.clone();

    println!("Copied factory is a: {} from {}", copy_factory, factory);
}

fn main() {
    let value: i8 = 5;

    let copy_value: i8 = value;

    let another_copy: i8 = value;

    copy_values_to_function(value);
    float_example();
    data_structure();
    struct_data();
    enum_example();
    heap_copy_example();

    println!("Values from: {}, {}, {}", value, copy_value, another_copy);
}