#[doc(hidden)]
#[macro_export]
/// Implements ButtonExt
macro_rules! impl_button_ext {
    ($name: ident, $flname: ident) => {
        paste::paste! {
            unsafe impl ButtonExt for $name {
                fn shortcut(&self) -> $crate::enums::Shortcut {
                    unsafe {
                        assert!(!self.was_deleted());
                        std::mem::transmute([<$flname _shortcut>](self.inner))
                    }
                }

                fn set_shortcut(&mut self, shortcut: $crate::enums::Shortcut) {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _set_shortcut>](self.inner, shortcut.bits() as i32)
                    }
                }

                fn clear(&mut self) {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _clear>](self.inner);
                    }
                }

                fn is_set(&self) -> bool {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _value>](self.inner) != 0
                    }
                }

                fn set(&mut self, flag: bool) {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _set_value>](self.inner, flag as i32)
                    }
                }

                fn value(&self) -> bool {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _value>](self.inner) != 0
                    }
                }

                fn set_value(&mut self, flag: bool) {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _set_value>](self.inner, flag as i32)
                    }
                }

                fn set_down_frame(&mut self, f: $crate::enums::FrameType) {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _set_down_box>](self.inner, f as i32) }
                }

                fn down_frame(&self) -> $crate::enums::FrameType {
                    assert!(!self.was_deleted());
                    unsafe { std::mem::transmute([<$flname _down_box>](self.inner)) }
                }
            }
        }
    };
}

pub use impl_button_ext;
