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




use common_sequence_diagram_io::conversion::lang_to_repr::FromInteractionTermToInternalRepresentation;

use crate::core::syntax::interaction::Interaction;
use crate::seqdiag_lib_interface::internal_representation::*;






impl FromInteractionTermToInternalRepresentation<HibouLangCioII> for Interaction {

    fn get_subinteractions<'a>(&'a self) -> Vec<&'a Self> {
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



    fn identify_pattern_at_interaction_root(&self) -> Option<HibouLeafPattern> {
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
            Interaction::Strict(i1, i2) => {
                if let Interaction::Emission(ref em_act) = **i1 {
                    // if on the left of the strict we have an emission "act1 = l1!m1"
                    if let Some(HibouLeafPattern::BROADCAST(b2)) = i2.identify_pattern_at_interaction_root() {
                        if b2.origin == HibouBroadcastOrigin::ENV && b2.msg_id == em_act.ms_id {
                            // and on the right of the strict we have identified a pattern of the form "seq(l2?m1,...)" i.e.
                            // a broadcast pattern with no known origin and the same message "m1"
                            // then we return the broadcast, having found the origin as "l1" i.e. "act1.lf_id"

                            let broadcast = HibouBroadcastLeafPattern::new(
                                HibouBroadcastOrigin::LF(em_act.orig_lf_id),
                                em_act.ms_id,
                                b2.lf_targets,
                                em_act.target_gates.clone()
                            );
                            return Some(HibouLeafPattern::BROADCAST(broadcast));
                        }
                    } 
                }
                return None;
            },
            Interaction::CoReg(cr,i1, i2) => {
                if cr.is_empty() {
                    if let (
                        Some(HibouLeafPattern::BROADCAST(b1)),
                        Some(HibouLeafPattern::BROADCAST(b2))
                    ) = (i1.identify_pattern_at_interaction_root(),i2.identify_pattern_at_interaction_root()) {
                        let mut gt_targs = vec![];
                        for t in b1.gt_targets {
                            gt_targs.push(t);
                        }
                        for t in b2.gt_targets {
                            if !gt_targs.contains(&t) {
                                gt_targs.push(t);
                            }
                        }
                        let same_message = b1.msg_id == b2.msg_id;
                        if same_message {
                            // both patterns concern the same message
                            if b1.origin == HibouBroadcastOrigin::ENV && b2.origin == HibouBroadcastOrigin::ENV {
                                // we merge two receptions casts to a single reception cast
                                let mut lf_targets = b1.lf_targets;
                                lf_targets.extend(b2.lf_targets);
                                return Some(
                                    HibouLeafPattern::BROADCAST(
                                        HibouBroadcastLeafPattern::new(
                                            HibouBroadcastOrigin::ENV, 
                                            b1.msg_id, 
                                            lf_targets,
                                            gt_targs
                                        )
                                    )
                                );
                            }
                            // ***
                            if let HibouBroadcastOrigin::LF(orig_lf) = b1.origin {
                                // we have a broadcast from a given lifeline on the left
                                if b1.lf_targets.is_empty() && b2.lf_targets == vec![orig_lf] {
                                    // we have an emission to self
                                    return Some(
                                        HibouLeafPattern::BROADCAST(
                                            HibouBroadcastLeafPattern::new(
                                                HibouBroadcastOrigin::LF(orig_lf), 
                                                b1.msg_id, 
                                                vec![orig_lf],
                                                gt_targs
                                            )
                                        )
                                    );
                                }
                            }
                        }
                    }
                }
                return None;
            },
            _ => {
                return None;
            }
        };
    }

}







