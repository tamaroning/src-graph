#![feature(rustc_private, let_chains)]

extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_typeck;

mod graph;
mod source_info;

use graph::output_dot;
use rustc_hir::ItemKind;
use rustc_middle::ty::subst::GenericArgKind;
use source_info::{Adt, SourceInfo};
use std::{env, path::Path, process, str, fs::create_dir};

const SRC_GRAPH_DIR: &'static str = "./.src_graph";

// NOTE: do not output to stdout because Cargo parses stdout
fn main() {
    rustc_driver::init_rustc_env_logger();
    std::process::exit(rustc_driver::catch_with_exit_code(move || {
        let orig_args: Vec<String> = env::args().collect();

        let mut rustc_args = orig_args;

        // When this driver is executed by setting RUSTC_WORKSPACE_WRAPPER, 
        // Cargo sets "rustc" as the first argument.
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
        let mut info = SourceInfo::new();

        queries.global_ctxt().unwrap().take().enter(|tcx| {
            let items = tcx.hir().items();

            for item in items {
                // TODO: support other ADTs than struct
                match &item.kind {
                    ItemKind::Struct(variant, _) => {
                        //let parent_ty = tcx.typeck(item.def_id);
                        let parent_def_path = tcx.def_path(item.def_id.to_def_id());

                        let parent_path = parent_def_path
                            .to_filename_friendly_no_crate()
                            .replace('-', "_");

                        let parent_adt = Adt::new(parent_path);
                        info.register_adt(parent_adt.clone());

                        for field in variant.fields() {
                            // Get a type T of the fields
                            let child_ty = rustc_typeck::hir_ty_to_ty(tcx, field.ty);

                            // check each type S reachable from T
                            // e.g. Foo<Bar<i32>, u32, T> where T is generic -> [Foo<Bar<i32>, Bar<i32>, i32, u32]
                            for ty in child_ty.walk() {
                                if let GenericArgKind::Type(ty) = ty.unpack() {
                                    // If S has a type of ADT
                                    if let Some(adt_def) = ty.ty_adt_def() {
                                        let def_path = tcx.def_path(adt_def.did);
                                        let child_path = def_path
                                            .to_filename_friendly_no_crate()
                                            .replace('-', "_");

                                        // Get crate name which defines S
                                        let crate_name = tcx.crate_name(def_path.krate);
                                        let crate_name = crate_name.as_str().to_string();

                                        // If S is NOT defined in std
                                        if !is_in_std(&crate_name) {
                                            info.add_dependency(&parent_adt, Adt::new(child_path));
                                        }
                                    }
                                }
                            }
                        }
                    }
                    _ => (),
                }
            }
        });

        create_dir(SRC_GRAPH_DIR).unwrap();

        output_dot(&Path::new(SRC_GRAPH_DIR).join("struct_deps.dot"), &info);
        rustc_driver::Compilation::Stop
    }
}

fn is_in_std(crate_name: &str) -> bool {
    matches!(crate_name, "std" | "core" | "alloc" | "proc_macro" | "test")
}
