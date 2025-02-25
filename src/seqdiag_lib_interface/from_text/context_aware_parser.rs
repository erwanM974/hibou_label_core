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



use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, value};
use nom::error::ParseError;
use nom::IResult;


use common_sequence_diagram_io::from_text::context_aware_parser::ContextAwareInteractionParser;



use common_sequence_diagram_io::from_text::util::delimited_lang_parser::DelimitedInteractionLanguageParser;
use common_sequence_diagram_io::from_text::util::generic_broadcast_parser::GenericBroadcastParser;
use common_sequence_diagram_io::from_text::util::parse_utils::parse_element_of_preexisting_vec_and_return_index;
use nom::character::complete::multispace0;
use nom::multi::separated_list0;
use nom::sequence::tuple;

use crate::core::syntax::interaction::*;
use crate::core::general_context::GeneralContext;
use crate::seqdiag_lib_interface::internal_representation::{HibouBroadcastLeafPattern, HibouBroadcastOrigin, HibouLangCioII, HibouLeafPattern, HibouOperators};


impl DelimitedInteractionLanguageParser for GeneralContext {

    fn left_parenthesis_char(&self) -> char {
        '('
    }

    fn right_parenthesis_char(&self) -> char {
        ')'
    }

    fn separator_char(&self) -> char {
        ','
    }
}



impl GenericBroadcastParser<HibouBroadcastOrigin,usize,HibouBroadcastOrigin,HibouBroadcastLeafPattern> for GeneralContext {
    fn make_pattern(&self,origin : Option<HibouBroadcastOrigin>, message : usize, targets : Vec<HibouBroadcastOrigin>) -> HibouBroadcastLeafPattern {
        let orig = match origin {
            None => {
                HibouBroadcastOrigin::ENV
            },
            Some(got) => {
                got
            }
        };
        let mut lf_targets = vec![];
        let mut gt_targets = vec![];
        for t in targets {
            match t {
                HibouBroadcastOrigin::GT(gt_id) => {
                    gt_targets.push(gt_id);
                },
                HibouBroadcastOrigin::LF(lf_id) => {
                    lf_targets.push(lf_id);
                },
                _ => {
                    panic!()
                }
            }
        }
        HibouBroadcastLeafPattern::new(orig, message, lf_targets,gt_targets)
    }

    fn get_empty_target_char(&self) -> char {
        '|'
    }

    fn get_tag_for_message_reception_by_target(&self) -> &'static str {
        "->"
    }

    fn get_tag_for_message_transmission_from_origin(&self) -> &'static str {
        "--"
    }

    fn parse_message<'a, E: ParseError<&'a str>>(&self, input : &'a str) -> IResult<&'a str, usize,E> {
        parse_element_of_preexisting_vec_and_return_index(self.get_ms_names(),input)
    }

    fn parse_broadcast_origin<'a, E: ParseError<&'a str>>(&self, input : &'a str) -> IResult<&'a str, HibouBroadcastOrigin,E> {
        alt(
            (
                map(
                    |x| parse_element_of_preexisting_vec_and_return_index(self.get_lf_names(),x),
                    |l| HibouBroadcastOrigin::LF(l)
                ),
                map(
                    |x| parse_element_of_preexisting_vec_and_return_index(self.get_gt_names(),x),
                    |g| HibouBroadcastOrigin::GT(g)
                )
            )
        )(input)
    }

    fn parse_single_broadcast_targets<'a, E: ParseError<&'a str>>(&self, input : &'a str) -> IResult<&'a str, HibouBroadcastOrigin,E> {
        alt(
            (
                map(
                    |x| parse_element_of_preexisting_vec_and_return_index(self.get_lf_names(),x),
                    HibouBroadcastOrigin::LF
                ),
                map(
                    |x| parse_element_of_preexisting_vec_and_return_index(self.get_gt_names(),x),
                    HibouBroadcastOrigin::GT
                )
            )
        )(input)
    }
}

impl GeneralContext {

    fn parse_list_of_lifelines<'a, E: ParseError<&'a str>>(&self, input : &'a str) -> IResult<
        &'a str,
        Vec<usize>,
        E> {
        separated_list0(
            tuple((tag(","),multispace0)),
            |x| parse_element_of_preexisting_vec_and_return_index(self.get_lf_names(),x)
        )(input)
    }

}


impl ContextAwareInteractionParser<HibouLangCioII> for GeneralContext {

    fn parse_operator<'a, E: ParseError<&'a str>>(&self, input : &'a str) -> IResult<
        &'a str,
        HibouOperators, 
        E> {
        alt(
            (
            value(HibouOperators::Strict, tag("strict")),
            value(HibouOperators::Coreg(vec![]), tag("seq")),
            value(HibouOperators::Coreg(self.get_all_lfs_ids()), tag("par")),
            map(
                tuple(
                    (
                        value( (), tag("coreg")),
                        multispace0,
                        value( (), tag("(")),
                        multispace0,
                        |x| self.parse_list_of_lifelines(x),
                        value( (), tag(")")),
                        )
                ),
                |(_,_,_,_,x,_)| HibouOperators::Coreg(x)
            ),
            value(HibouOperators::Alt, tag("alt")),
            value(HibouOperators::Loop(LoopKind::SStrictSeq), tag("loopS")),
            value(HibouOperators::Loop(LoopKind::HHeadFirstWS), tag("loopH")),
            value(HibouOperators::Loop(LoopKind::Coreg(vec![])), tag("loopW")),
            value(HibouOperators::Loop(LoopKind::Coreg(self.get_all_lfs_ids())), tag("loopP")),
            map(
                tuple(
                    (
                        value( (), tag("loopC")),
                        multispace0,
                        value( (), tag("(")),
                        multispace0,
                        |x| self.parse_list_of_lifelines(x),
                        value( (), tag(")")),
                        )
                ),
                |(_,_,_,_,x,_)| HibouOperators::Loop(LoopKind::Coreg(x))
            ),
            )
        )
        (input)
    }

    fn parse_explicit_pattern<'a, E: ParseError<&'a str>>(&self, input : &'a str) -> IResult<
        &'a str,
        HibouLeafPattern,
        E> {
        // we have two kinds of patterns :
        // *m -> l* for the reception of *m* by *l*
        // *l -- m -> X* for the emission of *m* by *l* to *X*, with *X* itself being either of three patterns:
        //     *|* for the empty target
        //     *l2* for another lifeline
        //     *(l2,l3)* for two or more lifelines
        alt(
            (
                map(|x| self.parse_broadcast_pattern(x), |y| HibouLeafPattern::BROADCAST(y)),
                value(HibouLeafPattern::EMPTY,alt((tag("0"),tag("o"))))
            )
        )
        (input)
    }



}








