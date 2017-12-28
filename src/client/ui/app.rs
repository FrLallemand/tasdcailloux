extern crate xdg;

use super::{Content, Header};
use gtk;
use gtk::*;
use gdk_pixbuf;
use std::process;
use tasdcailloux::models::element::Element;
use std::cell::RefCell;
use connection::*;
use std::thread;
use std::fs::File;
use std::io::{BufWriter, Write};

pub struct App {
    pub window:  Window,
    pub header:  Header,
    pub content: Content,
    pub origin_list: RefCell<Vec<Element>>,
    pub name: String
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
        let name = String::from("tasdcailloux");

        window.set_titlebar(&header.container);
        window.set_wmclass("tascailloux", "tasDCailloux");
        window.set_default_size(800, 600);
        window.add(&content.container);
        window.connect_delete_event(move |_, _| {
            main_quit();
            Inhibit(false)
        });

        let xdg_dirs = xdg::BaseDirectories::with_prefix(&name).unwrap();
        xdg_dirs.create_cache_directory("images")
            .expect("cannot create cache directory");

        App { window, header, content, origin_list: RefCell::new(list), name }
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
        let app_name = self.name.clone();
        self.content.list.connect_row_selected( move |_, _| {
            if let Some(row) = list.get_selected_row() {
                if let Some(id) = row.get_name() {
                    stack.set_visible_child_name(&id);

                    let xdg_dirs = xdg::BaseDirectories::with_prefix(&app_name).unwrap();
                    let find_cache_path = xdg_dirs.find_cache_file(format!("images/{}_{}", id, 0));
                    let image_cache = xdg_dirs.place_cache_file(format!("images/{}_{}", id, 0)).unwrap();
                    let pix = match find_cache_path {
                        Some(cache_path) => {
                            // Load it
                            gdk_pixbuf::Pixbuf::new_from_file_at_scale(&cache_path.to_str().unwrap(),
                                                                       250, 250, true)
                        },
                        None => {
                            // Download and load it
                            //TODO : use future, make all this shit async. Good luck, you're on your own ;)
                            let id_clone = id.clone();
                            let image_cache_clone = image_cache.clone();
                            let thr = thread::spawn(move || {
                                let img = get_image_for(id_clone.parse().unwrap(), 0)("Unable to get image");
                                let cache_path = image_cache_clone.to_str().unwrap();
                                let file = File::create(cache_path).unwrap();
                                let mut writer = BufWriter::new(file);
                                writer.write(&img).expect("unable to write file !");
                            });
                            thr.join().unwrap();
                            let cache_path = image_cache.to_str().unwrap();
                            gdk_pixbuf::Pixbuf::new_from_file_at_scale(&cache_path,
                                                                       250, 250, true)
                        }
                    };
                    stack_elements.borrow().get(&id.parse().unwrap()).unwrap().element_image.set_from_pixbuf(&pix.unwrap());
                }
            }
        });
    }
}
