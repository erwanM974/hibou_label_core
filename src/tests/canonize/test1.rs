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

use super::util::{get_gen_ctx, tool_test_canonize};




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
        Some(&expected_int),
        Some(&expected_canonized),
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
        Some(&expected_int),
        Some(&expected_canonized),
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
        Some(&expected_int),
        Some(&expected_canonized),
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
        Some(&expected_int),
        Some(&expected_canonized),
        None,
        true,
        simplified_text
    )
}




#[test]
pub fn test_kleene_desequencing_undisturbed() {

    let gen_ctx = get_gen_ctx();
    let input_text = r#"
seq(
    loopS(
        l1 -- m1 -> l3
    ),
    l3 -- m3 -> l1,
    loopS(
        l1 -- m2 -> l3
    )
)
    "#;

    let simplified_text = input_text;

    let expected_int : String = r#"
CoReg([],
    Loop(SStrictSeq,
        Strict(
            Emission(EmissionAction{orig_lf_id:0,ms_id:0,target_gates:[]}),
            Reception(ReceptionAction{origin_gate:None,ms_id:0,targ_lf_id:2})
        )
    ),
    CoReg([],
        Strict(
            Emission(EmissionAction{orig_lf_id:2,ms_id:2,target_gates:[]}),
            Reception(ReceptionAction{origin_gate:None,ms_id:2,targ_lf_id:0})
        ),
        Loop(SStrictSeq,
            Strict(
                Emission(EmissionAction{orig_lf_id:0,ms_id:1,target_gates:[]}),
                Reception(ReceptionAction{origin_gate:None,ms_id:1,targ_lf_id:2})
            )
        )
    )
)
    "#.chars().filter(|c| !c.is_whitespace()).collect();

    let expected_canonized : String = expected_int.clone();

    tool_test_canonize(
        gen_ctx,
        input_text,
        Some(&expected_int),
        Some(&expected_canonized),
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
        Some(&expected_int),
        Some(&expected_canonized),
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
        Some(&expected_int),
        Some(&expected_canonized),
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
        Some(&expected_int),
        Some(&expected_canonized),
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
        Some(&expected_int),
        Some(&expected_canonized),
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
        Some(&expected_int),
        Some(&expected_canonized),
        None,
        true,
        simplified_text
    )
}






