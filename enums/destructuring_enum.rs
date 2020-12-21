#[allow(dead_code)]
enum Display {
    Grayscale(u32, u32, u32),
    Picture(&'static str),
    Monochrome(u32),
    Resolution(Option<(u64, u64)>)
}

#[derive(Debug)]
struct Application {
    id: u8,
    port: u16,
    name: String
}

fn get_display(display: Display) -> String {
    match display {
        Display::Grayscale(red, green, blue) => format!("Values are: {} {} {}", red, green, blue),
        Display::Picture(value) => format!("{}", value),
        Display::Monochrome(pixel) => format!("{}", pixel),
        Display::Resolution(value) => match value {
            Some((x, y)) => format!("Resolution => x and y is {}, {}",x , y),
            None => format!("no value present")
        }
    }
}

fn get_application_1(app:Application) -> String {
    match app {
        Application {
            id, port, name
        } => format!("Id -> {} of application {} is running on port {}", id, name, port)
    }
}

fn get_application_2(app:Application) -> String {
    match app {
        Application {
            id, ..
        } => format!("Id of application is {}", id)
    }
}


fn main() {
    let value = Display::Grayscale(20, 154, 10);
    let generated_value = get_display(value);

    let app_1 = Application {
        id: 101,
        port: 5999,
        name: "api_interface".to_owned()
    };
    let generated_app_1 = get_application_1(app_1);

    let app_2 = Application {
        id: 101,
        port: 5999,
        name: "api_interface".to_owned()
    };
    let generated_app_2 = get_application_2(app_2);

    let optional_display = Display::Resolution(Some((800, 600)));

    let generated_optional_value = get_display(optional_display);

    println!("{:?}", generated_value);
    println!("{:?}", generated_app_1);
    println!("{:?}", generated_app_2);
    println!("{:?}", generated_optional_value);
}