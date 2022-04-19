#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

//
#[macro_export]
macro_rules! continent_code {
    (
        $( #[$meta:meta] )*
        $pub:vis enum $name:ident {
            $(
                $variant:ident $( | $alias:ident )*,
            )+
        }
    ) => {
        $(#[$meta])*
        $pub enum $name {
            $(
                $variant,
            )+
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
            type Err = ::alloc::boxed::Box::<str>;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(
                        ::core::stringify!($variant) $( | ::core::stringify!($alias) )* => Ok(Self::$variant),
                    )+
                    s => Err(s.into())
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
                }
            }
        }

        //
        impl ::core::default::Default for $name {
            fn default() -> Self {
                Self::AS
            }
        }

        //
        impl_partial_eq_str! { str, $name }
        impl_partial_eq_str! { &'a str, $name }
        impl_partial_eq_str! { ::alloc::borrow::Cow<'a, str>, $name }
        impl_partial_eq_str! { ::alloc::string::String, $name }

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
continent_code! {
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum ContinentCode {
        AS,
        AF,
        NA,
        SA,
        AN,
        EU,
        OC | AU,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use alloc::string::ToString as _;

    use csv::ReaderBuilder;

    #[test]
    fn test_continent_code() {
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .from_reader(include_str!("../tests/Continent/Area_and_population.csv").as_bytes());

        let mut n = 0;
        for record in rdr.records().skip(2) {
            let record = record.unwrap();
            let name = &record[0];
            let code = match name {
                "Asia" => "AS",
                "Africa" => "AF",
                "North America" => "NA",
                "South America" => "SA",
                "Antarctica" => "AN",
                "Europe" => "EU",
                "Oceania" => "OC",
                s => panic!("{}", s),
            };
            assert_eq!(code.parse::<ContinentCode>().unwrap().to_string(), code);
            n += 1;
        }

        assert_eq!("AU".parse::<ContinentCode>().unwrap().to_string(), "OC");

        assert_eq!(ContinentCode::VARS.len(), n);
    }
}
