use std::ffi::c_void;

use objc2::foundation::{MainThreadMarker, NSInteger, NSObject, NSPoint, NSRect};
use objc2::rc::{Id, Shared};
use objc2::runtime::{Bool, Object};
use objc2::{extern_class, msg_send, msg_send_bool, msg_send_id, ClassType};

use super::{NSCursor, NSResponder, NSTextInputContext, NSWindow};

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub(crate) struct NSView;

    unsafe impl ClassType for NSView {
        #[inherits(NSObject)]
        type Superclass = NSResponder;
    }
);

// Documented as "Main Thread Only".
// > generally thread safe, although operations on views such as creating,
// > resizing, and moving should happen on the main thread.
// <https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/CocoaFundamentals/AddingBehaviortoaCocoaProgram/AddingBehaviorCocoa.html#//apple_ref/doc/uid/TP40002974-CH5-SW47>
//
// > If you want to use a thread to draw to a view, bracket all drawing code
// > between the lockFocusIfCanDraw and unlockFocus methods of NSView.
// <https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/Multithreading/ThreadSafetySummary/ThreadSafetySummary.html#//apple_ref/doc/uid/10000057i-CH12-123351-BBCFIIEB>

type NSTrackingRectTag = NSInteger;

impl NSView {
    pub fn frame(&self) -> NSRect {
        unsafe { msg_send![self, frame] }
    }

    pub fn bounds(&self) -> NSRect {
        unsafe { msg_send![self, bounds] }
    }

    pub fn inputContext(&self, _mtm: MainThreadMarker) -> Option<Id<NSTextInputContext, Shared>> {
        unsafe { msg_send_id![self, inputContext] }
    }

    pub fn window(&self) -> Option<Id<NSWindow, Shared>> {
        unsafe { msg_send_id![self, window] }
    }

    pub fn visibleRect(&self) -> NSRect {
        unsafe { msg_send![self, visibleRect] }
    }

    pub fn hasMarkedText(&self) -> bool {
        unsafe { msg_send_bool![self, hasMarkedText] }
    }

    pub fn setWantsBestResolutionOpenGLSurface(&mut self, value: bool) {
        unsafe { msg_send![self, setWantsBestResolutionOpenGLSurface: Bool::from(value)] }
    }

    pub fn setWantsLayer(&mut self, wants_layer: bool) {
        unsafe { msg_send![self, setWantsLayer: Bool::from(wants_layer)] }
    }

    pub fn convertPoint_fromView(&self, point: NSPoint, view: &NSView) -> NSPoint {
        unsafe { msg_send![self, convertPoint: point, fromView: view] }
    }

    pub fn setPostsFrameChangedNotifications(&mut self, value: bool) {
        unsafe { msg_send![self, setPostsFrameChangedNotifications: value] }
    }

    pub unsafe fn removeTrackingRect(&self, tag: NSTrackingRectTag) {
        unsafe { msg_send![self, removeTrackingRect: tag] }
    }

    pub unsafe fn addTrackingRect(
        &self,
        rect: NSRect,
        owner: &Object,
        user_data: *mut c_void,
        assume_inside: bool,
    ) -> NSTrackingRectTag {
        unsafe {
            msg_send![
                self,
                addTrackingRect: rect,
                owner: owner,
                userData: user_data,
                assumeInside: Bool::from(assume_inside),
            ]
        }
    }

    pub unsafe fn addCursorRect(&self, rect: NSRect, cursor: &NSCursor) {
        // NSCursor safe to take by shared reference since it is already immutable
        unsafe { msg_send![self, addCursorRect: rect, cursor: cursor] }
    }
}
