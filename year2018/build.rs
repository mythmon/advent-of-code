fn main() {
    lalrpop::process_root().expect("Could not generate parsers");
}
