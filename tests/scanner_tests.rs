use pretty_assertions::assert_eq;
use std::fs;

fn test(name: &str) {
    let filename = format!("{name}.cool");
    let input_path = format!("tests/data/{filename}");
    let output_path = format!("{input_path}.out");
    let input = fs::read_to_string(&input_path).unwrap();
    let reference_output = fs::read_to_string(&output_path).unwrap();
    let header = format!("#name \"{}\"", &filename);
    let lines = scanner::tokenize(&input)
        .iter()
        .map(|token| format!("{}", token))
        .collect::<Vec<_>>()
        .join("\n");
    let output = format!(
        "{}\n{}{}",
        header,
        lines,
        if lines.len() > 0 { "\n" } else { "" }
    );
    assert_eq!(reference_output, output);
}

#[test]
fn all_else_true() {
    test("all_else_true.cl");
}

#[test]
fn arith() {
    test("arith");
}

#[test]
fn atoi() {
    test("atoi");
}

#[test]
fn backslash() {
    test("backslash");
}

#[test]
fn backslash2() {
    test("backslash2");
}

#[test]
fn badidentifiers() {
    test("badidentifiers");
}

#[test]
fn badkeywords() {
    test("badkeywords");
}

#[test]
fn book_list() {
    test("book_list.cl");
}

#[test]
fn bothcomments() {
    test("bothcomments");
}

#[test]
fn comment_in_string() {
    test("comment_in_string.cl");
}

#[test]
fn endcomment() {
    test("endcomment");
}

#[test]
fn eofstring() {
    test("eofstring");
}

#[test]
fn escaped_chars_in_comment() {
    test("escaped_chars_in_comment.cl");
}

#[test]
fn escapedeof() {
    test("escapedeof");
}

#[test]
fn escapednull() {
    test("escapednull");
}

#[test]
fn escapedquote() {
    test("escapedquote");
}

#[test]
fn escapedunprintables() {
    test("escapedunprintables");
}

#[test]
fn hairyscary() {
    test("hairyscary");
}

#[test]
fn integers2() {
    test("integers2");
}

#[test]
fn invalidcharacters() {
    test("invalidcharacters");
}

#[test]
fn invalidinvisible() {
    test("invalidinvisible");
}

#[test]
fn io() {
    test("io");
}

#[test]
fn keywords() {
    test("keywords");
}

#[test]
fn life() {
    test("life");
}

#[test]
fn lineno2() {
    test("lineno2");
}

#[test]
fn lineno3() {
    test("lineno3");
}

#[test]
fn longcomment() {
    test("longcomment");
}

#[test]
fn longstring_escapedbackslashes() {
    test("longstring_escapedbackslashes");
}

#[test]
fn multilinecomment() {
    test("multilinecomment");
}

#[test]
fn nestedcomment() {
    test("nestedcomment");
}

#[test]
fn new_complex() {
    test("new_complex");
}

#[test]
fn null_in_code() {
    test("null_in_code.cl");
}

#[test]
fn null_in_string() {
    test("null_in_string.cl");
}

#[test]
fn null_in_string_followed_by_tokens() {
    test("null_in_string_followed_by_tokens.cl");
}

#[test]
fn null_in_string_unescaped_newline() {
    test("null_in_string_unescaped_newline.cl");
}

#[test]
fn objectid() {
    test("objectid.test");
}

#[test]
fn opencomment() {
    test("opencomment");
}

#[test]
fn operators() {
    test("operators");
}

#[test]
fn palindrome() {
    test("palindrome");
}

#[test]
fn pathologicalstrings() {
    test("pathologicalstrings");
}

#[test]
fn s03() {
    test("s03.test");
}

#[test]
fn s04() {
    test("s04.test");
}

#[test]
fn s05() {
    test("s05.test");
}

#[test]
fn s14() {
    test("s14.test");
}

#[test]
fn s16() {
    test("s16.test");
}

#[test]
fn s19() {
    test("s19.test");
}

#[test]
fn s25() {
    test("s25.test");
}

#[test]
fn s26() {
    test("s26.test");
}

#[test]
fn s31() {
    test("s31.test");
}

#[test]
fn s32() {
    test("s32.test");
}

#[test]
fn s33() {
    test("s33.test");
}

#[test]
fn s34() {
    test("s34.test");
}

#[test]
fn simplestrings() {
    test("simplestrings");
}

#[test]
fn sort_list() {
    test("sort_list.cl");
}

#[test]
fn stringcomment() {
    test("stringcomment");
}

#[test]
fn stringwithescapes() {
    test("stringwithescapes");
}

#[test]
fn twice_512_nested_comments() {
    test("twice_512_nested_comments.cl");
}

#[test]
fn validcharacters() {
    test("validcharacters");
}

#[test]
fn weirdcharcomment() {
    test("weirdcharcomment");
}

#[test]
fn wq0607_c1() {
    test("wq0607-c1");
}

#[test]
fn wq0607_c2() {
    test("wq0607-c2");
}

#[test]
fn wq0607_c3() {
    test("wq0607-c3");
}

#[test]
fn wq0607_c4() {
    test("wq0607-c4");
}
