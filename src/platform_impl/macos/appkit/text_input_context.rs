use objc2::foundation::{NSObject, NSString};
use objc2::rc::{Id, Shared};
use objc2::{extern_class, msg_send, msg_send_id, ClassType};

type NSTextInputSourceIdentifier = NSString;

extern_class!(
    /// Main-Thread-Only!
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub(crate) struct NSTextInputContext;

    unsafe impl ClassType for NSTextInputContext {
        type Superclass = NSObject;
    }
);

impl NSTextInputContext {
    pub fn invalidateCharacterCoordinates(&self) {
        unsafe { msg_send![self, invalidateCharacterCoordinates] }
    }

    pub fn discardMarkedText(&self) {
        unsafe { msg_send![self, discardMarkedText] }
    }

    pub fn selectedKeyboardInputSource(&self) -> Option<Id<NSTextInputSourceIdentifier, Shared>> {
        unsafe { msg_send_id![self, selectedKeyboardInputSource] }
    }
}
