use enum_as_inner::EnumAsInner;
use serde::Serialize;

use crate::state_variables::*;
use std::{cell::RefCell, fmt, rc::Rc, ops::Deref};
// use self::Freshness::*;


/// This private enum does not change its variant once initialized,
/// which protects state variables from changing type.
/// We have to store the State enum *inside* each variant
/// so that the type is retained even when the content is Stale.
#[derive(Clone, Debug)]
pub enum StateVar {
    String(StateVarInner<String>),
    Boolean(StateVarInner<bool>),
    Integer(StateVarInner<i64>),
    Number(StateVarInner<f64>),
}



#[derive(Clone, Debug)]
pub struct StateVarInner<T> {
    value: Rc<RefCell<T>>,
    freshness: Freshness
}

impl <T> StateVarInner<T> {
    // use get_read_only_view to main a reference to the value
    // that you can repeatedly access its value
    // but allow the value to be modified when not accessing the value
    fn get_read_only_view(&self) -> TypedReadOnlyView<T> {
        TypedReadOnlyView { val: self.value.clone() }
    }

    // use get_value to get a single reference to the value
    fn get_value_assuming_fresh<'a>(&'a self) -> impl Deref<Target = T> + 'a {
        if self.freshness != Freshness::Fresh {
            panic!("State variable is not fressh, cannot get its fresh value");
        }
        self.value.borrow()
    }
}

pub struct TypedReadOnlyView<T> {
    // this must remain private to ensure read only
    val: Rc<RefCell<T>>,
}

impl <T> TypedReadOnlyView<T> {
    fn borrow<'a>(&'a self) -> impl Deref<Target = T> + 'a {
        // Note: this does not check to make sure that the value is fresh
        self.val.borrow()
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
            Freshness::Fresh => {
                match state_var {
                    StateVar::Number(inner_sv) => serde_json::json!(*inner_sv.get_value_assuming_fresh()),
                    StateVar::Integer(inner_sv) => serde_json::json!(*inner_sv.get_value_assuming_fresh()),
                    StateVar::String(inner_sv) => serde_json::json!(inner_sv.get_value_assuming_fresh().clone()),
                    StateVar::Boolean(inner_sv) => serde_json::json!(*inner_sv.get_value_assuming_fresh()),
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
            StateVarVariant::Number(_) => StateVar::Number(
                StateVarInner { value: Rc::new(RefCell::new(f64::NAN)), freshness: Freshness::Unresolved }
            ),
            StateVarVariant::Integer(_) => StateVar::Integer(
                StateVarInner { value: Rc::new(RefCell::new(0)), freshness: Freshness::Unresolved }
            ),
            StateVarVariant::Boolean(_) => StateVar::Boolean(
                StateVarInner { value: Rc::new(RefCell::new(false)), freshness: Freshness::Unresolved }
            ),
            StateVarVariant::String(_) => StateVar::String(
                StateVarInner { value: Rc::new(RefCell::new(String::from(""))), freshness: Freshness::Unresolved }
            ),
        }
    }

    pub fn set_value(&mut self, new_value: StateVarValue) -> Result<StateVarValue, String> {

        match self {
            StateVar::String(sv_inner) => {    
                *sv_inner.value.borrow_mut() = new_value.clone().try_into()?;
                sv_inner.freshness = Freshness::Fresh;            
            },
            StateVar::Integer(sv_inner) => {
                *sv_inner.value.borrow_mut() = new_value.clone().try_into()?;
                sv_inner.freshness = Freshness::Fresh;            
            },
            StateVar::Number(sv_inner) => {
                *sv_inner.value.borrow_mut() = new_value.clone().try_into()?;
                sv_inner.freshness = Freshness::Fresh;            
            },
            StateVar::Boolean(sv_inner) => {
                *sv_inner.value.borrow_mut() = new_value.clone().try_into()?;
                sv_inner.freshness = Freshness::Fresh;            
            }
        }

        Ok(new_value)
    }

    pub fn mark_stale(&mut self) {

        match self {
            StateVar::String(sv_inner)  => sv_inner.freshness = Freshness::Stale,
            StateVar::Boolean(sv_inner) => sv_inner.freshness = Freshness::Stale,
            StateVar::Number(sv_inner)  => sv_inner.freshness = Freshness::Stale,
            StateVar::Integer(sv_inner) => sv_inner.freshness = Freshness::Stale,
        };
    }


    pub fn get_freshness(&self) -> Freshness {

        match self {
            StateVar::String(sv_inner) => sv_inner.freshness,
            StateVar::Number(sv_inner) => sv_inner.freshness,
            StateVar::Integer(sv_inner) => sv_inner.freshness,
            StateVar::Boolean(sv_inner) => sv_inner.freshness,
        }
    }

    pub fn get_value_assuming_fresh(&self) -> StateVarValue {
        match self {
            StateVar::Boolean(sv_bool) => StateVarValue::Boolean(*sv_bool.get_value_assuming_fresh()),
            StateVar::Number(sv_number) => StateVarValue::Number(*sv_number.get_value_assuming_fresh()),
            StateVar::Integer(sv_int) => StateVarValue::Integer(*sv_int.get_value_assuming_fresh()),
            StateVar::String(sv_string) => StateVarValue::String(sv_string.get_value_assuming_fresh().clone()),
        }

    }

    

}








/// A special endpoint on the dependency graph which is associated with a
/// particular state var. Actions often update these.
/// An EssentialStateVar cannot be stale so it does not need a ValueTypeProtector
#[derive(Clone, Debug, Serialize)]
pub struct EssentialStateVar (pub StateVarValue);

impl EssentialStateVar {
    pub fn set_value(&mut self, new_value: StateVarValue) -> Result<(), String> {
        self.0.set_protect_type(new_value)
    }
}


// Boilerplate to display EssentialStateVar and StateVar better

// impl fmt::Debug for StateVar {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.write_str(&format!("{:?}", &self.get_state()))
//     }
// }

impl StateVarValue {
    fn set_protect_type(&mut self,  new_value: StateVarValue) -> Result<(), String> {
        match self {
            StateVarValue::String(state) => {
                *state = new_value.try_into()?;
            },
            StateVarValue::Integer(state) => {
                *state = new_value.try_into()?;
            },
            StateVarValue::Number(state) => {
                *state = new_value.try_into()?;
            },
            StateVarValue::Boolean(state) => {
                *state = new_value.try_into()?;
            },
            StateVarValue::MathExpr(state) => {
                *state = new_value.try_into()?;
            }
        }

        Ok(())
    }
}


