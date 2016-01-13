extern crate gtk;


mod win {
    use gtk;
    use gtk::traits::*;
    use gtk::signal::Inhibit;
    use gtk::widgets::Builder;
    use gtk::{Window, Button, MessageDialog};


    pub fn main() {
        if gtk::init().is_err() {
            println!("Failed to initialize GTK.");
            return;
        }
        let glade_src = include_str!("builder_basics.glade");
        let builder = Builder::new_from_string(glade_src).unwrap();

        unsafe {
            let window: Window = builder.get_object("window1").unwrap();
            let bigbutton: Button = builder.get_object("button1").unwrap();
            let dialog: MessageDialog = builder.get_object("messagedialog1").unwrap();

            window.connect_delete_event(|_, _| {
                gtk::main_quit();
                Inhibit(false)
            });

            bigbutton.connect_clicked(move |_| {
                dialog.run();
                dialog.hide();
            });

            window.show_all();
        }

        gtk::main();
    }
}

fn main() {
    win::main()
}
