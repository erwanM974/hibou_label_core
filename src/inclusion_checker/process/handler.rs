/*
Copyright 2020 Erwan Mahe (github.com/erwanM974)

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/



use std::collections::BTreeSet;

use graph_process_manager_core::process::handler::AbstractAlgorithmOperationHandler;
use crate::core::semantics::execute::execute_interaction;
use crate::core::semantics::frontier::global_frontier;
use crate::core::syntax::interaction::Interaction;
use crate::rewriting::{canonize::canonize_interaction, lang::HibouRewritableLangOperator};
use maplit::btreeset;

use super::{conf::InteractionInclusionCheckingConfig, context::InteractionInclusionCheckingContextAndParameterization, node::InteractionInclusionCheckingNode, step::InteractionInclusionCheckingStepKind};

pub struct InteractionInclusionCheckingHandler {}

impl AbstractAlgorithmOperationHandler<InteractionInclusionCheckingConfig> for InteractionInclusionCheckingHandler {

    fn process_new_step(
        _context_and_param : &InteractionInclusionCheckingContextAndParameterization,
        parent_node : &InteractionInclusionCheckingNode,
        step_to_process : &InteractionInclusionCheckingStepKind
    ) -> InteractionInclusionCheckingNode {
        match step_to_process {
            InteractionInclusionCheckingStepKind::ExecuteAction(frt_elt,next_including_candidates) => {
                InteractionInclusionCheckingNode::new(
                    execute_interaction(
                        &parent_node.included_candidate, 
                        &frt_elt.position, 
                        &frt_elt.target_lf_ids, 
                        false
                    ).interaction,
                    parent_node.loop_depth + frt_elt.max_loop_depth,
                    next_including_candidates.clone()
                )
            },
            InteractionInclusionCheckingStepKind::Normalize(included_candidate,including_candidates) => {
                InteractionInclusionCheckingNode::new(
                    included_candidate.clone(), 
                    parent_node.loop_depth,
                    including_candidates.clone()
                )
            },
            InteractionInclusionCheckingStepKind::ContextSimplification(included_candidate,including_candidates) => {
                InteractionInclusionCheckingNode::new(
                    included_candidate.clone(), 
                    parent_node.loop_depth,
                    including_candidates.clone()
                )
            },
        }
    }

    fn collect_next_steps(
        _context_and_param : &InteractionInclusionCheckingContextAndParameterization,
        parent_node : &InteractionInclusionCheckingNode
    ) -> Vec<InteractionInclusionCheckingStepKind> {
        if let Some((smpl_included,smpl_including)) = simplify_context(
            &parent_node.included_candidate,
            &parent_node.including_candidates
        ) {
            return vec![
                InteractionInclusionCheckingStepKind::ContextSimplification(
                    smpl_included, 
                    smpl_including
                )
                ];
        }
        // ***
        // *** ***
        // ***
        let normalized_included_candidate = canonize_interaction(
            &parent_node.included_candidate, 
            None, 
            true
        );
        let mut normalized_including_candidate = btreeset![];
        for cand in &parent_node.including_candidates {
            normalized_including_candidate.insert(
                canonize_interaction(
                        cand, 
                    None, 
                    true
                )
            );
        }
        if normalized_included_candidate != parent_node.included_candidate || normalized_including_candidate != parent_node.including_candidates {
            return vec![
                InteractionInclusionCheckingStepKind::Normalize(
                    normalized_included_candidate, 
                    normalized_including_candidate
                )
                ];
        }
        // ***
        // *** ***
        // ***
        let mut executions = vec![];
        for frt_elt in global_frontier(&parent_node.included_candidate,  true) {
            let mut next_including = btreeset! {};
            for inc_cand in &parent_node.including_candidates {
                for cand_frt_elt in global_frontier(inc_cand, true) {
                    if cand_frt_elt.target_actions == frt_elt.target_actions {
                        let follow_up = execute_interaction(
                            inc_cand, 
                            &cand_frt_elt.position, 
                            &cand_frt_elt.target_lf_ids, 
                            false
                        ).interaction;
                        next_including.insert(follow_up);
                    }
                }
            }
            executions.push(InteractionInclusionCheckingStepKind::ExecuteAction(frt_elt, next_including));
        }
        executions
    }

}




fn same_context_on_the_left(
    op : HibouRewritableLangOperator,
    left_context_int : &Interaction,
    including : &BTreeSet<Interaction>
) -> Option<BTreeSet<Interaction>> {
    match op {
        HibouRewritableLangOperator::Strict => {
            let mut ctx_smpl_inc = btreeset! {};
            for inc in including {
                match inc {
                    Interaction::Strict(i1, i2) => {
                        if &**i1 == left_context_int {
                            ctx_smpl_inc.insert((**i2).clone());
                        } else {
                            return None;
                        }
                    },
                    _ => {
                        return None;
                    }
                }
            }
            Some(ctx_smpl_inc)
        },
        HibouRewritableLangOperator::Alt => {
            let mut ctx_smpl_inc = btreeset! {};
            for inc in including {
                match inc {
                    Interaction::Alt(i1, i2) => {
                        if &**i1 == left_context_int {
                            ctx_smpl_inc.insert((**i2).clone());
                        } else {
                            return None;
                        }
                    },
                    _ => {
                        return None;
                    }
                }
            }
            Some(ctx_smpl_inc)
        },
        HibouRewritableLangOperator::CoReg(cr) => {
            let mut ctx_smpl_inc = btreeset! {};
            for inc in including {
                match inc {
                    Interaction::CoReg(inc_cr, i1, i2) => {
                        if inc_cr == &cr && &**i1 == left_context_int {
                            ctx_smpl_inc.insert((**i2).clone());
                        } else {
                            return None;
                        }
                    },
                    _ => {
                        return None;
                    }
                }
            }
            Some(ctx_smpl_inc)
        },
        _ => {
            None 
        }
    }
}


fn same_context_on_the_right(
    op : HibouRewritableLangOperator,
    right_context_int : &Interaction,
    including : &BTreeSet<Interaction>
) -> Option<BTreeSet<Interaction>> {
    match op {
        HibouRewritableLangOperator::Strict => {
            let mut ctx_smpl_inc = btreeset! {};
            for inc in including {
                match inc {
                    Interaction::Strict(i1, i2) => {
                        if &**i2 == right_context_int {
                            ctx_smpl_inc.insert((**i1).clone());
                        } else {
                            return None;
                        }
                    },
                    _ => {
                        return None;
                    }
                }
            }
            Some(ctx_smpl_inc)
        },
        HibouRewritableLangOperator::Alt => {
            let mut ctx_smpl_inc = btreeset! {};
            for inc in including {
                match inc {
                    Interaction::Alt(i1, i2) => {
                        if &**i2 == right_context_int {
                            ctx_smpl_inc.insert((**i1).clone());
                        } else {
                            return None;
                        }
                    },
                    _ => {
                        return None;
                    }
                }
            }
            Some(ctx_smpl_inc)
        },
        HibouRewritableLangOperator::CoReg(cr) => {
            let mut ctx_smpl_inc = btreeset! {};
            for inc in including {
                match inc {
                    Interaction::CoReg(inc_cr, i1, i2) => {
                        if inc_cr == &cr && &**i2 == right_context_int {
                            ctx_smpl_inc.insert((**i1).clone());
                        } else {
                            return None;
                        }
                    },
                    _ => {
                        return None;
                    }
                }
            }
            Some(ctx_smpl_inc)
        },
        _ => {
            None 
        }
    }
}




fn same_context_unary(
    op : HibouRewritableLangOperator,
    including : &BTreeSet<Interaction>
) -> Option<BTreeSet<Interaction>> {    
    match op {
        HibouRewritableLangOperator::Loop(lk) => {
            let mut ctx_smpl_inc = btreeset! {};
            for inc in including {
                match inc {
                    Interaction::Loop(inc_lk, i1) => {
                        if inc_lk == &lk {
                            ctx_smpl_inc.insert((**i1).clone());
                        } else {
                            return None;
                        }
                    },
                    _ => {
                        return None;
                    }
                }
            }
            Some(ctx_smpl_inc)
        },
        _ => {
            None 
        } 
    }
}

fn simplify_context(
    included : &Interaction, 
    including : &BTreeSet<Interaction>
) -> Option<(Interaction,BTreeSet<Interaction>)> {
    match included {
        Interaction::Alt(i1, i2) => {
            if let Some(x) = same_context_on_the_right(
                HibouRewritableLangOperator::Alt,
                i2,
                including
            ) {
                return Some(((**i1).clone(),x));
            }
            // ***
            if let Some(x) = same_context_on_the_left(
                HibouRewritableLangOperator::Alt,
                i1,
                including
            ) {
                return Some(((**i2).clone(),x));
            }
        },
        Interaction::Strict(i1, i2) => {
            if let Some(x) = same_context_on_the_right(
                HibouRewritableLangOperator::Strict,
                i2,
                including
            ) {
                return Some(((**i1).clone(),x));
            }
            // ***
            if let Some(x) = same_context_on_the_left(
                HibouRewritableLangOperator::Strict,
                i1,
                including
            ) {
                return Some(((**i2).clone(),x));
            }
        },
        Interaction::CoReg(cr, i1, i2) => {
            if let Some(x) = same_context_on_the_right(
                HibouRewritableLangOperator::CoReg(cr.clone()),
                i2,
                including
            ) {
                return Some(((**i1).clone(),x));
            }
            // ***
            if let Some(x) = same_context_on_the_left(
                HibouRewritableLangOperator::CoReg(cr.clone()),
                i1,
                including
            ) {
                return Some(((**i2).clone(),x));
            }
        },
        Interaction::Loop(lk, i1) => {
            if let Some(x) = same_context_unary(
                HibouRewritableLangOperator::Loop(lk.clone()),
                including
            ) {
                return Some(((**i1).clone(),x));
            }
        }
        _ => {
            // ***
        }
    }
    None 
}