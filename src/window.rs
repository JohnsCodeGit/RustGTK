//mod application_row;
//use crate::application_row::ApplicationRow;

use libadwaita::{
    gtk::{Orientation, Box, Button, Separator, prelude::*,},
    prelude::*,
    Application, ApplicationWindow, HeaderBar,ViewSwitcherBar, ViewStack, Leaflet, ViewSwitcherTitle

};
fn button_clicked(my_str: &str){
    println!("{}", my_str);
}

pub struct RustGTKWindow {
    pub window: ApplicationWindow,
    pub btn_of_click: Button,
    pub btn_of_clicknt: Button,
    pub home_box: Box,
    pub other_box: Box,
    pub main_box: Box,
    pub nav_box: Box,
    pub seppy: Separator,
    pub nav_home: Button,
    pub nav_switch: ViewSwitcherTitle,
    pub stack: ViewStack,
    pub header: HeaderBar,
    pub view_switch: ViewSwitcherBar,
    pub leaflet: Leaflet,
    
}

impl RustGTKWindow {
    pub fn new(app: &Application) -> Self {
        
        Self{
            
            window: ApplicationWindow::new(app),
            btn_of_click: Button::with_label("Click Me"),
            btn_of_clicknt: Button::with_label("Don't click Me"),
            home_box: Box::new(Orientation::Vertical, 10),
            other_box: Box::new(Orientation::Vertical, 10),
            main_box: Box::new(Orientation::Vertical, 0),
            nav_box: Box::new(Orientation::Vertical, 0),
            seppy: Separator::new(Orientation::Horizontal),
            nav_home: Button::with_label("Home"),
            nav_switch : ViewSwitcherTitle::new(),
            stack: ViewStack::new(),
            header: HeaderBar::new(),
            view_switch: ViewSwitcherBar::new(),
            leaflet: Leaflet::new(),
            
        }
    
    }
    pub fn init(&self){

        //window init
        self.window.set_default_width(900);
        self.window.set_default_height(600);
        self.window.set_title(Some("Libby Addy App"));
        self.header.set_centering_policy(libadwaita::CenteringPolicy::Strict);

        //add items to navbar
        self.stack.add_titled(&self.home_box, Some("home"), "Home");
        self.stack.add_titled(&self.other_box, Some("click"), "Click");
        //self.stack.add_titled(&self.seppy, Some("sep"), "Seppy");
        
        //setup titlebar nav
        self.nav_switch.set_stack(Some(&self.stack));  
        self.header.set_title_widget(Some(&self.nav_switch));
        self.main_box.append(&self.header);
        self.nav_box.append(&self.stack);
        self.nav_box.append(&self.view_switch);
        self.nav_box.set_vexpand(true);
        self.nav_box.set_hexpand(true);
        self.main_box.append(&self.nav_box);

        self.leaflet.append(&self.main_box);
        self.window.set_content(Some(&self.leaflet));
        
        //show window
        self.window.show();
    }

    pub fn build_home_gui(&self){     
        //Add ListView for notes

        let model = gio::ListStore::new(gio::AppInfo::static_type());
        gio::AppInfo::all().iter().for_each(|app_info| {
            model.append(app_info);
        });
        let sorter = gtk4::CustomSorter::new(move |obj1, obj2| {
            let app_info1 = obj1.downcast_ref::<gio::AppInfo>().unwrap();
            let app_info2 = obj2.downcast_ref::<gio::AppInfo>().unwrap();
    
            app_info1
                .name()
                .to_lowercase()
                .cmp(&app_info2.name().to_lowercase())
                .into()
        });
        let sorted_model = gtk4::SortListModel::new(Some(&model), Some(&sorter));
        let selection_model = gtk4::SingleSelection::new(Some(&sorted_model));

        let factory = gtk4::SignalListItemFactory::new();
        factory.connect_setup(move |_factory, item| {
            // let row = ApplicationRow::new();
            // item.set_child(Some(&row));
        });
        let list = gtk4::ListView::new(Some(&selection_model), Some(&factory));

        self.home_box.append(&list);
    }

}