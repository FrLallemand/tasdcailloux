use gtk;
use gtk::*;

pub struct Header {
    pub container:  HeaderBar,

}

impl Header {
    pub fn new() -> Header {
        let container = HeaderBar::new();
        container.set_title("TasDCailloux");
        container.set_show_close_button(true);

        Header{ container }
    }
}
