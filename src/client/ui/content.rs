use gtk::*;
use gtk;
use tasdcailloux::models::element::Element;
use std::cell::RefCell;
use std::collections::HashMap;

pub struct Content {
    pub container: Box,
    pub list: ListBox,
    pub stack: Stack,
    pub stack_elements : RefCell<HashMap<i32, StackWindow>>
}

pub struct ListPanelRow {
    pub container: ListBoxRow,
    pub label: Label
}

#[derive(Clone)]
pub struct StackWindow {
    pub container: Box,
    pub element_image: Image,
    pub name_label: Label
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

        Content { container, list, stack , stack_elements: RefCell::new(HashMap::new()) }
    }

    pub fn add_row(&self, element: &Element, unknown: bool){
        let row = ListPanelRow::new(element, unknown);
        let stack_element = StackWindow::new(&element);
        self.list.add(&row.container);
        self.stack.add_named(&stack_element.container, &element.id.to_string());
        self.stack_elements.borrow_mut().insert(element.id, stack_element);
    }
}

impl ListPanelRow {
    pub fn new(element: &Element, unknown: bool) -> ListPanelRow {
        let container = ListBoxRow::new();
        let grid = Grid::new();
        let label = Label::new(element.name.as_ref());
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
    pub fn new(element: &Element) -> StackWindow {
        let container = Box::new(Orientation::Vertical, 0);
        container.set_hexpand(true);
        container.set_vexpand(true);
        container.set_margin_top(20);

        let summary = Box::new(Orientation::Horizontal, 0);
        let image_box = Box::new(Orientation::Vertical, 0);
        let image = Image::new();

        //TODO : use gtk resources
        let style_context = image.get_style_context().unwrap();
        let style = include_str!("../../../data/style/image_preview.css");
        let css = CssProvider::new();
        gtk::CssProviderExt::load_from_data(&css,style.as_bytes()).unwrap();
        style_context.add_provider(&css, STYLE_PROVIDER_PRIORITY_APPLICATION);

        image.set_margin_top(5);
        image.set_margin_bottom(5);
        image.set_margin_start(5);
        image.set_margin_end(5);

        image_box.add(&image);
        image_box.set_size_request(300, 300);
        image_box.set_homogeneous(true);
        image.set_name("image_frame");

        summary.add(&image_box);
        summary.set_border_width(10);
        summary.set_homogeneous(true);
        summary.set_spacing(20);

        let description = Grid::new();
        description.set_row_spacing(2);
        description.set_column_spacing(20);

        let name_entry = Entry::new_with_buffer(&EntryBuffer::new(Some(element.name.as_ref())));
        let name_label = Label::new("Name : ");
        name_label.set_halign(Align::Start);
        name_entry.set_placeholder_text("Unknown");
        description.attach(&name_label, 0, 0, 1, 1);
        description.attach(&name_entry, 1, 0, 1, 1);

        let id_entry = Entry::new_with_buffer(&EntryBuffer::new(Some(&element.id.to_string())));
        let id_label = Label::new("Id : ");
        id_label.set_halign(Align::Start);
        id_entry.set_editable(false);
        description.attach(&id_label, 0, 1, 1, 1);
        description.attach(&id_entry, 1, 1, 1, 1);

        let weight_entry = Entry::new_with_buffer(&EntryBuffer::new(Some(&element.weight.to_string())));
        let weight_label = Label::new("Weight : ");
        weight_label.set_halign(Align::Start);
        weight_entry.set_placeholder_text("Unknown");
        description.attach(&weight_label, 0, 2, 1, 1);
        description.attach(&weight_entry, 1, 2, 1, 1);

        summary.add(&description);

        container.add(&summary);

        StackWindow { container, element_image: image , name_label }
    }
}
