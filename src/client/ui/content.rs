use gtk;
use gtk::*;

pub struct Content {
    pub container: Box,
    pub listPanel: ListPanel,
    pub viewStack: Stack
}

pub struct ListPanel {
    pub container: ScrolledWindow,
    pub mainList: ListBox
}

pub struct ListPanelRow {
    pub container: Grid,
    pub label: Label
}

impl Content {
    pub fn new() -> Content{
        let container = Box::new(Orientation::Horizontal, 0);
        let listPanel = ListPanel::new();
        let viewStack = Stack::new();

        container.add(&listPanel.container);
        container.add(&viewStack);
        Content { container, listPanel, viewStack }
    }
}

impl ListPanel {
    pub fn new() -> ListPanel{
        let container = ScrolledWindow::new(None, None);
        let mainList = ListBox::new();

        container.set_property_width_request(200);
        container.add(&mainList);

        ListPanel { container, mainList }
    }

    pub fn add_row(&self, label: &str, unknown: bool){
        let row = ListPanelRow::new(label, unknown);
        self.mainList.add(&row.container);
    }
}

impl ListPanelRow {
    pub fn new(text: &str, unknown: bool) -> ListPanelRow {
        let container = Grid::new();
        let label = Label::new(text);
        if(unknown){
            label.set_markup("<i>Unknown</i>");
        }
        container.attach(&label, 0, 0, 2, 1 );
        container.set_border_width(12);
        ListPanelRow { container, label }
    }
}
