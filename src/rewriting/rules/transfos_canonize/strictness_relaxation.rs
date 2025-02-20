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



use std::collections::HashSet;

use simple_term_rewriter::builtin_trs::rules::simpl_binary::GenericBinaryOperatorSimplifier;
use simple_term_rewriter::builtin_trs::rules::simpl_unary::GenericUnaryOperatorSimplifier;
use simple_term_rewriter::core::conversion::from_rewritable_term::FromRewritableTermToDomainSpecificTerm;
use simple_term_rewriter::core::term::LanguageTerm;

use crate::core::semantics::frontier::global_frontier;
use crate::core::syntax::interaction::{Interaction, LoopKind};
use crate::rewriting::lang::HibouRewritableLangOperator;


pub struct HibouStrictnessRelaxer {}


// loopS(i1) -> loopW(i1) if some conditions are met
impl GenericUnaryOperatorSimplifier<HibouRewritableLangOperator> for HibouStrictnessRelaxer {
    fn is_unary(&self, op : &HibouRewritableLangOperator) -> bool {
        op.arity() == 1
    }

    fn try_simplify_under_unary_operator(
        &self,
        top_operator : &HibouRewritableLangOperator,
        term_underneath : &LanguageTerm<HibouRewritableLangOperator>
    ) -> Option<LanguageTerm<HibouRewritableLangOperator>> {
        if top_operator == &HibouRewritableLangOperator::Loop(LoopKind::SStrictSeq) {
            let i1 = Interaction::from_rewritable_term(
                term_underneath
            );
            let last_locations_on_i1 : HashSet<usize> = {
                let last_actions_of_i1 = global_frontier(&i1.reverse_interaction(),true);
                // ***
                last_actions_of_i1.iter()
                .fold( 
                    HashSet::new(),
                    |mut p, x| {p.extend(x.target_lf_ids.clone()); p}
                )
            };
            if last_locations_on_i1.len() == 1 {
                // if there is a single lifeline such that all final actions on i1 occur on that lifeline
                let first_locations_on_i1 : HashSet<usize> = {
                    let first_actions_of_i1 = global_frontier(&i1,true);
                    first_actions_of_i1.iter()
                    .fold( 
                        HashSet::new(),
                        |mut p, x| {p.extend(x.target_lf_ids.clone()); p}
                    )
                };
                if last_locations_on_i1 == first_locations_on_i1 {
                    // and if all first actions on i1 also occur on that same lifeline
                    // then we can perform the operation
                    // loopS(i1) -> loopW(i1) while preserving the semantics
                    return Some(
                        LanguageTerm::new(
                            HibouRewritableLangOperator::Loop(LoopKind::Coreg(vec![])),
                            vec![term_underneath.clone()]
                        )
                    );
                }
            }
        }
        None 
    }
}





// strict(i1,i2) -> seq(i1,i2) if some conditions are met
impl GenericBinaryOperatorSimplifier<HibouRewritableLangOperator> for HibouStrictnessRelaxer {
    fn is_binary(&self, op : &HibouRewritableLangOperator) -> bool {
        op.arity() == 2
    }

    fn try_simplify_under_binary_operator(
        &self,
        top_operator : &HibouRewritableLangOperator,
        left : &LanguageTerm<HibouRewritableLangOperator>,
        right : &LanguageTerm<HibouRewritableLangOperator>,
    ) -> Option<LanguageTerm<HibouRewritableLangOperator>> {
        if top_operator == &HibouRewritableLangOperator::Strict {
            // ***
            let last_locations_on_i1 : HashSet<usize> = {
                let i1 = Interaction::from_rewritable_term(
                    left
                );
                let last_actions_of_i1 = global_frontier(&i1.reverse_interaction(),true);
                // ***
                last_actions_of_i1.iter()
                .fold( 
                    HashSet::new(),
                    |mut p, x| {p.extend(x.target_lf_ids.clone()); p}
                )
            };
            // ***
            if last_locations_on_i1.len() == 1 {
                // if there is a single lifeline such that all final actions on i1 occur on that lifeline
                let first_locations_on_i2 : HashSet<usize> = {
                    let i2 = Interaction::from_rewritable_term(
                        right
                    );
                    let first_actions_of_i2 = global_frontier(&i2,true);
                    first_actions_of_i2.iter()
                    .fold( 
                        HashSet::new(),
                        |mut p, x| {p.extend(x.target_lf_ids.clone()); p}
                    )
                };
                if last_locations_on_i1 == first_locations_on_i2 {
                    // and if all first actions on i2 also occur on that same lifeline
                    // then we can perform the operation
                    // strict(i1,i2) -> seq(i1,i2) while preserving the semantics
                    return Some(
                        LanguageTerm::new(
                            HibouRewritableLangOperator::CoReg(vec![]),
                            vec![
                                left.clone(),
                                right.clone(),
                            ]
                        )
                    );
                }
            }
        }
        None 
    }

}


