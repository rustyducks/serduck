
#[cfg(feature = "proto_debug")]
use std::env;
#[cfg(feature = "proto_debug")]
use std::fs;
#[cfg(feature = "proto_debug")]
use std::path::Path;
#[cfg(feature = "proto_debug")]
use protobuf_codegen_pure::Customize;
#[cfg(feature = "proto_debug")]
use protoc_rust;

fn main() {
    #[cfg(feature = "proto_debug")]
    {    
        let out_dir = env::var("OUT_DIR").unwrap();

        let generated_with_native_dir = format!("{}/generated_with_native", out_dir);

        if Path::new(&generated_with_native_dir).exists() {
            fs::remove_dir_all(&generated_with_native_dir).unwrap();
        }

        fs::create_dir(&generated_with_native_dir).unwrap();


        protoc_rust::Codegen::new()
            .customize(Customize {
                gen_mod_rs: Some(true),
                ..Default::default()
            })
            .out_dir(generated_with_native_dir)
            .input("src/protoduck/messages.proto")
            .include("src/protoduck")
            .run()
            .expect("protoc");
    }
}
