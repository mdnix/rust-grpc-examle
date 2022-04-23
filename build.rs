fn main() {
    tonic_build::configure()
        .compile(&["proto/greeter.proto"], &["proto"])
        .unwrap();
}