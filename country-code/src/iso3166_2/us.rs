//! [ISO 3166-2:US](https://en.wikipedia.org/wiki/ISO_3166-2:US)

use crate::iso3166_1::alpha_2::CountryCode;

//
country_subdivision_code! {
    CountryCode, CountryCode::US;

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum CountrySubdivisionCode {
        AL,
        AK,
        AZ,
        AR,
        CA,
        CO,
        CT,
        DE,
        FL,
        GA,
        HI,
        ID,
        IL,
        IN,
        IA,
        KS,
        KY,
        LA,
        ME,
        MD,
        MA,
        MI,
        MN,
        MS,
        MO,
        MT,
        NE,
        NV,
        NH,
        NJ,
        NM,
        NY,
        NC,
        ND,
        OH,
        OK,
        OR,
        PA,
        RI,
        SC,
        SD,
        TN,
        TX,
        UT,
        VT,
        VA,
        WA,
        WV,
        WI,
        WY,
        DC,
        AS,
        GU,
        MP,
        PR,
        UM,
        VI,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use alloc::string::ToString as _;

    use csv::Reader;

    #[test]
    fn test_country_subdivision_code() {
        let mut rdr = Reader::from_reader(include_str!("../../tests/ISO_3166-2/US.csv").as_bytes());

        let mut n = 0;
        for record in rdr.records() {
            let record = record.unwrap();
            let code = &record[0];
            assert_eq!(
                code.parse::<CountrySubdivisionCode>().unwrap().to_string(),
                code
            );
            n += 1;
        }

        assert_eq!(CountrySubdivisionCode::COUNTRY_CODE, CountryCode::US);

        assert_eq!(CountrySubdivisionCode::VARS.len(), n);
    }
}
