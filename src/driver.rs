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

use rustc_hir as hir;
use rustc_lint::{LateContext, LateLintPass, LintContext};
use rustc_lint_defs::impl_lint_pass;
use rustc_session::declare_tool_lint;
use std::{path::Path, process, str};

// NOTE: do not output to stdout because it is parsed by Cargo
fn main() {
    rustc_driver::init_rustc_env_logger();
    std::process::exit(rustc_driver::catch_with_exit_code(move || {
        let orig_args: Vec<String> = std::env::args().collect();

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

impl rustc_driver::Callbacks for CallBacks {
    fn config(&mut self, config: &mut rustc_interface::Config) {
        config.register_lints = Some(Box::new(move |_sess, lint_store| {
            lint_store.register_late_pass(|| Box::new(VariantDef));
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

declare_tool_lint! {
    pub crate::VARIANT_DEF,
    Warn,
    "",
    report_in_external_macro: false
}

struct VariantDef;
impl_lint_pass!(VariantDef => [VARIANT_DEF]);

impl<'tcx> LateLintPass<'tcx> for VariantDef {
    fn check_variant(&mut self, cx: &LateContext<'tcx>, vari: &'tcx hir::Variant<'tcx>) {
        println!("variant def found");
        let source_map = cx.sess().source_map();
        let snippet = source_map.span_to_snippet(vari.span).unwrap();
        dbg!(snippet);
    }
}
