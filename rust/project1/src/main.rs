fn main() {
    let too = Foo {
        foo: i32,
        goo: i32,
    };
    for thang in too.iter() {
        print!("ya");
    }
}

struct Foo {
    foo: i32,
    goo: i32,
}

