use std::fs::File;
use std::io::Read;
use std::process::Command;

#[test]
fn cargo_readme_up_to_date() {
    println!("Checking that `cargo readme > README.md` is up to date...");

    let expected = Command::new("cargo")
        .arg("readme")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("should run `cargo readme` OK")
        .stdout;
    let expected = String::from_utf8_lossy(&expected);

    let actual = {
        let mut file = File::open(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))
            .expect("should open README.md file");
        let mut s = String::new();
        file.read_to_string(&mut s)
            .expect("should read contents of file to string");
        s
    };

    if actual != expected {
        panic!("Run `cargo readme > README.md` to update README.md");
    }
}

macro_rules! test {
    ( $name:ident => $wasm:expr; $expected:expr ) => {
        test!($name => $wasm; []; $expected);
    };
    ( $name:ident => $wasm:expr ; $flags:expr ; $expected:expr ) => {
        #[test]
        #[cfg(feature = "exe")]
        fn $name() {
            let flags = $flags;
            let flags: &[&str] = &flags[..];
            let output = Command::new("cargo")
                .arg("run")
                .arg("--")
                .args(flags)
                .arg(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/", $wasm))
                .output()
                .expect("should `cargo run` OK");

            let mut expected_file = File::open(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/", $expected))
                .expect(concat!("should open ", $expected));

            let mut expected = vec![];
            expected_file.read_to_end(&mut expected).expect("should read to end");

            println!("actual =\n{}", String::from_utf8_lossy(&output.stdout));
            println!("expected =\n{}", String::from_utf8_lossy(&expected));

            if output.stdout != expected {
                panic!("{} does not match", $expected);
            }
        }

    }
}

test!(hello => "hello.wasm"; "hello.expected");
test!(hello_dash_i => "hello.wasm"; ["-i"]; "hello-dash-i.expected");
test!(small_hello => "small-hello.wasm"; "small-hello.expected");
test!(small_hello_dash_j => "small-hello.wasm"; ["-j"]; "small-hello-dash-j.expected");
test!(small_hello_dash_e => "small-hello.wasm"; ["-e"]; "small-hello-dash-e.expected");
