extern crate protoc_rust;

fn main() {

        protoc_rust::run(protoc_rust::Args {
        out_dir: "src",
        input: &["protos/filedescriptorset.proto"],
        includes: &["protos"],
        customize: protoc_rust::Customize {
            ..Default::default()
        },
    }).expect("protoc");
}
