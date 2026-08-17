#![allow(dead_code)]

mod common;
mod p1_composition;
mod p2_accessor_trait;
mod p3_supertraits;
mod p4_enum;
mod p5_data_oriented;
mod p6_blanket_and_mixins;
mod p7_delegation_macro;

fn main() {
    p1_composition::demo();
    p2_accessor_trait::demo();
    p3_supertraits::demo();
    p4_enum::demo();
    p5_data_oriented::demo();
    p6_blanket_and_mixins::demo();
    p7_delegation_macro::demo();
}
