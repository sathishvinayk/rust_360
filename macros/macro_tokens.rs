use std::collections::HashMap;

macro_rules! stmt_example {
    ($a: expr) => {
        {
            let mut a: HashMap<u8, u8> = $a;

            a.insert(1,2);

            a
        }
    };
}
macro_rules! ty_example {
    ($a: ty) => {
        {
            let value: $a = (1, String::from("hi"));
            value
        }
    };
}

macro_rules! pat_example {
    ($a: pat) => {
        {
            let x = Some(2);
            match x {
                $a => println!("Within 1 to 3"),
                _ => println!("After 1 to 3")
            }
        }
    };
}

macro_rules! path_example {
    ($a: path) => {
        use $a;

        println!("Duration is {:?}", Duration::new(10, 0));
    };
}

macro_rules! meta_example {
    ($a: meta) => {
        #[$a]
        let x = ();
    };
}

macro_rules! tt_single_token {
    ($a: tt) => {
        println!("Token between integers: {}", 2 $a 2);
    };
}

macro_rules! tt_multi_token {
    ($a: tt) => {
        {
            println!("Multiple Tokens: {}", $a);
        }
    }
}

macro_rules! lifetime_example {
    ($a: lifetime) => {
        fn func<$a>(x: &$a i8, y: &$a i8) {
            println!("Remainder: {}", x % y);
        }
    };
}

macro_rules! literal_example {
    ($a: literal) => {
        println!("Literal value: {}", $a);
    };
}

macro_rules! vis_example {
    ($a: vis) => {
        $a fn public_function() {
            println!("Assign pub to make this function public!");
        }
    };
}

macro_rules! item_example {
    ($a: item) => {
        $a
    };
}

macro_rules! block_example {
    ($a: block) => {
        let value = $a;

        println!("The value recieved is {}", value);
    };
}


fn main() {
   let a = stmt_example!(HashMap::new());
   let ty = ty_example!((u8, String));
   pat_example!(Some(6));
   path_example!(std::time::Duration);
   meta_example!(allow(unused_variables));
   tt_single_token!(+);
   tt_multi_token!( { let a = true ; if a { 1 } else { 0 }});
   lifetime_example!('ctx);
   literal_example!("Am a string literal");
   vis_example!(pub);
   item_example!(fn x() { println!("Passing a function item token!")} );
   block_example!(
       {
           let a;
           if true {
               a = String::from("True")
           } else {
               a = String::from("False")
           }
           a
       }
   );
   
   x();
   func(&10, &2);
   public_function();
   println!("Value is x: {:?}", a[&1]);
   println!("ty value is x: {:?}", ty.0);
}