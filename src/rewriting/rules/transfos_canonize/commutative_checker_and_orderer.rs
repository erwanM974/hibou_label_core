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

use simple_term_rewriter::core::conversion::from_rewritable_term::FromRewritableTermToDomainSpecificTerm;
use simple_term_rewriter::{builtin_trs::rules::reorder_commute::CommutativeCheckerAndOrderer, core::term::LanguageTerm};

use crate::core::syntax::interaction::Interaction;
use crate::core::syntax::lang_traits::involve::involves::InvolvesLifelines;
use crate::rewriting::lang::HibouRewritableLangOperator;
use crate::rewriting::lang_ord::compare_hibou_lang_operators;



pub struct HibouCommutativeCheckerAndOrderer {
    pub consider_alt : bool, 
    pub consider_coreg : bool, 
}


impl CommutativeCheckerAndOrderer<HibouRewritableLangOperator> for HibouCommutativeCheckerAndOrderer {
    fn may_commute_under(
        &self,
        parent_op :&HibouRewritableLangOperator,
        left_sub_term : &LanguageTerm<HibouRewritableLangOperator>,
        right_sub_term : &LanguageTerm<HibouRewritableLangOperator>,
    ) -> bool {
        match parent_op {
            HibouRewritableLangOperator::Alt => {
                self.consider_alt
            },
            HibouRewritableLangOperator::CoReg(cr) => {
                if self.consider_coreg {
                    let i1 = Interaction::from_rewritable_term(
                        left_sub_term
                    );
                    let i2 = Interaction::from_rewritable_term(
                        right_sub_term
                    );
                    let involved_in_both : HashSet<usize> = i1.involved_lifelines().intersection(&i2.involved_lifelines()).cloned().collect();
                    // if the concurrent region contains all the lifelines that are involved in both interactions then they may commute
                    involved_in_both.iter().all(|lf| cr.contains(lf))
                } else {
                    false
                }
            },
            _ => {
                false 
            }
        }
    }

    fn compare_operators(
        &self,
        op1 : &HibouRewritableLangOperator,
        op2 : &HibouRewritableLangOperator
    ) -> std::cmp::Ordering {
        compare_hibou_lang_operators(op1,op2)
    }

    fn get_arity(
        &self,
        op : &HibouRewritableLangOperator
    ) -> usize {
        op.arity()
    }
    
    fn is_a_binary_operator_we_may_consider(
        &self,
        op : &HibouRewritableLangOperator
    ) -> bool {
        match op {
            HibouRewritableLangOperator::Alt => {
                self.consider_alt
            },
            HibouRewritableLangOperator::CoReg(_) => {
                self.consider_coreg
            }   
            _ => {
                false
            }
        }
    }
    
    fn is_associative(
        &self,
        op : &HibouRewritableLangOperator
    ) -> bool {
        op.is_binary_associative()
    }
}














