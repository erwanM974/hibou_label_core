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




pub struct HibouLeftSequencingCompatibilizer {}




// strict(seq(i1,...in),i0) -> seq( isn, strict(seq(is1,...,is(n-1)), i0) ) if some conditions are met
impl GenericBinaryOperatorSimplifier<HibouRewritableLangOperator> for HibouLeftSequencingCompatibilizer {
    fn is_binary(&self, op : &HibouRewritableLangOperator) -> bool {
        op.arity() == 2
    }

    fn try_simplify_under_binary_operator(
        &self,
        top_operator : &HibouRewritableLangOperator,
        left : &LanguageTerm<HibouRewritableLangOperator>,
        right : &LanguageTerm<HibouRewritableLangOperator>,
    ) -> Option<LanguageTerm<HibouRewritableLangOperator>> {
        if top_operator == &HibouRewritableLangOperator::Strict && left.operator == HibouRewritableLangOperator::CoReg(vec![]) {
            // we have a term of the form strict(seq(i1,...in),i0)
            let i0: &LanguageTerm<HibouRewritableLangOperator> = right;
            // ***
            let first_locations_on_i0 : HashSet<usize> = {
                let i0_as_int = Interaction::from_rewritable_term(
                    i0
                );
                let first_actions_of_i0 = global_frontier(&i0_as_int,true);
                // ***
                first_actions_of_i0.iter()
                .fold( 
                    HashSet::new(),
                    |mut p, x| {p.extend(x.target_lf_ids.clone()); p}
                )
            };
            // ***
            let unfolded_on_the_left = get_associative_sub_terms_recursively(
                left,
                &left.operator
            );
            // ***
            // we look for the i_k we will flush to the left within "unfolded_on_the_left" 
            for idx in 0..unfolded_on_the_left.len() {
                let ik = unfolded_on_the_left.get(idx).unwrap();
                let empty_cr = vec![];
                let may_commute = if idx > 0 {
                    (0..(idx-1)).all(|earlier_idx| {
                        let earlier_i = unfolded_on_the_left.get(earlier_idx).unwrap();
                        may_commute_under_coreg(&empty_cr, earlier_i, ik)
                    })
                } else {
                    true
                };
                if may_commute {
                    let last_locations_on_ik : HashSet<usize> = {
                        let ik_as_int = Interaction::from_rewritable_term(
                            ik
                        );
                        let last_actions_of_ik = global_frontier(&ik_as_int.reverse_interaction(),true);
                        last_actions_of_ik.iter()
                        .fold( 
                            HashSet::new(),
                            |mut p, x| {p.extend(x.target_lf_ids.clone()); p}
                        )
                    };
                    // ***
                    let do_transform = if last_locations_on_ik.len() == 1 && first_locations_on_i0 == last_locations_on_ik {
                        true 
                    }  else {
                        let involved_later = {
                            let mut involved_later = btreeset! {};
                            for later_idx in (idx+1)..unfolded_on_the_left.len() {
                                let later_i = unfolded_on_the_left.get(later_idx).unwrap();
                                let must_be_involved_in_later_i = Interaction::from_rewritable_term(&later_i)
                                .lifelines_that_must_be_involved();
                                involved_later.extend(must_be_involved_in_later_i);
                            }
                            involved_later
                        };
                        last_locations_on_ik.iter().all(|lf_id| involved_later.contains(lf_id))
                    };
                    // ***
                    if do_transform {
                        // strict(seq(i1,...in),i0) -> seq( isn, strict(seq(is1,...,is(n-1)), i0) )
                        // here isn is ik
                        let new_right_left = {
                            let mut other_sub_inteactions_except_ik = vec![];
                            for (other_idx, other_sub_int) in unfolded_on_the_left.iter().enumerate() {
                                if other_idx != idx {
                                    other_sub_inteactions_except_ik.push((*other_sub_int).clone())
                                }
                            }
                            fold_associative_sub_terms_recursively(
                                &left.operator,
                                &mut other_sub_inteactions_except_ik,
                                &None
                            )
                        };
                        let new_right = LanguageTerm::new(
                            HibouRewritableLangOperator::Strict,
                            vec![new_right_left,i0.clone()]
                        );
                        return Some(
                            LanguageTerm::new(
                                HibouRewritableLangOperator::CoReg(vec![]),
                                vec![
                                    (*ik).clone(),
                                    new_right
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




