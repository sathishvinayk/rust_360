#[derive(Debug)]
struct Kind {
    x: i8,
    y: bool,
    z: String,
    en: Coat
}

#[derive(Debug)]
enum Coat {
    Thin,
    Thick
}

impl Default for Coat { 
    fn default() -> Self {
        Self::Thin
    }
}

impl Default for Kind {
    fn default() -> Self {
        Kind {
            x: 2,
            y: true,
            z: String::from("abc"),
            en: Coat::Thick
        }
    }
}

fn main() {
    let coat: Coat = Default::default();
    let kind: Kind = Kind {
        x: Default::default(),
        y: Default::default(),
        z: Default::default(),
        en: Default::default()
    };

    let kind2: Kind = Kind {
        x: 02,
        ..Default::default()
    };

    println!("Default of Kind is {:?}", kind);
    println!("Default of Kind2 is {:?}", kind2);

    println!("Default of Coat is {:?}", coat);
}