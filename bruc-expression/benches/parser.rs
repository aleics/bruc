#![feature(test)]

extern crate test;

use test::Bencher;

use bruc_expression::expr::Interpretable;
use bruc_expression::vars::Variables;
use bruc_expression::PredicateParser;

#[bench]
fn bench_parsing_boolean_predicate(b: &mut Bencher) {
    b.iter(|| PredicateParser::new("true || false").parse());
}

#[bench]
fn bench_interpret_boolean_predicate(b: &mut Bencher) {
    let expression = PredicateParser::new("true || false").parse().unwrap();
    let vars = Variables::new();
    b.iter(|| {
        let result: bool = expression.interpret(&vars).unwrap();
        result
    });
}

#[bench]
fn bench_parsing_numeric_predicate(b: &mut Bencher) {
    b.iter(|| PredicateParser::new("3 <= 2").parse());
}

#[bench]
fn bench_interpret_numeric_predicate(b: &mut Bencher) {
    let expression = PredicateParser::new("3 <= 2").parse().unwrap();
    let vars = Variables::new();
    b.iter(|| {
        let result: bool = expression.interpret(&vars).unwrap();
        result
    });
}

#[bench]
fn bench_interpret_boolean_predicate_with_vars(b: &mut Bencher) {
    let expression = PredicateParser::new("a || b").parse().unwrap();
    let vars = Variables::from_pairs(vec![("a", false.into()), ("b", true.into())]);
    b.iter(|| {
        let result: bool = expression.interpret(&vars).unwrap();
        result
    });
}

#[bench]
fn bench_interpret_numeric_predicate_with_vars(b: &mut Bencher) {
    let expression = PredicateParser::new("a <= b").parse().unwrap();
    let vars = Variables::from_pairs(vec![("a", 3.0.into()), ("b", 2.0.into())]);
    b.iter(|| {
        let result: bool = expression.interpret(&vars).unwrap();
        result
    });
}

#[bench]
fn bench_interpret_hybrid_predicate_with_struct_vars(b: &mut Bencher) {
    struct Data {
        a: f32,
        b: f32,
        c: bool,
    }

    let data = Data {
        a: 3.0,
        b: 2.0,
        c: true,
    };

    let vars = Variables::from_pairs(vec![
        ("a", data.a.into()),
        ("b", data.b.into()),
        ("c", data.c.into()),
    ]);
    let expression = PredicateParser::new("(a <= b) && c").parse().unwrap();
    b.iter(|| {
        let result: bool = expression.interpret(&vars).unwrap();
        result
    });
}
