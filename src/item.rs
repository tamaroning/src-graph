use crate::SOURCE_INFO;
use rustc_hir as hir;
use rustc_lint::{LateContext, LateLintPass, LintContext};
use rustc_lint_defs::impl_lint_pass;
use rustc_session::declare_tool_lint;

declare_tool_lint! {
    pub crate::ITEM,
    Warn,
    "",
    report_in_external_macro: false
}

pub struct Item;

impl Item {
    pub fn new() -> Self {
        Item
    }
}

impl_lint_pass!(Item => [ITEM]);

impl<'tcx> LateLintPass<'tcx> for Item {
    fn check_item(&mut self, cx: &LateContext<'tcx>, item: &'tcx hir::Item<'tcx>) {
        let source_map = cx.sess().source_map();
        let snippet = source_map.span_to_snippet(item.span).unwrap();
        dbg!(snippet);
    }

    fn check_crate(&mut self, _: &LateContext<'tcx>) {
        println!("crate");
        let info = SOURCE_INFO.get().unwrap();
        dbg!(info);
    }
}
