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


use common_sequence_diagram_io::conversion::repr_to_lang::FromInternalRepresentationToInteractionTerm;

use crate::core::syntax::action::{EmissionAction, ReceptionAction};
use crate::core::syntax::interaction::Interaction;

use crate::seqdiag_lib_interface::internal_representation::{HibouBroadcastOrigin, HibouLangCioII, HibouLeafPattern, HibouOperators};




impl FromInternalRepresentationToInteractionTerm<HibouLangCioII> for Interaction {

    fn instantiate_interaction_under_operator(operator : &HibouOperators, sub_ints : &mut Vec<Self>) -> Option<Self> {
        if let HibouOperators::Loop(lk) = operator {
            let i1 = sub_ints.pop().unwrap();
            return Some(Interaction::Loop(lk.clone(),Box::new(i1)));
        } 
        let i2 = sub_ints.pop().unwrap();
        let i1 = sub_ints.pop().unwrap();
        match operator {
            HibouOperators::Strict => {
                Some(Interaction::Strict(Box::new(i1), Box::new(i2)))
            },
            HibouOperators::Coreg(cr) => {
                Some(Interaction::CoReg(cr.clone(), Box::new(i1), Box::new(i2)))
            },
            HibouOperators::Alt => {
                Some(Interaction::Alt(Box::new(i1), Box::new(i2)))
            },
            HibouOperators::And => {
                Some(Interaction::And(Box::new(i1), Box::new(i2)))
            }
            HibouOperators::Loop(_) => {
                panic!()
            }
        }
        
    }

    fn get_empty_interaction() -> Self {
        Interaction::Empty
    }
    
    fn transform_pattern_to_term(pattern : &HibouLeafPattern) -> Interaction {
        match pattern {
            HibouLeafPattern::EMPTY => {
                Interaction::Empty
            },
            HibouLeafPattern::BROADCAST(broadcast) => {
                if broadcast.lf_targets.is_empty() {
                    match broadcast.origin {
                        HibouBroadcastOrigin::LF(orig_lf_id) => {
                            let emission = EmissionAction::new(
                                orig_lf_id, 
                                broadcast.msg_id,
                                broadcast.gt_targets.clone()
                            );
                            Interaction::Emission(emission)
                        },
                        _ => {
                            panic!()
                        }
                    }
                } else {
                    match broadcast.origin {
                        HibouBroadcastOrigin::GT(gt_id) => {
                            let mut receptions = broadcast.lf_targets.iter().map(
                                |lf_id| Interaction::Reception(ReceptionAction::new( Some(gt_id),broadcast.msg_id,*lf_id,))
                            ).collect();
                            let recs_int = Self::fold_associative_operands_recursively(&HibouOperators::Coreg(vec![]), &mut receptions);
                            recs_int
                        },
                        HibouBroadcastOrigin::ENV => {
                            let mut receptions = broadcast.lf_targets.iter().map(
                                |lf_id| Interaction::Reception(ReceptionAction::new(None, broadcast.msg_id,*lf_id))
                            ).collect();
                            let recs_int = Self::fold_associative_operands_recursively(&HibouOperators::Coreg(vec![]), &mut receptions);
                            recs_int
                        },
                        HibouBroadcastOrigin::LF(orig_lf_id) => {
                            let mut receptions = broadcast.lf_targets.iter().map(
                                |lf_id| Interaction::Reception(ReceptionAction::new(None, broadcast.msg_id,*lf_id))
                            ).collect();
                            let recs_int = Self::fold_associative_operands_recursively(
                                &HibouOperators::Coreg(vec![]), 
                                &mut receptions
                            );
                            let em_int = Interaction::Emission(
                                EmissionAction::new(orig_lf_id, broadcast.msg_id,broadcast.gt_targets.clone())
                            );
                            Interaction::Strict(Box::new(em_int), Box::new(recs_int))
                        }
                    }
                }
            }
        }

    }
}







