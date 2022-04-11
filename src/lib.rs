use std::{
    env::temp_dir,
    fs::{create_dir_all, remove_dir_all, write},
    io::{stderr, Write},
    process::Command,
};

use anyhow::{anyhow, Result};
use libloading::{library_filename, Library};
use rand::{thread_rng, Rng};

#[macro_export]
macro_rules! inline_rust {
    ($code:expr,$( $arg:ident : $aty:ty ),*) => {
        unsafe {
            let __code: String = $code;
            let mut __args = String::new();
            $(
                let __arg = stringify!($arg);
                let __ty = stringify!($aty);
                __args.push_str(&__arg);
                __args.push(':');
                __args.push_str(&__ty);
                __args.push(',');
            )*
            let __lib = $crate::generate_rust_cdylib(format!(
                r#"#[no_mangle]
pub extern "C" fn inline({args}) {{
    {}
}}"#,
                __code,
                args = __args
            ))
            .unwrap(); // TODO no unwrap ):
            let __func: ::libloading::Symbol<unsafe extern "C" fn(
                $(
                    $aty ,
                )*
            ) -> ()> =
                __lib.get(b"inline").unwrap();
            __func(
                $(
                    $arg ,
                )*
            );
            __lib.close().unwrap();
        }
    };
}

pub fn generate_rust_cdylib(code: String) -> Result<Library> {
    let dir = temp_dir().join(format!("inline-rust_{}", thread_rng().gen::<u64>()));
    let dir_str = dir.as_os_str().to_str().ok_or(anyhow!("to_str none"))?;
    create_dir_all(&dir)?;
    let src = dir.join("main.rs");
    write(&src, code)?;
    let src_str = src.as_os_str().to_str().ok_or(anyhow!("to_str none"))?;
    let rustc_result = Command::new("rustc")
        .args([
            "--crate-type",
            "cdylib",
            "--crate-name",
            "ck_inline",
            "--out-dir",
            dir_str,
            src_str,
        ])
        .output()?;

    if !rustc_result.status.success() {
        stderr().write_all(&rustc_result.stderr)?;
        assert!(rustc_result.status.success());
    }

    unsafe {
        let dylib = Library::new(dir.join(library_filename("ck_inline")))?;
        remove_dir_all(dir)?;
        Ok(dylib)
    }
}
