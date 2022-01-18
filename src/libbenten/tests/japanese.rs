#[macro_use]
mod shared;

use benten::BentenResponse;

define_layout_test!("japanese");

#[test]
fn kana() {
    test_input(&[
        (45, BentenResponse::Commit("い".to_string())),
        (25, BentenResponse::Commit("く".to_string()))
    ])
    // let mut engine = BentenEngine::new(BentenConfig { id: "japanese".to_string() });

    // //kana
    // assert_eq!(engine.on_key_press(45), BentenResponse::Commit(String::from("い")));
    // assert_eq!(engine.on_key_press(25), BentenResponse::Commit(String::from("く")));

    // //han
    // assert_eq!(engine.on_key_press(47), BentenResponse::Empty);
    // assert_eq!(engine.on_key_press(18), BentenResponse::Suggest(String::from("日")));
    // assert_eq!(engine.on_key_press(65), BentenResponse::Commit(String::from("日")));

    // //back to kana
    // assert_eq!(engine.on_key_press(25), BentenResponse::Commit(String::from("く")));
    // assert_eq!(engine.on_key_press(65), BentenResponse::Null);

    // //han + next
    // assert_eq!(engine.on_key_press(47), BentenResponse::Empty);
    // assert_eq!(engine.on_key_press(18), BentenResponse::Suggest(String::from("日")));
    // assert_eq!(engine.on_key_press(23), BentenResponse::Suggest(String::from("曰")));
    // assert_eq!(engine.on_key_press(65), BentenResponse::Commit(String::from("曰")));
}