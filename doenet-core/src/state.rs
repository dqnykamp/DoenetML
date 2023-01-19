use enum_as_inner::EnumAsInner;
use serde::Serialize;

use crate::state_variables::*;
use std::{cell::RefCell, fmt, rc::Rc, ops::Deref};
use self::State::*;

#[derive(Clone)]
pub struct StateVar {

    // Why we need RefCells: the Box does not allow mutability in the thing it wraps.
    // If it any point we might want to mutate a field, its value should be wrapped in a RefCell.

    // This field should remain private
    value_type_protector: ValueTypeProtector,
}


/// This private enum does not change its variant once initialized,
/// which protects state variables from changing type.
/// We have to store the State enum *inside* each variant
/// so that the type is retained even when the content is Stale.
#[derive(Clone, Debug)]
enum ValueTypeProtector {
    String(StateVarInner<String>),
    Boolean(StateVarInner<bool>),
    Integer(StateVarInner<i64>),
    Number(StateVarInner<f64>),
}


pub struct StateVarInner<T> {
    val: Rc<RefCell<T>>,
    state: State
}

impl <T> StateVarInner<T> {
    // use get_read_only_view to main a reference to the value
    // that you can repeatedly access its value
    // but allow the value to be modified when not accessing the value
    fn get_read_only_view(&self) -> TypedReadOnlyView<T> {
        TypedReadOnlyView { val: self.val.clone() }
    }

    // use get_value to get a single reference to the value
    fn get_value<'a>(&'a self) -> impl Deref<Target = T> + 'a {
        self.val.borrow()
    }
}

pub struct TypedReadOnlyView<T> {
    // this must remain private to ensure read only
    val: Rc<RefCell<T>>,
}

impl <T> TypedReadOnlyView<T> {
    fn borrow<'a>(&'a self) -> impl Deref<Target = T> + 'a {
        self.val.borrow()
    }
}


#[derive(Debug, Clone, PartialEq, EnumAsInner)]
pub enum State {
    Fresh,
    Stale,
    Unresolved,
}

impl From<&StateVar> for serde_json::Value {
    fn from(state_var: &StateVar) -> serde_json::Value {
        match state_var.get_state() {
            State::Fresh => {
                let inner = state_var.get_inner();
                match inner {
                    ValueTypeProtector::Number(inner_sv) => serde_json::json!(inner_sv.get_value()),
                    ValueTypeProtector::Integer(inner_sv) => serde_json::json!(inner_sv.get_value()),
                    ValueTypeProtector::String(inner_sv) => serde_json::json!(inner_sv.get_value()),
                    ValueTypeProtector::Boolean(inner_sv) => serde_json::json!(inner_sv.get_value()),
                }
            },
            State::Stale => serde_json::Value::Null,
            State::Unresolved => serde_json::Value::Null,
        }
    }
}


impl StateVar {

    /// Create Unresolved StateVar of the given type
    pub fn new(value_type: &StateVarVariant) -> Self {

        match value_type {
            StateVarVariant::Boolean(_) => StateVar {
                value_type_protector: ValueTypeProtector::Boolean(StateVarInner { val: Rc::new(RefCell::new(false)), state: Unresolved })
            },
            StateVarVariant::Integer(_) => StateVar {
                value_type_protector: ValueTypeProtector::Integer(StateVarInner { val: Rc::new(RefCell::new(0)), state: Unresolved })
            },
            StateVarVariant::Number(_) =>  StateVar {
                value_type_protector: ValueTypeProtector::Number(StateVarInner { val: Rc::new(RefCell::new(f64::NAN)), state: Unresolved })
            },
            StateVarVariant::String(_) => StateVar {
                value_type_protector: ValueTypeProtector::String(StateVarInner { val: Rc::new(RefCell::new(String::from(""))), state: Unresolved })
            }
        }
    }

    pub fn get_inner(&self) -> &ValueTypeProtector {
        &self.value_type_protector
    }

    pub fn set_value(&self, new_value: StateVarValue) -> Result<StateVarValue, String> {

        self.value_type_protector.borrow_mut().set_value(new_value)
    }


    pub fn mark_stale(&self) {

        let type_protector = &mut *self.value_type_protector.borrow_mut();

        *type_protector = match type_protector {
            ValueTypeProtector::String(_)  => ValueTypeProtector::String(Stale),
            ValueTypeProtector::Boolean(_) => ValueTypeProtector::Boolean(Stale),
            ValueTypeProtector::Number(_)  => ValueTypeProtector::Number(Stale),
            ValueTypeProtector::Integer(_) => ValueTypeProtector::Integer(Stale),
        }
    }


    pub fn get_state(&self) -> State {

        let type_protector = &*self.value_type_protector.borrow();

        match type_protector {
            ValueTypeProtector::String(sv_inner) => sv_inner.state,
            ValueTypeProtector::Number(sv_inner) => sv_inner.state,
            ValueTypeProtector::Integer(sv_inner) => sv_inner.state,
            ValueTypeProtector::Boolean(sv_inner) => sv_inner.state,
        }
    }


}


impl ValueTypeProtector {

    fn set_value(&mut self, new_value: StateVarValue) -> Result<StateVarValue, String> {

        match self {
            ValueTypeProtector::String(state) => {                
                *state = Fresh(new_value.clone().try_into()?);
            },
            ValueTypeProtector::Integer(state) => {
                *state = Fresh(new_value.clone().try_into()?);
            },
            ValueTypeProtector::Number(state) => {
                *state = Fresh(new_value.clone().try_into()?);
            },
            ValueTypeProtector::Boolean(state) => {
                *state = Fresh(new_value.clone().try_into()?);
            }
        }

        Ok(new_value)
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

impl fmt::Debug for StateVar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("{:?}", &self.get_state()))
    }
}

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


