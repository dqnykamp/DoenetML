use crate::component::AttributeName;
use crate::component::ComponentProfile;
use crate::component::ComponentType;
use crate::ComponentName;

use crate::math_expression::MathExpression;
use crate::state::StateVarReadOnlyView;
use crate::utils::log;

/// The name (camelCase) of a state variable that could be
/// a basic or an array depending on the component.
pub type StateVarName = &'static str;

/// This can contain the value of a state variable of any type,
/// which is useful for function parameters.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
#[serde(untagged)]
pub enum StateVarValue {
    String(String),
    Number(f64),
    Integer(i64),
    Boolean(bool),
    MathExpr(MathExpression),
}

/// A DependencyInstruction is used to make a Dependency when core is created,
/// which holds the specific information.
#[derive(Clone, Debug)]
pub enum DependencyInstruction {
    Child {
        /// The dependency will only match child components that fulfill at least one of these profiles
        desired_profiles: Vec<ComponentProfile>,

        /// Whether or not to parse children into an expression on core creation, store that expression
        /// in essential data, and give that expression as one of the dependency values for this
        /// dependency instruction
        parse_into_expression: bool,
    },
    StateVar {
        component_name: Option<ComponentName>,
        state_var_name: StateVarName,
    },
    Parent {
        state_var_name: StateVarName,
    },
    Attribute {
        attribute_name: AttributeName,
        default_value: StateVarValue
    },
    Essential {
        /// Use the string of this attribute
        prefill: Option<AttributeName>,
    },
}

#[derive(Debug)]
pub enum StateVarUpdateInstruction<T> {
    SetValue(T),
    NoChange,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DependencySource {
    StateVar {
        component_type: ComponentType,
        state_var_ind: usize,
    },
    Essential {
        value_type: &'static str,
    },
}

/// Passed into determine_state_vars_from_dependencies
/// TODO: This struct doesn't quite fit the result of an EssentialDependencyInstruction.
#[derive(Debug)]
pub struct DependencyValue {
    pub source: DependencySource,
    pub value: StateVarReadOnlyView,
}

// pub enum StateVarReadOnlyOrEssential {
//     StateVar(StateVarReadOnlyView),
//     Essential(StateVarMutableView)
// }

// /////////// StateVarValue boilerplate ///////////

impl TryFrom<StateVarValue> for String {
    type Error = &'static str;
    fn try_from(v: StateVarValue) -> Result<Self, Self::Error> {
        match v {
            StateVarValue::String(x) => Ok(x.to_string()),
            StateVarValue::Number(_) => Err("cannot convert StateVarValue::Number to string"),
            StateVarValue::Integer(_) => Err("cannot convert StateVarValue::Integer to string"),
            StateVarValue::Boolean(_) => Err("cannot convert StateVarValue::Boolean to string"),
            StateVarValue::MathExpr(_) => Err("cannot convert StateVarValue::MathExpr to string"),
        }
    }
}
impl TryFrom<StateVarValue> for bool {
    type Error = &'static str;
    fn try_from(v: StateVarValue) -> Result<Self, Self::Error> {
        match v {
            StateVarValue::Boolean(x) => Ok(x),
            StateVarValue::Number(_) => Err("cannot convert StateVarValue::Number to boolean"),
            StateVarValue::Integer(_) => Err("cannot convert StateVarValue::Integer to boolean"),
            StateVarValue::String(_) => Err("cannot convert StateVarValue::String to boolean"),
            StateVarValue::MathExpr(_) => Err("cannot convert StateVarValue::MathExpr to boolean"),
        }
    }
}
impl TryFrom<StateVarValue> for f64 {
    type Error = &'static str;
    fn try_from(v: StateVarValue) -> Result<Self, Self::Error> {
        match v {
            StateVarValue::Number(x) => Ok(x),
            StateVarValue::Integer(x) => Ok(x as f64),
            StateVarValue::String(_) => Err("cannot convert StateVarValue::String to number"),
            StateVarValue::Boolean(_) => Err("cannot convert StateVarValue::Boolean to number"),
            StateVarValue::MathExpr(_) => Err("cannot convert StateVarValue::MathExpr to number"),
        }
    }
}
impl TryFrom<StateVarValue> for i64 {
    type Error = &'static str;
    fn try_from(v: StateVarValue) -> Result<Self, Self::Error> {
        match v {
            StateVarValue::Integer(x) => Ok(x),
            StateVarValue::Number(_) => Err("cannot convert StateVarValue::Number to integer"),
            StateVarValue::String(_) => Err("cannot convert StateVarValue::String to integer"),
            StateVarValue::Boolean(_) => Err("cannot convert StateVarValue::Boolean to integer"),
            StateVarValue::MathExpr(_) => Err("cannot convert StateVarValue::MathExpr to integer"),
        }
    }
}

impl TryFrom<StateVarValue> for MathExpression {
    type Error = &'static str;
    fn try_from(v: StateVarValue) -> Result<Self, Self::Error> {
        match v {
            StateVarValue::MathExpr(x) => Ok(x),
            StateVarValue::Integer(_) => Err("cannot convert StateVarValue::Integer to MathExpr"),
            StateVarValue::Number(_) => Err("cannot convert StateVarValue::Number to MathExpr"),
            StateVarValue::String(_) => Err("cannot convert StateVarValue::String to MathExpr"),
            StateVarValue::Boolean(_) => Err("cannot convert StateVarValue::Boolean to MathExpr"),
        }
    }
}

impl From<StateVarValue> for serde_json::Value {
    fn from(v: StateVarValue) -> serde_json::Value {
        match v {
            StateVarValue::Integer(v) => serde_json::json!(v),
            StateVarValue::Number(v) => serde_json::json!(v),
            StateVarValue::String(v) => serde_json::json!(v),
            StateVarValue::Boolean(v) => serde_json::json!(v),
            StateVarValue::MathExpr(v) => serde_json::json!(v),
        }
    }
}
impl TryFrom<StateVarValue> for usize {
    type Error = &'static str;
    fn try_from(v: StateVarValue) -> Result<Self, Self::Error> {
        match v {
            StateVarValue::Integer(x) => match x >= 0 {
                true => Ok(x as usize),
                false => Err("cannot convert negative int to usize"),
            },
            StateVarValue::Number(_) => Err("cannot convert StateVarValue::Number to usize"),
            StateVarValue::String(_) => Err("cannot convert StateVarValue::String to usize"),
            StateVarValue::Boolean(_) => Err("cannot convert StateVarValue::Boolean to usize"),
            StateVarValue::MathExpr(_) => Err("cannot convert StateVarValue::MathExpr to usize"),
        }
    }
}

impl From<String> for StateVarValue {
    fn from(v: String) -> StateVarValue {
        StateVarValue::String(v)
    }
}
impl From<bool> for StateVarValue {
    fn from(v: bool) -> StateVarValue {
        StateVarValue::Boolean(v)
    }
}
impl From<f64> for StateVarValue {
    fn from(v: f64) -> StateVarValue {
        StateVarValue::Number(v)
    }
}
impl From<i64> for StateVarValue {
    fn from(v: i64) -> StateVarValue {
        StateVarValue::Integer(v)
    }
}

impl StateVarValue {
    /// Not necessarily a component type
    pub fn type_as_str(&self) -> &'static str {
        match self {
            Self::String(_) => "string",
            Self::Boolean(_) => "boolean",
            Self::Integer(_) => "integer",
            Self::Number(_) => "number",
            Self::MathExpr(_) => "mathExpression",
        }
    }

    pub fn into_number(self) -> Result<StateVarValue, String> {
        match self {
            StateVarValue::Integer(i) => Ok(StateVarValue::Number(i as f64)),
            StateVarValue::Number(n) => Ok(StateVarValue::Number(n)),
            _ => Err("cannot convert value into number".to_string()),
        }
    }
}

impl std::fmt::Display for StateVarValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
