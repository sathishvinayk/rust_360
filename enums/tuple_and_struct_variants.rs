#[derive(Debug, PartialEq)]
enum Application {
    User (i8),
    Code (char),
    Git { 
        version: f32,
        base: String
    }
}
fn main() {
    let user = Application::User(2);
    let code = Application::Code('æ');

    let git = Application::Git {
        version: 1.99,
        base: "bitbucket".to_owned()
    };

    println!("User value is {:?}", user);
    println!("Code value is {:?}", code);

    println!("{:?}, \n {:?}, \n {:?}", user, code, git);
}