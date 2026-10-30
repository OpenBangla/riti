fn main() {
    println!("cargo:rerun-if-changed=data/khipro-classic/bn-khipro-classic.mim");

    m17n_im::Parser::parse_from_path("data/khipro-classic/bn-khipro-classic.mim")
        .unwrap()
        .generate(
            &mut m17n_im::Preprocessor::default(),
            &mut std::fs::File::create("src/khipro/classic.rs").unwrap(),
        )
        .unwrap();
}
