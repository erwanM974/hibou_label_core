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


use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use graph_process_manager_loggers::graphviz::drawers::node_drawer::CustomNodeDrawerForGraphvizLogger;
use graph_process_manager_loggers::graphviz::item::BuiltinGraphvizLoggerItemStyle;

use graphviz_dot_builder::colors::GraphvizColor;
use graphviz_dot_builder::graph::graph::GraphVizDiGraph;
use graphviz_dot_builder::graph::style::{GraphvizGraphStyleItem, GvGraphRankDir};
use graphviz_dot_builder::item::cluster::GraphVizCluster;
use graphviz_dot_builder::item::node::node::GraphVizNode;
use graphviz_dot_builder::item::node::style::{GraphvizNodeStyleItem, GvNodeShape, GvNodeStyleKind};
use graphviz_dot_builder::traits::{DotBuildable, DotTranslatable};

use crate::core::general_context::GeneralContext;
use crate::seqdiag_lib_interface::io::InteractionDrawingKind;
use crate::seqdiag_lib_interface::io::draw_interaction_on_file;

use crate::inclusion_checker::process::conf::InteractionInclusionCheckingConfig;
use crate::inclusion_checker::process::context::InteractionInclusionCheckingContextAndParameterization;
use crate::inclusion_checker::process::node::InteractionInclusionCheckingNode;



pub struct HibouInclusionCheckingNodeDrawer {
    pub gen_ctx : GeneralContext,
    pub draw_kind : InteractionDrawingKind
}

impl HibouInclusionCheckingNodeDrawer {
    pub fn new(gen_ctx : GeneralContext,draw_kind : InteractionDrawingKind) -> Self {
        Self { gen_ctx, draw_kind }
    }
}

impl CustomNodeDrawerForGraphvizLogger<InteractionInclusionCheckingConfig> for HibouInclusionCheckingNodeDrawer {

    fn get_node_node_inner_style_and_draw_if_needed(
        &self,
        _context_and_param : &InteractionInclusionCheckingContextAndParameterization,
        node : &InteractionInclusionCheckingNode,
        full_path : &Path
    ) -> BuiltinGraphvizLoggerItemStyle {

        let temp_file_path = format!("{}_temp.dot", full_path.to_str().unwrap());
        // ***
        let mut temp_file = File::create(&temp_file_path).unwrap();
        let mut digraph = GraphVizDiGraph::new(vec![GraphvizGraphStyleItem::Rankdir(GvGraphRankDir::TB)]);
        // ***
        {
            let cluster2_color = if node.including_candidates.is_empty() {
                GraphvizColor::coral
            } else {
                GraphvizColor::aquamarine
            };
            let mut cluster2 = GraphVizCluster::new(
                "including_cand".to_owned(),
                 vec![
                    GraphvizNodeStyleItem::Label("including candidates".to_string()),
                    GraphvizNodeStyleItem::FillColor(cluster2_color)
                    ], 
                 vec![], 
                 vec![]
            );
            if node.including_candidates.is_empty() {
                cluster2.add_node(
                    GraphVizNode::new(
                        "cnd0".to_string(), 
                        vec![
                            GraphvizNodeStyleItem::Label("".to_string()),
                            GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle),
                            GraphvizNodeStyleItem::Style(vec![GvNodeStyleKind::Invis]),
                        ]
                    )
                );
            }
            for (cand_id,cand) in node.including_candidates.iter().enumerate() {
                let including_cand_image_path = format!("{}_cand{}.png",full_path.to_str().unwrap(),cand_id);
                draw_interaction_on_file(
                    &Path::new(&including_cand_image_path),
                    &self.gen_ctx,
                    cand,
                    &self.draw_kind
                );
                cluster2.add_node(
                    GraphVizNode::new(
                        format!("cnd{}",cand_id), 
                        vec![
                            GraphvizNodeStyleItem::Label("".to_string()),
                            GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle),
                            GraphvizNodeStyleItem::Image(including_cand_image_path),
                        ]
                    )
                );
            }
            digraph.add_cluster(cluster2);
        }
        {
            let mut cluster1 = GraphVizCluster::new(
                "included_cand".to_owned(),
                 vec![
                    GraphvizNodeStyleItem::Label("included candidate".to_string()),
                    GraphvizNodeStyleItem::FillColor(GraphvizColor::gray)
                    ], 
                 vec![], 
                 vec![]
            );
            let included_cand_image_path = format!("{}_cand.png",full_path.to_str().unwrap());
            draw_interaction_on_file(
                &Path::new(&included_cand_image_path),
                &self.gen_ctx,
                &node.included_candidate,
                &self.draw_kind
            );
            cluster1.add_node(
                GraphVizNode::new(
                    "inc".to_owned(), 
                    vec![
                        GraphvizNodeStyleItem::Label("".to_string()),
                        GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle),
                        GraphvizNodeStyleItem::Image(included_cand_image_path),
                    ]
                )
            );
            digraph.add_cluster(cluster1);
        }
        // ***
        let _ = temp_file.write( digraph.to_dot_string().as_bytes() );
        // ***
        let _ = Command::new("dot")
            .arg("-Tpng")
            .arg(temp_file_path)
            .arg("-o")
            .arg(full_path)
            .output();

        BuiltinGraphvizLoggerItemStyle::CustomImage
    }

}




