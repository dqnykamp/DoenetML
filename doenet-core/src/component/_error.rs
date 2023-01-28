use super::*;
use crate::state::{
    StateVarInterface, StateVarMutableViewTyped, StateVarParameters, StateVarReadOnlyView,
    StateVarReadOnlyViewTyped, StateVarTyped, UpdatesRequested,
};
use evalexpr::{ContextWithMutableVariables, HashMapContext, Operator};

use crate::math_expression::MathExpression;
use crate::ComponentProfile;

use lazy_static::lazy_static;

use crate::base_definitions::*;


integer_state_variable_from_attribute!("start_index", StartIndex);
integer_state_variable_from_attribute!("end_index", EndIndex);


#[derive(Debug)]
struct Message {
    string_child_value: StateVarReadOnlyViewTyped<String>,
}

impl Message {
    pub fn new() -> Self {
        Message {
            string_child_value: StateVarReadOnlyViewTyped::new(),
        }
    }
}

impl StateVarInterface<String> for Message {
    fn return_dependency_instructions(&self) -> Vec<DependencyInstruction> {
        vec![DependencyInstruction::Child {
            desired_profiles: vec![ComponentProfile::Text],
            parse_into_expression: false,
        }]
    }

    fn set_dependencies(&mut self, dependencies: &Vec<Vec<DependencyValue>>) -> () {

        if let DependencyValue{ value: StateVarReadOnlyView::String(child_string_value), ..} = &dependencies[0][0] {
            self.string_child_value  = child_string_value.create_new_read_only_view();
        } else {
            panic!("Got a non-string message for error");
        }

    }

    fn calculate_state_var_from_dependencies(
        &self,
        state_var: &StateVarMutableViewTyped<String>,
    ) -> () {

        state_var.set_value(self.string_child_value.get_value_assuming_fresh().clone());
    }
}


lazy_static! {

    pub static ref GENERATE_STATE_VARS: fn() -> Vec<StateVar> = || {
        vec![
            StateVar::Integer(StateVarTyped::new(
                Box::new(StartIndex::new()),
                StateVarParameters {
                    name: "startIndex",
                    for_renderer: true,
                    initial_essential_value: 0,
                    ..Default::default()
                },
            )),
            StateVar::Integer(StateVarTyped::new(
                Box::new(EndIndex::new()),
                StateVarParameters {
                    name: "endIndex",
                    for_renderer: true,
                    initial_essential_value: 0,
                    ..Default::default()
                },
            )),
            StateVar::String(StateVarTyped::new(
                Box::new(Message::new()),
                StateVarParameters {
                    name: "message",
                    for_renderer: true,
                    ..Default::default()
                },
            )),
        ]
    };
    pub static ref STATE_VARIABLES_NAMES_IN_ORDER: Vec<&'static str> = GENERATE_STATE_VARS()
        .iter()
        .map(|sv| sv.get_name())
        .collect();



    pub static ref MY_COMPONENT_DEFINITION: ComponentDefinition = ComponentDefinition {
        component_type: "_error",

        state_var_index_map: STATE_VARIABLES_NAMES_IN_ORDER
            .iter()
            .enumerate()
            .map(|(i, v)| (*v, i))
            .collect(),

        state_var_names: STATE_VARIABLES_NAMES_IN_ORDER.to_vec(),

        state_var_component_types: GENERATE_STATE_VARS()
            .iter()
            .map(|sv| sv.get_default_component_type())
            .collect(),

        generate_state_vars: *GENERATE_STATE_VARS,

        attribute_names: vec!["start_index", "end_index",],
        ..Default::default()
    };
}
