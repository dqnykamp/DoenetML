use enum_as_inner::EnumAsInner;
use serde::Serialize;

use crate::state_variables::*;
use std::{
    cell::{Ref, RefCell},
    fmt,
    ops::Deref,
    rc::Rc,
};
// use self::Freshness::*;

use crate::math_expression::MathExpression;

#[derive(Debug)]
pub enum StateVar {
    Number(StateVarTyped<f64>),
    Integer(StateVarTyped<i64>),
    String(StateVarTyped<String>),
    Boolean(StateVarTyped<bool>),
    MathExpr(StateVarTyped<MathExpression>),
}

#[derive(Debug)]
pub enum StateVarReadOnly {
    Number(StateVarReadOnlyTyped<f64>),
    Integer(StateVarReadOnlyTyped<i64>),
    String(StateVarReadOnlyTyped<String>),
    Boolean(StateVarReadOnlyTyped<bool>),
    MathExpr(StateVarReadOnlyTyped<MathExpression>),
}

#[derive(Debug)]
pub struct StateVarTyped<T: Default> {
    inner: Rc<RefCell<StateVarInner<T>>>,
}

#[derive(Debug)]
struct StateVarInner<T: Default> {
    value: T,
    freshness: Freshness,
}

impl<T: Default> StateVarInner<T> {
    pub fn get_value_assuming_fresh<'a>(&'a self) -> &'a T {
        if self.freshness != Freshness::Fresh {
            panic!("State variable is not fresh, cannot get its fresh value");
        }
        &self.value
    }

    pub fn mark_stale(&mut self) {
        self.freshness = Freshness::Stale;
    }

    pub fn set_value(&mut self, new_val: T) {
        self.value = new_val;
        self.freshness = Freshness::Fresh;
    }
}

impl<T: Default> StateVarTyped<T> {
    pub fn new() -> Self {
        StateVarTyped {
            inner: Rc::new(RefCell::new(StateVarInner {
                value: T::default(),
                freshness: Freshness::Unresolved,
            })),
        }
    }

    // use get_read_only_view to main a reference to the value
    // that you can repeatedly access its value
    // but allow the value to be modified when not accessing the value
    pub fn create_new_read_only_view(&self) -> StateVarReadOnlyTyped<T> {
        StateVarReadOnlyTyped {
            inner: self.inner.clone(),
        }
    }

    pub fn get_value_assuming_fresh<'a>(&'a self) -> impl Deref<Target = T> + 'a {
        Ref::map(self.inner.borrow(), |v| v.get_value_assuming_fresh())
    }

    pub fn set_value(&self, new_val: T) {
        self.inner.borrow_mut().set_value(new_val);
    }

    pub fn create_new_mutable_view(&self) -> StateVarTyped<T> {
        StateVarTyped {
            inner: self.inner.clone(),
        }
    }

    pub fn mark_stale(&self) {
        self.inner.borrow_mut().mark_stale()
    }

    pub fn get_freshness(&self) -> Freshness {
        self.inner.borrow().freshness
    }
}

#[derive(Debug)]
pub struct StateVarReadOnlyTyped<T: Default> {
    // this must remain private to ensure read only
    inner: Rc<RefCell<StateVarInner<T>>>,
}

impl<T: Default> StateVarReadOnlyTyped<T> {
    pub fn new() -> Self {
        StateVarReadOnlyTyped {
            inner: Rc::new(RefCell::new(StateVarInner {
                value: T::default(),
                freshness: Freshness::Unresolved,
            })),
        }
    }
    pub fn get_value_assuming_fresh<'a>(&'a self) -> impl Deref<Target = T> + 'a {
        Ref::map(self.inner.borrow(), |v| v.get_value_assuming_fresh())
    }

    pub fn create_new_read_only_view(&self) -> StateVarReadOnlyTyped<T> {
        StateVarReadOnlyTyped { inner: self.inner.clone() }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, EnumAsInner)]
pub enum Freshness {
    Fresh,
    Stale,
    Unresolved,
}

impl From<&StateVar> for serde_json::Value {
    fn from(state_var: &StateVar) -> serde_json::Value {
        match state_var.get_freshness() {
            Freshness::Fresh => match state_var {
                StateVar::Number(inner_sv) => {
                    serde_json::json!(*inner_sv.get_value_assuming_fresh())
                }
                StateVar::Integer(inner_sv) => {
                    serde_json::json!(*inner_sv.get_value_assuming_fresh())
                }
                StateVar::String(inner_sv) => {
                    serde_json::json!(inner_sv.get_value_assuming_fresh().clone())
                }
                StateVar::Boolean(inner_sv) => {
                    serde_json::json!(*inner_sv.get_value_assuming_fresh())
                }
                StateVar::MathExpr(inner_sv) => {
                    serde_json::json!(*inner_sv.get_value_assuming_fresh())
                }
            },
            Freshness::Stale => serde_json::Value::Null,
            Freshness::Unresolved => serde_json::Value::Null,
        }
    }
}

impl StateVar {
    /// Create Unresolved StateVar of the given type
    pub fn new(value_type: &StateVarVariant) -> Self {
        match value_type {
            StateVarVariant::Number(_) => StateVar::Number(StateVarTyped {
                inner: Rc::new(RefCell::new(StateVarInner {
                    value: f64::NAN,
                    freshness: Freshness::Unresolved,
                })),
            }),
            StateVarVariant::Integer(_) => StateVar::Integer(StateVarTyped {
                inner: Rc::new(RefCell::new(StateVarInner {
                    value: 0,
                    freshness: Freshness::Unresolved,
                })),
            }),
            StateVarVariant::String(_) => StateVar::String(StateVarTyped {
                inner: Rc::new(RefCell::new(StateVarInner {
                    value: String::from(""),
                    freshness: Freshness::Unresolved,
                })),
            }),
            StateVarVariant::Boolean(_) => StateVar::Boolean(StateVarTyped {
                inner: Rc::new(RefCell::new(StateVarInner {
                    value: false,
                    freshness: Freshness::Unresolved,
                })),
            }),
        }
    }

    pub fn new_with_value(value: StateVarValue) -> Self {
        match value {
            StateVarValue::Number(typed_val) => StateVar::Number(StateVarTyped {
                inner: Rc::new(RefCell::new(StateVarInner {
                    value: typed_val,
                    freshness: Freshness::Fresh,
                })),
            }),
            StateVarValue::Integer(typed_val) => StateVar::Integer(StateVarTyped {
                inner: Rc::new(RefCell::new(StateVarInner {
                    value: typed_val,
                    freshness: Freshness::Fresh,
                })),
            }),
            StateVarValue::String(typed_val) => StateVar::String(StateVarTyped {
                inner: Rc::new(RefCell::new(StateVarInner {
                    value: typed_val,
                    freshness: Freshness::Fresh,
                })),
            }),
            StateVarValue::Boolean(typed_val) => StateVar::Boolean(StateVarTyped {
                inner: Rc::new(RefCell::new(StateVarInner {
                    value: typed_val,
                    freshness: Freshness::Fresh,
                })),
            }),
            StateVarValue::MathExpr(typed_val) => StateVar::MathExpr(StateVarTyped {
                inner: Rc::new(RefCell::new(StateVarInner {
                    value: typed_val,
                    freshness: Freshness::Fresh,
                })),
            }),
        }
    }

    // pub fn try_set_value(&mut self, new_value: StateVarValue) -> Result<StateVarValue, String> {
    //     match self {
    //         StateVar::Number(sv_typed) => {
    //             let sv_inner = sv_typed.inner.borrow_mut();
    //             sv_inner.value = new_value.clone().try_into()?;
    //             sv_inner.freshness = Freshness::Fresh;
    //         }
    //         StateVar::Integer(sv_typed) => {
    //             let sv_inner = sv_typed.inner.borrow_mut();
    //             sv_inner.value = new_value.clone().try_into()?;
    //             sv_inner.freshness = Freshness::Fresh;
    //         }
    //         StateVar::String(sv_typed) => {
    //             let sv_inner = sv_typed.inner.borrow_mut();
    //             sv_inner.value = new_value.clone().try_into()?;
    //             sv_inner.freshness = Freshness::Fresh;
    //         }
    //         StateVar::Boolean(sv_typed) => {
    //             let sv_inner = sv_typed.inner.borrow_mut();
    //             sv_inner.value = new_value.clone().try_into()?;
    //             sv_inner.freshness = Freshness::Fresh;
    //         }
    //         StateVar::MathExpr(sv_typed) => {
    //             let sv_inner = sv_typed.inner.borrow_mut();
    //             sv_inner.value = new_value.clone().try_into()?;
    //             sv_inner.freshness = Freshness::Fresh;
    //         }
    //     }

    //     Ok(new_value)
    // }

    pub fn mark_stale(&mut self) {
        match self {
            StateVar::Number(sv_typed) => sv_typed.mark_stale(),
            StateVar::Integer(sv_typed) => sv_typed.mark_stale(),
            StateVar::String(sv_typed) => sv_typed.mark_stale(),
            StateVar::Boolean(sv_typed) => sv_typed.mark_stale(),
            StateVar::MathExpr(sv_typed) => sv_typed.mark_stale(),
        };
    }

    pub fn get_freshness(&self) -> Freshness {
        match self {
            StateVar::Number(sv_typed) => sv_typed.get_freshness(),
            StateVar::Integer(sv_typed) => sv_typed.get_freshness(),
            StateVar::String(sv_typed) => sv_typed.get_freshness(),
            StateVar::Boolean(sv_typed) => sv_typed.get_freshness(),
            StateVar::MathExpr(sv_typed) => sv_typed.get_freshness(),
        }
    }

    // use get_read_only_view to main a reference to the value
    // that you can repeatedly access its value
    // but allow the value to be modified when not accessing the value
    pub fn get_read_only_view(&self) -> StateVarReadOnly {
        match self {
            StateVar::Number(sv_inneer) => {
                StateVarReadOnly::Number(sv_inneer.create_new_read_only_view())
            }
            StateVar::Integer(sv_inner) => {
                StateVarReadOnly::Integer(sv_inner.create_new_read_only_view())
            }
            StateVar::String(sv_inner) => {
                StateVarReadOnly::String(sv_inner.create_new_read_only_view())
            }
            StateVar::Boolean(sv_inner) => {
                StateVarReadOnly::Boolean(sv_inner.create_new_read_only_view())
            }
            StateVar::MathExpr(sv_inner) => {
                StateVarReadOnly::MathExpr(sv_inner.create_new_read_only_view())
            }
        }
    }

    pub fn get_value_assuming_fresh(&self) -> StateVarValue {
        match self {
            StateVar::Number(sv_inneer) => {
                StateVarValue::Number(*sv_inneer.get_value_assuming_fresh())
            }
            StateVar::Integer(sv_inner) => {
                StateVarValue::Integer(*sv_inner.get_value_assuming_fresh())
            }
            StateVar::String(sv_inner) => {
                StateVarValue::String(sv_inner.get_value_assuming_fresh().clone())
            }
            StateVar::Boolean(sv_inner) => {
                StateVarValue::Boolean(*sv_inner.get_value_assuming_fresh())
            }
            StateVar::MathExpr(sv_inner) => {
                StateVarValue::MathExpr(sv_inner.get_value_assuming_fresh().clone())
            }
        }
    }


    pub fn get_type_as_str(&self) -> &'static str {
        match self {
            Self::String(_) => "string",
            Self::Boolean(_) => "boolean",
            Self::Integer(_) => "integer",
            Self::Number(_) => "number",
            Self::MathExpr(_) => "mathExpression",
        }
    }
}

// /// A special endpoint on the dependency graph which is associated with a
// /// particular state var. Actions often update these.
// /// An EssentialStateVar cannot be stale

pub type EssentialStateVar = StateVar; 

// #[derive(Clone, Debug)]
// pub enum EssentialStateVar {
//     Number(EssentialStateVarInner<f64>),
//     Integer(EssentialStateVarInner<i64>),
//     String(EssentialStateVarInner<String>),
//     Boolean(EssentialStateVarInner<bool>),
//     MathExpr(EssentialStateVarInner<MathExpression>),
// }

// #[derive(Clone, Debug)]
// pub struct EssentialStateVarInner<T> {
//     value: Rc<RefCell<T>>,
// }

// impl<T> EssentialStateVarInner<T> {
//     // use get_read_only_view to main a reference to the value
//     // that you can repeatedly access its value
//     // but allow the value to be modified when not accessing the value
//     pub fn get_read_only_view(&self) -> TypedReadOnlyView<T> {
//         TypedReadOnlyView {
//             val: self.value.clone(),
//         }
//     }

//     // use get_value to get a single reference to the value
//     pub fn get_value<'a>(&'a self) -> impl Deref<Target = T> + 'a {
//         self.value.borrow()
//     }

//     pub fn set_value(&mut self, new_val: T) {
//         *self.value.borrow_mut() = new_val;
//     }
// }

// impl From<&EssentialStateVar> for serde_json::Value {
//     fn from(state_var: &EssentialStateVar) -> serde_json::Value {
//         match state_var {
//             EssentialStateVar::Number(inner_sv) => {
//                 serde_json::json!(*inner_sv.get_value())
//             }
//             EssentialStateVar::Integer(inner_sv) => {
//                 serde_json::json!(*inner_sv.get_value())
//             }
//             EssentialStateVar::String(inner_sv) => {
//                 serde_json::json!(inner_sv.get_value().clone())
//             }
//             EssentialStateVar::Boolean(inner_sv) => {
//                 serde_json::json!(*inner_sv.get_value())
//             }
//             EssentialStateVar::MathExpr(inner_sv) => {
//                 serde_json::json!(*inner_sv.get_value())
//             }
//         }
//     }
// }

// impl EssentialStateVar {
//     /// Create EssentialStateVar of the given type
//     pub fn new(value_type: &StateVarVariant) -> Self {
//         match value_type {
//             StateVarVariant::Number(_) => EssentialStateVar::Number(EssentialStateVarInner {
//                 value: Rc::new(RefCell::new(f64::NAN)),
//             }),
//             StateVarVariant::Integer(_) => EssentialStateVar::Integer(EssentialStateVarInner {
//                 value: Rc::new(RefCell::new(0)),
//             }),
//             StateVarVariant::String(_) => EssentialStateVar::String(EssentialStateVarInner {
//                 value: Rc::new(RefCell::new(String::from(""))),
//             }),
//             StateVarVariant::Boolean(_) => EssentialStateVar::Boolean(EssentialStateVarInner {
//                 value: Rc::new(RefCell::new(false)),
//             }),
//         }
//     }

//     pub fn try_set_value(&mut self, new_value: StateVarValue) -> Result<StateVarValue, String> {
//         match self {
//             EssentialStateVar::Number(sv_inner) => {
//                 *sv_inner.value.borrow_mut() = new_value.clone().try_into()?;
//             }
//             EssentialStateVar::Integer(sv_inner) => {
//                 *sv_inner.value.borrow_mut() = new_value.clone().try_into()?;
//             }
//             EssentialStateVar::String(sv_inner) => {
//                 *sv_inner.value.borrow_mut() = new_value.clone().try_into()?;
//             }
//             EssentialStateVar::Boolean(sv_inner) => {
//                 *sv_inner.value.borrow_mut() = new_value.clone().try_into()?;
//             }
//             EssentialStateVar::MathExpr(sv_inner) => {
//                 *sv_inner.value.borrow_mut() = new_value.clone().try_into()?;
//             }
//         }

//         Ok(new_value)
//     }

//     // use get_read_only_view to main a reference to the value
//     // that you can repeatedly access its value
//     // but allow the value to be modified when not accessing the value
//     fn get_read_only_view(&self) -> StateVarReadOnly {
//         match self {
//             EssentialStateVar::Number(sv_inneer) => {
//                 StateVarReadOnly::Number(sv_inneer.get_read_only_view())
//             }
//             EssentialStateVar::Integer(sv_inner) => {
//                 StateVarReadOnly::Integer(sv_inner.get_read_only_view())
//             }
//             EssentialStateVar::String(sv_inner) => {
//                 StateVarReadOnly::String(sv_inner.get_read_only_view())
//             }
//             EssentialStateVar::Boolean(sv_inner) => {
//                 StateVarReadOnly::Boolean(sv_inner.get_read_only_view())
//             }
//             EssentialStateVar::MathExpr(sv_inner) => {
//                 StateVarReadOnly::MathExpr(sv_inner.get_read_only_view())
//             }
//         }
//     }

//     pub fn get_value(&self) -> StateVarValue {
//         match self {
//             EssentialStateVar::Number(sv_inneer) => StateVarValue::Number(*sv_inneer.get_value()),
//             EssentialStateVar::Integer(sv_inner) => StateVarValue::Integer(*sv_inner.get_value()),
//             EssentialStateVar::String(sv_inner) => {
//                 StateVarValue::String(sv_inner.get_value().clone())
//             }
//             EssentialStateVar::Boolean(sv_inner) => StateVarValue::Boolean(*sv_inner.get_value()),
//             EssentialStateVar::MathExpr(sv_inner) => StateVarValue::MathExpr(*sv_inner.get_value()),
//         }
//     }
// }

// Boilerplate to display EssentialStateVar and StateVar better

// impl fmt::Debug for StateVar {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.write_str(&format!("{:?}", &self.get_state()))
//     }
// }
