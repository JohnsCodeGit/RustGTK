
mod window;
use window::RustGTKWindow;
use libadwaita::{
    gtk::Orientation,
    prelude::{ApplicationExt, ApplicationExtManual, BoxExt, WidgetExt},
    Application, ApplicationWindow, HeaderBar, WindowTitle,                             
};

fn init(app: &Application){
    let gtk_window = RustGTKWindow::new(app);
    gtk_window.init();
    gtk_window.build_home_gui();
    
   
}

fn main(){
    
    let app = Application::new(Some("com.example.RustGTK"), Default::default());
   
    app.connect_activate(init);

    app.run();
    
} 