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


use simple_term_rewriter::{builtin_trs::rules::modulo_associative_flattened_transfo::ModuloAssociativeGenericFlattenedChecker, core::{conversion::from_rewritable_term::FromRewritableTermToDomainSpecificTerm, term::LanguageTerm}};

use crate::{core::syntax::interaction::{Interaction, LoopKind}, inclusion_checker::check_inclusion::{check_inclusion_of_interactions, InteractionInclusionGlobalVerdict}, rewriting::lang::HibouRewritableLangOperator};

use super::commutative_checker_and_orderer::may_commute_under_coreg;


pub struct HibouKleeneDesequencer {}


impl ModuloAssociativeGenericFlattenedChecker<HibouRewritableLangOperator> for HibouKleeneDesequencer {
    fn is_an_associative_binary_operator_we_may_consider(
        &self, 
        op : &HibouRewritableLangOperator
    ) -> bool {
        match op {
            HibouRewritableLangOperator::Strict => {
                true
            },
            HibouRewritableLangOperator::CoReg(_) => {
                true
            },
            _ => {
                false 
            }
        }
    }

    fn if_required_is_a_parent_unary_operator_we_may_consider(
        &self,
        _op : &HibouRewritableLangOperator
    ) -> Option<bool> {
        None 
    }

    fn transform_flattened_sub_terms(
        &self, 
        considered_ac_op : &HibouRewritableLangOperator, 
        _considered_parent_op : Option<&HibouRewritableLangOperator>,
        flattened_subterms : Vec<&LanguageTerm<HibouRewritableLangOperator>>
    ) -> Option<(Option<HibouRewritableLangOperator>,Vec<LanguageTerm<HibouRewritableLangOperator>>)> {
        // example transformation:
        // seq(i1,loopW(i2),loopW(i2),i3) -> seq(i1,loop(i2),i3)

        if let Some((earlier_to_remove, later_to_remove, new_loop, index_to_add)) = try_merge_loops_in_sequence_of_sub_interactions(
            considered_ac_op,&flattened_subterms
        ) {
            let mut new_flattened_sub_terms = vec![];
            for (sub_int_idx, sub_int) in flattened_subterms.iter().enumerate() {
                if sub_int_idx != earlier_to_remove && sub_int_idx != later_to_remove {
                    new_flattened_sub_terms.push((*sub_int).clone());
                }
            }
            new_flattened_sub_terms.insert(index_to_add - 1, new_loop);
            return Some((None,new_flattened_sub_terms));
        }

        None
    }
}


/** 
 * Returns the indices of two loop sub-interactions that we may merge
 * **/
fn try_merge_loops_in_sequence_of_sub_interactions(
    sequencing_operator : &HibouRewritableLangOperator,
    sub_interactions : &Vec<&LanguageTerm<HibouRewritableLangOperator>>
) -> Option<(usize,usize,LanguageTerm<HibouRewritableLangOperator>,usize)> {
    for (earlier_sub_int_idx,earlier_sub_int) in sub_interactions.iter().enumerate() {
        if let HibouRewritableLangOperator::Loop(earlier_lk) = &earlier_sub_int.operator {
            let earlier_loop_content = earlier_sub_int.sub_terms.first().unwrap();
            for later_idx in (earlier_sub_int_idx+1)..sub_interactions.len() {
                let later_sub_int = sub_interactions.get(later_idx).unwrap();
                if let HibouRewritableLangOperator::Loop(later_lk) = &later_sub_int.operator {
                    // we have found two loops : one earlier and one later in the sequence of sub-interactions
                    // for them to be mergeable it must be so that:
                    // 1: they are consecutive modulo AC 
                    // 2: one includes the other
                    let consecutive_at_merge_index = {
                        let mut earlier_index_up_to_commutations = earlier_sub_int_idx;
                        let mut later_index_up_to_commutations = later_idx;
                        // ***
                        let mut consecutive = None;
                        'try_make_consecutive : loop {
                            if later_index_up_to_commutations == earlier_index_up_to_commutations + 1 {
                                consecutive = Some(earlier_index_up_to_commutations);
                                break 'try_make_consecutive;
                            }
                            if sequencing_operator == &HibouRewritableLangOperator::Strict {
                                break 'try_make_consecutive;
                            }
                            if let HibouRewritableLangOperator::CoReg(cr) = sequencing_operator {
                                // if the earlier sub_int can commute with the next sub_int, we increase "earlier_index_up_to_commutations"
                                let just_after_earlier = sub_interactions.get(
                                    earlier_index_up_to_commutations + 1
                                ).unwrap();
                                if may_commute_under_coreg(cr, earlier_sub_int, just_after_earlier) {
                                    earlier_index_up_to_commutations += 1;
                                } else {
                                    // if the later sub_int can commute with the previous sub_int, we decrease "later_index_up_to_commutations"
                                    let just_before_later = sub_interactions.get(
                                        later_index_up_to_commutations - 1
                                    ).unwrap();
                                    if may_commute_under_coreg(cr, just_before_later, later_sub_int) {
                                        later_index_up_to_commutations -= 1;
                                    }
                                } 
                            }
                        }
                        consecutive
                    };
                    // ***
                    if let Some(merge_index) = consecutive_at_merge_index {
                        let later_loop_content = later_sub_int.sub_terms.first().unwrap();
                        // condition 1 is met: the two loops are consecutive modulo AC in the sequence of flattened sub-interactions
                        // we now check condition 2
                        if let Some((new_lk,new_loop_content)) = merge_loops(
                            earlier_lk, 
                            earlier_loop_content, 
                            later_lk, 
                            later_loop_content
                        ) {
                            let seq_op_cannot_enrich_interleavings = match sequencing_operator {
                                HibouRewritableLangOperator::Strict => {
                                    true 
                                },
                                HibouRewritableLangOperator::CoReg(seq_op_cr) => {
                                    match &new_lk {
                                        LoopKind::Coreg(lk_cr) => {
                                            seq_op_cr.iter().all(|lf_id| lk_cr.contains(lf_id))
                                        },
                                        _ => {
                                            false 
                                        }
                                    }
                                },
                                _ => {
                                    panic!("should not be reached")
                                }
                            };
                            if seq_op_cannot_enrich_interleavings {
                                // it is safe to remove the two loops at indices "earlier_sub_int_idx" and "later_idx"
                                // and replace them with the new loop, inserted at index "merge_index"
                                let new_loop = LanguageTerm::new(
                                    HibouRewritableLangOperator::Loop(new_lk), 
                                    vec![new_loop_content]
                                );
                                return Some(
                                    (
                                        earlier_sub_int_idx,later_idx,new_loop,merge_index
                                    )
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    None 
}



fn merge_loops(
    loop1_lk : &LoopKind,
    loop1_content : &LanguageTerm<HibouRewritableLangOperator>,
    loop2_lk : &LoopKind,
    loop2_content : &LanguageTerm<HibouRewritableLangOperator>
) -> Option<(LoopKind,LanguageTerm<HibouRewritableLangOperator>)> {
    let l1_as_int = Interaction::from_rewritable_term(loop1_content);
    let l2_as_int = Interaction::from_rewritable_term(loop2_content);
    if loop1_lk.is_more_permissive(loop2_lk).unwrap() && 
    check_inclusion_of_interactions(&l2_as_int,&l1_as_int,None) == InteractionInclusionGlobalVerdict::IsIncluded {
        // we can retain the first loop and eliminate the second
        return Some(
            (
                loop1_lk.clone(),
                loop1_content.clone()
            )
        );
    }
    if loop2_lk.is_more_permissive(loop1_lk).unwrap() && 
    check_inclusion_of_interactions(&l1_as_int,&l2_as_int,None) == InteractionInclusionGlobalVerdict::IsIncluded {
        // we can retain the second loop and eliminate the first
        return Some(
            (
                loop2_lk.clone(),
                loop2_content.clone()
            )
        );
    }
    None 
}

