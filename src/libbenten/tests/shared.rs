use benten::{ BentenEngine, BentenResponse };

#[track_caller]
pub fn test_input_impl(mut engine: BentenEngine, keys: &[(u16, BentenResponse)]) {
    for (key, response) in keys.iter() {
        let rep = engine.on_key_press(key.to_owned());
        eprintln!("Key: {:?}, Rep: {:?}", key, rep);
        assert_eq!(&rep, response);
    }
}

#[allow(unused_macros)]
macro_rules! define_layout_test {
    ($layout:expr) => {
        use shared::test_input_impl;
        use benten::{ BentenEngine, BentenConfig };

        #[allow(dead_code)]
        #[track_caller]
        fn test_input(keys: &[(u16, BentenResponse)]) {
            let context = BentenEngine::new(BentenConfig { 
                id: $layout.to_string(),
                ..BentenConfig::default()
            });
            test_input_impl(context, keys);
        }
    };

    ($layout:expr) => {
        define_layout_test!($layout);
    };
}