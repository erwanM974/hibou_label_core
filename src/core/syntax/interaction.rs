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


use std::hash::Hash;

use super::action::{EmissionAction, ReceptionAction};


#[derive(Clone, PartialEq, Debug, Eq, PartialOrd, Ord, Hash)]
pub enum LoopKind {
    HHeadFirstWS,
    SStrictSeq,
    Coreg(Vec<usize>)
}

impl LoopKind {

    pub fn is_more_permissive(&self, other : &LoopKind) -> Option<bool> {
        match (self,other) {
            (LoopKind::Coreg(cr1),LoopKind::Coreg(cr2)) => {
                if cr2.iter().all(|lf| cr1.contains(lf)) {
                    return Some(true);
                } 
                if cr1.iter().all(|lf| cr2.contains(lf)) {
                    return Some(false);
                }
                None
            },
            (LoopKind::Coreg(_),LoopKind::SStrictSeq) => {
                Some(true)
            },
            (LoopKind::SStrictSeq,LoopKind::Coreg(_)) => {
                Some(false)
            },
            (LoopKind::SStrictSeq,LoopKind::SStrictSeq) => {
                Some(true)
            },
            (_,_) => {
                None
            }
        }
    }

    pub fn get_most_permissive(&self, other : &LoopKind) -> Option<LoopKind> {
        match self.is_more_permissive(other) {
            None => {
                None
            },
            Some(result) => {
                if result {
                    Some(self.clone())
                } else {
                    Some(other.clone())
                }
            }
        }
        /*
        match (self,other) {
            (LoopKind::Coreg(cr1),LoopKind::Coreg(cr2)) => {
                if cr1.iter().all(|lf| cr2.contains(lf)) {
                    let most_permissive = LoopKind::Coreg(cr2.clone());
                    return Some(most_permissive);
                }
                if cr2.iter().all(|lf| cr1.contains(lf)) {
                    let most_permissive = LoopKind::Coreg(cr1.clone());
                    return Some(most_permissive);
                } 
                None
            },
            (LoopKind::Coreg(cr1),LoopKind::SStrictSeq) => {
                Some(LoopKind::Coreg(cr1.clone()))
            },
            (LoopKind::SStrictSeq,LoopKind::Coreg(cr1)) => {
                Some(LoopKind::Coreg(cr1.clone()))
            },
            (LoopKind::SStrictSeq,LoopKind::SStrictSeq) => {
                Some(LoopKind::SStrictSeq)
            },
            (_,_) => {
                None
            }
        }*/
    }
}

#[derive(Clone, PartialEq, Debug, Eq, Hash, PartialOrd)]
pub enum Interaction {
    Empty,
    Emission(EmissionAction),
    Reception(ReceptionAction),
    Strict(Box<Interaction>,Box<Interaction>),
    CoReg(Vec<usize>,Box<Interaction>,Box<Interaction>),
    Alt(Box<Interaction>,Box<Interaction>),
    Loop(LoopKind,Box<Interaction>),
    And(Box<Interaction>,Box<Interaction>),
    //Sync(Vec<TraceAction>,Box<Interaction>,Box<Interaction>)
}


impl Interaction {

    pub fn reverse_interaction(&self) -> Interaction {
        match *self {
            Interaction::Empty => {
                Interaction::Empty
            },
            Interaction::Emission(ref em_act) => {
                Interaction::Emission(em_act.clone())
            },
            Interaction::Reception(ref rc_act) => {
                Interaction::Reception(rc_act.clone())
            },
            Interaction::Strict(ref i1, ref i2) => {
                Interaction::Strict(Box::new(i2.reverse_interaction()),Box::new(i1.reverse_interaction()))
            },
            Interaction::CoReg(ref cr, ref i1, ref i2) => {
                Interaction::CoReg(cr.clone(),Box::new(i2.reverse_interaction()),Box::new(i1.reverse_interaction()))
            },
            Interaction::Alt(ref i1, ref i2) => {
                Interaction::Alt(Box::new(i2.reverse_interaction()),Box::new(i1.reverse_interaction()))
            },
            Interaction::Loop(ref lk, ref i1) => {
                Interaction::Loop(lk.clone(), Box::new(i1.reverse_interaction()))
            },
            /*Interaction::Sync(ref s,ref i1, ref i2) => {
                Interaction::Sync(s.clone(),Box::new(i2.reverse()),Box::new(i1.reverse()))
            },*/
            _ => {
                panic!("non-conform interaction");
            }
        }
    }

    pub fn express_empty(&self) -> bool {
        match *self {
            Interaction::Empty => {
                true
            },
            Interaction::Emission(_) => {
                false
            },
            Interaction::Reception(_) => {
                false
            },
            Interaction::Strict(ref i1, ref i2) => {
                i1.express_empty() && i2.express_empty()
            },
            Interaction::CoReg(_, ref i1, ref i2) => {
                i1.express_empty() && i2.express_empty()
            },
            Interaction::Alt(ref i1, ref i2) => {
                i1.express_empty() || i2.express_empty()
            },
            Interaction::Loop(_, _) => {
                true
            },
            /*Interaction::Sync(_,ref i1, ref i2) => {
                i1.express_empty() && i2.express_empty()
            },*/
            _ => {
                panic!("non-conform interaction");
            }
        }
    }

}


