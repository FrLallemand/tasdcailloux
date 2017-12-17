use super::{Content, Header, ListPanel};
use connection::*;
use gtk;
use gtk::*;
use std::process;
use std::sync::{Arc, RwLock};
use nanomsg::{Socket, Protocol};
use std::io::{Read, Write};
use bincode::{serialize, deserialize, Infinite};
use tasdcailloux::models::{Message, Error};
use tasdcailloux::models::MessageType as AppMessageType;
use tasdcailloux::models::element::Element;

pub struct App {
    pub window:  Window,
    pub header:  Header,
    pub content: Content,
}

pub struct ConnectedApp(App);

impl ConnectedApp {
    pub fn then_execute(self) {
        self.0.window.show_all();
        gtk::main();
    }
}


impl App {
    pub fn new() -> App{
        if gtk::init().is_err() {
            eprintln!("failed to initialize GTK Application");
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

        App { window, header, content }
    }

    pub fn connect_events(self) -> ConnectedApp {
        let socket = Arc::new(RwLock::new(establish_connection()));

        {
            let listPanel = &self.content.listPanel;
            self.create_list(&listPanel, socket.clone());
        }

        ConnectedApp(self)
    }

    pub fn create_list(&self, listPanel: &ListPanel, socket: Arc<RwLock<Socket>>) {


        let message = Message{ message_type: AppMessageType::GetAll };
        let encoded: Vec<u8> = serialize(&message, Infinite).unwrap();
        let mut s = socket.write().unwrap();
        s.write(&encoded).unwrap();
        let mut msg = Vec::new();
        s.read_to_end(&mut msg).unwrap();
        let decoded: Result<Vec<Element>, Error> = deserialize(&msg).unwrap();
        match decoded{
            Ok(elements) => {
                for element in elements {
                    if(&element.name == ""){
                        listPanel.add_row(&element.name, true);
                    } else {
                        listPanel.add_row(&element.name, false);
                    }
                }
            },
            Err(e) => println!("Error : {:?}",e)
        }

    }
}
