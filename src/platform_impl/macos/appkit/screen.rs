use objc2::foundation::{
    CGFloat, MainThreadMarker, NSArray, NSDictionary, NSObject, NSRect, NSSize, NSString,
};
use objc2::rc::{Id, Shared};
use objc2::runtime::{Bool, Object};
use objc2::{extern_class, msg_send, msg_send_id, ClassType};

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub(crate) struct NSScreen;

    unsafe impl ClassType for NSScreen {
        type Superclass = NSObject;
    }
);

type NSDeviceDescriptionKey = NSString;

impl NSScreen {
    /// # Safety
    ///
    /// The application object must have been created.
    pub unsafe fn main(_mtm: MainThreadMarker) -> Option<Id<Self, Shared>> {
        unsafe { msg_send_id![Self::class(), mainScreen] }
    }

    /// # Safety
    ///
    /// The application object must have been created.
    pub unsafe fn screens(_mtm: MainThreadMarker) -> Id<NSArray<Self, Shared>, Shared> {
        unsafe { msg_send_id![Self::class(), screens] }.unwrap()
    }

    pub fn frame(&self) -> NSRect {
        unsafe { msg_send![self, frame] }
    }

    pub fn visibleFrame(&self) -> NSRect {
        unsafe { msg_send![self, visibleFrame] }
    }

    pub fn deviceDescription(&self) -> Id<NSDictionary<NSDeviceDescriptionKey, Object>, Shared> {
        unsafe { msg_send_id![self, deviceDescription] }.unwrap()
    }

    pub fn backingScaleFactor(&self) -> CGFloat {
        unsafe { msg_send![self, backingScaleFactor] }
    }
}
