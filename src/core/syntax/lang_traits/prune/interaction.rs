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

use crate::core::syntax::{interaction::Interaction, lang_traits::{avoid::avoids::AvoidsLifelines, involve::involves::InvolvesLifelines}};

use super::prunable::LifelinePrunable;



impl LifelinePrunable for Interaction {
    fn prune(&self, lf_ids : &BTreeSet<usize>) -> Interaction {
        match self {
            Interaction::Empty => {
                Interaction::Empty
            },
            Interaction::Emission(_) => {
                self.clone()
            },
            Interaction::Reception(_) => {
                self.clone()
            },
            Interaction::CoReg(cr, i1, i2) => {
                let pruned_i1 = i1.prune(lf_ids);
                let pruned_i2 = i2.prune(lf_ids);
                if pruned_i1 == Interaction::Empty {
                    pruned_i2
                }else {
                    if pruned_i2 == Interaction::Empty {
                        pruned_i1
                    } else {
                        Interaction::CoReg( cr.clone(),Box::new(pruned_i1) , Box::new(pruned_i2) )
                    }
                }
            },
            /*Interaction::Sync(sync_acts, i1, i2) => {
                let pruned_i1 = i1.prune(lf_ids);
                let pruned_i2 = i2.prune(lf_ids);
                // ***
                let acts1 = pruned_i1.get_all_trace_actions();
                let acts2 = pruned_i2.get_all_trace_actions();
                // ***
                let sync_acts_as_set : BTreeSet<TraceAction> = BTreeSet::from_iter(sync_acts.iter().cloned());
                let intersetc1 = sync_acts_as_set.intersection(&acts1).count();
                let intersetc2 = sync_acts_as_set.intersection(&acts2).count();
                // ***
                let new_i : Interaction;
                if intersetc1 == 0 && intersetc2 == 0 {
                    if pruned_i1 == Interaction::Empty {
                        return pruned_i2;
                    } else {
                        if pruned_i2 == Interaction::Empty {
                            return pruned_i1;
                        } else {
                            return Interaction::Par( Box::new(pruned_i1) ,
                                                     Box::new(pruned_i2) );
                        }
                    }
                } else {
                    return Interaction::Sync(sync_acts.clone(),
                                             Box::new(pruned_i1) ,
                                             Box::new(pruned_i2));
                }
            },*/
            Interaction::Strict(i1, i2) => {
                let pruned_i1 = i1.prune(lf_ids);
                let pruned_i2 = i2.prune(lf_ids);
                if pruned_i1 == Interaction::Empty {
                    pruned_i2
                }else {
                    if pruned_i2 == Interaction::Empty {
                        pruned_i1
                    } else {
                        Interaction::Strict( Box::new(pruned_i1) , Box::new(pruned_i2) )
                    }
                }
            },
            Interaction::Alt(i1, i2) => {
                if i1.avoids_all_of(lf_ids) {
                    if i2.avoids_all_of(lf_ids) {
                        let pruned_i1 = i1.prune(lf_ids);
                        let pruned_i2 = i2.prune(lf_ids);
                        match (pruned_i1,pruned_i2) {
                            (Interaction::Empty,Interaction::Empty) => {
                                Interaction::Empty
                            },
                            (Interaction::Empty,Interaction::Loop(lk,i21)) => {
                                Interaction::Loop(lk,i21)
                            },
                            (Interaction::Loop(lk,i11),Interaction::Empty) => {
                                Interaction::Loop(lk,i11)
                            },
                            (pi1,pi2) => {
                                Interaction::Alt( Box::new( pi1),
                                                  Box::new( pi2) )
                            }
                        }
                    } else {
                        i1.prune(lf_ids)
                    }
                } else {
                    i2.prune(lf_ids)
                }
            },
            Interaction::Loop(lkind, i1) => {
                if i1.avoids_all_of(lf_ids) {
                    let pruned_i1 = i1.prune(lf_ids);
                    if pruned_i1 != Interaction::Empty {
                        return Interaction::Loop(lkind.clone(), Box::new(pruned_i1) );
                    }
                }
                return Interaction::Empty;
            },
            Interaction::And(_,_) => {
                panic!("non-conform interaction");
            }
        }
    }

    fn prune_with_affected(&self, lf_ids : &BTreeSet<usize>) -> (Interaction,BTreeSet<usize>) {
        match self {
            Interaction::Empty => {
                (Interaction::Empty,btreeset!{})
            },
            Interaction::Emission(_) => {
                (self.clone(),btreeset!{})
            },
            Interaction::Reception(_) => {
                (self.clone(),btreeset!{})
            },
            Interaction::CoReg(cr, i1, i2) => {
                let (pruned_i1,mut aff1) = i1.prune_with_affected(lf_ids);
                let (pruned_i2,aff2) = i2.prune_with_affected(lf_ids);
                aff1.extend(aff2);
                if pruned_i1 == Interaction::Empty {
                    (pruned_i2,aff1)
                }else {
                    if pruned_i2 == Interaction::Empty {
                        (pruned_i1,aff1)
                    } else {
                        (Interaction::CoReg( cr.clone(),Box::new(pruned_i1) , Box::new(pruned_i2) ),aff1)
                    }
                }
            },
            Interaction::Strict(i1, i2) => {
                let (pruned_i1,mut aff1) = i1.prune_with_affected(lf_ids);
                let (pruned_i2,aff2) = i2.prune_with_affected(lf_ids);
                aff1.extend(aff2);
                if pruned_i1 == Interaction::Empty {
                    (pruned_i2,aff1)
                }else {
                    if pruned_i2 == Interaction::Empty {
                        (pruned_i1,aff1)
                    } else {
                        (Interaction::Strict( Box::new(pruned_i1) , Box::new(pruned_i2) ),aff1)
                    }
                }
            },
            Interaction::Alt(i1, i2) => {
                if i1.avoids_all_of(lf_ids) {
                    if i2.avoids_all_of(lf_ids) {
                        let (pruned_i1,mut aff1) = i1.prune_with_affected(lf_ids);
                        let (pruned_i2,aff2) = i2.prune_with_affected(lf_ids);
                        aff1.extend(aff2);
                        return (Interaction::Alt( Box::new( pruned_i1), Box::new( pruned_i2) ), aff1);
                    } else {
                        let pruned_i1 = i1.prune(lf_ids);
                        let mut aff = i1.lifelines_that_may_be_involved();
                        aff.extend(i2.lifelines_that_may_be_involved());
                        return (pruned_i1,aff);
                    }
                } else {
                    let pruned_i2 = i2.prune(lf_ids);
                    let mut aff = i1.lifelines_that_may_be_involved();
                    aff.extend(i2.lifelines_that_may_be_involved());
                    return (pruned_i2,aff);
                }
            },
            Interaction::Loop(lkind, i1) => {
                if i1.avoids_all_of(lf_ids) {
                    let (pruned_i1,aff1) = i1.prune_with_affected(lf_ids);
                    if pruned_i1 != Interaction::Empty {
                        return (Interaction::Loop(lkind.clone(), Box::new(pruned_i1) ),aff1);
                    } else {
                        return (Interaction::Empty,aff1);
                    }
                }
                return (Interaction::Empty,i1.lifelines_that_may_be_involved());
            },
            Interaction::And(_,_) => {
                panic!("non-conform interaction");
            }
        }
    }
}



