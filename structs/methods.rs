#[derive(Debug, PartialEq)]
struct PanelData {
    keypad: u8,
    display: Vec<u8>
}

impl PanelData {
    fn new() -> PanelData {
        PanelData {
            keypad: 3,
            display: Vec::new(),
        }
    }

    fn add_value(&mut self, value: u64) {
        self.display.push(value as u8)
    }

    fn get_value(&mut self) -> Option<u8> {
        if self.display.is_empty() {
            None
        } else {
            self.display.pop()
        }
    }
}

impl PanelData {
    fn count(&self) -> usize {
        self.display.len()
    }

    fn taking_ownership(self) -> u8 {
        self.keypad
    }
}

impl PanelData {
    // Some more methods
}

fn main() {
    let mut panel = PanelData::new();

    panel.add_value(91);

    assert_eq!(panel, PanelData { keypad: 3, display: vec![91]});
    assert_eq!(panel.get_value(), Some(91));
    assert_eq!(panel.count(), 0);

    let take = panel.taking_ownership();  // Error: borrow of moved value: `panel`
    panel.add_value(22);  // Error: borrow of moved value: `panel`
}