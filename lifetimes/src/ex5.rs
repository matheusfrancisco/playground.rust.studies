#[test]
fn ex5_splitting_borrows_on_structs() {
    struct Foo {
        a: usize,
        b: usize,
    }

    fn change_field(field: &mut usize) {
        *field += 1;
    }

    let mut foo = Foo { a: 1, b: 2 };
    let a_ref = &mut foo.a;
    let b_ref = &mut foo.b;

    change_field(a_ref);
    change_field(b_ref);

    println!("foo: a = {}, b = {}", foo.a, foo.b);
}

#[test]
fn ex5_splitting_borrows_on_structs_2() {
    struct Data<'a> {
        field_usize: &'a mut usize,
        field_string: &'a str,
    }

    impl Data<'_> {
        fn new<'a>(str_param: &'a str, usize_param: &'a mut usize) -> Data<'a> {
            Data {
                field_string: str_param,
                field_usize: usize_param,
            }
        }

        fn change_field(&mut self) {
            *self.field_usize += 1;
        }

        fn change_string(&mut self) {
            self.field_string = "Changed string";
        }
    }

    let mut num = 10;
    let mut data = Data::new("Hello", &mut num);
    println!(
        "Data: field_string = {}, field_usize = {}",
        data.field_string, data.field_usize
    );

    fn change_field(field: &mut usize) {
        *field += 5;
    }

    change_field(data.field_usize);
    data.change_string();
    println!(
        "Data after changes: field_string = {}, field_usize = {}",
        data.field_string, data.field_usize
    );
}
