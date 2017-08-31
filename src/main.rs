//extern crate lalrpop_util;
//extern crate regex;

mod tiger;

#[test]
fn tiger_program() {
    assert!(tiger::parse_program("1").is_ok());
}

#[test]
fn tiger_tydec() {
    //assert!(tiger::parse_decs("").is_ok());
    assert!(tiger::parse_decs("type a = b").is_ok());
    assert!(tiger::parse_decs("type a = {}").is_ok());
    assert!(tiger::parse_decs("type a = { }").is_ok());
    assert!(tiger::parse_decs("type a = { b: c }").is_ok());
    assert!(tiger::parse_decs("type a = { b: c, d :e }").is_ok());
    assert!(tiger::parse_decs("type a = array of b").is_ok());

    assert!(tiger::parse_decs("type intlist = {hd: int, tl: intlist}").is_ok());
    assert!(tiger::parse_decs(r#"type tree = {key: int, children: treelist}
type treelist = {hf: tree, tl: treelist}"#).is_ok());

    assert!(tiger::parse_decs("type = b").is_err());
    assert!(tiger::parse_decs("type a =").is_err());

    assert!(tiger::parse_decs("type = { b: c }").is_err());
    assert!(tiger::parse_decs("type a = b: c }").is_err());
    assert!(tiger::parse_decs("type a = { b: }").is_err());
    assert!(tiger::parse_decs("type a = { :c }").is_err());

    assert!(tiger::parse_decs("type 0 = array of b").is_err());
    assert!(tiger::parse_decs("type = array of b").is_err());
    assert!(tiger::parse_decs("type a = array of 1").is_err());
    assert!(tiger::parse_decs("type a = array of").is_err());
    assert!(tiger::parse_decs("type a = of b").is_err());
}

#[test]
fn tiger_vardec() {
    assert!(tiger::parse_decs("var a := 1").is_ok());
    assert!(tiger::parse_decs("var a: b := 1").is_ok());
    assert!(tiger::parse_decs("var a := a[10] of 0").is_ok());
    assert!(tiger::parse_decs("var arr1:arrtype := arrtype [10] of 0").is_ok());
}

#[test]
fn tiger_lvalue() {
    assert!(tiger::parse_lvalue("a").is_ok());
    assert!(tiger::parse_lvalue("a.b").is_ok());
    assert!(tiger::parse_lvalue("a[0]").is_ok());
}

#[test]
fn tiger_exp() {
    assert!(tiger::parse_exp("19").is_ok());
    assert!(tiger::parse_exp("a").is_ok());
    assert!(tiger::parse_exp("a.b").is_ok());
    assert!(tiger::parse_exp("a[0]").is_ok());
    assert!(tiger::parse_exp("a[b]").is_ok());
    assert!(tiger::parse_exp("nil").is_ok());
    assert!(tiger::parse_exp("break").is_ok());
    assert!(tiger::parse_exp("-19").is_ok());
    assert!(tiger::parse_exp("-a").is_ok());
    assert!(tiger::parse_exp("a := 1").is_ok());
    assert!(tiger::parse_exp("a := b").is_ok());

    assert!(tiger::parse_exp("while a do b").is_ok());

    assert!(tiger::parse_exp("0 := a").is_err());
}

#[test]
fn tiger_array_exp() {
    assert!(tiger::parse_exp("a[5] of 0").is_ok());
}

#[test]
fn tiger_record_exp() {
    assert!(tiger::parse_exp("a{}").is_ok());
    assert!(tiger::parse_exp("a{a=0}").is_ok());
    assert!(tiger::parse_exp("a{a=0, b =1, c=-1}").is_ok());
}

#[test]
fn tiger_if_exp() {
    assert!(tiger::parse_exp("if 1 then a").is_ok());
    assert!(tiger::parse_exp("if a then 1").is_ok());
    assert!(tiger::parse_exp("if a then 0 else 1").is_ok());
    assert!(tiger::parse_exp("if a then b else c").is_ok());
    assert!(tiger::parse_exp("if a then if b then c else d").is_ok());
    assert!(tiger::parse_exp("if a then if b then if c then d").is_ok());
    assert!(tiger::parse_exp("if a then if b then c else d else e").is_ok());
    assert!(tiger::parse_exp("if a then if b then if c then d else e").is_ok());
    assert!(tiger::parse_exp("if a then if b then c else d else if e then f else g").is_ok());
    assert!(tiger::parse_exp("if a then while b do if c then d").is_ok());
    assert!(tiger::parse_exp("if a then while b do if c then d else e").is_ok());
}

#[test]
fn tiger_while_exp() {
    assert!(tiger::parse_exp("while b <> 0 do b := b - 1").is_ok());
}

#[test]
fn tiger_for_exp() {
    assert!(tiger::parse_exp("for a := 0 to 10 do a").is_ok());
}

#[test]
fn tiger_arith_exp() {
    assert!(tiger::parse_exp("1 + 2").is_ok());
    assert!(tiger::parse_exp("a + 1").is_ok());
    assert!(tiger::parse_exp("1 + 2 + 3").is_ok());
    assert!(tiger::parse_exp("1 * 2 + 3").is_ok());
    assert!(tiger::parse_exp("1 + 2 * 3").is_ok());
    assert!(tiger::parse_exp("-1").is_ok());
    assert!(tiger::parse_exp("-a").is_ok());
    assert!(tiger::parse_exp("-1 + 2").is_ok());
    assert!(tiger::parse_exp("1 + -2").is_ok());
}

#[test]
fn tiger_bool_exp() {
    assert!(tiger::parse_exp("a & b").is_ok());
    assert!(tiger::parse_exp("a & b & c").is_ok());
    assert!(tiger::parse_exp("a | b").is_ok());
    assert!(tiger::parse_exp("a | b | c").is_ok());
    assert!(tiger::parse_exp("a & b | c").is_ok());
    assert!(tiger::parse_exp("a | b & c").is_ok());
    assert!(tiger::parse_exp("a & b | c & d").is_ok());
    assert!(tiger::parse_exp("a & b | c & d | e & f").is_ok());
}

#[test]
fn tiger_rel_exp() {
    assert!(tiger::parse_exp("a = b").is_ok());
    assert!(tiger::parse_exp("a <> b").is_ok());
    assert!(tiger::parse_exp("a > b").is_ok());
    assert!(tiger::parse_exp("a >= b").is_ok());
    assert!(tiger::parse_exp("a < b").is_ok());
    assert!(tiger::parse_exp("a <= b").is_ok());
    assert!(tiger::parse_exp("a=(b=c)").is_ok());

    assert!(tiger::parse_exp("a = b = c").is_err());
}

#[cfg(not(test))]
fn main() {
    println!("Hello, world!");
}
