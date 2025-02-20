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
use simple_term_rewriter::core::conversion::from_rewritable_term::FromRewritableTermToDomainSpecificTerm;
use simple_term_rewriter::core::term::LanguageTerm;

use crate::core::semantics::frontier::global_frontier;
use crate::core::syntax::interaction::Interaction;
use crate::rewriting::lang::HibouRewritableLangOperator;




pub struct HibouSequencingCompatibilizer {}







// seq(strict(i1,i2),i3)) -> strict(i1,seq(i2,i3)) if some conditions are met
impl GenericBinaryOperatorSimplifier<HibouRewritableLangOperator> for HibouSequencingCompatibilizer {
    fn is_binary(&self, op : &HibouRewritableLangOperator) -> bool {
        op.arity() == 2
    }

    fn try_simplify_under_binary_operator(
        &self,
        top_operator : &HibouRewritableLangOperator,
        left : &LanguageTerm<HibouRewritableLangOperator>,
        right : &LanguageTerm<HibouRewritableLangOperator>,
    ) -> Option<LanguageTerm<HibouRewritableLangOperator>> {
        if top_operator == &HibouRewritableLangOperator::CoReg(vec![]) && left.operator == HibouRewritableLangOperator::Strict {
            // we have a term of the form seq(strict(i1,i2),i3))
            let i1 = left.sub_terms.first().unwrap();
            let i2 = left.sub_terms.get(1).unwrap();
            // i3 is "right"
            // ***
            let last_locations_on_i2 : HashSet<usize> = {
                let i2_as_int = Interaction::from_rewritable_term(
                    i2
                );
                let last_locations_on_i2 = global_frontier(&i2_as_int.reverse_interaction(),true);
                // ***
                last_locations_on_i2.iter()
                .fold( 
                    HashSet::new(),
                    |mut p, x| {p.extend(x.target_lf_ids.clone()); p}
                )
            };
            // ***
            if last_locations_on_i2.len() == 1 {
                // if there is a single lifeline such that all first actions on i3 occur on that lifeline
                let first_locations_on_i3 : HashSet<usize> = {
                    let i3 = Interaction::from_rewritable_term(
                        right
                    );
                    let first_actions_of_i3 = global_frontier(&i3,true);
                    first_actions_of_i3.iter()
                    .fold( 
                        HashSet::new(),
                        |mut p, x| {p.extend(x.target_lf_ids.clone()); p}
                    )
                };
                if last_locations_on_i2 == first_locations_on_i3 {
                    // and if all first actions on i3 also occur on that same lifeline
                    // then we can perform the operation
                    // seq(strict(i1,i2),i3)) -> strict(i1,seq(i2,i3)) while preserving the semantics
                    return Some(
                        LanguageTerm::new(
                            HibouRewritableLangOperator::Strict,
                            vec![
                                i1.clone(),
                                LanguageTerm::new(
                                    HibouRewritableLangOperator::CoReg(vec![]),
                                    vec![
                                        i2.clone(),
                                        right.clone()
                                    ]
                                )
                            ]
                        )
                    );
                }
            }
        }
        None 
    }

}




