#[macro_use]
extern crate mutils;

use std::collections::HashMap;

fn main() {
    vi!(v, i32);
    vi!(s, &str);
    vvi!(vv, i32);

    v.push(10);
    vv.push(v);
    s.push("hello");
    s.push("world");

    for_enum!(ind, elem, s, {
        println!("{} {}", ind, elem);
    });

    prvec(&vv);

    let word_list = vec!["glass", "table", "chair", "chair"];
    hmi!(words, &str, i32);

    for item in word_list  {
        let count = words.entry(item).or_insert(0);
        *count += 1;
    }
    prhm(&words);
}

fn prvec(v: &vvt!(i32)) {
    println!("{:?}", v);
}

fn prhm(hm: &hmt!(&str, i32)) {
    println!("{:?}", hm);
}
