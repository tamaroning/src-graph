#![feature(rustc_private, let_chains)]

extern crate rustc_ast;
extern crate rustc_driver;
extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_hir_pretty;
extern crate rustc_interface;
extern crate rustc_lint;
extern crate rustc_lint_defs;
extern crate rustc_resolve;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_typeck;

mod graph;
mod source_info;

use graph::output_dot;
use rustc_hir::ItemKind;
use rustc_hir_pretty::ty_to_string;
use source_info::SourceInfo;
use std::{env, path::Path, process, str};

use crate::source_info::Adt;

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

impl rustc_driver::Callbacks for CallBacks {
    fn config(&mut self, _config: &mut rustc_interface::Config) {}

    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &rustc_interface::interface::Compiler,
        queries: &'tcx rustc_interface::Queries<'tcx>,
    ) -> rustc_driver::Compilation {
        println!("after analysis");
        let mut info = source_info::SourceInfo::new();

        queries.global_ctxt().unwrap().take().enter(|tcx| {
            let krate = tcx.hir().krate();
            let items = tcx.hir().items();

            for item in items {
                match &item.kind {
                    ItemKind::Struct(variant, _) => {
                        //let parent_ty = tcx.typeck(item.def_id);
                        let parent_def_path = tcx.def_path(item.def_id.to_def_id());

                        let parent_path = parent_def_path.to_string_no_crate_verbose();
                        dbg!(&parent_path);
                        let parent_adt = Adt::new(parent_path);
                        info.register_adt(parent_adt.clone());

                        for field in variant.fields() {
                            let child_ty = rustc_typeck::hir_ty_to_ty(tcx, field.ty);
                            if let Some(child_adt_def) = child_ty.ty_adt_def() {
                                let child_def_path = tcx.def_path(child_adt_def.did);
                                let crate_name = tcx.crate_name(child_def_path.krate);
                                let crate_name = crate_name.as_str().to_string();
                                let child_path = child_def_path.to_string_no_crate_verbose();
                                if !is_in_std(&crate_name) || true {
                                    info.add_dependency(&parent_adt, Adt::new(child_path));
                                }
                            }
                        }
                    }
                    _ => (),
                }
            }
        });

        dbg!(&info);

        output_dot(&Path::new("./example.dot"));
        rustc_driver::Compilation::Stop
    }
}

// TODO: refine
fn is_in_std(crate_name: &str) -> bool {
    match crate_name {
        "std" | "core" | "alloc" | "proc_macro" | "test" => true,
        _ => false,
    }
}
