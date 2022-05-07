use gtk4::{prelude::*, HeaderBar, StackSwitcher};
use gtk4::{Application, ApplicationWindow, Button, Box, Separator};


fn button_clicked(my_str: &str){
    println!("{}", my_str);
}

pub struct RustGTKWindow {
    pub window: ApplicationWindow,
    pub btn_of_click: Button,
    pub btn_of_clicknt: Button,
    pub home_box: Box,
    pub other_box: Box,
    pub nav_box: Box,
    pub seppy: Separator,
    pub nav_home: Button,
    pub nav_switch: StackSwitcher,
    pub stack: gtk4::Stack,
    pub header: HeaderBar,
    
}

impl RustGTKWindow {
    pub fn new(app: &Application) -> Self {
        
        Self{
            
            window: ApplicationWindow::new(app),
            btn_of_click: Button::with_label("Click Me"),
            btn_of_clicknt: Button::with_label("Don't click Me"),
            home_box: Box::new(gtk4::Orientation::Vertical, 5),
            other_box: Box::new(gtk4::Orientation::Vertical, 5),
            nav_box: Box::new(gtk4::Orientation::Vertical, 10),
            seppy: Separator::new(gtk4::Orientation::Horizontal),
            nav_home: Button::with_label("Home"),
            nav_switch : gtk4::StackSwitcher::new(),
            stack: gtk4::Stack::new(),
            header: HeaderBar::new(),
            
        }
    
    }
    pub fn init(&self){
        self.window.set_default_width(800);
        self.window.set_default_height(600);
        self.window.set_title(Some(""));
        self.header.set_title_widget(Some(&self.nav_box));
        self.window.set_titlebar(Some(&self.header));

        self.stack.set_transition_type(gtk4::StackTransitionType::SlideLeftRight);
        self.stack.set_transition_duration(200);

        self.home_box.append(&self.btn_of_click);
        self.other_box.append(&self.btn_of_clicknt);

        self.stack.add_titled(&self.home_box, Some("home"), "Home");
        self.stack.add_titled(&self.other_box, Some("click"), "Click");

        self.nav_switch.set_stack(Some(&self.stack));

        self.nav_box.append(&self.nav_switch);
        self.window.set_child(Some(&self.stack));
        
        self.window.present();
    }

    pub fn home(&self){     
        //Add ListView for notes
    }

}