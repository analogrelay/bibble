macro_rules! integer_newtype_derive_one {
    ($newtyp: ty, $inttyp: ty) => {
        impl ::std::cmp::PartialEq<$inttyp> for $newtyp {
            fn eq(&self, other: &$inttyp) -> bool {
                self.0.get().eq(&(*other as u32))
            }
        }

        impl ::std::cmp::PartialOrd<$inttyp> for $newtyp {
            fn partial_cmp(&self, other: &$inttyp) -> Option<::std::cmp::Ordering> {
                self.0.get().partial_cmp(&(*other as u32))
            }
        }

        impl ::std::convert::From<$inttyp> for $newtyp {
            fn from(v: $inttyp) -> $newtyp {
                <$newtyp>::new(v as u32)
            }
        }
    };
}

macro_rules! integer_newtype {
    ($name: ident, $doc_summary: expr) => {
        #[doc = $doc_summary]
        #[doc = ""]
        #[doc = "This type wraps a `NonZeroU32`, so using it in a `Option<T>` has no effect on the size."]
        #[derive(Hash, Debug, PartialEq, PartialOrd, Eq, Ord, Copy, Clone)]
        pub struct $name(::std::num::NonZeroU32);

        impl $name {
            #[doc = "Constructs an instance of this type from the provided integer"]
            #[doc = ""]
            #[doc = "# Panics"]
            #[doc = "Panics if the provided integer is '0'"]
            pub fn new(val: u32) -> $name {
                let val = match ::std::num::NonZeroU32::new(val as u32) {
                    Some(c) => c,
                    None => panic!("must be a positive integer greater than 0")
                };
                $name(val)
            }

            /// Gets the underlying numeric value.
            pub const fn value(&self) -> u32 {
                self.0.get()
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "0x{:04X}", self.value())
            }
        }

        integer_newtype_derive_one!($name, usize);
        integer_newtype_derive_one!($name, u64);
        integer_newtype_derive_one!($name, u32);
        integer_newtype_derive_one!($name, u16);
        integer_newtype_derive_one!($name, isize);
        integer_newtype_derive_one!($name, i64);
        integer_newtype_derive_one!($name, i32);
        integer_newtype_derive_one!($name, i16);
    };
}
