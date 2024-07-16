//! Move clauses on associated types to be parent clauses. The distinction is not semantically
//! meaningful. We should ideally to this directly when translating but this is currently
//! difficult; instead we do this as a post-processing pass.
use std::collections::HashMap;
use std::mem;

use derive_visitor::{visitor_enter_fn_mut, DriveMut};

use crate::{ast::*, ids::Vector};

use super::{ctx::UllbcPass, TransformCtx};

pub struct Transform;
impl UllbcPass for Transform {
    fn transform_ctx(&self, ctx: &mut TransformCtx<'_>) {
        // For each trait, we move the item-local clauses to be top-level parent clauses, and
        // record the mapping from the old to the new ids.
        let trait_item_clause_ids: Vector<
            TraitDeclId,
            HashMap<TraitItemName, Vector<TraitClauseId, TraitClauseId>>,
        > = ctx.translated.trait_decls.map_ref_mut(|decl| {
            decl.types
                .iter_mut()
                .map(|(name, (clauses, _))| {
                    let id_map = mem::take(clauses).map(|mut clause| {
                        decl.parent_clauses.push_with(|id| {
                            clause.clause_id = id;
                            clause
                        })
                    });
                    (name.clone(), id_map)
                })
                .collect()
        });

        // Move the item-local trait refs to match what we did in the trait declarations.
        for timpl in ctx.translated.trait_impls.iter_mut() {
            for (_, (refs, _)) in timpl.types.iter_mut() {
                for trait_ref in mem::take(refs) {
                    // Note: this assumes that we listed the types in the same order as in the trait
                    // decl, which we do.
                    timpl.parent_trait_refs.push(trait_ref);
                }
            }
        }

        // Update trait refs.
        ctx.translated
            .drive_mut(&mut visitor_enter_fn_mut(|trkind: &mut TraitRefKind| {
                use TraitRefKind::*;
                if let ItemClause(..) = trkind {
                    take_mut::take(trkind, |trkind| {
                        let ItemClause(trait_ref, trait_decl, item_name, item_clause_id) = trkind
                        else {
                            unreachable!()
                        };
                        let new_id = trait_item_clause_ids[trait_decl][&item_name][item_clause_id];
                        ParentClause(trait_ref, trait_decl, new_id)
                    })
                }
            }));
    }
}
