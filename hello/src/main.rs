fn main() {
    // raw string
    let s1: &str = r"abc
    def
    ghi";
    println!("{s1}");

    let s2: &str = r#"abc
    def
    ghi"#;
    println!("{s2}");

    let s3: &str = r##"abc
    "#def#"
    ghi"##;
    println!("{s3}");

    //     abc
    //     def
    //     ghi
    // abc
    //     def
    //     ghi
    // abc
    //     "#def#"
    //     ghi
}