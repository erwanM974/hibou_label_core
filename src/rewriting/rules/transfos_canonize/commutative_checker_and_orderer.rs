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


use std::cmp::Ordering;
use std::collections::HashSet;

use simple_term_rewriter::core::conversion::from_rewritable_term::FromRewritableTermToDomainSpecificTerm;
use simple_term_rewriter::{builtin_trs::rules::reorder_commute::CommutativeCheckerAndOrderer, core::term::LanguageTerm};

use crate::core::syntax::interaction::Interaction;
use crate::core::syntax::lang_traits::involve::involves::InvolvesLifelines;
use crate::rewriting::lang::HibouRewritableLangOperator;



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










fn compare_hibou_lang_operators(op1: &HibouRewritableLangOperator, op2: &HibouRewritableLangOperator) -> Ordering {
    match (op1,op2) {
        (HibouRewritableLangOperator::Empty,HibouRewritableLangOperator::Empty) => {
            Ordering::Equal
        },
        (HibouRewritableLangOperator::Empty,_) => {
            Ordering::Less
        },
        (_,HibouRewritableLangOperator::Empty) => {
            Ordering::Greater
        },
        // ***
        (HibouRewritableLangOperator::Emission(em1),HibouRewritableLangOperator::Emission(em2)) => {
            em1.cmp(em2)
        },
        (HibouRewritableLangOperator::Emission(_),_) => {
            Ordering::Less
        },
        (_,HibouRewritableLangOperator::Emission(_)) => {
            Ordering::Greater
        },
        // ***
        (HibouRewritableLangOperator::Reception(rc1),HibouRewritableLangOperator::Reception(rc2)) => {
            rc1.cmp(rc2)
        },
        (HibouRewritableLangOperator::Reception(_),_) => {
            Ordering::Less
        },
        (_,HibouRewritableLangOperator::Reception(_)) => {
            Ordering::Greater
        },
        // ***
        (HibouRewritableLangOperator::CoReg(cr1),HibouRewritableLangOperator::CoReg(cr2)) => {
            let max_cr_len = cr1.len().max(cr2.len());
            for i in 0..max_cr_len {
                match (cr1.get(i) ,cr2.get(i) ) {
                    ( Some( cr_ref1 ), Some(cr_ref2) ) => {
                        if cr_ref1 < cr_ref2 {
                            return Ordering::Less;
                        }
                        if cr_ref1 > cr_ref2 {
                            return Ordering::Greater;
                        }
                    },
                    (None,Some(_)) => {
                        return Ordering::Less;
                    },
                    (Some(_),None) => {
                        return Ordering::Greater;
                    },
                    (None,None) => {}
                }
            }
            Ordering::Equal
        },
        (HibouRewritableLangOperator::CoReg(_),_) => {
            Ordering::Less
        },
        (_,HibouRewritableLangOperator::CoReg(_)) => {
            Ordering::Greater
        },
        // ***
        (HibouRewritableLangOperator::Strict,HibouRewritableLangOperator::Strict) => {
            Ordering::Equal
        },
        (HibouRewritableLangOperator::Strict,_) => {
            Ordering::Less
        },
        (_,HibouRewritableLangOperator::Strict) => {
            Ordering::Greater
        },
        // ***
        (HibouRewritableLangOperator::Alt,HibouRewritableLangOperator::Alt) => {
            Ordering::Equal
        },
        (HibouRewritableLangOperator::Alt,_) => {
            Ordering::Less
        },
        (_,HibouRewritableLangOperator::Alt) => {
            Ordering::Greater
        },
        // ***
        (HibouRewritableLangOperator::Loop(k1),HibouRewritableLangOperator::Loop(k2)) => {
            k1.cmp(k2)
        },
        (HibouRewritableLangOperator::Loop(_),_) => {
            Ordering::Less
        },
        (_,HibouRewritableLangOperator::Loop(_)) => {
            Ordering::Greater
        },
        /*(Interaction::Sync(self_acts,self_i1,self_i2),Interaction::Sync(other_acts,other_i1,other_i2)) => {
            let max_acts_len = self_acts.len().max(other_acts.len());
            for i in 0..max_acts_len {
                match (self_acts.get(i) ,other_acts.get(i) ) {
                    ( Some( cr_ref1 ), Some(cr_ref2) ) => {
                        if cr_ref1 < cr_ref2 {
                            return Ordering::Less;
                        }
                        if cr_ref1 > cr_ref2 {
                            return Ordering::Greater;
                        }
                    },
                    (None,Some(_)) => {
                        return Ordering::Less;
                    },
                    (Some(_),None) => {
                        return Ordering::Greater;
                    },
                    (None,None) => {}
                }
            }
            // ***
            let cmp_left = self_i1.cmp(other_i1);
            match &cmp_left {
                Ordering::Equal => {
                    return self_i2.cmp(other_i2);
                },
                _ => {
                    return cmp_left;
                }
            }
        },
        (Interaction::Sync(_,_,_),_) => {
            return Ordering::Less;
        },
        (_,Interaction::Sync(_,_,_)) => {
            return Ordering::Greater;
        },*/
        (HibouRewritableLangOperator::And,HibouRewritableLangOperator::And) => {
            Ordering::Equal
        }
    }
}










