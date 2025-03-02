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

use common_sequence_diagram_io::conversion::lang_to_repr::FromInteractionTermToInternalRepresentation;
use maplit::hashset;

use crate::core::syntax::interaction::Interaction;
use crate::seqdiag_lib_interface::internal_representation::*;






impl FromInteractionTermToInternalRepresentation<HibouLangCioII> for Interaction {

    fn get_subinteractions(&self) -> Vec<&Self> {
        match self {
            Interaction::Strict(i1, i2) => {
                vec![&*i1,&*i2]
            },
            Interaction::Alt(i1, i2) => {
                vec![&*i1,&*i2]
            },
            Interaction::CoReg(_,i1, i2) => {
                vec![&*i1,&*i2]
            },
            Interaction::Loop(_,i1) => {
                vec![&*i1]
            },
            Interaction::And(i1, i2) => {
                vec![&*i1,&*i2]
            }
            Interaction::Empty => {
                vec![]
            },
            Interaction::Emission(_) => {
                vec![]
            },
            Interaction::Reception(_) => {
                vec![]
            },
        }
    }
    
    fn get_operator_at_root(&self) -> Option<HibouOperators> {
        match self {
            Interaction::Strict(_,_) => {
                Some(HibouOperators::Strict)
            },
            Interaction::Alt(_,_) => {
                Some(HibouOperators::Alt)
            },
            Interaction::CoReg(cr,_,_) => {
                Some(HibouOperators::Coreg(cr.clone()))
            },
            Interaction::Loop(lk,_) => {
                Some(HibouOperators::Loop(lk.clone()))
            },
            Interaction::And(_,_) => {
                Some(HibouOperators::And)
            },
            _ => {
                None 
            }
        }
    }



    fn identify_pattern_at_interaction_leaf(&self) -> Option<HibouLeafPattern> {
        match self {
            Interaction::Empty => {
                return Some(HibouLeafPattern::EMPTY);
            },
            Interaction::Emission(em_act) => {
                return Some(
                    HibouLeafPattern::BROADCAST(
                        HibouBroadcastLeafPattern::new(
                            HibouBroadcastOrigin::LF(em_act.orig_lf_id), 
                            em_act.ms_id, 
                            vec![], 
                            em_act.target_gates.clone()
                        )
                    )
                );
            },
            Interaction::Reception(rc_act) => {
                let origin = match rc_act.origin_gate {
                    Some(gt_id) => {
                        HibouBroadcastOrigin::GT(gt_id)
                    },
                    None => {
                        HibouBroadcastOrigin::ENV
                    },
                };
                return Some(
                    HibouLeafPattern::BROADCAST(
                        HibouBroadcastLeafPattern::new(
                            origin, 
                            rc_act.ms_id, 
                            vec![rc_act.targ_lf_id], 
                            vec![]
                        )
                    )
                );
            },
            _ => {
                return None;
            }
        };
    }
    
    fn merge_patterns_under_operator_if_possible(
        parent_op : &HibouOperators,
        p1 : &HibouLeafPattern,
        p2 : &HibouLeafPattern
    ) -> Option<HibouLeafPattern> {
        match (p1,p2) {
            (HibouLeafPattern::BROADCAST(b1),HibouLeafPattern::BROADCAST(b2)) => {
                match parent_op {
                    HibouOperators::Strict => {
                        // b1 must be an emission and b2 a reception of the same message
                        if b1.msg_id == b2.msg_id && 
                        b1.origin.is_lifeline() && 
                        b2.origin.is_environment() {
                            // here we have found a match
                            let mut gt_targs = vec![];
                            for t in b1.gt_targets.iter().cloned() {
                                gt_targs.push(t);
                            }
                            for t in b2.gt_targets.iter().cloned() {
                                if !gt_targs.contains(&t) {
                                    gt_targs.push(t);
                                }
                            }
                            let mut lf_targets = b1.lf_targets.clone();
                                lf_targets.extend(b2.lf_targets.iter().cloned());
                            let broadcast = HibouBroadcastLeafPattern::new(
                                b1.origin.clone(),
                                b1.msg_id,
                                lf_targets,
                                gt_targs
                            );
                            Some(HibouLeafPattern::BROADCAST(broadcast))
                        } else {
                            None 
                        }
                    },
                    HibouOperators::Coreg(cr) => {
                        if !cr.is_empty() {
                            return None;
                        }
                        // b1 and b2 must involve the same message
                        // either both are receptions
                        // or b1 is an emission occurring on the same lifeline than b2
                        if b1.msg_id == b2.msg_id {
                            let do_merge = match (&b1.origin, &b2.origin) {
                                (HibouBroadcastOrigin::ENV,HibouBroadcastOrigin::ENV) => {
                                    true
                                },
                                (HibouBroadcastOrigin::LF(orig_lf),HibouBroadcastOrigin::ENV) => {
                                    let b2_targs_as_hashset : HashSet<usize> = b2.lf_targets.iter().cloned().collect();
                                    b2_targs_as_hashset == hashset!{*orig_lf}
                                },
                                (_,_) => {
                                    false 
                                }
                            };
                            if do_merge {
                                // if we can merge the patterns, we do so
                                let mut gt_targs = vec![];
                                for t in b1.gt_targets.iter().cloned() {
                                    gt_targs.push(t);
                                }
                                for t in b2.gt_targets.iter().cloned() {
                                    if !gt_targs.contains(&t) {
                                        gt_targs.push(t);
                                    }
                                }
                                // ***
                                let mut lf_targets = b1.lf_targets.clone();
                                lf_targets.extend(b2.lf_targets.iter().cloned());
                                // ***
                                return Some(
                                    HibouLeafPattern::BROADCAST(
                                        HibouBroadcastLeafPattern::new(
                                            b1.origin.clone(), 
                                            b1.msg_id, 
                                            lf_targets,
                                            gt_targs
                                        )
                                    )
                                );
                            } else {
                                None 
                            }
                        } else {    
                            None 
                        }
                    },
                    _ => {
                        None 
                    }
                }
            },
            (_,_) => {
                None
            }
        }
    }

}







