use fltk::*;

fn main() {
    let app = app::App::default();
    let mut win = window::Window::new(100, 100, 800, 600, "Test");
    let mut buf = text::TextBuffer::default();
    let mut editor = text::TextEditor::new(0, 0, 800, 600, "");
    editor.set_buffer(Some(buf));
    win.show();

    win.set_callback(Box::new(move || {
        if app::event() == enums::Event::Close {
            println!("Closing, but deleting the buffer first!");
            unsafe { // This is unsafe since the editor might outlive the buffer
                buf.delete();
            }
            app.quit();
        }
    }));

    app.run().unwrap();
}