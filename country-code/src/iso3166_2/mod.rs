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

                let country_code_s = s.chars().take_while(|x| x != &'-' && x != &'_')
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
                match self {
                    $(
                        Self::$variant => ::core::write!(f, "{}-{}", $name::COUNTRY_CODE, ::core::stringify!($variant)),
                    )+
                    Self::Other(s) => ::core::write!(f, "{}-{}", $name::COUNTRY_CODE, s)
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

//
pub mod cn;
pub mod us;

//
//
//
use crate::iso3166_1::alpha_2::CountryCode;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SubdivisionCode {
    CN(cn::CountrySubdivisionCode),
    US(us::CountrySubdivisionCode),
    Other(CountryCode, ::alloc::boxed::Box<str>),
}

//
impl ::core::str::FromStr for SubdivisionCode {
    type Err = ::alloc::boxed::Box<str>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        #[allow(unused_imports)]
        use ::alloc::boxed::Box;

        let country_code_s = s
            .chars()
            .take_while(|x| x != &'-' && x != &'_')
            .collect::<::alloc::string::String>();
        let country_code = country_code_s.parse::<CountryCode>().map_err(|_| {
            Box::<str>::from(alloc::format!("Invalid country_code [{}]", country_code_s))
        })?;

        match country_code {
            CountryCode::CN => {
                let subdivision = s.parse::<cn::CountrySubdivisionCode>()?;
                Ok(Self::CN(subdivision))
            }
            CountryCode::US => {
                let subdivision = s.parse::<us::CountrySubdivisionCode>()?;
                Ok(Self::US(subdivision))
            }
            country => {
                let subdivision_code_s = if s.len() > country_code_s.len() + 1 {
                    &s[country_code_s.len() + 1..]
                } else {
                    return Err(Box::<str>::from(alloc::format!("Invalid [{}]", s)));
                };

                Ok(Self::Other(country, subdivision_code_s.into()))
            }
        }
    }
}

//
impl ::core::fmt::Display for SubdivisionCode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        match self {
            Self::CN(subdivision) => ::core::write!(f, "{}", subdivision),
            Self::US(subdivision) => ::core::write!(f, "{}", subdivision),
            Self::Other(country, s) => ::core::write!(f, "{}-{}", country, s),
        }
    }
}

//
#[cfg(feature = "serde")]
impl<'de> ::serde::Deserialize<'de> for SubdivisionCode {
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
impl ::serde::Serialize for SubdivisionCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        use ::alloc::string::ToString as _;

        self.to_string().serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use alloc::string::ToString as _;

    #[test]
    fn test_subdivision_code() {
        assert_eq!(
            SubdivisionCode::US(us::CountrySubdivisionCode::NY).to_string(),
            "US-NY"
        );
        assert_eq!(
            "US-NY".parse::<SubdivisionCode>().unwrap(),
            SubdivisionCode::US(us::CountrySubdivisionCode::NY)
        );

        assert_eq!(
            SubdivisionCode::Other(CountryCode::ZW, "BU".into()).to_string(),
            "ZW-BU"
        );
        assert_eq!(
            "ZW-BU".parse::<SubdivisionCode>().unwrap(),
            SubdivisionCode::Other(CountryCode::ZW, "BU".into())
        );
    }
}
