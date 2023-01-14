use std::collections::HashMap;

use crate::ComponentName;
use crate::component::AttributeName;
use crate::component::ComponentProfile;
use crate::component::ComponentType;

use crate::math_expression::MathExpression;
use crate::utils::log;


/// The name (camelCase) of a state variable that could be
/// a basic or an array depending on the component.
pub type StateVarName = &'static str;

/// camelCase
pub type InstructionName = &'static str;



/// State variable functions core uses.
/// The generics force component code to be consistent with the type of a state variable.
// #[derive(Debug)]
pub struct StateVarDefinition<T> {

    /// Some state variable's dependencies change based on other variables.
    // pub state_vars_to_determine_dependencies: fn() -> Vec<StateVarName>,

    /// The instructions that core can use to make Dependency structs.
    pub dependency_instructions: Vec<DependencyInstruction>,

    /// Determine the value and return that to core as an update instruction.
    pub determine_state_var_from_dependencies: fn(
        &Vec<Vec<DependencyValue>>
    ) -> Result<StateVarUpdateInstruction<T>, String>,

    pub for_renderer: bool,

    /// Determines whether to use essential data
    pub initial_essential_value: T,

    /// The inverse of `return_dependency_instructions`: For a desired value, return dependency
    /// values for the dependencies that would make this state variable return that value.
    pub request_dependencies_to_update_value: fn(
        T,
        Vec<Vec<(DependencySource, Option<StateVarValue>)>>
    ) -> Vec<(usize, Result<Vec<DependencyValue>, String>)>,
}



impl<T> Default for StateVarDefinition<T>
    where T: Default
{
    fn default() -> Self {
        StateVarDefinition {
            dependency_instructions: Vec::new(),
            determine_state_var_from_dependencies:
                |_| Ok(StateVarUpdateInstruction::SetValue(T::default())),
            for_renderer: false,
            initial_essential_value: T::default(),

            request_dependencies_to_update_value: |_, _| {
                log!("DEFAULT REQUEST_DEPENDENCIES_TO_UPDATE_VALUE DOES NOTHING");
                Vec::new()
            },
        }
    }
}




/// Since `StateVarDefinition` is generic, this enum is needed to store one in a HashMap or Vec.
// #[derive(Debug)]
pub enum StateVarVariant {
    String(StateVarDefinition<String>),
    Boolean(StateVarDefinition<bool>),
    Number(StateVarDefinition<f64>),
    Integer(StateVarDefinition<i64>),
}


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
#[derive(Debug, Clone)]
pub struct DependencyValue {
    pub source: DependencySource,
    pub value: StateVarValue,
}






/////////// DependencyValue boilerplate ///////////
// Note that these functions aren't cost free. They do allocate vectors, which you wouldn't have to do if 
// you were unwrapping manually

pub trait DepValueHashMap {
    fn dep_value(&self, instruction_name: InstructionName) -> Result<(Vec<&DependencyValue>, InstructionName), String>;
}

impl DepValueHashMap for HashMap<InstructionName, Vec<DependencyValue>> {

    fn dep_value(&self, instruction_name: InstructionName) -> Result<(Vec<&DependencyValue>, InstructionName), String> {
        if let Some(values) = self.get(instruction_name) {

            let values_vec = values.iter().collect();
            Ok((values_vec, instruction_name))
        } else {
            Err(format!("Instruction [{}] does not exist", instruction_name))
        }
    }
}


pub trait DepValueVec {
    fn has_zero_or_one_elements(&self) -> Result<(Option<&DependencyValue>, InstructionName), String>;
    fn has_exactly_one_element(&self) -> Result<(&DependencyValue, InstructionName), String>;

    fn into_string_list(&self) -> Result<Vec<String>, String>;

    fn into_number_list(&self) -> Result<Vec<f64>, String>;

    fn filter_include_component_type(&self, component_type: &ComponentType) -> (Vec<&DependencyValue>, InstructionName);
}

impl DepValueVec for (Vec<&DependencyValue>, InstructionName) {

   fn has_zero_or_one_elements(&self) -> Result<(Option<&DependencyValue>, InstructionName), String> {
        let (dep_values, name) = self;
        match dep_values.len() {
            0 => Ok((None, name)),
            1 => Ok((Some(&dep_values[0]), name)),
            _ => Err(format!("Expected instruction [{}] to have zero or one elements", name))
        }
    }

    fn has_exactly_one_element(&self) -> Result<(&DependencyValue, InstructionName), String> {
        let (dep_values, name) = self;

        if dep_values.len() == 1 {
            Ok((&dep_values[0], name))
        } else {
            Err(format!("Expected instruction [{}] to have exactly one element", name))
        }
    }

    fn into_string_list(&self) -> Result<Vec<String>, String> {
        let (dep_values, name) = self;

        dep_values.iter().map(|dep_value|
            dep_value.value.clone().try_into().map_err(|_|
                format!("Not all elements in instruction [{}] were strings", name)
            )
        ).collect()
    }


    fn into_number_list(&self) -> Result<Vec<f64>, String> {
        let (dep_values, name) = self;

        dep_values.iter().map(|dep_value|
            dep_value.value.clone().try_into().map_err(|_|
                format!("Not all elements in instruction [{}] were strings", name)
            )
        ).collect()
    }



    fn filter_include_component_type(&self, component_type: &ComponentType) -> (Vec<&DependencyValue>, InstructionName) {
        let (dep_values, name) = self;

        let filtered_dep_values = dep_values.iter()
            .filter(|dep_value| match &dep_value.source {
                DependencySource::StateVar { component_type: comp, ..} => comp == component_type,
                _ => false,
            })
            .map(|&dep_value| dep_value)
            .collect();

        (filtered_dep_values, name)
    }
}


pub trait DepValueSingle {
    fn into_bool(&self) -> Result<bool, String>;
    fn into_string(&self) -> Result<String, String>;
    fn into_number(&self) -> Result<f64, String>;
    fn into_integer(&self) -> Result<i64, String>;
    fn into_math_expression(&self) -> Result<MathExpression, String>;
    fn value(&self) -> StateVarValue;
}

impl DepValueSingle for DependencyValue {
    fn into_bool(&self) -> Result<bool, String> {
        self.value.clone().try_into().map_err(|_|
            format!("Instruction is a {}, expected a bool", self.value.type_as_str()))
    }

    fn into_string(&self) -> Result<String, String> {
        self.value.clone().try_into().map_err(|_|
            format!("Instruction is a {}, expected a string", self.value.type_as_str()))
    }

    fn into_number(&self) -> Result<f64, String> {
        self.value.clone().try_into().map_err(|_|
            format!("Instruction is a {}, expected a number", self.value.type_as_str()))
    }

    fn into_integer(&self) -> Result<i64, String> {
        self.value.clone().try_into().map_err(|_|
            format!("Instruction is a {}, expected an integer", self.value.type_as_str()))
    }

    fn into_math_expression(&self) -> Result<MathExpression, String> {
        self.value.clone().try_into().map_err(|_|
            format!("Instruction is a {}, expected a math expression", self.value.type_as_str()))
    }

    fn value(&self) -> StateVarValue {
        self.value.clone()
    }
}



pub trait DepValueOption {
    fn is_bool_if_exists(&self) -> Result<Option<bool>, String>;
    fn into_if_exists<T: TryFrom<StateVarValue>>(&self) -> Result<Option<T>, String>;
    fn value(&self) -> Option<StateVarValue>;
}

impl DepValueOption for Option<&DependencyValue> {

    fn is_bool_if_exists(&self) -> Result<Option<bool>, String> {
        self.into_if_exists().map_err(|e| e + ", expected a bool")
    }

    fn into_if_exists<T: TryFrom<StateVarValue>>(&self) -> Result<Option<T>, String> {
        let dep_value_opt = self;

        dep_value_opt.and_then(|dep_value| Some(dep_value.value.clone().try_into().map_err(|_|
                format!("could not convert value {} from instruction",
                    dep_value.value.type_as_str())
            )))
            .map_or(Ok(None), |v| v.map(Some)) // flip nested Option<Result<T>>
    }

    fn value(&self) -> Option<StateVarValue> {
        match self {
            Some(dep_value) => Some(dep_value.value.clone()),
            None => None,
        }
    }
}


/////////// StateVarValue boilerplate ///////////

impl TryFrom<StateVarValue> for String {
    type Error = &'static str;
    fn try_from(v: StateVarValue) -> Result<Self, Self::Error> {
        match v {
            StateVarValue::String(x) => Ok( x.to_string() ),
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
            StateVarValue::Boolean(x) => Ok( x ),
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
            StateVarValue::Number(x) => Ok( x ),
            StateVarValue::Integer(x) => Ok( x as f64 ),
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
            StateVarValue::Integer(x) => Ok( x ),
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
            StateVarValue::MathExpr(x) => Ok ( x ),
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
            StateVarValue::Number(v) =>  serde_json::json!(v),
            StateVarValue::String(v) =>  serde_json::json!(v),
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








// Boilerplate matching over StateVarVariant

impl StateVarVariant {

    pub fn return_dependency_instructions(&self)
         -> &Vec<DependencyInstruction> {

        match self {
            Self::String(def) => &def.dependency_instructions,
            Self::Boolean(def) => &def.dependency_instructions,
            Self::Number(def) => &def.dependency_instructions,
            Self::Integer(def) => &def.dependency_instructions,
        }
    }

    
    pub fn determine_state_var_from_dependencies(&self,
        dependency_values: &Vec<Vec<DependencyValue>>
    ) -> Result<StateVarUpdateInstruction<StateVarValue>, String> {

        use StateVarUpdateInstruction::*;

        match self {
            Self::String(def) => {
                let instruction = (def.determine_state_var_from_dependencies)(dependency_values)?;
                Ok(match instruction {                    
                    NoChange => NoChange,
                    SetValue(val) => SetValue(StateVarValue::String(val)),
                })
            },
            Self::Integer(def) => {
                let instruction = (def.determine_state_var_from_dependencies)(dependency_values)?;
                Ok(match instruction {
                    NoChange => NoChange,
                    SetValue(val) => SetValue(StateVarValue::Integer(val)),
                })
            },
            Self::Number(def) => {
                let instruction = (def.determine_state_var_from_dependencies)(dependency_values)?;
                Ok(match instruction {
                    NoChange => NoChange,
                    SetValue(val) => SetValue(StateVarValue::Number(val)),
                })
            },
            Self::Boolean(def) => {
                let instruction = (def.determine_state_var_from_dependencies)(dependency_values)?;
                Ok(match instruction {
                    NoChange => NoChange,
                    SetValue(val) => SetValue(StateVarValue::Boolean(val)),
                })
            },
        }
    }

    pub fn request_dependencies_to_update_value(
        &self,
        desired_value: StateVarValue,
        dependency_sources: Vec<Vec<(DependencySource, Option<StateVarValue>)>>
    ) -> Result<Vec<(usize, Result<Vec<DependencyValue>, String>)>, String> {

        match self {
            Self::String(def) =>  {
                Ok((def.request_dependencies_to_update_value)(
                    desired_value.clone().try_into().map_err(|_| // only cloned for error msg
                        format!("Requested String be updated to {:#?}", desired_value)
                    )?,
                    dependency_sources,
                ))
            },
            Self::Integer(def) => {
                Ok((def.request_dependencies_to_update_value)(
                    desired_value.clone().try_into().map_err(|_| // only cloned for error msg
                        format!("Requested Integer be updated to {:#?}", desired_value)
                    )?,
                    dependency_sources,
                ))
            },
            Self::Number(def) =>  {
                Ok((def.request_dependencies_to_update_value)(
                    desired_value.clone().try_into().map_err(|_| // only cloned for error msg
                        format!("Requested Number be updated to {:#?}", desired_value)
                    )?,
                    dependency_sources,
                ))
            },
            Self::Boolean(def) => {
                Ok((def.request_dependencies_to_update_value)(
                    desired_value.clone().try_into().map_err(|_| // only cloned for error msg
                        format!("Requested Boolean be updated to {:#?}", desired_value)
                    )?,
                    dependency_sources,
                ))
            },


        }       
    }



    // Both array and non-array functions



    pub fn initial_essential_value(&self) -> StateVarValue {
        match self {
            Self::String(def) =>  StateVarValue::String(def.initial_essential_value.clone()),
            Self::Integer(def) => StateVarValue::Integer(def.initial_essential_value),
            Self::Number(def) =>  StateVarValue::Number(def.initial_essential_value),
            Self::Boolean(def) => StateVarValue::Boolean(def.initial_essential_value),
        }
    }


    pub fn for_renderer(&self) -> bool {
        match self {
            Self::String(def) =>  def.for_renderer,
            Self::Integer(def) => def.for_renderer,
            Self::Number(def) =>  def.for_renderer,
            Self::Boolean(def) => def.for_renderer,
        }
    }



}




