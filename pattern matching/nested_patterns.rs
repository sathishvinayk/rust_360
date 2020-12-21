#[allow(dead_code)]
#[derive(Debug)]
struct Profile { id: u8, user: String, profile_type: Namespace }

#[allow(dead_code)]
#[derive(Debug)]
enum Namespace {
    Consumer,
    Employee
}

#[allow(dead_code)]
#[derive(Debug)]
enum Barcode {
    Code(u8, u8, String),
    Name(Namespace)
}

#[allow(dead_code)]
#[derive(Debug)]
enum Components {
    Analytics,
    Process,
    Ps(Profile),
    Bar(Barcode)
}

fn nested(component: Components) {
    match component {
        Components::Analytics => println!("The component is analytics!"),
        Components::Process => println!("The component is process!"),
        Components::Ps(Profile {
            id, user, profile_type
        }) => println!("id and user is {}, {} and type is: {:?}", id , user, profile_type),
        Components::Bar(Barcode::Code(
            id,
            sid, 
            name
        )) => {
            println!("Id and user is {}, {} and type is {:?}", id, sid, name);
        }
        Components::Bar(Barcode::Name(Namespace::Consumer)) => {
            println!("Barcode is for consumer")
        }
        Components::Bar(Barcode::Name(Namespace::Employee)) => {
            println!("Barcode is for Employee")
        }
    }
}

fn main() {
    let ana_component = Components::Analytics;
    let base_component = Components::Ps(Profile {
        id: 12,
        user: "Base component".to_owned(),
        profile_type: Namespace::Consumer
    });

    let bar_component = Components::Bar(Barcode::Code(12, 95, String::from("akjskajgskas")));

    nested(ana_component);
    nested(base_component);
    nested(bar_component);
}