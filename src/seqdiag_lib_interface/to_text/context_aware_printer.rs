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
use common_sequence_diagram_io::internal_representation::InteractionInternalRepresentation;
use common_sequence_diagram_io::to_text::context_aware_printer::ContextAwareInteractionPrinter;
use maplit::btreeset;
use crate::core::general_context::GeneralContext;
use crate::core::syntax::lang_traits::involve::involves::InvolvesLifelines;
use crate::seqdiag_lib_interface::internal_representation::{HibouBroadcastOrigin, HibouLangCioII, HibouLeafPattern, HibouOperators};
use crate::core::syntax::interaction::{Interaction, LoopKind};


impl ContextAwareInteractionPrinter<HibouLangCioII> for GeneralContext {

    fn left_parenthesis(&self) -> &str {
        "("
    }

    fn right_parenthesis(&self) -> &str {
        ")"
    }
    
    fn operand_separator(&self) -> &str {
        ","
    }

    fn print_operator(
        &self, 
        operator : &HibouOperators, 
        sub_ints : &[InteractionInternalRepresentation<HibouLangCioII>]
    ) -> String {
        match operator {
            HibouOperators::Strict => "strict".to_owned(),
            HibouOperators::Alt => "alt".to_owned(),
            HibouOperators::Loop(loop_kind) => {
                match loop_kind {
                    LoopKind::Coreg(cr) => {
                        if cr.is_empty() {
                            "loopW".to_owned()
                        } else {
                            let involved = {
                                let unique_sub_int = sub_ints.first().unwrap();
                                let as_interaction : Interaction = FromInternalRepresentationToInteractionTerm::<HibouLangCioII>::from_io_repr(
                                    unique_sub_int
                                );
                                as_interaction.involved_lifelines()
                            };
                            if involved.iter().all(|lf_id| cr.contains(lf_id)) {
                                "loopP".to_owned()
                            } else {
                                let conc_lfs : Vec<String> = cr.iter().map(
                                    |lf_id| self.get_lf_name(*lf_id).unwrap().to_owned()
                                ).collect();
                                format!("loopC({})", conc_lfs.join(","))
                            }
                        }
                    },
                    LoopKind::HHeadFirstWS =>"loopH".to_owned(),
                    LoopKind::SStrictSeq => "loopS".to_owned(),
                }
            },
            HibouOperators::And => "and".to_owned(),
            HibouOperators::Coreg(cr) => {
                if cr.is_empty() {
                    "seq".to_owned()
                } else {
                    let involved = {
                        let mut involved = btreeset!{};
                        for sub_int in sub_ints {
                            let sub_int_as_interaction = Interaction::from_io_repr(sub_int);
                            for lf_id in sub_int_as_interaction.involved_lifelines() {
                                involved.insert(lf_id);
                            }
                        }
                        involved
                    };
                    if involved.iter().all(|lf_id| cr.contains(lf_id)) {
                        "par".to_owned()
                    } else {
                        let lf_names : Vec<String> = cr.iter()
                            .map(|x| self.get_lf_name(*x).unwrap().to_owned()).collect();
                        format!("coreg({:})", lf_names.join(","))
                    }
                }
            },
        }
    }

    fn print_explicit_pattern(&self, leaf_pattern : &HibouLeafPattern) -> String {
        match leaf_pattern {
            HibouLeafPattern::EMPTY => {
                "0".to_owned()
            },
            HibouLeafPattern::BROADCAST(brd) => {
               let start = match brd.origin {
                   HibouBroadcastOrigin::ENV => {
                       "".to_owned()
                   },
                   HibouBroadcastOrigin::GT(gt_id) => {
                       format!("{} -- ", self.get_gt_name(gt_id).unwrap())
                   },
                   HibouBroadcastOrigin::LF(gt_id) => {
                       format!("{} -- ", self.get_lf_name(gt_id).unwrap())
                   }
               };
                let targs_num = brd.gt_targets.len() + brd.lf_targets.len();
                let end : String = match targs_num {
                    0 => {"|".to_owned()},
                    1 => {
                        if brd.gt_targets.is_empty() {
                            let targ_lf_id = brd.lf_targets.first().unwrap();
                            self.get_lf_name(*targ_lf_id).unwrap().to_owned()
                        } else {
                            let targ_gt_id = brd.gt_targets.first().unwrap();
                            self.get_gt_name(*targ_gt_id).unwrap().to_owned()
                        }
                    },
                    _ => {
                        let mut targs = vec![];
                        for lf_id in &brd.lf_targets {
                            targs.push(
                                self.get_lf_name(*lf_id).unwrap().to_owned()
                            )
                        }
                        for gt_id in &brd.gt_targets {
                            targs.push(
                                self.get_gt_name(*gt_id).unwrap().to_owned()
                            )
                        }
                        format!("({})", targs.join(","))
                    }
                };
                format!("{}{} -> {}",start, self.get_ms_name(brd.msg_id).unwrap(), end)
            }
        }
    }

}




