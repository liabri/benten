#[macro_use]
mod shared;

use benten::{ Function, BentenResponse };

define_layout_test!("japanese");

#[test]
fn kana() {
    todo!()
}

#[test]
fn kana_level_1() {
    test_input(&[
        (21, BentenResponse::Commit("み".to_string())),
        (25, BentenResponse::Commit("く".to_string()))
    ])
}

#[test]
fn kana_level_2() {
    test_input(&[
        (50, BentenResponse::Undefined),
        (21, BentenResponse::Commit("=".to_string())),
        (25, BentenResponse::Commit("〼".to_string()))
    ])
}

#[test]
fn kana_level_3() {
    test_input(&[
        (108, BentenResponse::Undefined),
        (21, BentenResponse::Commit("+".to_string())),
        (25, BentenResponse::Commit("〒".to_string()))
    ])
}

#[test]
fn kana_level_4() {
    test_input(&[
        (50, BentenResponse::Undefined),
        (108, BentenResponse::Undefined),
        (21, BentenResponse::Commit("゠".to_string())),
        (25, BentenResponse::Undefined)
    ])
}

#[test]
fn kana_level_5() {
    test_input(&[
        (66, BentenResponse::Undefined),
        (21, BentenResponse::Commit("ミ".to_string())),
        (25, BentenResponse::Commit("ク".to_string()))
    ])
}

#[test]
fn kana_level_6() {
    test_input(&[
        (66, BentenResponse::Undefined),
        (50, BentenResponse::Undefined),
        (21, BentenResponse::Undefined),
        (25, BentenResponse::Undefined)
    ])
}

#[test]
fn cangjie_commit_key() {
    test_input(&[
        (47, BentenResponse::Function(Function::ChangeMethodTo("cangjie5".to_string()))),
        (18, BentenResponse::Suggest(String::from("日"))),
        (65, BentenResponse::Commit(String::from("日"))),

        //commit then change method to kana
        (25, BentenResponse::Commit(String::from("く"))),
        (65, BentenResponse::Undefined)
    ])
}

#[test]
fn cangjie_next_prev_key() {
    test_input(&[
        (47, BentenResponse::Function(Function::ChangeMethodTo("cangjie5".to_string()))),
        (18, BentenResponse::Suggest(String::from("日"))),
        (23, BentenResponse::Suggest(String::from("曰"))),
        (65, BentenResponse::Commit(String::from("曰")))
    ])
}

#[test]
fn cangjie_backspace_key() {
    test_input(&[
        (47, BentenResponse::Function(Function::ChangeMethodTo("cangjie5".to_string()))),
        (24, BentenResponse::Suggest(String::from("手"))),
        (24, BentenResponse::Suggest(String::from("抙"))),
        (22, BentenResponse::Suggest(String::from("手"))),
        (65, BentenResponse::Commit(String::from("手"))),

        (47, BentenResponse::Function(Function::ChangeMethodTo("cangjie5".to_string()))),
        (24, BentenResponse::Suggest(String::from("手"))),
        (24, BentenResponse::Suggest(String::from("抙"))),
        (22, BentenResponse::Suggest(String::from("手"))),
        (22, BentenResponse::Function(Function::ChangeMethodTo("kana".to_string()))),
        // //swap back to kana when empty
        // (25, BentenResponse::Commit(String::from("く"))),
    ])
}

#[test]
fn cangjie_on_no_result() {
    test_input(&[
        (47, BentenResponse::Function(Function::ChangeMethodTo("cangjie5".to_string()))),
        (24, BentenResponse::Suggest(String::from("手"))),
        (24, BentenResponse::Suggest(String::from("抙"))),
        (24, BentenResponse::Suggest(String::from("掱"))),

        (24, BentenResponse::Undefined),
        //on fail restart sequence
        (24, BentenResponse::Suggest(String::from("手"))),
        (24, BentenResponse::Suggest(String::from("抙"))),
        (24, BentenResponse::Suggest(String::from("掱"))),
        (65, BentenResponse::Commit(String::from("掱"))),
    ])
}