use ffi::message_box;
use ffi::MessageBoxType::*;

mod ffi;

fn main() {
    _ = message_box(
            "Hello", 
            "Title", 
            &[CANCEL_TRY_CONTINUE, ICON_INFORMATION, TEXT_RTL, DEF_BUTTON3]
        );
}
