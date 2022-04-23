// cargo expand --verbose --test macro_currency_code
currency_code::currency_code! {
    #[derive(Debug)]
    pub enum MyCurrencyCode {
        USD,
    }
}
