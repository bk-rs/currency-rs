//! [ISO 4217 - Wikipedia](https://en.wikipedia.org/wiki/ISO_4217)

use crate::currency_code;

currency_code! {
    #[derive(Debug, Clone, Eq)]
    pub enum CurrencyCode {
        AED,
        AFN,
        ALL,
        AMD,
        ANG,
        AOA,
        ARS,
        AUD,
        AWG,
        AZN,
        BAM,
        BBD,
        BDT,
        BGN,
        BHD,
        BIF,
        BMD,
        BND,
        BOB,
        BOV,
        BRL,
        BSD,
        BTN,
        BWP,
        BYN,
        BZD,
        CAD,
        CDF,
        CHE,
        CHF,
        CHW,
        CLF,
        CLP,
        CNY,
        COP,
        COU,
        CRC,
        CUC,
        CUP,
        CVE,
        CZK,
        DJF,
        DKK,
        DOP,
        DZD,
        EGP,
        ERN,
        ETB,
        EUR,
        FJD,
        FKP,
        GBP,
        GEL,
        GHS,
        GIP,
        GMD,
        GNF,
        GTQ,
        GYD,
        HKD,
        HNL,
        HRK,
        HTG,
        HUF,
        IDR,
        ILS,
        INR,
        IQD,
        IRR,
        ISK,
        JMD,
        JOD,
        JPY,
        KES,
        KGS,
        KHR,
        KMF,
        KPW,
        KRW,
        KWD,
        KYD,
        KZT,
        LAK,
        LBP,
        LKR,
        LRD,
        LSL,
        LYD,
        MAD,
        MDL,
        MGA,
        MKD,
        MMK,
        MNT,
        MOP,
        MRU,
        MUR,
        MVR,
        MWK,
        MXN,
        MXV,
        MYR,
        MZN,
        NAD,
        NGN,
        NIO,
        NOK,
        NPR,
        NZD,
        OMR,
        PAB,
        PEN,
        PGK,
        PHP,
        PKR,
        PLN,
        PYG,
        QAR,
        RON,
        RSD,
        RUB,
        RWF,
        SAR,
        SBD,
        SCR,
        SDG,
        SEK,
        SGD,
        SHP,
        SLL,
        SOS,
        SRD,
        SSP,
        STN,
        SVC,
        SYP,
        SZL,
        THB,
        TJS,
        TMT,
        TND,
        TOP,
        TRY,
        TTD,
        TWD,
        TZS,
        UAH,
        UGX,
        USD,
        USN,
        UYI,
        UYU,
        UYW,
        UZS,
        VED,
        VES,
        VND,
        VUV,
        WST,
        XAF,
        XAG,
        XAU,
        XBA,
        XBB,
        XBC,
        XBD,
        XCD,
        XDR,
        XOF,
        XPD,
        XPF,
        XPT,
        XSU,
        XTS,
        XUA,
        XXX,
        YER,
        ZAR,
        ZMW,
        ZWL,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use alloc::string::ToString as _;

    use csv::Reader;

    #[test]
    fn test_currency_code() {
        // Wikipedia
        let mut rdr =
            Reader::from_reader(include_str!("../tests/ISO_4217/Active_codes.csv").as_bytes());

        let mut n = 0;
        for record in rdr.records() {
            let record = record.unwrap();
            let code = &record[0];
            let code = &code[0..3];
            assert_eq!(code.parse::<CurrencyCode>().unwrap().to_string(), code);
            n += 1;
        }

        assert_eq!(CurrencyCode::VARS.len(), n);

        // FromStr
        assert_eq!(
            "ZZZ".parse::<CurrencyCode>().unwrap(),
            CurrencyCode::Other("ZZZ".into())
        );
        assert_eq!(
            "x".parse::<CurrencyCode>().err().unwrap(),
            crate::error::ParseError::Invalid("x".into())
        );
        #[cfg(feature = "std")]
        {
            std::println!("{}", "x".parse::<CurrencyCode>().err().unwrap());
        }

        // PartialEq
        assert_eq!(CurrencyCode::USD, CurrencyCode::USD);
        assert_eq!(CurrencyCode::USD, CurrencyCode::Other("USD".into()));
        assert_eq!(CurrencyCode::USD, "USD");

        #[cfg(feature = "std")]
        {
            // Hash
            let mut h = std::collections::HashSet::new();
            h.insert(CurrencyCode::USD);
            h.insert(CurrencyCode::Other("USD".into()));
            assert_eq!(h.len(), 1);
        }

        #[cfg(feature = "serde")]
        {
            #[derive(serde::Serialize, serde::Deserialize)]
            struct Foo {
                code: CurrencyCode,
            }

            assert_eq!(
                serde_json::from_str::<Foo>(r#"{"code":"USD"}"#)
                    .unwrap()
                    .code,
                CurrencyCode::USD
            );
            assert_eq!(
                serde_json::to_string(&Foo {
                    code: CurrencyCode::USD
                })
                .unwrap(),
                r#"{"code":"USD"}"#
            );
        }
    }
}
