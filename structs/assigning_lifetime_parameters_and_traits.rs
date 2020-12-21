struct Identification<'a> {
    id: &'a u8,
    age: &'a u32,
    name: &'a str
}

struct Factory<'a, T> {
    work: &'a T
}

#[derive(Debug, Clone)]
struct Connection {
    online: Vec<u32>
}

fn user<'a>(id: &'a u8, age: &'a u32, name: &'a str) -> Identification<'a> {
    Identification {
        id, age, name
    }
}

fn main() {
    let name = String::from("bob");
    let identification = Identification {
        id: &2,
        age: &62,
        name: &name
    };

    let identity = user(&4, &35, &"David");

    let factory_1 = Factory {
        work: &"Frontline"
    };

    let factory_2 = Factory {
        work: &vec![12]
    };

    let connect = Connection { online: vec![1] };
    let cloned_value = connect.clone();

    assert_eq!(*identification.id, 2);
    assert_eq!(*identification.age, 62);
    assert_eq!(*identification.name, String::from("bob"));

    assert_eq!(*identity.id, 4);
    assert_eq!(*identity.age, 35);
    assert_eq!(*identity.name, String::from("David"));

    assert_eq!(*factory_1.work, String::from("Frontline"));
    assert_eq!(*factory_2.work, vec![12]);

    println!("{:?}", connect);

    println!("{:?}", cloned_value);
}