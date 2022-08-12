use objc2::foundation::{MainThreadMarker, NSArray, NSInteger, NSObject, NSUInteger};
use objc2::rc::{Id, Shared};
use objc2::runtime::{Bool, Object};
use objc2::{extern_class, msg_send, msg_send_id, ClassType};

use super::{NSEvent, NSResponder, NSWindow};

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub(crate) struct NSApplication;

    unsafe impl ClassType for NSApplication {
        #[inherits(NSObject)]
        type Superclass = NSResponder;
    }
);

// TODO
type NSApplicationPresentationOptions = NSUInteger;
type NSRequestUserAttentionType = NSUInteger;
type NSApplicationActivationPolicy = NSInteger;

impl NSApplication {
    /// This can only be called on the main thread since it may initialize
    /// the application and since it's parameters may be changed by the main
    /// thread at any time (hence it is only safe to access on the main thread).
    pub fn shared(_mtm: MainThreadMarker) -> Id<Self, Shared> {
        let app = unsafe { msg_send_id![Self::class(), sharedApplication] };
        // SAFETY: `sharedApplication` always initializes the app if it isn't already
        unsafe { app.unwrap_unchecked() }
    }

    pub fn currentEvent(&self) -> Option<Id<NSEvent, Shared>> {
        unsafe { msg_send_id![self, currentEvent] }
    }

    pub fn presentationOptions(&self) -> NSApplicationPresentationOptions {
        unsafe { msg_send![self, presentationOptions] }
    }

    pub fn windows(&self) -> Id<NSArray<NSWindow, Shared>, Shared> {
        unsafe { msg_send_id![self, windows] }.unwrap()
    }

    // TODO: NSApplicationDelegate
    pub unsafe fn setDelegate(&self, delegate: &Object) {
        unsafe { msg_send![self, setDelegate: delegate] }
    }

    pub unsafe fn setPresentationOptions(&self, options: NSApplicationPresentationOptions) {
        unsafe { msg_send![self, setPresentationOptions: options] }
    }

    pub fn hide(&self, sender: Option<&Object>) {
        unsafe { msg_send![self, hide: sender] }
    }

    pub fn hideOtherApplications(&self, sender: Option<&Object>) {
        unsafe { msg_send![self, hideOtherApplications: sender] }
    }

    pub fn stop(&self, sender: Option<&Object>) {
        unsafe { msg_send![self, stop: sender] }
    }

    pub fn activateIgnoringOtherApps(&self, ignore: bool) {
        unsafe { msg_send![self, activateIgnoringOtherApps: Bool::from(ignore)] }
    }

    pub unsafe fn requestUserAttention(&self, type_: NSRequestUserAttentionType) -> NSInteger {
        unsafe { msg_send![self, requestUserAttention: type_] }
    }

    pub unsafe fn setActivationPolicy(&self, policy: NSApplicationActivationPolicy) {
        unsafe { msg_send![self, setActivationPolicy: policy] }
    }

    // TODO: NSMenu
    pub unsafe fn setMainMenu(&self, menu: &Object) {
        unsafe { msg_send![self, setMainMenu: menu] }
    }

    pub unsafe fn run(&self) {
        unsafe { msg_send![self, run] }
    }
}
