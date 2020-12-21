struct Cards<'ctx> {
    card: &'ctx str
}

struct Parrot<'xfx, 'gtx> {
    name: &'xfx str,
    color: &'gtx str
}

fn construct() {
    let name = String::from("charlie");
    let value;
    let pallete;

    {
        let color = String::from("green");
        {
            let parrot = Parrot {
                name: &name,
                color: &color  //Error: borrowed value does not live long enough
            };
            value = parrot.name;
            pallete = parrot.color;
        }
    }

    println!("New values are : {}, {}", value, pallete);  //Error: `color` does not live long enough
}

fn main() {
    let create_card = Cards {
        card: "Spade"
    };

    let create_card2 = Cards {
        card: "Jack"
    };

    println!("Card type is: {}", create_card.card);
    println!("Second card type is: {}", create_card2.card);

    construct();
}