#[derive(Debug, PartialEq)]
pub struct PanelData {
    pub keypad: u8,
    display: Vec<u32>
}

fn panel_data(keypad: u8, display: Vec<u32>) -> PanelData {
    PanelData {
        keypad,
        display
    }
}

fn main() {
    let keypad: u8 = 10;
    let display: Vec<u32> = vec![12, 34];

    let panel = PanelData {
        keypad,
        display
    };

    assert_eq!(panel, PanelData { keypad: 10, display: vec![12, 34]} );
    assert_eq!(panel.keypad, 10);
    assert_eq!(panel.display, vec![12, 34]);

    let panel = panel_data(12, vec![4, 512]);

    assert_eq!(panel, panel_data(12, vec![4, 512]));
}