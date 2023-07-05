mod submod {
    use pub_fields::pub_fields;

    #[pub_fields]
    pub struct MyStruct {
        a: usize,
        b: usize,
        c: usize,
    }
}

#[test]
fn test_it() {
    let _x = submod::MyStruct { a: 3, b: 4, c: 5 };
}
