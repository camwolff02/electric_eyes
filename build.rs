extern crate prost_build;

fn main() {
    embuild::espidf::sysenv::output();

    // build protobufs
    prost_build::compile_protos(&["src/message.proto"],
                                &["src/"]).unwrap();
}
