#![allow(unused_imports)]
use little_schemer::helper_functions::{
    split_whitespace_not_in_parantheses, split_whitespace_not_in_parantheses_advanced_to_quote,
};
use little_schemer::AtomTypes::{Bool, Integer, String, Symbol};
use little_schemer::ExpressionTypes::{Atom, Function, List, Nil, Syntactic, Variable};
use little_schemer::FunctionTypes::{self, CustomFunction, InBuildFunction};
use little_schemer::Interpreter;
use little_schemer::SyntacticTypes::{Let, Quote};
mod common;
use common::{
    assert_eval_eq_after_predefine_ast_precompute, assert_eval_eq_ast_precompute,
    ast_precompute_execute, execute_form_with_ast, execute_programm_with_ast,
};
#[test]
fn rember_test_1() {
    assert_eval_eq_after_predefine_ast_precompute(
        r#"
(define rember 
    (lambda (a lat) 
        (cond 
            ((null? lat) (quote ())) 
            ((eq? (car lat) a) (cdr lat)) 
            (else 
                (cons (car lat) 
                    (rember a (cdr lat))))))) "#,
        r#"(rember "mint" '("chopsticks" "chomps" "and" "mint" "jelly"))"#,
        r#"'("chopsticks" "chomps" "and" "jelly")"#,
    )
}
#[test]
fn rember_test_2() {
    assert_eval_eq_after_predefine_ast_precompute(
        r#"
(define rember 
    (lambda (a lat) 
        (cond 
            ((null? lat) (quote ())) 
            ((eq? (car lat) a) (cdr lat)) 
            (else 
                (cons (car lat) 
                    (rember a (cdr lat))))))) "#,
        r#"(rember "and" '("bacond" "letuce" "and" "tomato"))"#,
        r#"'("bacond" "letuce" "tomato")"#,
    )
}
#[test]
fn firsts_test_1() {
    assert_eval_eq_after_predefine_ast_precompute(
        r#"
(define firsts
    (lambda (l)
        (cond
            ((null? l) (quote ()))
            (else (cons (car (car l))
                (firsts (cdr l)))))))"#,
        r#"(firsts '((a b) (c d) (e f)))"#,
        r#"'(a c e)"#,
    )
}
#[test]
fn insert_right_test_1() {
    assert_eval_eq_after_predefine_ast_precompute(
        r#"
(define insertR
    (lambda (new old lat)
        (cond
            ((null? lat) (quote ()))
            (else (cond
                ((eq? (car lat) old)
                    (cons old
                        (cons new (cdr lat))))
                (else (cons (car lat)
                    (insertR new old
                        (cdr lat)))))))))"#,
        r#"(insertR "neww" "and" '("bacond" "letuce" "and" "tomato"))"#,
        r#"'("bacond" "letuce" "and" "neww" "tomato")"#,
    )
}
#[test]
fn insert_left_test_1() {
    assert_eval_eq_after_predefine_ast_precompute(
        r#"
(define insertL
    (lambda (new old lat)
        (cond
            ((null? lat) (quote ()))
            (else (cond
                ((eq? (car lat) old)
                    (cons new
                        (cons old (cdr lat))))
                (else (cons (car lat)
                    (insertL new old
                        (cdr lat)))))))))"#,
        r#"(insertL "neww" "and" '("bacond" "letuce" "and" "tomato"))"#,
        r#"'("bacond" "letuce" "neww" "and" "tomato")"#,
    )
}
#[test]
fn subst_test_1() {
    assert_eval_eq_after_predefine_ast_precompute(
        r#"
(define subst
    (lambda (new old lat)
        (cond
            ((null? lat) (quote ()))
            (else (cond
                ((eq? (car lat) old)
                    (cons new
                        (cdr lat)))
                (else (cons (car lat)
                    (subst new old
                        (cdr lat)))))))))"#,
        r#"(subst "neww" "and" '("bacond" "letuce" "and" "tomato"))"#,
        r#"'("bacond" "letuce" "neww" "tomato")"#,
    )
}
#[test]
fn subst2_test_1() {
    assert_eval_eq_after_predefine_ast_precompute(
        r#"
(define subst2
    (lambda (new o1 o2 lat)
        (cond
            ((null? lat) (quote ()))
            (else (cond
                ((eq? (car lat) o1)
                    (cons new
                        (cdr lat)))
                ((eq? (car lat) o2)
                    (cons new
                        (cdr lat)))
                (else (cons (car lat)
                    (subst2 new o1 o2
                        (cdr lat)))))))))"#,
        r#"(subst2 'vanilla 'chocalate 'banana '(banana ice cream with chocolate topping))"#,
        r#"'(vanilla ice cream with chocolate topping)"#,
    )
}
#[test]
fn subst2_test_2() {
    assert_eval_eq_after_predefine_ast_precompute(
        r#"
(define subst2
    (lambda (new o1 o2 lat)
        (cond
            ((null? lat) (quote ()))
            (else (cond
                ((or 
                    (eq? (car lat) o1) 
                    (eq? (car lat) o2))
                (cons new (cdr lat)))
                (else (cons (car lat)
                    (subst2 new o1 o2
                        (cdr lat)))))))))"#,
        r#"(subst2 'vanilla 'chocalate 'banana '(banana ice cream with chocolate topping))"#,
        r#"'(vanilla ice cream with chocolate topping)"#,
    )
}
#[test]
fn multirember_test_1() {
    assert_eval_eq_after_predefine_ast_precompute(
        r#"
(define multirember 
    (lambda (a lat) 
        (cond 
            ((null? lat) (quote ())) 
            ((eq? (car lat) a) (multirember a (cdr lat))) 
            (else 
                (cons (car lat) 
                    (multirember a (cdr lat))))))) "#,
        r#"(multirember 'cup '(coffee cup tea cup aand hick cup))"#,
        r#"'(coffee tea aand hick)"#,
    )
}
#[test]
fn multiinsert_right_test_1() {
    assert_eval_eq_after_predefine_ast_precompute(
        r#"
(define multiinsertR
    (lambda (new old lat)
        (cond
            ((null? lat) (quote ()))
            (else (cond
                ((eq? (car lat) old)
                    (cons old
                        (cons new (multiinsertR new old (cdr lat)))))
                (else (cons (car lat)
                    (multiinsertR new old
                        (cdr lat)))))))))"#,
        r#"(multiinsertR 'newcup 'cup '(coffee cup tea cup aand hick cup))"#,
        r#"'(coffee cup newcup tea cup newcup aand hick cup newcup)"#,
    )
}
#[test]
fn multiinsert_left_test_1() {
    assert_eval_eq_after_predefine_ast_precompute(
        r#"
(define multiinsertL
    (lambda (new old lat)
        (cond
            ((null? lat) (quote ()))
            (else (cond
                ((eq? (car lat) old)
                    (cons new
                        (cons old (multiinsertL new old (cdr lat)))))
                (else (cons (car lat)
                    (multiinsertL new old
                        (cdr lat)))))))))"#,
        r#"(multiinsertL 'newcup 'cup '(coffee cup tea cup aand hick cup))"#,
        r#"'(coffee newcup cup tea newcup cup aand hick newcup cup)"#,
    )
}
#[test]
fn multsubst_left_test_1() {
    assert_eval_eq_after_predefine_ast_precompute(
        r#"
(define multisubst
    (lambda (new old lat)
        (cond
            ((null? lat) (quote ()))
            (else (cond
                ((eq? (car lat) old)
                    (cons new (multisubst new old (cdr lat))))
                (else (cons (car lat)
                    (multisubst new old
                        (cdr lat)))))))))"#,
        r#"(multisubst 'newcup 'cup '(coffee cup tea cup aand hick cup))"#,
        r#"'(coffee newcup tea newcup aand hick newcup)"#,
    )
}
