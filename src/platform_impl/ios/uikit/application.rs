use objc2::foundation::NSObject;
use objc2::{extern_class, ClassType};

use super::UIResponder;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub(crate) struct UIApplication;

    unsafe impl ClassType for UIApplication {
        #[inherits(NSObject)]
        type Superclass = UIResponder;
    }
);
