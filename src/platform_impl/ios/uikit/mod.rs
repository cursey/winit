#![deny(unsafe_op_in_unsafe_fn)]
#![allow(unused_imports)] // TMP
#![allow(dead_code)] // TMP

mod application;
mod device;
mod responder;
mod screen;
mod view;
mod view_controller;
mod window;

pub(crate) use self::application::UIApplication;
pub(crate) use self::device::UIDevice;
pub(crate) use self::responder::UIResponder;
pub(crate) use self::screen::UIScreen;
pub(crate) use self::view::UIView;
pub(crate) use self::view_controller::UIViewController;
pub(crate) use self::window::UIWindow;
