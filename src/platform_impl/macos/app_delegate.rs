use cocoa::appkit::NSApplicationActivationPolicy;
use objc2::foundation::NSObject;
use objc2::rc::{Id, Shared};
use objc2::runtime::{Bool, Object};
use objc2::{declare_class, ClassType};

use super::app_state::AppState;

declare_class!(
    #[derive(Debug)]
    pub(super) struct WinitApplicationDelegate {
        activation_policy: NSApplicationActivationPolicy,
        default_menu: Bool,
    }

    unsafe impl ClassType for WinitApplicationDelegate {
        type Superclass = NSObject;
    }

    unsafe impl WinitApplicationDelegate {
        #[sel(initWithActivationPolicy:defaultMenu:)]
        fn init(
            &mut self,
            activation_policy: NSApplicationActivationPolicy,
            default_menu: Bool,
        ) -> *mut Self {
            let this: *mut Self = unsafe { msg_send![super(self, NSObject::class()), init] };
            if let Some(this) = unsafe { this.as_mut() } {
                *this.activation_policy = activation_policy;
                *this.default_menu = default_menu;
            }
            this
        }

        #[sel(applicationDidFinishLaunching:)]
        fn did_finish_launching(&self, _sender: *const Object) {
            trace_scope!("applicationDidFinishLaunching:");
            AppState::launched(*self.activation_policy, self.default_menu.as_bool());
        }

        #[sel(applicationWillTerminate:)]
        fn will_terminate(&self, _sender: *const Object) {
            trace_scope!("applicationWillTerminate:");
            // TODO: Notify every window that it will be destroyed, like done in iOS?
            AppState::exit();
        }
    }
);

pub(super) type ApplicationDelegate = WinitApplicationDelegate;

impl ApplicationDelegate {
    pub(super) fn new(
        activation_policy: NSApplicationActivationPolicy,
        default_menu: bool,
    ) -> Id<Self, Shared> {
        unsafe {
            msg_send_id![
                msg_send_id![Self::class(), alloc],
                initWithActivationPolicy: activation_policy,
                defaultMenu: Bool::from(default_menu),
            ]
            .unwrap()
        }
    }
}
