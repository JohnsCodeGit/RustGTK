use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Button, Box, Separator};


fn button_clicked(my_str: &str){
    println!("{}", my_str);
}

fn attach_button_listener(btn: &Button, id: &'static str){
  
    btn.connect_clicked(move |_| {
        button_clicked(&id);
    });
}


pub struct RustGTKWindow {
    pub window: ApplicationWindow,
    pub btn_of_click: Button,
    pub btn_of_clicknt: Button,
    pub btn_box: Box,
    pub widgey_box: Box,
    pub seppy: Separator,
    
}

impl RustGTKWindow {
    pub fn new(app: &Application) -> Self {
        
        Self{
            
            window: ApplicationWindow::new(app),
            btn_of_click: Button::with_label("Click Me"),
            btn_of_clicknt: Button::with_label("Don't click Me"),
            btn_box: Box::new(gtk4::Orientation::Horizontal, 5),
            widgey_box: Box::new(gtk4::Orientation::Vertical, 5),
            seppy: Separator::new(gtk4::Orientation::Horizontal),
        }
    
    }
    pub fn setup_gui(&self){

        self.window.set_default_width(800);
        self.window.set_default_height(600);
        self.window.set_title(Some("New Ways"));
    
        attach_button_listener(&self.btn_of_click, "Click");
        attach_button_listener(&self.btn_of_clicknt, "Clicknt");

        self.btn_box.append(&self.btn_of_click);
        self.btn_box.append(&self.btn_of_clicknt);
        self.widgey_box.append(&self.btn_box);
        self.widgey_box.append(&self.seppy);
    
        self.window.set_child(Some(&self.widgey_box));
        self.window.present();
        
    }

}