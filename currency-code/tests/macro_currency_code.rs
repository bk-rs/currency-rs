// cargo expand --verbose --all-features --test macro_currency_code

currency_code::currency_code! {
    #[derive(Debug, Clone)]
    pub enum MyCurrencyCode {
        USD,
    }
}
