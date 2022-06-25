#![feature(rustc_private, let_chains)]

extern crate rustc_ast;
extern crate rustc_driver;
extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_lint;
extern crate rustc_lint_defs;
extern crate rustc_session;
extern crate rustc_span;

mod item;

use once_cell::sync::OnceCell;
use std::{env, path::Path, process, str};

// NOTE: do not output to stdout because it is parsed by Cargo
fn main() {
    rustc_driver::init_rustc_env_logger();
    std::process::exit(rustc_driver::catch_with_exit_code(move || {
        let orig_args: Vec<String> = env::args().collect();

        let mut rustc_args = orig_args;

        // When this driver is executed by setting RUSTC_WORKSPACE_WRAPPER, Cargo sets "rustc" as the first argument.
        // Code below handles this case.
        let wrapper_mode =
            rustc_args.get(1).map(Path::new).and_then(Path::file_stem) == Some("rustc".as_ref());
        if wrapper_mode {
            rustc_args.remove(1);
        }

        let have_sys_root_arg = rustc_args.iter().any(|arg| arg == "--sysroot");

        // If sys_root is not set, gets and appends it
        if !have_sys_root_arg {
            let out = process::Command::new("rustc")
                .arg("--print=sysroot")
                .output()
                .unwrap();
            let sys_root = str::from_utf8(&out.stdout).unwrap().trim().to_string();
            rustc_args.extend(vec!["--sysroot".to_string(), sys_root]);
        }

        rustc_driver::RunCompiler::new(&rustc_args, &mut CallBacks).run()
    }));
}

struct CallBacks;

#[derive(Debug)]
pub struct SourceInfo {
    structs: Vec<String>,
}

impl SourceInfo {
    fn new() -> Self {
        SourceInfo { structs: vec![] }
    }
}

static SOURCE_INFO: OnceCell<SourceInfo> = OnceCell::new();

impl rustc_driver::Callbacks for CallBacks {
    fn config(&mut self, config: &mut rustc_interface::Config) {
        SOURCE_INFO.set(SourceInfo::new()).unwrap();

        config.register_lints = Some(Box::new(move |_sess, lint_store| {
            lint_store.register_late_pass(|| Box::new(item::Item::new()));
        }));
    }

    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &rustc_interface::interface::Compiler,
        _queries: &'tcx rustc_interface::Queries<'tcx>,
    ) -> rustc_driver::Compilation {
        println!("after analysis");
        rustc_driver::Compilation::Stop
    }
}
