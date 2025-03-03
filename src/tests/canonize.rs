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


use crate::{core::general_context::GeneralContext, interfaces::HibouGraphvizLoggerParam, rewriting::canonize::canonize_interaction, seqdiag_lib_interface::io::parse_interaction_from_text};



fn tool_test_canonize(
        gen_ctx : GeneralContext,
        input_text : &str, 
        expected_interaction : &str,
        expected_canonic : &str,
        svg_logger : Option<&'static str>,
        keep_only_one : bool,
        simplified_text : &str
    ) {
    // we parse the input text and verifify that the obtained interaction is indeed the expected one
    let int = parse_interaction_from_text(
        input_text,
        &gen_ctx
    ).unwrap();
    let got_int : String = format!("{:?}",int).chars().filter(|c| !c.is_whitespace()).collect();
    assert_eq!(expected_interaction, got_int);

    // we canonize the interaction and verify that it is indeed the expected one
    let gv_log_prm = HibouGraphvizLoggerParam::SeqDiagAndTermTree;
    let graphviz_param : Option<(&GeneralContext,&str,&HibouGraphvizLoggerParam)> = match svg_logger {
        Some(logger_output) => {
            Some((&gen_ctx,logger_output,&gv_log_prm))
        },
        None => {
            None 
        }
    };
    let canonized = canonize_interaction(
        &int,
        graphviz_param,
        keep_only_one
    );
    let got_can : String = format!("{:?}",canonized).chars().filter(|c| !c.is_whitespace()).collect();
    assert_eq!(expected_canonic, got_can);

    // finally we parse and canonize the simplified textual version and see if we have the same thing
    let simplified_canonic = {
        canonize_interaction(
            &parse_interaction_from_text(
                simplified_text,
                &gen_ctx
            ).unwrap(),
            None,
            true
        )
    };
    assert_eq!(canonized,simplified_canonic);
}













fn get_gen_ctx() -> GeneralContext {
    GeneralContext::new(
        vec![
            "l1".to_string(),
            "l2".to_string(),
            "l3".to_string(),
        ], 
        vec![
            "m1".to_string(),
            "m2".to_string(),
            "m3".to_string(),
        ],
        vec![]
    )
}


#[test]
pub fn test_undisturbed_alt_factorize() {

    let gen_ctx = get_gen_ctx();
    let input_text = r#"
seq(
        l1 -- m1 -> l2,
        alt(
                l2 -- m2 -> l1,
                0
        )
)
    "#;

    let simplified_text = r#"
seq(
        l1 -- m1 -> l2,
        alt(
                l2 -- m2 -> l1,
                0
        )
)
    "#;

    let expected_int : String = r#"
CoReg([],
    Strict(
        Emission(EmissionAction{orig_lf_id:0,ms_id:0,target_gates:[]}),
        Reception(ReceptionAction{origin_gate:None,ms_id:0,targ_lf_id:1})
    ),
    Alt(
        Strict(
            Emission(EmissionAction{orig_lf_id:1,ms_id:1,target_gates:[]}),
            Reception(ReceptionAction{origin_gate:None,ms_id:1,targ_lf_id:0})
        ),
        Empty
    )
)
    "#.chars().filter(|c| !c.is_whitespace()).collect();

    let expected_canonized : String = r#"
CoReg([],
    Strict(
        Emission(EmissionAction{orig_lf_id:0,ms_id:0,target_gates:[]}),
        Reception(ReceptionAction{origin_gate:None,ms_id:0,targ_lf_id:1})
    ),
    Alt(
        Empty,
        Strict(
            Emission(EmissionAction{orig_lf_id:1,ms_id:1,target_gates:[]}),
            Reception(ReceptionAction{origin_gate:None,ms_id:1,targ_lf_id:0})
        )
    )
)
    "#.chars().filter(|c| !c.is_whitespace()).collect();

    tool_test_canonize(
        gen_ctx,
        input_text,
        &expected_int,
        &expected_canonized,
        None,//Some("test1"),
        true,
        &simplified_text
    )
}




#[test]
pub fn test_deduplicate_and_kleene_tightening() {

    let gen_ctx = get_gen_ctx();
    let input_text = r#"
loopS(
    alt(
        l1 -- m1 -> l2,
        loopS(
            l1 -- m2 -> l2
        ),
        l1 -- m1 -> l2
    )
)
    "#;

    let simplified_text = r#"
loopS(
    alt(
        l1 -- m1 -> l2,
        l1 -- m2 -> l2
    )
)
    "#;

    let expected_int : String = r#"
Loop(SStrictSeq,
    Alt(
        Strict(
            Emission(EmissionAction{orig_lf_id:0,ms_id:0,target_gates:[]}),
            Reception(ReceptionAction{origin_gate:None,ms_id:0,targ_lf_id:1})
        ),
        Alt(
            Loop(SStrictSeq,
                Strict(
                    Emission(EmissionAction{orig_lf_id:0,ms_id:1,target_gates:[]}),
                    Reception(ReceptionAction{origin_gate:None,ms_id:1,targ_lf_id:1})
                )
            ),
            Strict(
                Emission(EmissionAction{orig_lf_id:0,ms_id:0,target_gates:[]}),
                Reception(ReceptionAction{origin_gate:None,ms_id:0,targ_lf_id:1})
            )
        )
    )
)
    "#.chars().filter(|c| !c.is_whitespace()).collect();

    let expected_canonized : String = r#"
Loop(SStrictSeq,
    Alt(
        Strict(
            Emission(EmissionAction{orig_lf_id:0,ms_id:0,target_gates:[]}),
            Reception(ReceptionAction{origin_gate:None,ms_id:0,targ_lf_id:1})
        ),
        Strict(
            Emission(EmissionAction{orig_lf_id:0,ms_id:1,target_gates:[]}),
            Reception(ReceptionAction{origin_gate:None,ms_id:1,targ_lf_id:1})
        )
    )
)
    "#.chars().filter(|c| !c.is_whitespace()).collect();

    tool_test_canonize(
        gen_ctx,
        input_text,
        &expected_int,
        &expected_canonized,
        None,
        true,
        simplified_text
    )
}




#[test]
pub fn test_simpl_neutral_and_kleene_nesting() {

    let gen_ctx = get_gen_ctx();
    let input_text = r#"
loopS(
    loopW(
        seq(
            0,
            l1 -- m1 -> l2,
            loopS(
                0
            )
        )
    )
)
    "#;

    let simplified_text = r#"
loopW(
    l1 -- m1 -> l2
)
    "#;

    let expected_int : String = r#"
Loop(SStrictSeq,
    Loop(Coreg([]),
        CoReg([],
            Empty,
            CoReg([],
                Strict(
                    Emission(EmissionAction{orig_lf_id:0,ms_id:0,target_gates:[]}),
                    Reception(ReceptionAction{origin_gate:None,ms_id:0,targ_lf_id:1})
                ),
                Loop(SStrictSeq,
                    Empty
                )
            )
        )
    )
)
    "#.chars().filter(|c| !c.is_whitespace()).collect();

    let expected_canonized : String = r#"
Loop(Coreg([]),
    Strict(
        Emission(EmissionAction{orig_lf_id:0,ms_id:0,target_gates:[]}),
        Reception(ReceptionAction{origin_gate:None,ms_id:0,targ_lf_id:1})
    )
)
    "#.chars().filter(|c| !c.is_whitespace()).collect();

    tool_test_canonize(
        gen_ctx,
        input_text,
        &expected_int,
        &expected_canonized,
        None,
        true,
        simplified_text
    )
}






#[test]
pub fn test_coreg_minimization() {

    let gen_ctx = get_gen_ctx();
    let input_text = r#"
coreg(l1)(
    l1 -- m3 -> l1,
    coreg(l1,l2,l3)(
        l1 -- m2 -> l1,
        l1 -- m1 -> l1
    )
)
    "#;

    let simplified_text = r#"
coreg(l1)(
    l1 -- m3 -> l1,
    coreg(l1)(
        l1 -- m2 -> l1,
        l1 -- m1 -> l1
    )
)
    "#;

    let expected_int : String = r#"
CoReg([0],
    Strict(
        Emission(EmissionAction{orig_lf_id:0,ms_id:2,target_gates:[]}),
        Reception(ReceptionAction{origin_gate:None,ms_id:2,targ_lf_id:0})
    ),
    CoReg([0,1,2],
        Strict(
            Emission(EmissionAction{orig_lf_id:0,ms_id:1,target_gates:[]}),
            Reception(ReceptionAction{origin_gate:None,ms_id:1,targ_lf_id:0})
        ),
        Strict(
            Emission(EmissionAction{orig_lf_id:0,ms_id:0,target_gates:[]}),
            Reception(ReceptionAction{origin_gate:None,ms_id:0,targ_lf_id:0})
        )
    )
)
    "#.chars().filter(|c| !c.is_whitespace()).collect();

    let expected_canonized : String = r#"
CoReg([0],
    CoReg([],
        Emission(EmissionAction{orig_lf_id:0,ms_id:0,target_gates:[]}),
        Reception(ReceptionAction{origin_gate:None,ms_id:0,targ_lf_id:0})
    ),
    CoReg([0],
        CoReg([],
            Emission(EmissionAction{orig_lf_id:0,ms_id:1,target_gates:[]}),
            Reception(ReceptionAction{origin_gate:None,ms_id:1,targ_lf_id:0})
        ),
        CoReg([],
            Emission(EmissionAction{orig_lf_id:0,ms_id:2,target_gates:[]}),
            Reception(ReceptionAction{origin_gate:None,ms_id:2,targ_lf_id:0})
        )
    )
)
    "#.chars().filter(|c| !c.is_whitespace()).collect();

    tool_test_canonize(
        gen_ctx,
        input_text,
        &expected_int,
        &expected_canonized,
        None,
        true,
        simplified_text
    )
}





#[test]
pub fn test_kleene_desequencing() {

    let gen_ctx = get_gen_ctx();
    let input_text = r#"
seq(
    loopW(
        l1 -- m1 -> l2
    ),
    m3 -> l3,
    loopW(
        alt(
            l1 -- m1 -> l2,
            l1 -- m2 -> l2
        )
    )
)
    "#;

    let simplified_text = r#"
seq(
    m3 -> l3,
    loopW(
        alt(
            l1 -- m1 -> l2,
            l1 -- m2 -> l2
        )
    )
)
    "#;

    let expected_int : String = r#"
CoReg([],
    Loop(Coreg([]),
        Strict(
            Emission(EmissionAction{orig_lf_id:0,ms_id:0,target_gates:[]}),
            Reception(ReceptionAction{origin_gate:None,ms_id:0,targ_lf_id:1})
        )
    ),
    CoReg([],
        Reception(ReceptionAction{origin_gate:None,ms_id:2,targ_lf_id:2}),
        Loop(Coreg([]),
            Alt(
                Strict(
                    Emission(EmissionAction{orig_lf_id:0,ms_id:0,target_gates:[]}),
                    Reception(ReceptionAction{origin_gate:None,ms_id:0,targ_lf_id:1})
                ),
                Strict(
                    Emission(EmissionAction{orig_lf_id:0,ms_id:1,target_gates:[]}),
                    Reception(ReceptionAction{origin_gate:None,ms_id:1,targ_lf_id:1})
                )
            )
        )
    )
)
    "#.chars().filter(|c| !c.is_whitespace()).collect();

    let expected_canonized : String = r#"
CoReg([],
    Reception(ReceptionAction{origin_gate:None,ms_id:2,targ_lf_id:2}),
    Loop(Coreg([]),
        Alt(
            Strict(
                Emission(EmissionAction{orig_lf_id:0,ms_id:0,target_gates:[]}),
                Reception(ReceptionAction{origin_gate:None,ms_id:0,targ_lf_id:1})
            ),
            Strict(
                Emission(EmissionAction{orig_lf_id:0,ms_id:1,target_gates:[]}),
                Reception(ReceptionAction{origin_gate:None,ms_id:1,targ_lf_id:1})
            )
        )
    )
)
    "#.chars().filter(|c| !c.is_whitespace()).collect();

    tool_test_canonize(
        gen_ctx,
        input_text,
        &expected_int,
        &expected_canonized,
        None,
        true,
        simplified_text
    )
}








#[test]
pub fn test_right_seq_compat() {

    let gen_ctx = get_gen_ctx();
    let input_text = r#"
strict(
    l1 -- m1 -> l2,
    seq(
        l1 -- m2 -> l2,
        l2 -- m3 -> l1    
    )
)
    "#;

    let simplified_text = r#"
seq(
    strict(
        l1 -- m1 -> l2,
        l1 -- m2 -> l2
    ),
    l2 -- m3 -> l1    
)
    "#;

    let expected_int : String = r#"
Strict(
    Strict(
        Emission(EmissionAction{orig_lf_id:0,ms_id:0,target_gates:[]}),
        Reception(ReceptionAction{origin_gate:None,ms_id:0,targ_lf_id:1})
    ),
    CoReg([],
        Strict(
            Emission(EmissionAction{orig_lf_id:0,ms_id:1,target_gates:[]}),
            Reception(ReceptionAction{origin_gate:None,ms_id:1,targ_lf_id:1})
        ),
        Strict(
            Emission(EmissionAction{orig_lf_id:1,ms_id:2,target_gates:[]}),
            Reception(ReceptionAction{origin_gate:None,ms_id:2,targ_lf_id:0})
        )
    )
)
    "#.chars().filter(|c| !c.is_whitespace()).collect();

    let expected_canonized : String = r#"
CoReg([],
    Strict(
        Emission(EmissionAction{orig_lf_id:0,ms_id:0,target_gates:[]}),
        Strict(
            Reception(ReceptionAction{origin_gate:None,ms_id:0,targ_lf_id:1}),
            Strict(
                Emission(EmissionAction{orig_lf_id:0,ms_id:1,target_gates:[]}),
                Reception(ReceptionAction{origin_gate:None,ms_id:1,targ_lf_id:1})
            )
        )
    ),
    Strict(
        Emission(EmissionAction{orig_lf_id:1,ms_id:2,target_gates:[]}),
        Reception(ReceptionAction{origin_gate:None,ms_id:2,targ_lf_id:0})
    )
)
    "#.chars().filter(|c| !c.is_whitespace()).collect();

    tool_test_canonize(
        gen_ctx,
        input_text,
        &expected_int,
        &expected_canonized,
        None,
        true,
        simplified_text
    )
}






#[test]
pub fn test_left_seq_compat() {

    let gen_ctx = get_gen_ctx();
    let input_text = r#"
strict(
    seq(
        l1 -- m1 -> |,
        l1 -- m2 -> |
    ),
    l2 -- m3 -> |
)
    "#;

    let simplified_text = r#"
seq(
    l1 -- m1 -> |,
    strict(
        l1 -- m2 -> |,
        l2 -- m3 -> |
    )
)
    "#;

    let expected_int : String = r#"
Strict(
    CoReg([],
        Emission(EmissionAction{orig_lf_id:0,ms_id:0,target_gates:[]}),
        Emission(EmissionAction{orig_lf_id:0,ms_id:1,target_gates:[]})
    ),
    Emission(EmissionAction{orig_lf_id:1,ms_id:2,target_gates:[]})
)
    "#.chars().filter(|c| !c.is_whitespace()).collect();

    let expected_canonized : String = r#"
CoReg([],
    Emission(EmissionAction{orig_lf_id:0,ms_id:0,target_gates:[]}),
    Strict(
        Emission(EmissionAction{orig_lf_id:0,ms_id:1,target_gates:[]}),
        Emission(EmissionAction{orig_lf_id:1,ms_id:2,target_gates:[]})
    )
)
    "#.chars().filter(|c| !c.is_whitespace()).collect();

    tool_test_canonize(
        gen_ctx,
        input_text,
        &expected_int,
        &expected_canonized,
        None,
        true,
        simplified_text
    )
}





#[test]
pub fn test_strictness_relax() {

    let gen_ctx = get_gen_ctx();
    let input_text = r#"
strict(
    l1 -- m1 -> l2,
    seq(
        l2 -- m2 -> l1,
        l2 -- m3 -> l1
    )
)
    "#;

    let simplified_text = r#"
seq(
    l1 -- m1 -> l2,
    l2 -- m2 -> l1,
    l2 -- m3 -> l1
)
    "#;

    let expected_int : String = r#"
Strict(
    Strict(
        Emission(EmissionAction{orig_lf_id:0,ms_id:0,target_gates:[]}),
        Reception(ReceptionAction{origin_gate:None,ms_id:0,targ_lf_id:1})
    ),
    CoReg([],
        Strict(
            Emission(EmissionAction{orig_lf_id:1,ms_id:1,target_gates:[]}),
            Reception(ReceptionAction{origin_gate:None,ms_id:1,targ_lf_id:0})
        ),
        Strict(
            Emission(EmissionAction{orig_lf_id:1,ms_id:2,target_gates:[]}),
            Reception(ReceptionAction{origin_gate:None,ms_id:2,targ_lf_id:0})
        )
    )
)
    "#.chars().filter(|c| !c.is_whitespace()).collect();

    let expected_canonized : String = r#"
CoReg([],
    Strict(
        Emission(EmissionAction{orig_lf_id:0,ms_id:0,target_gates:[]}),
        Reception(ReceptionAction{origin_gate:None,ms_id:0,targ_lf_id:1})
    ),
    CoReg([],
        Strict(
            Emission(EmissionAction{orig_lf_id:1,ms_id:1,target_gates:[]}),
            Reception(ReceptionAction{origin_gate:None,ms_id:1,targ_lf_id:0})
        ),
        Strict(
            Emission(EmissionAction{orig_lf_id:1,ms_id:2,target_gates:[]}),
            Reception(ReceptionAction{origin_gate:None,ms_id:2,targ_lf_id:0})
        )
    )
)
    "#.chars().filter(|c| !c.is_whitespace()).collect();

    tool_test_canonize(
        gen_ctx,
        input_text,
        &expected_int,
        &expected_canonized,
        None,
        true,
        simplified_text
    )
}




#[test]
pub fn test_strictness_relax_kleene() {

    let gen_ctx = get_gen_ctx();
    let input_text = r#"
loopS(
    seq(
        l1 -- m1 -> l2,
        l2 -- m2 -> l1
    )
)
    "#;

    let simplified_text = r#"
loopW(
    seq(
        l1 -- m1 -> l2,
        l2 -- m2 -> l1
    )
)
    "#;

    let expected_int : String = r#"
Loop(SStrictSeq,
    CoReg([],
        Strict(
            Emission(EmissionAction{orig_lf_id:0,ms_id:0,target_gates:[]}),
            Reception(ReceptionAction{origin_gate:None,ms_id:0,targ_lf_id:1})
        ),
        Strict(
            Emission(EmissionAction{orig_lf_id:1,ms_id:1,target_gates:[]}),
            Reception(ReceptionAction{origin_gate:None,ms_id:1,targ_lf_id:0})
        )
    )
)
    "#.chars().filter(|c| !c.is_whitespace()).collect();

    let expected_canonized : String = r#"
Loop(Coreg([]),
    CoReg([],
        Strict(
            Emission(EmissionAction{orig_lf_id:0,ms_id:0,target_gates:[]}),
            Reception(ReceptionAction{origin_gate:None,ms_id:0,targ_lf_id:1})
        ),
        Strict(
            Emission(EmissionAction{orig_lf_id:1,ms_id:1,target_gates:[]}),
            Reception(ReceptionAction{origin_gate:None,ms_id:1,targ_lf_id:0})
        )
    )
)
    "#.chars().filter(|c| !c.is_whitespace()).collect();

    tool_test_canonize(
        gen_ctx,
        input_text,
        &expected_int,
        &expected_canonized,
        None,
        true,
        simplified_text
    )
}




#[test]
pub fn test_strictness_relax_kleene_undisturbed() {

    let gen_ctx = get_gen_ctx();
    let input_text = r#"
loopS(
    l1 -- m1 -> l2
)
    "#;

    let simplified_text = input_text;

    let expected_int : String = r#"
Loop(SStrictSeq,
    Strict(
        Emission(EmissionAction{orig_lf_id:0,ms_id:0,target_gates:[]}),
        Reception(ReceptionAction{origin_gate:None,ms_id:0,targ_lf_id:1})
    )
)
    "#.chars().filter(|c| !c.is_whitespace()).collect();

    let expected_canonized : String = expected_int.clone();

    tool_test_canonize(
        gen_ctx,
        input_text,
        &expected_int,
        &expected_canonized,
        None,
        true,
        simplified_text
    )
}

