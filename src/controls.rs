use gtk::{self, Button, Orientation};
use gtk::{Align, prelude::*};

pub fn build_controls() -> gtk::Box {
    let b_start = Button::builder()
        .label("Start")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .can_shrink(true)
        .can_focus(true)
        .valign(Align::Center)
        .build();
    b_start.add_css_class("bg-green");
    b_start.connect_clicked(|_| {
        println!("Started!");
    });

    let b_pause = Button::builder()
        .label("Pause")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .can_shrink(true)
        .can_focus(true)
        .valign(Align::Center)
        .build();
    b_pause.add_css_class("bg-yellow");
    b_pause.connect_clicked(|_| {
        println!("Paused!");
    });

    let b_stop = Button::builder()
        .label("Stop")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .can_shrink(true)
        .can_focus(true)
        .vexpand(false)
        .valign(Align::Center)
        .build();
    b_stop.add_css_class("bg-red");
    b_stop.connect_clicked(|_| {
        println!("Stopped!");
    });

    let controls = gtk::Box::builder()
        .halign(Align::Center)
        .orientation(Orientation::Horizontal)
        .build();
    controls.append(&b_start);
    controls.append(&b_pause);
    controls.append(&b_stop);
    controls
}
