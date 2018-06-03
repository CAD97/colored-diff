extern crate colored_diff;

fn main() {
    #[derive(Debug, PartialEq)]
    struct Foo {
        lorem: &'static str,
        ipsum: u32,
        dolor: Result<String, String>,
    }

    let x = Some(Foo { lorem: "Hello World!", ipsum: 42, dolor: Ok("hey".to_string())});
    let y = Some(Foo { lorem: "Hello Wrold!", ipsum: 42, dolor: Ok("hey ho!".to_string())});

    let x = format!("{:#?}", x);
    let y = format!("{:#?}", y);

    colored_diff::init();
    println!("{}", colored_diff::PrettyDifference { expected: &x, actual: &y })
}
