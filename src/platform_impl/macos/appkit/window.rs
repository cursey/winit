use objc2::foundation::{CGFloat, NSObject, NSRect, NSSize};
use objc2::rc::{Id, Shared};
use objc2::runtime::{Bool, Object};
use objc2::{extern_class, msg_send, msg_send_id, ClassType};

use super::{NSResponder, NSScreen};

extern_class!(
    /// Main-Thread-Only!
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub(crate) struct NSWindow;

    unsafe impl ClassType for NSWindow {
        #[inherits(NSObject)]
        type Superclass = NSResponder;
    }
);

// Documented as "Main Thread Only", but:
// > Thread safe in that you can create and manage them on a secondary thread.
// <https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/CocoaFundamentals/AddingBehaviortoaCocoaProgram/AddingBehaviorCocoa.html#//apple_ref/doc/uid/TP40002974-CH5-SW47>
// <https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/Multithreading/ThreadSafetySummary/ThreadSafetySummary.html#//apple_ref/doc/uid/10000057i-CH12-123364>
//
// So could in theory be `Send`, and perhaps also `Sync` - but we would like
// interior mutability on windows, since that's just much easier, and in that
// case, they can't be!

impl NSWindow {
    pub fn frame(&self) -> NSRect {
        unsafe { msg_send![self, frame] }
    }

    pub fn backingScaleFactor(&self) -> CGFloat {
        unsafe { msg_send![self, backingScaleFactor] }
    }

    pub fn contentRectForFrameRect(&self, windowFrame: NSRect) -> NSRect {
        unsafe { msg_send![self, contentRectForFrameRect: windowFrame] }
    }

    pub fn screen(&self) -> Option<Id<NSScreen, Shared>> {
        unsafe { msg_send_id![self, screen] }
    }

    pub fn setContentSize(&self, contentSize: NSSize) {
        unsafe { msg_send![self, setContentSize: contentSize] }
    }

    pub fn setMinSize(&self, minSize: NSSize) {
        unsafe { msg_send![self, setMinSize: minSize] }
    }

    pub fn setMaxSize(&self, maxSize: NSSize) {
        unsafe { msg_send![self, setMaxSize: maxSize] }
    }

    pub fn setFrame_display(&self, frameRect: NSRect, flag: bool) {
        unsafe { msg_send![self, setFrame: frameRect, display: Bool::from(flag)] }
    }

    pub fn setMovable(&self, movable: bool) {
        unsafe { msg_send![self, setMovable: Bool::from(movable)] }
    }

    pub fn miniaturize(&self, sender: *mut Object) {
        unsafe { msg_send![self, miniaturize: sender] }
    }

    pub fn deminiaturize(&self, sender: *mut Object) {
        unsafe { msg_send![self, deminiaturize: sender] }
    }
}
