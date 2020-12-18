fn main() {
    let mut versions = vec![];
    if cfg!(feature = "llvm-8") {
        versions.push(8);
    }
    if cfg!(feature = "llvm-9") {
        versions.push(9);
    }
    if cfg!(feature = "llvm-10") {
        versions.push(10);
    }
    if cfg!(feature = "llvm-11") {
        versions.push(11);
    }
    match versions.len() {
        0 => panic!("llvm-ir: Please select an LLVM version using a Cargo feature."),
        1 => {},
        _ => panic!("llvm-ir: Multiple LLVM versions selected. Please activate only one LLVM version feature. (Got {:?})", versions),
    };
}
