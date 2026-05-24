mod editor;
mod input_fields;
pub use editor::ProjectEditor;
pub use input_fields::{InputField, MultiLineInput, PathInput, SingleLineInput};

#[derive(Debug)]
pub enum EditorEvent {
    Save,
    Cancel,
}
