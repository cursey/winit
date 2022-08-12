use objc2::foundation::NSObject;
use objc2::{extern_class, ClassType};

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub(crate) struct UIDevice;

    unsafe impl ClassType for UIDevice {
        type Superclass = NSObject;
    }
);
