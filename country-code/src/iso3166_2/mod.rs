//! [ISO 3166-2 - Wikipedia](https://en.wikipedia.org/wiki/ISO_3166-2)

//
#[macro_export]
macro_rules! country_subdivision_code {
    (
        $country_code_ty:ty, $country_code_val:expr;

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
            pub const COUNTRY_CODE: $country_code_ty = $country_code_val;

            pub const VARS: &'static [$name] = &[
                $(
                    $name::$variant,
                )+
            ];
        }

        //
        impl ::core::str::FromStr for $name {
            type Err = ::alloc::boxed::Box::<str>;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                use ::alloc::boxed::Box;

                let country_code_s = s.chars().take_while(|x| x != &char::from('-') && x != &char::from('_'))
                                                .collect::<::alloc::string::String>();
                let country_code = country_code_s.parse::<$country_code_ty>()
                                                    .map_err(|_| Box::<str>::from(alloc::format!("Invalid country_code [{}]", country_code_s)))?;

                if country_code != Self::COUNTRY_CODE {
                    return Err(Box::<str>::from(alloc::format!("Invalid [{}]", s)))
                }

                let subdivision_code_s = if s.len() > country_code_s.len() + 1 {
                    &s[country_code_s.len() + 1..]
                } else {
                    return Err(Box::<str>::from(alloc::format!("Invalid [{}]", s)))
                };

                match subdivision_code_s {
                    $(
                        ::core::stringify!($variant) => Ok(Self::$variant),
                    )+
                    s => Ok(Self::Other(s.into()))
                }
            }
        }

        //
        impl ::core::fmt::Display for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                use alloc::string::ToString as _;

                match self {
                    $(
                        Self::$variant => ::core::write!(f, "{}-{}", $name::COUNTRY_CODE.to_string(), ::core::stringify!($variant)),
                    )+
                    Self::Other(s) => ::core::write!(f, "{}-{}", $name::COUNTRY_CODE.to_string(), s)
                }
            }
        }

        //
        impl ::core::default::Default for $name {
            fn default() -> Self {
                Self::Other("".into())
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

pub mod cn;
pub mod us;
