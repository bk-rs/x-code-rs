//! [ISO 3166-2:CN](https://en.wikipedia.org/wiki/ISO_3166-2:CN)

use crate::iso3166_1::alpha_2::CountryCode;

//
country_subdivision_code! {
    CountryCode, CountryCode::CN;

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum CountrySubdivisionCode {
        AH,
        BJ,
        CQ,
        FJ,
        GD,
        GS,
        GX,
        GZ,
        HA,
        HB,
        HE,
        HI,
        HK,
        HL,
        HN,
        JL,
        JS,
        JX,
        LN,
        MO,
        NM,
        NX,
        QH,
        SC,
        SD,
        SH,
        SN,
        SX,
        TJ,
        TW,
        XJ,
        XZ,
        YN,
        ZJ,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use alloc::string::ToString as _;

    use csv::Reader;

    #[test]
    fn test_country_subdivision_code() {
        let mut rdr = Reader::from_reader(include_str!("../../tests/ISO_3166-2/CN.csv").as_bytes());

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

        assert_eq!(CountrySubdivisionCode::COUNTRY_CODE, CountryCode::CN);

        assert_eq!(CountrySubdivisionCode::VARS.len(), n);
    }
}
