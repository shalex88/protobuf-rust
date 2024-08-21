fn main() {
    prost_build::compile_protos(&["proto/device_irs/customer.proto"], &["proto/device_irs"]).unwrap();
}