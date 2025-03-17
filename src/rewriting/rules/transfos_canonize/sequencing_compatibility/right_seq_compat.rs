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

use maplit::btreeset;
use simple_term_rewriter::builtin_trs::rules::simpl_binary::GenericBinaryOperatorSimplifier;
use simple_term_rewriter::builtin_trs::util::{fold_associative_sub_terms_recursively, get_associative_sub_terms_recursively};
use simple_term_rewriter::core::terms::conversion::from_rewritable_term::FromRewritableTermToDomainSpecificTerm;
use simple_term_rewriter::core::terms::term::LanguageTerm;

use crate::core::semantics::frontier::global_frontier;
use crate::core::syntax::interaction::Interaction;
use crate::core::syntax::lang_traits::involve::involves::InvolvesLifelines;
use crate::rewriting::lang::HibouRewritableLangOperator;

use crate::rewriting::rules::transfos_canonize::commutative_checker_and_orderer::may_commute_under_coreg;




pub struct HibouRightSequencingCompatibilizer {}




// strict(i1,seq(i2,...in)) -> seq( strict(i1,seq(is1,...,is(n-1))), isn) ) if some conditions are met
impl GenericBinaryOperatorSimplifier<HibouRewritableLangOperator> for HibouRightSequencingCompatibilizer {
    fn is_binary(&self, op : &HibouRewritableLangOperator) -> bool {
        op.arity() == 2
    }

    fn try_simplify_under_binary_operator(
        &self,
        top_operator : &HibouRewritableLangOperator,
        left : &LanguageTerm<HibouRewritableLangOperator>,
        right : &LanguageTerm<HibouRewritableLangOperator>,
    ) -> Option<LanguageTerm<HibouRewritableLangOperator>> {
        if top_operator == &HibouRewritableLangOperator::Strict && right.operator == HibouRewritableLangOperator::CoReg(vec![]) {
            // we have a term of the form strict(i1,seq(i2,...in))
            let i1 = left;
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
            let unfolded_on_the_right = get_associative_sub_terms_recursively(
                right,
                &right.operator
            );
            // ***
            // we look for the i_k we will flush to the right within "unfolded_on_the_right" 
            for idx in (0..unfolded_on_the_right.len()).rev() {
                let ik = unfolded_on_the_right.get(idx).unwrap();
                let empty_cr = vec![];
                let may_commute = ((idx+1)..unfolded_on_the_right.len()).all(|later_idx| {
                    let later_i = unfolded_on_the_right.get(later_idx).unwrap();
                    may_commute_under_coreg(&empty_cr, ik, later_i)
                });
                if may_commute {
                    let first_locations_on_ik : HashSet<usize> = {
                        let ik_as_int = Interaction::from_rewritable_term(
                            ik
                        );
                        let first_actions_of_ik = global_frontier(&ik_as_int,true);
                        first_actions_of_ik.iter()
                        .fold( 
                            HashSet::new(),
                            |mut p, x| {p.extend(x.target_lf_ids.clone()); p}
                        )
                    };
                    // ***
                    let do_transform = if last_locations_on_i1.len() == 1 && last_locations_on_i1 == first_locations_on_ik {
                        true 
                    } else {
                        let involved_earlier = {
                            let mut involed_earlier = btreeset! {};
                            for earlier_idx in 0..idx {
                                let earlier_i = unfolded_on_the_right.get(earlier_idx).unwrap();
                                let must_be_involved_in_earlier_i = Interaction::from_rewritable_term(&earlier_i)
                                .lifelines_that_must_be_involved();
                                involed_earlier.extend(must_be_involved_in_earlier_i);
                            }
                            involed_earlier
                        };
                        first_locations_on_ik.iter().all(|lf_id| involved_earlier.contains(lf_id))
                    };
                    // ***
                    if do_transform {
                        // strict(i1,seq(i2,...in)) -> seq( strict(i1,seq(is1,...,is(n-1))), isn) )
                        // here isn is ik
                        let new_left_right = {
                            let mut other_sub_inteactions_except_ik = vec![];
                            for (other_idx, other_sub_int) in unfolded_on_the_right.iter().enumerate() {
                                if other_idx != idx {
                                    other_sub_inteactions_except_ik.push((*other_sub_int).clone())
                                }
                            }
                            fold_associative_sub_terms_recursively(
                                &right.operator,
                                &mut other_sub_inteactions_except_ik,
                                &None
                            )
                        };
                        let new_left = LanguageTerm::new(
                            HibouRewritableLangOperator::Strict,
                            vec![i1.clone(),new_left_right]
                        );
                        return Some(
                            LanguageTerm::new(
                                HibouRewritableLangOperator::CoReg(vec![]),
                                vec![
                                    new_left,
                                    (*ik).clone()
                                ]
                            )
                        );
                    }
                }
            }
        }
        None 
    }

}




