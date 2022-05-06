
mod window;
use window::RustGTKWindow;
use gtk4::prelude::*;
use gtk4::{Application};

fn init(app: &Application){
    let gtk_window = RustGTKWindow::new(app);
    gtk_window.setup_gui();
    gtk_window.window.present();
}

fn main(){
    
    let app = Application::new(Some("com.example.RustGTK"), Default::default());

    app.connect_activate(move |app| {
        
        init(&app);
        
    });

    app.run();
    
} 