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

use simple_term_rewriter::core::terms::conversion::from_rewritable_term::FromRewritableTermToDomainSpecificTerm;
use simple_term_rewriter::draw_term::draw_term_tree_with_graphviz;
use simple_term_rewriter::rewriting_process::conf::RewriteConfig;
use simple_term_rewriter::rewriting_process::context::RewritingProcessContextAndParameterization;
use simple_term_rewriter::rewriting_process::node::RewriteNodeKind;

use crate::core::general_context::GeneralContext;
use crate::core::syntax::interaction::Interaction;
use crate::rewriting::lang::HibouRewritableLangOperator;
use crate::seqdiag_lib_interface::io::{draw_interaction_on_file, InteractionDrawingKind};



pub struct HibouRewritingNodeDrawer {
    pub gen_ctx : GeneralContext,
    pub draw_kind : InteractionDrawingKind
}

impl CustomNodeDrawerForGraphvizLogger<RewriteConfig<HibouRewritableLangOperator>> for HibouRewritingNodeDrawer {

    fn get_node_node_inner_style_and_draw_if_needed(
        &self,
        _context_and_param : &RewritingProcessContextAndParameterization<HibouRewritableLangOperator>,
        node : &RewriteNodeKind<HibouRewritableLangOperator>,
        full_path : &Path
    ) -> BuiltinGraphvizLoggerItemStyle {
        match self.draw_kind {
            InteractionDrawingKind::AsSequenceDiagram => {
                // ***
                let int = Interaction::from_rewritable_term(&node.term);
                // ***
                draw_interaction_on_file(
                    full_path,
                    &self.gen_ctx,
                    &int,
                    &self.draw_kind
                );
            },
            InteractionDrawingKind::AsTermTree => {
                let temp_file_path = format!("{}_temp.dot",full_path.to_str().unwrap());
                // ***
                draw_term_tree_with_graphviz::<HibouRewritableLangOperator,GeneralContext>(
                    &self.gen_ctx,
                    &node.term,
                    &Path::new(&temp_file_path),
                    full_path
                );
            }
        }
        BuiltinGraphvizLoggerItemStyle::CustomImage
    }

}




