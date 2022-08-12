use objc2::foundation::{NSArray, NSObject};
use objc2::rc::Shared;
use objc2::{extern_class, msg_send, ClassType};

use super::NSEvent;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub(crate) struct NSResponder;

    unsafe impl ClassType for NSResponder {
        type Superclass = NSObject;
    }
);

// Documented as "Thread-Unsafe".

impl NSResponder {
    // TODO: Allow "immutably" on main thread
    pub unsafe fn interpretKeyEvents(&mut self, events: &NSArray<NSEvent, Shared>) {
        unsafe { msg_send![self, interpretKeyEvents: events] }
    }
}
