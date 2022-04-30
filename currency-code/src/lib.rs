#![cfg_attr(not(feature = "std"), no_std)]

pub extern crate alloc;

//
#[macro_export]
macro_rules! currency_code {
    (
        $( #[$meta:meta] )*
        $pub:vis enum $name:ident {
            $(
                $( #[$variant_meta:meta] )*
                $variant:ident,
            )+
        }
    ) => {
        $( #[$meta] )*
        $pub enum $name {
            $(
                $( #[$variant_meta] )*
                $variant,
            )+
            Other($crate::alloc::boxed::Box<str>),
        }

        //
        impl $name {
            pub const VARS: &'static [$name] = &[
                $(
                    Self::$variant,
                )+
            ];
        }

        //
        impl ::core::str::FromStr for $name {
            type Err = $crate::error::ParseError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(
                        ::core::stringify!($variant) => Ok(Self::$variant),
                    )+
                    s if s.len() == 3 => Ok(Self::Other(s.into())),
                    s => Err($crate::error::ParseError::Invalid(s.into()))
                }
            }
        }

        //
        impl ::core::fmt::Display for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                match self {
                    $(
                        Self::$variant => ::core::write!(f, "{}", ::core::stringify!($variant)),
                    )+
                    Self::Other(s) => ::core::write!(f, "{}", s)
                }
            }
        }

        //
        impl ::core::default::Default for $name {
            fn default() -> Self {
                Self::Other(Default::default())
            }
        }

        //
        impl ::core::cmp::PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                $crate::alloc::format!("{}", self) == $crate::alloc::format!("{}", other)
            }
        }

        impl ::core::cmp::Eq for $name {
        }

        //
        impl_macros::impl_partial_eq_str_for_display! { str, $name }
        impl_macros::impl_partial_eq_str_for_display! { &'a str, $name }
        impl_macros::impl_partial_eq_str_for_display! { $crate::alloc::borrow::Cow<'a, str>, $name }
        impl_macros::impl_partial_eq_str_for_display! { $crate::alloc::string::String, $name }

        //
        #[cfg(feature = "std")]
        impl ::std::hash::Hash for $name {
            fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
                $crate::alloc::format!("{}", self).hash(state);
            }
        }

        //
        #[cfg(feature = "serde")]
        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                use ::core::str::FromStr as _;

                let s = $crate::alloc::boxed::Box::<str>::deserialize(deserializer)?;
                Self::from_str(&s).map_err(::serde::de::Error::custom)
            }
        }

        //
        #[cfg(feature = "serde")]
        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                use $crate::alloc::string::ToString as _;

                self.to_string().serialize(serializer)
            }
        }
    };
}

//
pub mod error;

//
pub mod iso4217;

pub use iso4217::CurrencyCode;
