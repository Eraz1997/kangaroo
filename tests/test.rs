#[test]
fn test_add_fields2() -> Result<(), Box<dyn std::error::Error>> {
    use kangaroo::add_fields;

    #[add_fields(
        fields(name = test, ty = "u32"),
        fields(name = add_field2, ty = "i16"),
        fields(name = "test3", ty = "String"),
        getter(field = id,"test",test3),
        setter(field=id, name)
    )]
    #[derive(Debug)]
    struct MyStruct {
        id: u32,
        name: String,
    }
    let mut x = MyStruct {
        id: 5,
        name: "Hallo".to_string(),
        test: 6,
        add_field2: 7,
        test3: "hallo test3".to_string(),
    };

    println!("{:#?}", x);
    x.set_id(99).set_name("Name set".into());
    assert!(x.id() == &99);
    assert!(x.id == 99);
    assert!(x.name.as_str() == "Name set");
    assert!(x.test == 6);
    assert!(x.test() == &6);
    assert!(x.add_field2 == 7);
    assert!(x.test3.as_str() == "hallo test3");
    assert!(x.test3() == "hallo test3");
    Ok(())
}
