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
use simple_term_rewriter::{core::terms::conversion::from_rewritable_term::FromRewritableTermToDomainSpecificTerm, rewriting_process::context::AbstractRewritingPhase};
use simple_term_rewriter::core::terms::conversion::to_rewritable_term::FromDomainSpecificTermToRewritableTerm;
use simple_term_rewriter::core::rule::RewriteRule;
use simple_term_rewriter::rewriting_process::conf::RewriteConfig;
use simple_term_rewriter::rewriting_process::context::RewritingProcessContextAndParameterization;
use simple_term_rewriter::rewriting_process::node::RewriteNodeKind;
use simple_term_rewriter::rewriting_process::priorities::RewritePriorities;

use crate::{core::{general_context::GeneralContext, syntax::interaction::Interaction}, interfaces::HibouGraphvizLoggerParam, rewriting::loggers::glog::{all_the_rest_drawer::HibouRewritingAllTheRestDrawer, legend_writer::HibouRewritingLegendWriter, node_drawer::HibouRewritingNodeDrawer}, seqdiag_lib_interface::io::InteractionDrawingKind};

use super::{lang::HibouRewritableLangOperator, rules::high_level_hibou_rewrite_rules::HighLevelHibouRewriteRules};





fn get_graphviz_logger_from_param(
    gen_ctx : &GeneralContext,
    gv_log_param : &HibouGraphvizLoggerParam,
    fname : &str
) -> Vec<Box< dyn AbstractProcessLogger<RewriteConfig<HibouRewritableLangOperator>>>> {
    let mut node_drawers : Vec<Box<dyn CustomNodeDrawerForGraphvizLogger<RewriteConfig<HibouRewritableLangOperator>>>> = vec![];
    if gv_log_param.has_term_tree_repr() {
        node_drawers.push( 
            Box::new(HibouRewritingNodeDrawer{gen_ctx:gen_ctx.clone(),draw_kind:InteractionDrawingKind::AsTermTree}) 
        );
    }
    if gv_log_param.has_seq_diag_repr() {
        node_drawers.push( 
            Box::new(HibouRewritingNodeDrawer{gen_ctx:gen_ctx.clone(),draw_kind:InteractionDrawingKind::AsSequenceDiagram}) 
        );
    }
    let glog = GenericGraphVizLogger::new(
        GenericGraphVizLoggerConfiguration::new(
            GraphVizOutputFormat::svg,
            true,
            "temp".to_string(),
            ".".to_string(),
            format!("{}_rewr_proc", fname)
        ),
        Box::new(HibouRewritingLegendWriter{}),
        node_drawers,
        Box::new(HibouRewritingAllTheRestDrawer::new()),
        GraphVizProcessLoggerLayout::Vertical
    );
    vec![Box::new(glog)]
}





fn get_base_rules(coreg_simplifications : bool) -> Vec<Box<dyn RewriteRule<HibouRewritableLangOperator>>> {
    let mut rules = vec![
        HighLevelHibouRewriteRules::StrictFlushRight.get_low_level_rewrite_rule(),
        HighLevelHibouRewriteRules::AltAndCoregFlushRight.get_low_level_rewrite_rule(),
        HighLevelHibouRewriteRules::ReorderSubInteractionsUnderAlt.get_low_level_rewrite_rule(),
        HighLevelHibouRewriteRules::ReorderSubInteractionsUnderCoreg.get_low_level_rewrite_rule(),
        HighLevelHibouRewriteRules::EpsilonFixpoint.get_low_level_rewrite_rule(),
        HighLevelHibouRewriteRules::EpsilonNeutral.get_low_level_rewrite_rule(),
        HighLevelHibouRewriteRules::KleeneNesting.get_low_level_rewrite_rule(),
        HighLevelHibouRewriteRules::KleeneTighteningModuloAC.get_low_level_rewrite_rule(),
        HighLevelHibouRewriteRules::KleeneRolling.get_low_level_rewrite_rule(),
        HighLevelHibouRewriteRules::StrictnessRelaxationBinary.get_low_level_rewrite_rule(),
        HighLevelHibouRewriteRules::StrictnessRelaxationUnary.get_low_level_rewrite_rule(),
        HighLevelHibouRewriteRules::BasicAltDeduplication.get_low_level_rewrite_rule(),
        HighLevelHibouRewriteRules::SequencingCompatibilityLeft.get_low_level_rewrite_rule(),
        HighLevelHibouRewriteRules::SequencingCompatibilityRight.get_low_level_rewrite_rule(),
        HighLevelHibouRewriteRules::KleeneDesequencing.get_low_level_rewrite_rule(),
    ];
    if coreg_simplifications {
        rules.extend(
            vec![
                HighLevelHibouRewriteRules::CoregionMinimizationBasic.get_low_level_rewrite_rule(),
                HighLevelHibouRewriteRules::CoregionMinimizationKleene.get_low_level_rewrite_rule(),
            ]
        );
    }
    rules
}

fn get_phase_1(coreg_simplifications : bool) -> Vec<Box<dyn RewriteRule<HibouRewritableLangOperator>>> {
    let mut rules = get_base_rules(coreg_simplifications);
    rules.extend(
        vec![
            HighLevelHibouRewriteRules::DeFactorizeLeft.get_low_level_rewrite_rule(),
            HighLevelHibouRewriteRules::DeFactorizeRight.get_low_level_rewrite_rule()
        ]
    );
    rules 
}

fn get_phase_2(coreg_simplifications : bool) -> Vec<Box<dyn RewriteRule<HibouRewritableLangOperator>>> {
    let mut rules = get_base_rules(coreg_simplifications);
    rules.extend(
        vec![
            HighLevelHibouRewriteRules::FactorizeRight.get_low_level_rewrite_rule()
        ]
    );
    rules 
}

fn get_phase_3(coreg_simplifications : bool) -> Vec<Box<dyn RewriteRule<HibouRewritableLangOperator>>> {
    let mut rules = get_base_rules(coreg_simplifications);
    rules.extend(
        vec![
            HighLevelHibouRewriteRules::FactorizeLeft.get_low_level_rewrite_rule()
        ]
    );
    rules 
}



pub fn canonize_interaction(
    int : &Interaction,
    graphviz_param : Option<(&GeneralContext,&str,&HibouGraphvizLoggerParam)>,
    keep_only_one : bool,
    coreg_simplifications : bool
) -> Interaction {
    let loggers = match graphviz_param {
        None => {
            vec![]
        },
        Some((gen_ctx,fname,gv_log_param)) => {
            get_graphviz_logger_from_param(gen_ctx, gv_log_param, fname)
        }
    };

    let int_as_term = int.to_rewritable_term();

    let phase1 = AbstractRewritingPhase::<HibouRewritableLangOperator>::new(
        get_phase_1(coreg_simplifications),
        Some(1),
        Some(1)
    );

    let phase2 = AbstractRewritingPhase::new(
        get_phase_2(coreg_simplifications),
        Some(2),
        Some(2)
    );

    let phase3 = AbstractRewritingPhase::new(
        get_phase_3(coreg_simplifications),
        Some(1),
        None
    );

    let context_and_param = RewritingProcessContextAndParameterization::new(
        vec![phase1,phase2,phase3],
        keep_only_one
    );

    // ***

    let mut manager : GenericProcessManager<RewriteConfig<HibouRewritableLangOperator>> = GenericProcessManager::new(
        context_and_param,
        QueueSearchStrategy::DFS,
        GenericProcessPriorities::new(RewritePriorities{},false),
        GenericFiltersManager::new(
            vec![], 
            vec![], 
            vec![]
        ),
        loggers,
        true,
        RewriteNodeKind::new(
            int_as_term,
            0
        )
    );

    let _ = manager.start_process();

    let last_phase = manager.global_state.concrete_phases.last().unwrap();
    let irreducible = &last_phase.final_irreducible_terms;
    if irreducible.len() > 1 {
        println!("WARNING : more than 1 irreducible term found");
    }
    let result = irreducible.into_iter().next().unwrap();

    Interaction::from_rewritable_term(result)
}












