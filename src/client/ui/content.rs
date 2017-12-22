use gtk::*;
use tasdcailloux::models::element::Element;

pub struct Content {
    pub container: Box,
    pub list: ListBox,
    pub stack: Stack
}

pub struct ListPanelRow {
    pub container: ListBoxRow,
    pub label: Label
}

pub struct StackWindow {
    pub container: Box
}

impl Content {
    pub fn new() -> Content{
        let container = Box::new(Orientation::Horizontal, 0);
        let list_window = ScrolledWindow::new(None, None);
        let list = ListBox::new();
        let stack = Stack::new();

        list_window.set_property_width_request(200);
        list_window.add(&list);

        container.add(&list_window);
        container.add(&stack);
        Content { container, list, stack }
    }

    pub fn add_row(&self, element: &Element, unknown: bool){
        let row = ListPanelRow::new(element, unknown);
        let stack_element = StackWindow::new();
        self.list.add(&row.container);
        self.stack.add_named(&stack_element.container, &element.id.to_string());
    }

}

impl ListPanelRow {
    pub fn new(element: &Element, unknown: bool) -> ListPanelRow {
        let container = ListBoxRow::new();
        let grid = Grid::new();
        let label = Label::new(&element.name as &str);
        if unknown {
            label.set_markup("<i>Unknown</i>");
        }
        grid.attach(&label, 0, 0, 2, 1 );
        grid.set_border_width(12);
        container.set_name(&element.id.to_string());
        container.add(&grid);
        ListPanelRow { container, label }
    }
}

impl StackWindow {
    pub fn new() -> StackWindow {
        let container = Box::new(Orientation::Vertical, 0);
        container.set_hexpand(true);
        container.set_vexpand(true);
        StackWindow { container}
    }
}
