include!("pkg_json/npm_config_define.rs");

use human_panic::setup_panic;
use serde_derive::{Deserialize, Serialize};

fn main() {
    setup_panic!();
    let pkg_json_bytes = include_bytes!("../package.json");

    let pkg_define = load_package_json(pkg_json_bytes).unwrap();
    println!("pkg_define version {}", pkg_define.version);
    let version = find_cli_package_json_version(pkg_json_bytes);
    println!("this cli version {}", version.unwrap());

    let argument = std::env::args().nth(1).unwrap_or_else(|| {
        println!("must supply an integer argument");
        std::process::exit(1);
    });

    let result = rust_tests_example::factorial_of_str(argument.as_bytes());
    match result {
        Ok(num) => println!("{}", num),
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    }
}
