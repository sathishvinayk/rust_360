trait Panel {
    fn screen(&self);
}
struct HomeScreen {
    section: String
}

impl HomeScreen {
    fn from(text: &str) -> HomeScreen {
        HomeScreen {
            section: text.to_string()
        }
    }
}
struct Component {
    home: HomeScreen,
    widget: char,
    setting: bool
}

impl Component {
    fn widget_properties(home: &str, widget: char, setting: bool) -> Component {
        Component {
            home: HomeScreen::from(home),
            widget: widget,
            setting: setting
        }
    }
}
impl Panel for HomeScreen {
    fn screen(&self) {
        println!("Section: {}", self.section);
    }
}

impl Panel for Component {
    fn screen(&self) {
        println!("Widget ->  {}", self.widget);
        self.home.screen();
        println!("Setting ? -> {}", self.setting);
    }
}

fn view<T>(home: T) where T: Panel {
    home.screen();
}

fn main() {
    let screen_alone = HomeScreen::from("Hello");
    let screen_properties = Component::widget_properties("potrait", '\u{1F518}', true);

    view(screen_alone);
    println!(", ");
    view(screen_properties)
}