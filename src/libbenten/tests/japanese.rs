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
}

#[test]
fn han() {
    test_input(&[
        (47, BentenResponse::Empty),
        (18, BentenResponse::Suggest(String::from("日"))),
        (65, BentenResponse::Commit(String::from("日"))),
        (25, BentenResponse::Commit(String::from("く"))),
        (65, BentenResponse::Undefined)
    ])
}

#[test]
fn cangjie_next_prev_key() {
    test_input(&[
        (47, BentenResponse::Empty),
        (18, BentenResponse::Suggest(String::from("日"))),
        (23, BentenResponse::Suggest(String::from("曰"))),
        (65, BentenResponse::Commit(String::from("曰")))
    ])
}

#[test]
fn cangjie_backspace_key() {
    test_input(&[
        (47, BentenResponse::Empty),
        (24, BentenResponse::Suggest(String::from("手"))),
        (24, BentenResponse::Suggest(String::from("抙"))),
        (22, BentenResponse::Suggest(String::from("手"))),
        (65, BentenResponse::Commit(String::from("手")))
    ])
}