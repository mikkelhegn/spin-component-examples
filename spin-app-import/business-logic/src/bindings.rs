#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod component {
        #[allow(dead_code)]
        pub mod business_logic {
            #[allow(dead_code, clippy::all)]
            pub mod data_handler {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                /// The my-object record, aka. the object to pass between the two components.
                #[derive(Clone)]
                pub struct MyObject {
                    pub name: _rt::String,
                    pub steps: u32,
                    pub processed: Option<bool>,
                }
                impl ::core::fmt::Debug for MyObject {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("MyObject")
                            .field("name", &self.name)
                            .field("steps", &self.steps)
                            .field("processed", &self.processed)
                            .finish()
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_handle_data_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                    arg2: i32,
                    arg3: i32,
                    arg4: i32,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let result1 = T::handle_data(MyObject {
                        name: _rt::string_lift(bytes0),
                        steps: arg2 as u32,
                        processed: match arg3 {
                            0 => None,
                            1 => {
                                let e = _rt::bool_lift(arg4 as u8);
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        },
                    });
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let MyObject { name: name3, steps: steps3, processed: processed3 } = result1;
                    let vec4 = (name3.into_bytes()).into_boxed_slice();
                    let ptr4 = vec4.as_ptr().cast::<u8>();
                    let len4 = vec4.len();
                    ::core::mem::forget(vec4);
                    *ptr2.add(4).cast::<usize>() = len4;
                    *ptr2.add(0).cast::<*mut u8>() = ptr4.cast_mut();
                    *ptr2.add(8).cast::<i32>() = _rt::as_i32(steps3);
                    match processed3 {
                        Some(e) => {
                            *ptr2.add(12).cast::<u8>() = (1i32) as u8;
                            *ptr2.add(13).cast::<u8>() = (match e {
                                true => 1,
                                false => 0,
                            }) as u8;
                        }
                        None => {
                            *ptr2.add(12).cast::<u8>() = (0i32) as u8;
                        }
                    };
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_handle_data<T: Guest>(arg0: *mut u8) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    _rt::cabi_dealloc(l0, l1, 1);
                }
                pub trait Guest {
                    /// The function to pass on the my-object to.
                    fn handle_data(key: MyObject) -> MyObject;
                }
                #[doc(hidden)]
                macro_rules! __export_component_business_logic_data_handler_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "component:business-logic/data-handler#handle-data"] unsafe
                        extern "C" fn export_handle_data(arg0 : * mut u8, arg1 : usize,
                        arg2 : i32, arg3 : i32, arg4 : i32,) -> * mut u8 {
                        $($path_to_types)*:: _export_handle_data_cabi::<$ty > (arg0,
                        arg1, arg2, arg3, arg4) } #[export_name =
                        "cabi_post_component:business-logic/data-handler#handle-data"]
                        unsafe extern "C" fn _post_return_handle_data(arg0 : * mut u8,) {
                        $($path_to_types)*:: __post_return_handle_data::<$ty > (arg0) }
                        };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_component_business_logic_data_handler_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 16]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 16],
                );
            }
        }
    }
}
mod _rt {
    pub use alloc_crate::string::String;
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub use alloc_crate::vec::Vec;
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    pub unsafe fn bool_lift(val: u8) -> bool {
        if cfg!(debug_assertions) {
            match val {
                0 => false,
                1 => true,
                _ => panic!("invalid bool discriminant"),
            }
        } else {
            val != 0
        }
    }
    pub unsafe fn invalid_enum_discriminant<T>() -> T {
        if cfg!(debug_assertions) {
            panic!("invalid enum discriminant")
        } else {
            core::hint::unreachable_unchecked()
        }
    }
    pub fn as_i32<T: AsI32>(t: T) -> i32 {
        t.as_i32()
    }
    pub trait AsI32 {
        fn as_i32(self) -> i32;
    }
    impl<'a, T: Copy + AsI32> AsI32 for &'a T {
        fn as_i32(self) -> i32 {
            (*self).as_i32()
        }
    }
    impl AsI32 for i32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for char {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for usize {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
    extern crate alloc as alloc_crate;
    pub use alloc_crate::alloc;
}
/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]
macro_rules! __export_business_logic_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::component::business_logic::data_handler::__export_component_business_logic_data_handler_cabi!($ty
        with_types_in $($path_to_types_root)*::
        exports::component::business_logic::data_handler);
    };
}
#[doc(inline)]
pub(crate) use __export_business_logic_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.31.0:component:business-logic:business-logic:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 299] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xa6\x01\x01A\x02\x01\
A\x02\x01B\x05\x01k\x7f\x01r\x03\x04names\x05stepsy\x09processed\0\x04\0\x09my-o\
bject\x03\0\x01\x01@\x01\x03key\x02\0\x02\x04\0\x0bhandle-data\x01\x03\x04\x01%c\
omponent:business-logic/data-handler\x05\0\x04\x01'component:business-logic/busi\
ness-logic\x04\0\x0b\x14\x01\0\x0ebusiness-logic\x03\0\0\0G\x09producers\x01\x0c\
processed-by\x02\x0dwit-component\x070.216.0\x10wit-bindgen-rust\x060.31.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
