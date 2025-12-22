mod redoxri;
use redoxri::*;

fn main() -> Result<(), RxiError> {
    let _redoxri = Redoxri::new(&[
        "--cfg", "bootstrapped",
    ]);

    let out = Mcule::new("output", "out/")
        .add_step(&["mkdir", "out"])
        .compile();

    let karottenschaeler = Mcule::new("karottenschaeler", "out/libkarottenschaeler.rlib")
        .with(&[
            "libs/karottenschaeler/lib.rs".into(),
        ])
        .add_step(&[
            "rustc", "src/karottenschaeler/lib.rs", "-o" , "$out", "--crate-type", "lib",
            "-Clink-args=-lc",
            //"--extern", &("libc".to_owned() + "=" + &libc.outpath),
        ])
        .compile();

    let test = Mcule::new("main", "out/main")
        .with(&[
            karottenschaeler.clone(),
            "tests/main.rs".into(),
        ])
        .add_step(&[
            "rustc", "src/main.rs", "-o", "$out",
            "--extern", &(karottenschaeler.name + "=" + &karottenschaeler.outpath),
        ])
        .compile()
        .run();

    Ok(())
}
