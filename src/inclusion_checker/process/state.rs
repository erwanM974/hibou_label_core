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


use graph_process_manager_core::process::persistent_state::AbstractProcessMutablePersistentState;

use crate::inclusion_checker::check_inclusion::InteractionInclusionGlobalVerdict;

use super::conf::InteractionInclusionCheckingConfig;
use super::context::InteractionInclusionCheckingContextAndParameterization;
use super::filtration::InteractionInclusionCheckingFiltrationResult;
use super::node::InteractionInclusionCheckingNode;
use super::step::InteractionInclusionCheckingStepKind;




pub struct InteractionInclusionCheckingGlobalState {
    pub inclusion_verdict : InteractionInclusionGlobalVerdict,
    pub node_count : u32
}



impl AbstractProcessMutablePersistentState<InteractionInclusionCheckingConfig> for InteractionInclusionCheckingGlobalState {
    fn get_initial_state(
        _context_and_param: &InteractionInclusionCheckingContextAndParameterization,
        _initial_node : &InteractionInclusionCheckingNode
    ) -> Self {
        Self{inclusion_verdict : InteractionInclusionGlobalVerdict::IsIncluded, node_count : 0}
    }

    fn update_on_node_reached(
        &mut self,
        _context_and_param: &InteractionInclusionCheckingContextAndParameterization,
        _node: &InteractionInclusionCheckingNode
    ) {
        self.node_count += 1;
    }

    fn update_on_next_steps_collected_reached(
        &mut self,
        _context_and_param: &InteractionInclusionCheckingContextAndParameterization,
        _node: &InteractionInclusionCheckingNode,
        _steps: &[InteractionInclusionCheckingStepKind]
    ) {
        // nothing
    }

    fn update_on_filtered(
        &mut self,
        _context_and_param: &InteractionInclusionCheckingContextAndParameterization,
        _parent_node: &InteractionInclusionCheckingNode,
        filtration_result: &InteractionInclusionCheckingFiltrationResult
    ) {
        match filtration_result {
            InteractionInclusionCheckingFiltrationResult::SyntaxicEqualityImpliesInclusion => {
                // nothing
            },
            InteractionInclusionCheckingFiltrationResult::NoMoreIncludingCandidates => {
                self.inclusion_verdict = InteractionInclusionGlobalVerdict::IsNotIncluded;
            },
            InteractionInclusionCheckingFiltrationResult::NoCandidateAcceptsEmptyTrace => {
                self.inclusion_verdict = InteractionInclusionGlobalVerdict::IsNotIncluded;
            },
            InteractionInclusionCheckingFiltrationResult::MaxNodeNumber => {
                self.inclusion_verdict = InteractionInclusionGlobalVerdict::IsIncludedUpToExploredSemantics;
            },
            InteractionInclusionCheckingFiltrationResult::MaxLoopDepth => {
                self.inclusion_verdict = InteractionInclusionGlobalVerdict::IsIncludedUpToExploredSemantics;
            },
        }
    }

    fn warrants_termination_of_the_process(
        &self,
        _context_and_param: &InteractionInclusionCheckingContextAndParameterization
    ) -> bool {
        if self.inclusion_verdict == InteractionInclusionGlobalVerdict::IsNotIncluded {
            return true;
        }
        false 
    }
}