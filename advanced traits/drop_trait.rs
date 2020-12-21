use std::ops::Drop;

struct Email {
    from: String,
    recipients: Vec<String>
}

impl Drop for Email {
    fn drop(&mut self) {
        println!("Drop initiated! {}", self.from);
        if !self.recipients.is_empty() { 
            println!("Join all elements{}", self.recipients.join("-"));
        }
        println!("Exiting Drop trait");
    }
}

fn main() {
    {
        let mut a = Email {
            from: "joyce".to_string(),
            recipients: vec!["alan".to_string(), "ben".to_string()]
        };
        println!("Before assigning value");
        a = Email {
            from: "sam".to_string(),
            recipients: vec![]
        };
        println!("After assigning value");
    }
}