#![cfg_attr(not(feature = "gtk_3_10"), allow(unused_variables, unused_mut))]
use failure::{
    Fallible,
};
use log::info;
use super::pump;
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{
    AboutDialog, AppChooserDialog, ApplicationWindow, Builder, Button, Dialog, Entry,
    FileChooserAction, FileChooserDialog, FontChooserDialog, RecentChooserDialog, ResponseType,
    Scale, SpinButton, Spinner, Switch, Window,
};

use std::env::args;

// make moving clones into closures more convenient
macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
                move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
                move |$(clone!(@param $p),)+| $body
        }
    );
}

// upgrade weak reference or return
#[macro_export]
macro_rules! upgrade_weak {
    ($x:ident, $r:expr) => {{
        match $x.upgrade() {
            Some(o) => o,
            None => return $r,
        }
    }};
    ($x:ident) => {
        upgrade_weak!($x, ())
    };
}


// This function creates "actions" which connect on the declared actions from the menu items.
fn add_actions( application: &gtk::Application, builder: Builder) ->Fallible<()> {
    let switch: Switch = builder.get_object("app.pump01.switch").expect("Couldn't get app_button");
    // Thanks to this method, we can say that this item is actually a checkbox.
    switch.connect_changed_active(move |switch| {
        let mut is_active = switch.get_active();
        info!("Pump switch {}",is_active);
        pump::set_pump(21,!is_active as u8);
    });
    Ok(())
}


fn build_ui(application: &gtk::Application) -> Fallible<()> {
    let glade_src = include_str!("../assert/app.ui");
    let builder = Builder::new_from_string(glade_src);

    let window: ApplicationWindow = builder.get_object("app.window").expect("Couldn't get window1");
    window.set_application(Some(application));
    // let bigbutton: Button = builder.get_object("button1").expect("Couldn't get button1");
    // let dialog: MessageDialog = builder
        // .get_object("messagedialog1")
        // .expect("Couldn't get messagedialog1");

    // bigbutton.connect_clicked(move |_| {
        // dialog.run();
        // dialog.hide();
    // });

   
    add_actions(application,builder);
    window.show_all();
    Ok(())
}
use super::config::Config;
pub async fn create(config: &Config) -> Fallible<()>{
    let application = gtk::Application::new(
        Some("org.automata.irrigatron"),
        Default::default(),
    )?;

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
    Ok(())
}
