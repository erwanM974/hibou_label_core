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

use graph_process_manager_core::process::filter::{AbstractNodePostFilter, AbstractNodePreFilter, AbstractStepFilter};
use graph_process_manager_loggers::graphviz::drawers::legend_writer::ProcessLegendWriter;

use crate::inclusion_checker::process::conf::InteractionInclusionCheckingConfig;
use crate::inclusion_checker::process::context::InteractionInclusionCheckingContextAndParameterization;
use crate::inclusion_checker::process::priorities::InteractionInclusionCheckingPriorities;
use crate::inclusion_checker::process::state::InteractionInclusionCheckingGlobalState;
use crate::inclusion_checker::process::filter::InteractionInclusionCheckingStepFilter;





pub struct HibouInclusionCheckingLegendWriter {}


impl ProcessLegendWriter<InteractionInclusionCheckingConfig> for HibouInclusionCheckingLegendWriter {
    fn get_process_description(&self) -> String {
        "interaction inclusion checking".to_string()
    }

    fn get_parameters_description(
        &self, 
        _context_and_param : &InteractionInclusionCheckingContextAndParameterization
    ) -> Vec<Vec<String>> {
        vec![]
    }

    fn get_priorities_description(
        &self, 
        _priorities : &InteractionInclusionCheckingPriorities
    ) -> Vec<Vec<String>> {
        vec![]
    }

    fn get_step_filter_description(&self, filter : &dyn AbstractStepFilter<InteractionInclusionCheckingConfig>) -> Option<Vec<String>> {
        match filter.as_any().downcast_ref::<InteractionInclusionCheckingStepFilter>() {
            Some(x) => {
                match x {
                    InteractionInclusionCheckingStepFilter::MaxNodeNumber(num) => {
                        Some(vec![format!("MaxNodeNumber={}",num)])
                    },
                    InteractionInclusionCheckingStepFilter::MaxLoopInstanciation(num) => {
                        Some(vec![format!("MaxLoopInstanciation={}",num)])
                    },
                }
            }
            None => {
                None
            }
        } 
    }

    fn get_node_pre_filter_description(&self, _filter : &dyn AbstractNodePreFilter<InteractionInclusionCheckingConfig>) -> Option<Vec<String>> {
        None
    }

    fn get_node_post_filter_description(&self, _filter : &dyn AbstractNodePostFilter<InteractionInclusionCheckingConfig>) -> Option<Vec<String>> {
        None
    }

    fn get_final_global_state_description_for_legend(
        &self, 
        _context_and_param : &InteractionInclusionCheckingContextAndParameterization,
        final_state : &InteractionInclusionCheckingGlobalState
    ) -> Vec<String> {
        vec![
            format!("inclusion_verdict={:}", final_state.inclusion_verdict)
        ]
    }
}