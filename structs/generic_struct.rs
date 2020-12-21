#[derive(Debug, PartialEq)]
struct Layout<T> {
    plot: T,
    outline: T
}

#[derive(Debug, PartialEq)]
struct Layout2<T> {
    plot: Vec<T>
}

impl<T> Layout2<T> {
    fn new() -> Self {
        Layout2 {
            plot: Vec::new()
        }
    }

    fn push(&mut self, value: T) {
        self.plot.push(value)
    }
}

fn main() {
    let layout1 = Layout { plot: 8, outline: 100 };
    let layout2 = Layout { plot: "String1", outline: "String2" };

    let layout3 = Layout { plot: "Original_string_1".to_string(), outline: "Original_string_2".to_string() };

    // let layout4 = Layout { plot: "Hi there", outline: 'č'};

    let mut new_layout = Layout2::<char>::new();
    new_layout.push('č');

    assert_eq!(new_layout.plot, vec!['č']);

    let mut new_layout = Layout2::<String>::new();
    let value = String::from("Good Day!");
    new_layout.push(value);

    assert_eq!(new_layout.plot, vec!["Good Day!".to_owned()]);

    assert_eq!(layout1.plot, 8);
    assert_eq!(layout1.outline, 100);

    assert_eq!(layout2.plot, "String1");
    assert_eq!(layout2.outline, "String2");

    assert_eq!(layout3.plot, "Original_string_1".to_string());
    assert_eq!(layout3.outline, "Original_string_2".to_string());
}