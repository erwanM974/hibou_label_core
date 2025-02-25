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



    fn identify_pattern_at_interaction_root<'a>(&'a self) -> Option<(HibouLeafPattern,Option<(HibouOperators,&'a Self)>)> {
        match self {
            Interaction::Empty => {
                return Some((HibouLeafPattern::EMPTY,None));
            },
            Interaction::Emission(em_act) => {
                return Some(
                (
                    HibouLeafPattern::BROADCAST(
                        HibouBroadcastLeafPattern::new(
                            HibouBroadcastOrigin::LF(em_act.orig_lf_id), 
                            em_act.ms_id, 
                            vec![], 
                            em_act.target_gates.clone()
                        )
                    ),
                    None
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
                (
                    HibouLeafPattern::BROADCAST(
                        HibouBroadcastLeafPattern::new(
                            origin, 
                            rc_act.ms_id, 
                            vec![rc_act.targ_lf_id], 
                            vec![]
                        )
                    ),
                    None
                )
                );
            },
            Interaction::Strict(i1, i2) => {
                if let Some((HibouLeafPattern::BROADCAST(found_broadcast_on_the_left),rem1)) = i1.identify_pattern_at_interaction_root() {
                    if rem1.is_none() && found_broadcast_on_the_left.origin.is_lifeline() {
                        // if on the left of the strict we have a broadcast without remainders and with a known emitting lifeline
                        // ***
                        // we try to complete it on the right with another broadcast, but without known emitting lifeline
                        if let Some((HibouLeafPattern::BROADCAST(found_broadcast_on_the_right),rem2)) = i2.identify_pattern_at_interaction_root() {
                            if found_broadcast_on_the_right.origin == HibouBroadcastOrigin::ENV && found_broadcast_on_the_right.msg_id == found_broadcast_on_the_left.msg_id {
                                // here we have found a matching broadcast on the immediate right
                                // on the right of the strict we have identified a pattern of the form "seq(l?,...)" i.e.
                                // a broadcast pattern with no known origin and the same message
                                // then we return the broadcast
                                let mut gt_targs = vec![];
                                for t in found_broadcast_on_the_left.gt_targets {
                                    gt_targs.push(t);
                                }
                                for t in found_broadcast_on_the_right.gt_targets {
                                    if !gt_targs.contains(&t) {
                                        gt_targs.push(t);
                                    }
                                }
                                let mut lf_targets = found_broadcast_on_the_left.lf_targets;
                                    lf_targets.extend(found_broadcast_on_the_right.lf_targets);
                                let broadcast = HibouBroadcastLeafPattern::new(
                                    found_broadcast_on_the_left.origin,
                                    found_broadcast_on_the_left.msg_id,
                                    lf_targets,
                                    gt_targs
                                );
                                return Some((HibouLeafPattern::BROADCAST(broadcast),rem2));
                            }
                        }
                    }
                }
                return None;
            },
            Interaction::CoReg(cr,i1, i2) => {
                if !cr.is_empty() {
                    return None;
                }
                if let Some((HibouLeafPattern::BROADCAST(b1),rem1)) = i1.identify_pattern_at_interaction_root() {
                    // here there is a broadcast on the left side
                    // ***
                    if rem1.is_some() {
                        // here there is a remainder on the right child of the left sub-interaction
                        // so we do not propagate the broadcast any higher
                        return None;
                    }
                    // we try to complete the broadcast with information from the right sub-interaction
                    if let Some((HibouLeafPattern::BROADCAST(b2),rem2)) = i2.identify_pattern_at_interaction_root() {
                        // here we have also found a broadcast pattern on the right sub-interaction
                        // we might be able to merge both broadcast patterns into a single one
                        let merge_patterns : bool = 
                        if b1.msg_id == b2.msg_id {
                            // here both patterns involve the same message
                            if b1.origin == HibouBroadcastOrigin::ENV && b2.origin == HibouBroadcastOrigin::ENV {
                                // we can merge two receptions casts to a single reception cast
                                true 
                            } else {
                                if let HibouBroadcastOrigin::LF(orig_lf) = b1.origin {
                                    // here the left broadcast pattern has a known lifeline origin
                                    if b1.lf_targets.is_empty() && b2.lf_targets == vec![orig_lf] {
                                        // we have an emission to self
                                        true 
                                    } else {
                                        false
                                    }
                                } else {
                                    false 
                                }
                            }
                        } else {
                            false 
                        };
                        // ***
                        if merge_patterns {
                            // if we can merge the patterns, we do so
                            let mut gt_targs = vec![];
                            for t in b1.gt_targets {
                                gt_targs.push(t);
                            }
                            for t in b2.gt_targets {
                                if !gt_targs.contains(&t) {
                                    gt_targs.push(t);
                                }
                            }
                            // ***
                            let mut lf_targets = b1.lf_targets;
                            lf_targets.extend(b2.lf_targets);
                            // ***
                            return Some(
                            (
                                HibouLeafPattern::BROADCAST(
                                    HibouBroadcastLeafPattern::new(
                                        b1.origin, 
                                        b1.msg_id, 
                                        lf_targets,
                                        gt_targs
                                    )
                                ),
                                rem2
                            )
                            );
                        } else {
                            // otherwise, we only return the left one, and the right sub-interaction as a remainder
                            return Some(
                                (
                                    HibouLeafPattern::BROADCAST(b1),
                                    Some((
                                        HibouOperators::Coreg(vec![]),
                                        i2
                                    ))
                                )
                            );
                        }
                    } else {
                        // here we have not found any broadcast pattern on the right sub-interaction
                        // so we only return the one we have found on the left, 
                        // and the remainder is the right sub-interaction
                        return Some(
                            (
                                HibouLeafPattern::BROADCAST(b1),
                                Some((
                                    HibouOperators::Coreg(vec![]),
                                    i2
                                ))
                            )
                        );
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







