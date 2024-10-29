// Generated by `wit-bindgen` 0.32.0. DO NOT EDIT!
// Options used:
//   * additional derives ["serde::Serialize", "serde::Deserialize", "Hash", "Clone", "PartialEq", "Eq"]
#[allow(dead_code)]
pub mod component {
  #[allow(dead_code)]
  pub mod business_logic {
    #[allow(dead_code, clippy::all)]
    pub mod data_handler {
      #[used]
      #[doc(hidden)]
      static __FORCE_SECTION_REF: fn() =
      super::super::super::__link_custom_section_describing_imports;
      
      use super::super::super::_rt;
      #[derive(Clone, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
      pub struct MyObject {
        pub name: _rt::String,
        pub steps: u32,
        pub processed: Option<bool>,
      }
      impl ::core::fmt::Debug for MyObject {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
          f.debug_struct("MyObject").field("name", &self.name).field("steps", &self.steps).field("processed", &self.processed).finish()
        }
      }
      #[allow(unused_unsafe, clippy::all)]
      pub fn handle_data(key: &MyObject,) -> MyObject{
        unsafe {
          #[repr(align(4))]
          struct RetArea([::core::mem::MaybeUninit::<u8>; 16]);
          let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 16]);
          let MyObject{ name:name0, steps:steps0, processed:processed0, } = key;
          let vec1 = name0;
          let ptr1 = vec1.as_ptr().cast::<u8>();
          let len1 = vec1.len();
          let (result2_0,result2_1,) = match processed0 {
            Some(e) => (1i32, match e { true => 1, false => 0 }),
            None => {
              (0i32, 0i32)
            },
          };let ptr3 = ret_area.0.as_mut_ptr().cast::<u8>();
          #[cfg(target_arch = "wasm32")]
          #[link(wasm_import_module = "component:business-logic/data-handler")]
          extern "C" {
            #[link_name = "handle-data"]
            fn wit_import(_: *mut u8, _: usize, _: i32, _: i32, _: i32, _: *mut u8, );
          }

          #[cfg(not(target_arch = "wasm32"))]
          fn wit_import(_: *mut u8, _: usize, _: i32, _: i32, _: i32, _: *mut u8, ){ unreachable!() }
          wit_import(ptr1.cast_mut(), len1, _rt::as_i32(steps0), result2_0, result2_1, ptr3);
          let l4 = *ptr3.add(0).cast::<*mut u8>();
          let l5 = *ptr3.add(4).cast::<usize>();
          let len6 = l5;
          let bytes6 = _rt::Vec::from_raw_parts(l4.cast(), len6, len6);
          let l7 = *ptr3.add(8).cast::<i32>();
          let l8 = i32::from(*ptr3.add(12).cast::<u8>());
          MyObject{
            name: _rt::string_lift(bytes6),
            steps: l7 as u32,
            processed: match l8 {
              0 => None,
              1 => {
                let e = {
                  let l9 = i32::from(*ptr3.add(13).cast::<u8>());

                  _rt::bool_lift(l9 as u8)
                };
                Some(e)
              }
              _ => _rt::invalid_enum_discriminant(),
            },
          }
        }
      }

    }

  }
}
mod _rt {
  pub use alloc_crate::string::String;

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
  extern crate alloc as alloc_crate;
}

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.32.0:spin-deps:deps@0.1.0:deps:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 275] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\x98\x01\x01A\x02\x01\
A\x02\x01B\x05\x01k\x7f\x01r\x03\x04names\x05stepsy\x09processed\0\x04\0\x09my-o\
bject\x03\0\x01\x01@\x01\x03key\x02\0\x02\x04\0\x0bhandle-data\x01\x03\x03\0%com\
ponent:business-logic/data-handler\x05\0\x04\0\x19spin-deps:deps/deps@0.1.0\x04\0\
\x0b\x0a\x01\0\x04deps\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-co\
mponent\x070.217.0\x10wit-bindgen-rust\x060.32.0";

#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
  wit_bindgen::rt::maybe_link_cabi_realloc();
}
