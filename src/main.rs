extern crate gtk;

use std::process::exit;
use gtk::traits::*;
use gtk::signal::Inhibit;

fn main() {
    if gtk::init().is_err() {
        println!("Couldn't inititalize GTK");
        exit(-1);
    }

    let window = gtk::Window::new(gtk::WindowType::Toplevel).unwrap();

    window.set_default_size(400, 400);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.show_all();
    gtk::main();
}
