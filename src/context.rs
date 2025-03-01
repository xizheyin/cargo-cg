use rustc_hash::FxHashMap;
use rustc_hir::{def::DefKind, def_id::DefId};
use rustc_middle::ty::TyCtxt;
use rustc_span::Symbol;

#[derive(Clone)]
pub struct Context<'tcx> {
    pub tcx: TyCtxt<'tcx>,

    pub _all_generic_funcs_did_sym_map: FxHashMap<DefId, Symbol>,
}

impl<'tcx> Context<'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>, _args: FxHashMap<String, String>) -> Self {
        let mut _all_generic_funcs_did_sym_map = FxHashMap::default();
        for local_def_id in tcx.hir().body_owners() {
            let did = local_def_id.to_def_id();
            match tcx.def_kind(did) {
                DefKind::Fn | DefKind::AssocFn => {
                    let name = tcx.item_name(did);
                    if !_all_generic_funcs_did_sym_map.contains_key(&did) {
                        _all_generic_funcs_did_sym_map.insert(did, name);
                    }
                }
                _ => {}
            }
        }

        Self {
            tcx,

            _all_generic_funcs_did_sym_map,
        }
    }
}
