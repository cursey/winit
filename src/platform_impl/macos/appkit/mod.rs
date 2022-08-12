#![deny(unsafe_op_in_unsafe_fn)]
#![allow(unused_imports)] // TMP
#![allow(dead_code)] // TMP
#![allow(non_snake_case)]

mod application;
mod color;
mod cursor;
mod event;
mod image;
mod responder;
mod screen;
mod text_input_context;
mod view;
mod window;

pub(crate) use self::application::NSApplication;
pub(crate) use self::color::NSColor;
pub(crate) use self::cursor::NSCursor;
pub(crate) use self::event::NSEvent;
pub(crate) use self::image::NSImage;
pub(crate) use self::responder::NSResponder;
pub(crate) use self::screen::NSScreen;
pub(crate) use self::text_input_context::NSTextInputContext;
pub(crate) use self::view::NSView;
pub(crate) use self::window::NSWindow;
