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


use std::collections::{HashMap,HashSet};

use ab_glyph::{Font, FontRef, PxScale};
use common_sequence_diagram_io::internal_representation::InteractionInternalRepresentation;
use image::Rgb;
use image_colored_text::text::paragraph::*;
use image_colored_text::text::line::ColoredTextLine;


use common_sequence_diagram_io::to_image::common_interaction_drawer::CommonInteractionDrawerTrait;
use common_sequence_diagram_io::to_image::draw::context_aware_drawer::ContextAwareInteractionDrawer;
use common_sequence_diagram_io::to_image::drawable::leaf::util::MessageExchangeLineStyle;
use common_sequence_diagram_io::to_image::extract::context_aware_extractor::ContextAwareInteractionDrawingInstructionsExtractor;

use common_sequence_diagram_io::to_image::drawable::leaf::broadcast::*;
use common_sequence_diagram_io::to_image::drawable::operator::builtin_operator::{DrawableOperator, DrawableOperatorKind};
use common_sequence_diagram_io::conversion::repr_to_lang::FromInternalRepresentationToInteractionTerm;

use common_sequence_diagram_io::to_image::draw::util::draw_uniform_colored_background;
use maplit::hashset;

use crate::commons::hibou_color_palette::*;
use crate::commons::{DRAWING_GRAPHIC_FONT,SCALE};
use crate::core::general_context::GeneralContext;
use crate::core::syntax::lang_traits::involve::involves::InvolvesLifelines;
use crate::seqdiag_lib_interface::internal_representation::{HibouBroadcastOrigin, HibouLangCioII, HibouLeafPattern, HibouOperators};
use crate::core::syntax::interaction::{Interaction, LoopKind};




pub struct HibouDrawingContext {
    pub general_context : GeneralContext,
    pub font : FontRef<'static>,
    pub y_margin_between_seq_operands : f32,
    pub margin_between_items : f32,
    pub border_padding : f32,
    pub arrowhead_length : f32
}

impl HibouDrawingContext {

    pub fn new(general_context : GeneralContext) -> HibouDrawingContext {
        let font = ab_glyph::FontRef::try_from_slice(DRAWING_GRAPHIC_FONT).unwrap();
        let y_margin_between_seq_operands = 11.0;
        let margin_between_items = 6.0;
        let border_padding = 10.0;
        let arrowhead_length = 10.0;
        HibouDrawingContext{
            general_context,
            font,
            y_margin_between_seq_operands,
            margin_between_items,
            border_padding,
            arrowhead_length
        }
    }
}


impl CommonInteractionDrawerTrait for HibouDrawingContext {

    fn get_scale(&self) -> impl Into<PxScale> + Copy {
        SCALE
    }

    fn get_font(&self) -> &impl Font {
        &self.font
    }

    fn get_y_margin_between_seq_operands(&self) -> f32 {
        self.y_margin_between_seq_operands
    }

    fn get_margin_between_items(&self) -> f32 {
        self.margin_between_items
    }

    fn get_border_padding(&self) -> f32 {
        self.border_padding
    }

}

impl ContextAwareInteractionDrawingInstructionsExtractor<HibouLangCioII,usize> for HibouDrawingContext {
    fn lifelines_compare(&self, l1 : &usize, l2 : &usize) -> std::cmp::Ordering {
        l1.cmp(l2)
    }

    fn get_involved_lifelines(&self, pattern : &HibouLeafPattern) -> HashSet<usize> {
        match pattern{
            HibouLeafPattern::BROADCAST(ref brd) => {
                let mut lfs = HashSet::new();
                if let HibouBroadcastOrigin::LF(orig_lf) = &brd.origin {
                    lfs.insert(*orig_lf);
                }
                for lf in &brd.lf_targets {
                    lfs.insert(*lf);
                }
                lfs 
            },
            HibouLeafPattern::EMPTY => {
                HashSet::new()
            }
        }
    }

    fn get_lifeline_header(&self, l : &usize) -> ColoredTextParagraph {
        let lf_name = self.general_context.get_lf_name(*l).unwrap();
        ColoredTextParagraph::new(
            vec![ColoredTextLine::new(vec![(lf_name.to_owned(),Rgb(HC_LIFELINE))])],
            MultiLineTextAlignment::Center, 
            Some(Rgb(HCP_WHITE)), 
            Some(Rgb(HCP_BLACK))
        )
    }

    fn to_drawable_pattern(&self, pattern : &HibouLeafPattern) -> Option<DrawableBroadcastLeafPattern<usize>> {
        match pattern {
            HibouLeafPattern::BROADCAST(ref brd) => {
                // retrieve the message label
                let ms_name = self.general_context.get_ms_name(brd.msg_id).unwrap();
                let message = ColoredTextParagraph::new(
                    vec![ColoredTextLine::new(vec![(ms_name.to_owned(),Rgb(HC_MESSAGE))])],
                    MultiLineTextAlignment::Center, 
                    None, 
                    None
                );
                let line_style = MessageExchangeLineStyle::new(
                    false, 
                    false, 
                    Rgb(HCP_BLACK), 
                    self.arrowhead_length
                );
                let origin = match &brd.origin {
                    HibouBroadcastOrigin::ENV => {
                        DrawableBroadcastLeafPatternOrigin::Empty
                    },
                    HibouBroadcastOrigin::LF(orig_lf) => {
                        DrawableBroadcastLeafPatternOrigin::Lifeline(*orig_lf, PrePostAmbleDrawableActionItem::new(None,None))
                    },
                    HibouBroadcastOrigin::GT(gt_id) => {
                        let gt_name = self.general_context.get_gt_name(*gt_id).unwrap();
                        let gate = ColoredTextParagraph::new(
                            vec![ColoredTextLine::new(vec![(gt_name.to_owned(),Rgb(HC_GATE))])],
                            MultiLineTextAlignment::Center, 
                            None, 
                            Some(Rgb(HCP_BLACK))
                        );
                        DrawableBroadcastLeafPatternOrigin::InputOutsideGate(gate)
                    }
                };
                let mut lifeline_targets : HashMap<usize,TargetLifelineBroadcastDrawInstruction> = HashMap::new();
                {
                    let mut count_map : HashMap<usize,u32> = HashMap::new();
                    for lf in &brd.lf_targets {
                        *count_map.entry(*lf).or_default() += 1;
                    }
                    for (lf,occs) in count_map {
                        let paragraph = if occs <= 1 {
                            ColoredTextParagraph::new(
                                vec![], 
                                MultiLineTextAlignment::Center,
                                None,
                                None
                            )
                        } else {
                            ColoredTextParagraph::new(
                                vec![ColoredTextLine::new(vec![(format!("{}",occs),Rgb(HCP_BLACK))])], 
                                MultiLineTextAlignment::Center,
                                Some(Rgb(HCP_BRIGHT_GRAY)),
                                Some(Rgb(HCP_BLACK))
                            )
                        };
                        lifeline_targets.insert(
                            lf, 
                            TargetLifelineBroadcastDrawInstruction::Centered(CenteredDrawableActionItem::new(paragraph))
                        );
                    }
                }
                let mut output_outside_gates_targets = vec![];
                for gt_id in &brd.gt_targets {
                    let gt_name = self.general_context.get_gt_name(*gt_id).unwrap();
                    let gate = ColoredTextParagraph::new(
                        vec![ColoredTextLine::new(vec![(gt_name.to_owned(),Rgb(HC_GATE))])],
                        MultiLineTextAlignment::Center, 
                        None, 
                        Some(Rgb(HCP_BLACK))
                    );
                    output_outside_gates_targets.push(gate);
                }
                Some(
                    DrawableBroadcastLeafPattern::new(
                        message,
                        line_style,
                        origin,
                        lifeline_targets,
                        output_outside_gates_targets
                    )
                )
            },
            HibouLeafPattern::EMPTY => {
                None
            }
        }
    }

    fn to_drawable_operator(
        &self, 
        op : &HibouOperators, 
        sub_ints : &[InteractionInternalRepresentation<HibouLangCioII>]
    ) -> DrawableOperator<usize> {

        if let HibouOperators::Coreg(cr) = op {
            if cr.is_empty() {
                return DrawableOperator::new(Rgb(HCP_BLACK),DrawableOperatorKind::CoRegionLike(hashset!{}));
            }
            // ***
            let involved = {
                let unique_sub_int = sub_ints.first().unwrap();
                let as_interaction : Interaction = FromInternalRepresentationToInteractionTerm::<HibouLangCioII>::from_io_repr(
                    unique_sub_int
                );
                as_interaction.involved_lifelines()
            };
            // ***
            if involved.iter().all(|lf_id| cr.contains(lf_id)) {
                let op_label = ColoredTextParagraph::new(
                    vec![ColoredTextLine::new(vec![("par".to_owned(),Rgb(HCP_BLACK))])], 
                    MultiLineTextAlignment::Center,
                    None,
                    None
                );
                return DrawableOperator::new(Rgb(HCP_BLACK),DrawableOperatorKind::Framed(op_label));
            }
            // ***
            return DrawableOperator::new(Rgb(HCP_BLACK),DrawableOperatorKind::CoRegionLike(cr.iter().cloned().collect()));
        } 

        let colored_text_line = match op {
            HibouOperators::Strict => {
                ColoredTextLine::new(vec![("strict".to_owned(),Rgb(HCP_BLACK))])
            },
            HibouOperators::Alt => {
                ColoredTextLine::new(vec![("alt".to_owned(),Rgb(HCP_BLACK))])
            },
            HibouOperators::Loop(loop_kind) => {
                match loop_kind {
                    LoopKind::Coreg(cr) => {
                        if cr.is_empty() {
                            ColoredTextLine::new(vec![("loopW".to_owned(),Rgb(HCP_BLACK))])
                        } else {
                            let unique_sub_int = sub_ints.first().unwrap();
                            let as_interaction : Interaction = FromInternalRepresentationToInteractionTerm::<HibouLangCioII>::from_io_repr(
                                unique_sub_int
                            );
                            let involved = as_interaction.involved_lifelines();
                            if involved.iter().all(|lf_id| cr.contains(lf_id)) {
                                ColoredTextLine::new(vec![("loopP".to_owned(),Rgb(HCP_BLACK))])
                            } else {
                                let mut colored_segments = vec![("loopC(".to_owned(),Rgb(HCP_BLACK))];
                                let num_lfs_in_cr = cr.len();
                                for (x,lf_id) in cr.iter().enumerate() {
                                    colored_segments.push((self.general_context.get_lf_name(*lf_id).unwrap().to_owned(),Rgb(HC_LIFELINE)));
                                    if x < num_lfs_in_cr - 1 {
                                        colored_segments.push((",".to_owned(),Rgb(HCP_BLACK)));
                                    }
                                }
                                colored_segments.push((")".to_owned(),Rgb(HCP_BLACK)));
                                ColoredTextLine::new(colored_segments)
                            }
                        }
                    },
                    LoopKind::HHeadFirstWS => {
                        ColoredTextLine::new(vec![("loopH".to_owned(),Rgb(HCP_BLACK))])
                    },
                    LoopKind::SStrictSeq => {
                        ColoredTextLine::new(vec![("loopS".to_owned(),Rgb(HCP_BLACK))])
                    },
                }
            },
            HibouOperators::And => {
                ColoredTextLine::new(vec![("and".to_owned(),Rgb(HCP_BLACK))])
            },
            HibouOperators::Coreg(_) => {
                panic!("should never be reached")
            }
        };
        let op_label = ColoredTextParagraph::new(
            vec![colored_text_line], 
            MultiLineTextAlignment::Center,
            None,
            None
        );
        DrawableOperator::new(Rgb(HCP_BLACK),DrawableOperatorKind::Framed(op_label))
    }

}




impl ContextAwareInteractionDrawer<usize> for HibouDrawingContext {
    fn draw_background(&self, image : &mut image::RgbImage, img_width : f32, img_height : f32) {
        draw_uniform_colored_background(image,&img_width,&img_height,Rgb(HCP_WHITE));
    }
    
    fn get_lifelines_colors(&self, involved_lifelines : &[usize]) -> HashMap<usize,Rgb<u8>> {
        let mut lifelines_colors = HashMap::new();
        for lf in involved_lifelines {
            lifelines_colors.insert(*lf,Rgb(HCP_BLACK));
        }
        lifelines_colors
    }

    fn get_arrow_length(&self) -> f32 {
        20.0
    }

    fn get_nest_padding_unit(&self) -> f32 {
        3.0
    }
}