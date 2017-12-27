use super::{Content, Header};
use gtk;
use gtk::*;
use gdk_pixbuf;
use std::process;
use tasdcailloux::models::element::Element;
use std::cell::RefCell;
use std::io::{Read, Write};
use nanomsg::{Socket, Protocol};
use connection::*;
use futures::Future;
use std::{thread, time};

pub struct App {
    pub window:  Window,
    pub header:  Header,
    pub content: Content,
    pub origin_list: RefCell<Vec<Element>>,
 //   pub socket: Socket
}

pub struct ConnectedApp(App);

impl ConnectedApp {
    pub fn then_execute(self) {
        self.0.window.show_all();
        gtk::main();
    }
}


impl App {
    pub fn new(list: Vec<Element>) -> App{
        if gtk::init().is_err() {
            println!("failed to initialize GTK Application");
            process::exit(1);
        }
        let window = Window::new(WindowType::Toplevel);
        let header = Header::new();
        let content = Content::new();

        window.set_titlebar(&header.container);
        window.set_wmclass("tascailloux", "tasDCailloux");
        window.set_default_size(800, 600);
        window.add(&content.container);
        window.connect_delete_event(move |_, _| {
            main_quit();
            Inhibit(false)
        });
        App { window, header, content, origin_list: RefCell::new(list) }
    }

    pub fn connect_events(self) -> ConnectedApp {
        //let socket = Arc::new(RwLock::new(establish_connection()));
        {
            //let content = &self.content;
            self.create_list();
            self.connect_row_selected();
        }

        ConnectedApp(self)
    }

    pub fn create_list(&self) {
        let c = self.origin_list.borrow();
        for element in c.iter() {
            if &element.name == "" {
                self.content.add_row(element, true);
            } else {
                self.content.add_row(element, false);
            }
        }

    }

    pub fn connect_row_selected(&self) {
        let stack = self.content.stack.clone();
        let list = self.content.list.clone();
        let stack_elements = self.content.stack_elements.clone();
        self.content.list.connect_row_selected( move |_, _| {
            if let Some(row) = list.get_selected_row() {
                if let Some(id) = row.get_name() {
                    stack.set_visible_child_name(&id);
                    //stack_elements.borrow().get(&id.parse().unwrap()).unwrap().name_label.set_markup("plop");
                    let id_clone = id.clone();

                    //TODO : use future, make all this shit async. Good luck, you're on your own ;)
                    let thr = thread::spawn(move || {
                        get_image_for(id_clone.parse().unwrap(), 0);
                    });

                    thr.join();

                    let pix = gdk_pixbuf::Pixbuf::new_from_file_at_scale(&format!("{}_{}", id, 0),
                                                                        250, 250, true);

                    stack_elements.borrow().get(&id.parse().unwrap()).unwrap().element_image.set_from_pixbuf(&pix.unwrap());
                }
            }
        });
    }
}