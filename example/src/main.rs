#[macro_use]
extern crate mutils;

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
}

fn prvec(v: &vvt!(i32)) {
    println!("{:?}", v);
}
