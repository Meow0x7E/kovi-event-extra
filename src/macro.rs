#[macro_export]
macro_rules! _unable_convert {
    ($event:ident, $because:ident) => {
        Error::UnableConvert {
            source_event: String::from(stringify!($event)),
            target_event: String::from(Self::struct_name()),
            because: $because.to_string()
        }
    };
}

#[macro_export]
macro_rules! is_none_and_return {
    ($json:ident, $literal:literal, $ident:ident) => {{
        let it = $json.get($literal).and_then(|it| it.$ident());
        if it.is_none() {
            let because = rust_i18n::t!(r#"global.is_none"#, it => $literal);
            unable_convert!(because);
        }
        it.unwrap()
    }};
}
