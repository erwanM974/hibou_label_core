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

use graph_process_manager_core::{process::{filter::GenericFiltersManager, logger::AbstractProcessLogger, manager::GenericProcessManager}, queue::{priorities::GenericProcessPriorities, strategy::QueueSearchStrategy}};
use graph_process_manager_loggers::graphviz::{drawers::node_drawer::CustomNodeDrawerForGraphvizLogger, format::GraphVizProcessLoggerLayout, logger::{GenericGraphVizLogger, GenericGraphVizLoggerConfiguration}};
use graphviz_dot_builder::traits::GraphVizOutputFormat;
use maplit::btreeset;
use simple_term_rewriter::{core::conversion::to_rewritable_term::FromDomainSpecificTermToRewritableTerm, metrics::TermMetrics};

use crate::{core::{general_context::GeneralContext, syntax::interaction::Interaction}, inclusion_checker::loggers::glog::{all_the_rest_drawer::HibouInclusionCheckingAllTheRestDrawer, legend_writer::HibouInclusionCheckingLegendWriter, node_drawer::HibouInclusionCheckingNodeDrawer}, interfaces::HibouGraphvizLoggerParam, rewriting::{lang::HibouRewritableLangOperator, metrics::InteractionTermSymbolMetrics}, seqdiag_lib_interface::io::InteractionDrawingKind};

use super::process::{conf::InteractionInclusionCheckingConfig, context::InteractionInclusionCheckingContextAndParameterization, filter::{InteractionInclusionCheckingNodePreFilter, InteractionInclusionCheckingStepFilter}, node::InteractionInclusionCheckingNode, priorities::InteractionInclusionCheckingPriorities};





#[derive(Clone, PartialEq, Eq, Hash)]
pub enum InteractionInclusionGlobalVerdict {
    IsIncluded,
    IsNotIncluded,
    IsIncludedUpToExploredSemantics
}

impl std::fmt::Display for InteractionInclusionGlobalVerdict {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InteractionInclusionGlobalVerdict::IsIncluded => {
                write!(f, "IsIncluded")
            },
            InteractionInclusionGlobalVerdict::IsNotIncluded => {
                write!(f, "IsNotIncluded")
            },
            InteractionInclusionGlobalVerdict::IsIncludedUpToExploredSemantics => {
                write!(f, "IsIncludedUpToExploredSemantics")
            },
        }
    }
}







fn get_graphviz_logger_from_param(
    gen_ctx : &GeneralContext,
    gv_log_param : &HibouGraphvizLoggerParam,
    fname1 : &str,
    fname2 : &str,
) -> Vec<Box< dyn AbstractProcessLogger<InteractionInclusionCheckingConfig>>> {
    let mut node_drawers : Vec<Box<dyn CustomNodeDrawerForGraphvizLogger<InteractionInclusionCheckingConfig>>> = vec![];
    if gv_log_param.has_term_tree_repr() {
        node_drawers.push( 
            Box::new(HibouInclusionCheckingNodeDrawer::new(gen_ctx.clone(),InteractionDrawingKind::AsTermTree))
        );
    }
    if gv_log_param.has_seq_diag_repr() {
        node_drawers.push( 
            Box::new(HibouInclusionCheckingNodeDrawer::new(gen_ctx.clone(),InteractionDrawingKind::AsSequenceDiagram))
        );
    }
    let glog = GenericGraphVizLogger::new(
        GenericGraphVizLoggerConfiguration::new(
            GraphVizOutputFormat::svg,
            true,
            "temp".to_string(),
            ".".to_string(),
            format!("{}_inc_{}_check_proc", fname1, fname2)
        ),
        Box::new(HibouInclusionCheckingLegendWriter{}),
        node_drawers,
        Box::new(HibouInclusionCheckingAllTheRestDrawer::new(gen_ctx.clone())),
        GraphVizProcessLoggerLayout::Vertical
    );
    vec![Box::new(glog)]
}





pub fn check_inclusion_of_interactions(
    included_candidate : &Interaction,
    including_candidate : &Interaction,
    graphviz_param : Option<(&GeneralContext,&str,&str,&HibouGraphvizLoggerParam)>
) -> InteractionInclusionGlobalVerdict {
    let loggers : Vec<Box< dyn AbstractProcessLogger<InteractionInclusionCheckingConfig>>> = match graphviz_param {
        None => {
            vec![]
        },
        Some((gen_ctx,fname1,fname2,gv_log_param)) => {
            get_graphviz_logger_from_param(gen_ctx, gv_log_param, fname1,fname2)
        }
    };

    let context_and_param = InteractionInclusionCheckingContextAndParameterization{};

    // ***

    let int_metrics = TermMetrics::<HibouRewritableLangOperator,InteractionTermSymbolMetrics>::extract_from_term(
        &included_candidate.to_rewritable_term()
    );
    let max_loop_instanciations = match int_metrics.metrics_count.get(&InteractionTermSymbolMetrics::AnyLoop) {
        None => {
            1
        },
        Some(loop_number) => {
            (2*loop_number) + 1
        }
    };

    let mut manager : GenericProcessManager<InteractionInclusionCheckingConfig> = GenericProcessManager::new(
        context_and_param,
        QueueSearchStrategy::BFS,
        GenericProcessPriorities::new(InteractionInclusionCheckingPriorities{},false),
        GenericFiltersManager::new(
            vec![Box::new(InteractionInclusionCheckingNodePreFilter{})], 
            vec![], 
            vec![Box::new(InteractionInclusionCheckingStepFilter::MaxLoopInstanciation(max_loop_instanciations))]
        ),
        loggers,
        true
    );

    manager.start_process(InteractionInclusionCheckingNode::new(
        included_candidate.clone(),
        0,
        btreeset! {including_candidate.clone()})
    );

    manager.global_state.inclusion_verdict
}













