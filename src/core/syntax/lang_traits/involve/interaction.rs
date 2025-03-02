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




use std::collections::BTreeSet;

use maplit::btreeset;

use crate::core::syntax::interaction::Interaction;

use super::involves::InvolvesLifelines;





impl InvolvesLifelines for Interaction {
    fn lifelines_that_may_be_involved(&self) -> BTreeSet<usize> {
        match &self {
            &Interaction::Empty => {
                btreeset!{}
            },
            &Interaction::Emission(ref em_act) => {
                btreeset!{em_act.orig_lf_id}
            },
            &Interaction::Reception(ref rc_act) => {
                btreeset!{rc_act.targ_lf_id}
            },
            &Interaction::Strict(ref i1, ref i2) => {
                let mut content = i1.lifelines_that_may_be_involved();
                content.extend( i2.lifelines_that_may_be_involved() );
                content
            },
            &Interaction::CoReg(_, ref i1, ref i2) => {
                let mut content = i1.lifelines_that_may_be_involved();
                content.extend( i2.lifelines_that_may_be_involved() );
                content
            },
            /*&Interaction::Sync(_, ref i1, ref i2) => {
                let mut content = i1.lifelines_that_may_be_involved();
                content.extend( i2.lifelines_that_may_be_involved() );
                return content;
            },*/
            &Interaction::Alt(ref i1, ref i2) => {
                let mut content = i1.lifelines_that_may_be_involved();
                content.extend( i2.lifelines_that_may_be_involved() );
                content
            },
            &Interaction::Loop(_, i1) => {
                i1.lifelines_that_may_be_involved()
            },
            &Interaction::And(_,_) => {
                panic!("non-conform interaction");
            }
        }
    }


    fn lifelines_that_must_be_involved(&self) -> BTreeSet<usize> {
        match &self {
            &Interaction::Empty => {
                btreeset!{}
            },
            &Interaction::Emission(ref em_act) => {
                btreeset!{em_act.orig_lf_id}
            },
            &Interaction::Reception(ref rc_act) => {
                btreeset!{rc_act.targ_lf_id}
            },
            &Interaction::Strict(ref i1, ref i2) => {
                let mut content = i1.lifelines_that_may_be_involved();
                content.extend( i2.lifelines_that_may_be_involved() );
                content
            },
            &Interaction::CoReg(_, ref i1, ref i2) => {
                let mut content = i1.lifelines_that_may_be_involved();
                content.extend( i2.lifelines_that_may_be_involved() );
                content
            },
            /*&Interaction::Sync(_, ref i1, ref i2) => {
                let mut content = i1.lifelines_that_may_be_involved();
                content.extend( i2.lifelines_that_may_be_involved() );
                return content;
            },*/
            &Interaction::Alt(ref i1, ref i2) => {
                let must_i1 =  i1.lifelines_that_must_be_involved();
                let must_i2 =  i2.lifelines_that_must_be_involved();
                must_i1.intersection(&must_i2).into_iter().cloned().collect()
            },
            &Interaction::Loop(_, _) => {
                btreeset! {}
            },
            &Interaction::And(_,_) => {
                panic!("non-conform interaction");
            }
        }
    }

}