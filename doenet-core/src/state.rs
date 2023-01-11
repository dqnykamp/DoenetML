use enum_as_inner::EnumAsInner;
use serde::Serialize;

use crate::state_variables::*;
use std::{cell::RefCell, fmt};
use self::State::*;

#[derive(Clone)]
pub struct StateVar {

    // Why we need RefCells: the Box does not allow mutability in the thing it wraps.
    // If it any point we might want to mutate a field, its value should be wrapped in a RefCell.

    // This field should remain private
    value_type_protector: RefCell<ValueTypeProtector>,
}


/// This private enum does not change its variant once initialized,
/// which protects state variables from changing type.
/// We have to store the State enum *inside* each variant
/// so that the type is retained even when the content is Stale.
#[derive(Clone, Debug)]
enum ValueTypeProtector {
    String(State<String>),
    Boolean(State<bool>),
    Integer(State<i64>),
    Number(State<f64>),
}


#[derive(Debug, Clone, PartialEq, EnumAsInner)]
pub enum State<T> {
    Fresh(T),
    Stale,
    Unresolved,
}

impl From<&StateVar> for serde_json::Value {
    fn from(state: &StateVar) -> serde_json::Value {
        match state.get_state() {
            State::Fresh(value) => value.into(),
            State::Stale => serde_json::Value::Null,
            State::Unresolved => serde_json::Value::Null,
        }
    }
}


impl StateVar {

    /// Stale StateVar of the given type
    pub fn new(value_type: &StateVarVariant) -> Self {

        match value_type {
            StateVarVariant::Boolean(_) => StateVar {
                value_type_protector: RefCell::new(ValueTypeProtector::Boolean(Stale))
            },
            StateVarVariant::Integer(_) => StateVar {
                value_type_protector: RefCell::new(ValueTypeProtector::Integer(Stale))
            },
            StateVarVariant::Number(_) =>  StateVar {
                value_type_protector: RefCell::new(ValueTypeProtector::Number(Stale))
            },
            StateVarVariant::String(_) => StateVar {
                value_type_protector: RefCell::new(ValueTypeProtector::String(Stale))
            }
        }
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


    pub fn get_state(&self) -> State<StateVarValue> {

        let type_protector = &*self.value_type_protector.borrow();

        match type_protector {
            ValueTypeProtector::String(value_option) => match value_option {
                Fresh(val) => Fresh(StateVarValue::String(val.clone())),
                Stale => Stale,
                Unresolved => Unresolved
            },
            ValueTypeProtector::Number(value_option) => match value_option {
                Fresh(val) => Fresh(StateVarValue::Number(val.clone())),
                Stale => Stale,
                Unresolved => Unresolved
            },
            ValueTypeProtector::Boolean(value_option) => match value_option {
                Fresh(val) => Fresh(StateVarValue::Boolean(val.clone())),
                Stale => Stale,
                Unresolved => Unresolved
            },
            ValueTypeProtector::Integer(value_option) => match value_option {
                Fresh(val) => Fresh(StateVarValue::Integer(val.clone())),
                Stale => Stale,
                Unresolved => Unresolved
            }
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

