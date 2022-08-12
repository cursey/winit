use objc2::foundation::{MainThreadMarker, NSData, NSObject, NSPoint, NSString};
use objc2::rc::{Id, Shared};
use objc2::runtime::Sel;
use objc2::{extern_class, msg_send_id, ClassType};

use super::NSImage;

extern_class!(
    // TODO: Can this be mutable?
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub(crate) struct NSCursor;

    unsafe impl ClassType for NSCursor {
        type Superclass = NSObject;
    }
);

impl NSCursor {
    pub unsafe fn from_selector(sel: Sel, mtm: MainThreadMarker) -> Option<Id<Self, Shared>> {
        if unsafe { msg_send_bool![Self::class(), respondsToSelector: sel] } {
            Some(unsafe { Self::from_selector_unchecked(sel, mtm) })
        } else {
            warn!("Cursor `{:?}` appears to be invalid", sel);
            None
        }
    }

    pub unsafe fn from_selector_unchecked(sel: Sel, _mtm: MainThreadMarker) -> Id<Self, Shared> {
        unsafe { msg_send_id![Self::class(), performSelector: sel] }.unwrap()
    }

    pub fn new(image: &NSImage, hotSpot: NSPoint, _mtm: MainThreadMarker) -> Id<Self, Shared> {
        let this = unsafe { msg_send_id![Self::class(), alloc] };
        unsafe { msg_send_id![this, initWithImage: image, hotSpot: hotSpot] }.unwrap()
    }
}
