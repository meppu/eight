fn main() {
    tonic_build::configure()
        .compile(&["src/grpc/proto/eight.proto"], &["proto"])
        .unwrap();
}
