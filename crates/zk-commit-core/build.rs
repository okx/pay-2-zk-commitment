use ethers::contract::Abigen;

fn main() {
    println!("cargo:rerun-if-changed=./static/abis/**/*.json");
    println!("cargo:rerun-if-changed=./out/**/*.json");

    bindgen("Groth16Verifier", "./static/abis/Groth16Verifier.json");
    bindgen("TestERC20", "./static/abis/TestERC20.json");
    bindgen("PayCommitment", "./static/abis/PayCommitment.json");
}

fn bindgen(contract_name: &str, path: &str) {
    let bindings = Abigen::new(contract_name, path)
        .expect("could not instantiate Abigen")
        .generate()
        .expect("could not generage bindings");
    bindings
        .write_to_file(format!("./src/bindings/{}.rs", contract_name.to_lowercase()))
        .expect("could not write bindings to file");
}
