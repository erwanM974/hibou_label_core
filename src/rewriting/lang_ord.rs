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

use super::lang::HibouLangOperators;



pub fn compare_hibou_lang_operators(op1: &HibouLangOperators, op2: &HibouLangOperators) -> Ordering {
    match (op1,op2) {
        (HibouLangOperators::Empty,HibouLangOperators::Empty) => {
            Ordering::Equal
        },
        (HibouLangOperators::Empty,_) => {
            Ordering::Less
        },
        (_,HibouLangOperators::Empty) => {
            Ordering::Greater
        },
        // ***
        (HibouLangOperators::Emission(em1),HibouLangOperators::Emission(em2)) => {
            em1.cmp(em2)
        },
        (HibouLangOperators::Emission(_),_) => {
            Ordering::Less
        },
        (_,HibouLangOperators::Emission(_)) => {
            Ordering::Greater
        },
        // ***
        (HibouLangOperators::Reception(rc1),HibouLangOperators::Reception(rc2)) => {
            rc1.cmp(rc2)
        },
        (HibouLangOperators::Reception(_),_) => {
            Ordering::Less
        },
        (_,HibouLangOperators::Reception(_)) => {
            Ordering::Greater
        },
        // ***
        (HibouLangOperators::CoReg(cr1),HibouLangOperators::CoReg(cr2)) => {
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
        (HibouLangOperators::CoReg(_),_) => {
            Ordering::Less
        },
        (_,HibouLangOperators::CoReg(_)) => {
            Ordering::Greater
        },
        // ***
        (HibouLangOperators::Strict,HibouLangOperators::Strict) => {
            Ordering::Equal
        },
        (HibouLangOperators::Strict,_) => {
            Ordering::Less
        },
        (_,HibouLangOperators::Strict) => {
            Ordering::Greater
        },
        // ***
        (HibouLangOperators::Alt,HibouLangOperators::Alt) => {
            Ordering::Equal
        },
        (HibouLangOperators::Alt,_) => {
            Ordering::Less
        },
        (_,HibouLangOperators::Alt) => {
            Ordering::Greater
        },
        // ***
        (HibouLangOperators::Loop(k1),HibouLangOperators::Loop(k2)) => {
            k1.cmp(k2)
        },
        (HibouLangOperators::Loop(_),_) => {
            Ordering::Less
        },
        (_,HibouLangOperators::Loop(_)) => {
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
        (HibouLangOperators::And,HibouLangOperators::And) => {
            Ordering::Equal
        }
    }
}






