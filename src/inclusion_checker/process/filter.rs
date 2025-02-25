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



use graph_process_manager_core::process::filter::{AbstractNodePreFilter, AbstractStepFilter};

use super::{conf::InteractionInclusionCheckingConfig, context::InteractionInclusionCheckingContextAndParameterization, filtration::InteractionInclusionCheckingFiltrationResult, node::InteractionInclusionCheckingNode, state::InteractionInclusionCheckingGlobalState, step::InteractionInclusionCheckingStepKind};







pub enum InteractionInclusionCheckingStepFilter {
    MaxNodeNumber(u32),
    MaxLoopInstanciation(u32),
}

impl AbstractStepFilter<InteractionInclusionCheckingConfig> for InteractionInclusionCheckingStepFilter {
    fn apply_filter(
        &self,
        _context_and_param : &InteractionInclusionCheckingContextAndParameterization,
        global_state : &InteractionInclusionCheckingGlobalState,
        parent_node : &InteractionInclusionCheckingNode,
        step : &InteractionInclusionCheckingStepKind
    ) -> Option<InteractionInclusionCheckingFiltrationResult> {
        match self {
            InteractionInclusionCheckingStepFilter::MaxNodeNumber( max_node_number ) => {
                if global_state.node_count >= *max_node_number {
                    return Some( InteractionInclusionCheckingFiltrationResult::MaxNodeNumber );
                }
            },
            InteractionInclusionCheckingStepFilter::MaxLoopInstanciation( loop_num ) => {
                match step {
                    InteractionInclusionCheckingStepKind::ExecuteAction(frt_elt, _) => {
                        if parent_node.loop_depth + frt_elt.max_loop_depth > *loop_num {
                            return Some( InteractionInclusionCheckingFiltrationResult::MaxLoopDepth );
                        }
                    },
                    _ => {}
                }
            },
        }
        None
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}










pub struct InteractionInclusionCheckingNodePreFilter {}




impl AbstractNodePreFilter<InteractionInclusionCheckingConfig> for InteractionInclusionCheckingNodePreFilter {

    fn apply_filter(
        &self,
        _context_and_param : &InteractionInclusionCheckingContextAndParameterization,
        _global_state : &InteractionInclusionCheckingGlobalState,
        node : &InteractionInclusionCheckingNode,
    ) -> Option<InteractionInclusionCheckingFiltrationResult> {
        if node.including_candidates.is_empty() {
            return Some( InteractionInclusionCheckingFiltrationResult::NoMoreIncludingCandidates );
        }
        if node.included_candidate.express_empty() {
            if node.including_candidates.iter().all(|x| !x.express_empty()) {
                return Some( InteractionInclusionCheckingFiltrationResult::NoCandidateAcceptsEmptyTrace );
            }
        }
        None 
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}