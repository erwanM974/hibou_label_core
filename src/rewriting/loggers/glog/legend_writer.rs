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

use simple_term_rewriter::process::conf::RewriteConfig;
use simple_term_rewriter::process::context::RewritingProcessContextAndParameterization;
use simple_term_rewriter::process::filter::{RewriteNodePreFilter, RewriteStepFilter};
use simple_term_rewriter::process::loggers::glog::legend_writer_utils::{get_rewrite_node_pre_filter_description, get_rewrite_parameters_description, get_rewrite_priorities_description, get_rewrite_step_filter_description};
use simple_term_rewriter::process::priorities::RewritePriorities;
use simple_term_rewriter::process::state::RewritingProcessState;

use crate::rewriting::lang::HibouLangOperators;




pub struct HibouRewritingLegendWriter {}


impl ProcessLegendWriter<RewriteConfig<HibouLangOperators>> for HibouRewritingLegendWriter {
    fn get_process_description(&self) -> String {
        "rewriting interaction language".to_string()
    }

    fn get_parameters_description(&self, context_and_param : &RewritingProcessContextAndParameterization<HibouLangOperators>) -> Vec<Vec<String>> {
        get_rewrite_parameters_description(context_and_param)
    }

    fn get_priorities_description(&self, priorities : &RewritePriorities) -> Vec<Vec<String>> {
        get_rewrite_priorities_description(priorities)
    }

    fn get_step_filter_description(&self, filter : &dyn AbstractStepFilter<RewriteConfig<HibouLangOperators>>) -> Option<Vec<String>> {
        match filter.as_any().downcast_ref::<RewriteStepFilter>() {
            Some(x) => {
                Some(get_rewrite_step_filter_description(x))
            }
            None => {
                None
            }
        }
    }

    fn get_node_pre_filter_description(&self, filter : &dyn AbstractNodePreFilter<RewriteConfig<HibouLangOperators>>) -> Option<Vec<String>> {
        match filter.as_any().downcast_ref::<RewriteNodePreFilter<HibouLangOperators>>() {
            Some(x) => {
                Some(get_rewrite_node_pre_filter_description(x))
            }
            None => {
                None
            }
        }
    }

    fn get_node_post_filter_description(&self, _filter : &dyn AbstractNodePostFilter<RewriteConfig<HibouLangOperators>>) -> Option<Vec<String>> {
        None
    }

    fn get_final_global_state_description_for_legend(
        &self, 
        _context_and_param : &RewritingProcessContextAndParameterization<HibouLangOperators>,
        _final_state : &RewritingProcessState<HibouLangOperators>
    ) -> Vec<String> {
        vec![]
    }
}