#![allow(non_camel_case_types)]

use std::{ffi::CString, fmt::{Debug, Display}};

type wchar_t = i8;

#[link(name = "user32")]
extern "system" {
    fn MessageBoxA(hwnd: usize, text: *const wchar_t, caption: *const wchar_t, ty: usize) -> i32;
}

pub fn message_box<S: AsRef<str>>(text: S, title: S, mb_type: &[MessageBoxType]) -> Result<MessageBoxResult, MBError<S>> {
    let text = CString::new(text.as_ref()).map_err(|_| MBError::InvalidString(text))?;
    let title = CString::new(title.as_ref()).map_err(|_| MBError::InvalidString(title))?;
    let type_bits = mb_type.into_iter().fold(0, |acc, &field| acc | field as usize);

    unsafe {
        MessageBoxA(0, text.as_ptr(), title.as_ptr(), type_bits)
    }.try_into().map_err(|(val, msg)| MBError::ErroneousReturnValue(val, msg))
}

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub enum MessageBoxType {
    OK_CANCEL = 0x1,
    ABORT_RETRY_IGNORE = 0x2,
    YES_NO_CANCEL = 0x3,
    YES_NO = 0x4,
    RETRY_CANCEL = 0x5,
    CANCEL_TRY_CONTINUE = 0x6,

    HELP = 0x4000,

    ICON_ERROR = 0x10,
    ICON_QUESTION = 0x20,
    ICON_EXCLAMATION = 0x30,
    ICON_INFORMATION = 0x40,

    DEF_BUTTON2 = 0x100,
    DEF_BUTTON3 = 0x200,
    DEF_BUTTON4 = 0x300,

    APPLICATION_MODAL = 0x0,
    SYSTEM_MODAL = 0x1000,
    TASK_MODAL = 0x2000,
    
    DEFAULT_DESKTOP_ONLY = 0x20000,

    TEXT_RTL = 0x80000,
    RTL_READING = 0x100000,
    SET_FOREGROUND = 0x10000,
    TOPMOST = 0x40000,
    SERVICE_NOTIFICATION = 0x200000,
}

#[derive(Debug, Copy, Clone)]
pub enum MessageBoxResult {
    OK = 1,
    CANCEL = 2,
    ABORT = 3,
    RETRY = 4,
    IGNORE = 5,
    YES = 6,
    NO = 7,
    TRYAGAIN = 10,
    CONTINUE = 11,
}

impl TryFrom<i32> for MessageBoxResult {
    type Error = (i32, &'static str);

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        use MessageBoxResult::*;

        Ok(match value {
            1 => OK,
            2 => CANCEL,
            3 => ABORT,
            4 => RETRY,
            5 => IGNORE,
            6 => YES,
            7 => NO,
            10 => TRYAGAIN,
            11 => CONTINUE,
            _ => return Err((value, "Invalid return value from MessageBoxA")),
        })
    }
}

#[derive(Debug)]
pub enum MBError<T> {
    InvalidString(T),
    ErroneousReturnValue(i32, &'static str),
}

impl<T> Display for MBError<T>
where T: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidString(t) => t.fmt(f),
            Self::ErroneousReturnValue(ret, msg) => write!(f, "{}: {}", msg, ret),
        }
    }
}

impl<T> std::error::Error for MBError<T>
where T: Debug + Display {}