fn str_testing() {
    let st = String::from("🐨 🐨 hello 🐨");
    dbg!(&st);
    dbg!(&st.len());
    dbg!(&st.chars().nth(2));
}


fn main() {
    println!("Hello, world!");

    let wat = r#"
        (module
            (import "host" "host_func" (func $host_hello (param i32)))

            (func (export "hello")
                i32.const 3
                call $host_hello)
        )
    "#;

    str_testing();
}
