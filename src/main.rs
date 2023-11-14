use lawyer::cargo::Licenses;

fn main() {
    dbg!(Licenses::load("./Cargo.lock").unwrap());
}
