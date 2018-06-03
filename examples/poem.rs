extern crate colored_diff;

fn main() {
    let expected = "\
        Roses are red, violets are blue,\n\
        I wrote this library here,\n\
        just for you.\n\
        (It's true).\n\
    ";
    let actual = "\
        Roses are red, violets are blue,\n\
        I wrote this documentation here,\n\
        just for you.\n\
        (It's quite true).\n\
    ";

    colored_diff::init();
    println!("{}", colored_diff::PrettyDifference { expected, actual })
}
