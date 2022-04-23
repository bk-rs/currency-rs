#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

//
#[macro_export]
macro_rules! currency_code {
    (
        $( #[$meta:meta] )*
        $pub:vis enum $name:ident {
            $(
                $variant:ident,
            )+
        }
    ) => {
        $(#[$meta])*
        $pub enum $name {
            $(
                $variant,
            )+
            Other(::alloc::boxed::Box<str>),
        }

        //
        impl $name {
            pub const VARS: &'static [$name] = &[
                $(
                    $name::$variant,
                )+
            ];
        }

        //
        impl ::core::str::FromStr for $name {
            type Err = ::alloc::boxed::Box<str>;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(
                        ::core::stringify!($variant) => Ok(Self::$variant),
                    )+
                    s if s.len() == 3 => Ok(Self::Other(s.into())),
                    s => Err(::alloc::boxed::Box::<str>::from(alloc::format!("Invalid [{}]", s)))
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
                ::alloc::format!("{}", self) == ::alloc::format!("{}", other)
            }
        }

        //
        impl_partial_eq_str! { str, $name }
        impl_partial_eq_str! { &'a str, $name }
        impl_partial_eq_str! { ::alloc::borrow::Cow<'a, str>, $name }
        impl_partial_eq_str! { ::alloc::string::String, $name }

        //
        #[cfg(feature = "std")]
        impl ::std::hash::Hash for $name {
            fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
                ::alloc::format!("{}", self).hash(state);
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

                let s = ::alloc::boxed::Box::<str>::deserialize(deserializer)?;
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
                use ::alloc::string::ToString as _;

                self.to_string().serialize(serializer)
            }
        }
    };
}

//
#[macro_export]
macro_rules! impl_partial_eq_str {
    ($lhs:ty, $rhs: ty) => {
        #[allow(unused_lifetimes)]
        impl<'a> ::core::cmp::PartialEq<$lhs> for $rhs {
            fn eq(&self, other: &$lhs) -> bool {
                ::core::cmp::PartialEq::eq(&::alloc::format!("{}", self)[..], &other[..])
            }
        }
    };
}

//
pub mod iso4217;

pub use iso4217::CurrencyCode;
