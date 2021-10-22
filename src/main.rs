use argparser::ArgParse;

fn main() {
    let mut argparse = ArgParse::new(None);
    argparse.add_argument("name");

    println!("{:?}", argparse);
}