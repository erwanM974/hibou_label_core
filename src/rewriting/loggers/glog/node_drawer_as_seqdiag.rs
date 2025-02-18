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


use std::path::Path;
use graph_process_manager_loggers::graphviz::drawers::node_drawer::CustomNodeDrawerForGraphvizLogger;
use graph_process_manager_loggers::graphviz::item::BuiltinGraphvizLoggerItemStyle;

use simple_term_rewriter::core::conversion::from_rewritable_term::FromRewritableTermToDomainSpecificTerm;
use simple_term_rewriter::process::conf::RewriteConfig;
use simple_term_rewriter::process::context::RewritingProcessContextAndParameterization;
use simple_term_rewriter::process::node::RewriteNodeKind;

use crate::core::general_context::GeneralContext;
use crate::core::syntax::interaction::Interaction;
use crate::rewriting::lang::HibouLangOperators;
use crate::seqdiag_lib_interface::draw_interaction_as_sequence_diagram_on_file;



pub struct HibouRewritingNodeAsSequenceDiagramDrawer {
    pub gen_ctx : GeneralContext
}

impl CustomNodeDrawerForGraphvizLogger<RewriteConfig<HibouLangOperators>> for HibouRewritingNodeAsSequenceDiagramDrawer {

    fn get_node_node_inner_style_and_draw_if_needed(
        &self,
        _context_and_param : &RewritingProcessContextAndParameterization<HibouLangOperators>,
        node : &RewriteNodeKind<HibouLangOperators>,
        full_path : &Path
    ) -> BuiltinGraphvizLoggerItemStyle {
        // ***
        let int = Interaction::from_rewritable_term(&node.term);
        // ***
        draw_interaction_as_sequence_diagram_on_file(
            full_path,&self.gen_ctx,&int
        );
        BuiltinGraphvizLoggerItemStyle::CustomImage
    }

}




