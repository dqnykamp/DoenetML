use super::*;
// use crate::base_definitions::*;

use crate::state::{
    StateVarInterface, StateVarMutableViewTyped, StateVarParameters, StateVarReadOnlyView,
    StateVarReadOnlyViewTyped, StateVarTyped, UpdatesRequested,
};
// use crate::state_variables::*;

use crate::ComponentProfile;

use lazy_static::lazy_static;

#[derive(Debug)]
struct Value {
    string_child_values: Vec<StateVarReadOnlyViewTyped<String>>,
}

impl Value {
    pub fn new() -> Self {
        Value {
            string_child_values: Vec::new(),
        }
    }
}

impl StateVarInterface<String> for Value {
    fn return_dependency_instructions(&self) -> Vec<DependencyInstruction> {
        vec![DependencyInstruction::Child {
            desired_profiles: vec![ComponentProfile::Text],
            parse_into_expression: false,
        }]
    }

    fn set_dependencies(&mut self, dependencies: &Vec<Vec<DependencyValue>>) -> () {
        let children = &dependencies[0];

        let mut string_vals = Vec::with_capacity(children.len());

        for DependencyValue {
            value: child_value, ..
        } in children.iter()
        {
            if let StateVarReadOnlyView::String(child_string_value) = child_value {
                string_vals.push(child_string_value.create_new_read_only_view())
            } else {
                panic!("Got a non-string value when asked for a Text component profile");
            }
        }

        self.string_child_values = string_vals;
    }

    fn calculate_state_var_from_dependencies(
        &self,
        state_var: &StateVarMutableViewTyped<String>,
    ) -> () {
        // TODO: can we implement this without cloning the inner value?
        let value: String = self
            .string_child_values
            .iter()
            .map(|v| v.get_value_assuming_fresh().clone())
            .collect();

        state_var.set_value(value);
    }

    fn request_dependencies_to_update_value(
        &self,
        state_var: &StateVarReadOnlyViewTyped<String>,
        _is_initial_change: bool,
    ) -> Result<Vec<UpdatesRequested>, ()> {
        if self.string_child_values.len() != 1 {
            // TODO: implement for no children where saves to essential value
            Err(())
        } else {
            let desired_value = state_var.get_requested_value();

            self.string_child_values[0].request_value(desired_value.clone());

            Ok(vec![UpdatesRequested {
                instruction_ind: 0,
                dependency_ind: 0,
            }])
        }
    }
}

#[derive(Debug)]
struct Text {
    value_sv: StateVarReadOnlyViewTyped<String>,
}

impl Text {
    pub fn new() -> Self {
        Text {
            value_sv: StateVarReadOnlyViewTyped::new(),
        }
    }
}

impl StateVarInterface<String> for Text {
    fn return_dependency_instructions(&self) -> Vec<DependencyInstruction> {
        vec![DependencyInstruction::StateVar {
            component_name: None,
            state_var_name: "value",
        }]
    }

    fn set_dependencies(&mut self, dependencies: &Vec<Vec<DependencyValue>>) -> () {
        let dep_val = &dependencies[0][0].value;

        if let StateVarReadOnlyView::String(string_val) = dep_val {
            self.value_sv = string_val.create_new_read_only_view();
        } else {
            panic!("Something went wrong with text sv of text");
        }
    }

    fn calculate_state_var_from_dependencies(
        &self,
        state_var: &StateVarMutableViewTyped<String>,
    ) -> () {
        state_var.set_value(self.value_sv.get_value_assuming_fresh().clone());
    }
}

#[derive(Debug)]
struct Hidden {}

impl Hidden {
    pub fn new() -> Self {
        Hidden {}
    }
}

impl StateVarInterface<bool> for Hidden {
    fn calculate_state_var_from_dependencies(
        &self,
        state_var: &StateVarMutableViewTyped<bool>,
    ) -> () {
        state_var.set_value(false);
    }
}

#[derive(Debug)]
struct Disabled {}

impl Disabled {
    pub fn new() -> Self {
        Disabled {}
    }
}

impl StateVarInterface<bool> for Disabled {
    fn calculate_state_var_from_dependencies(
        &self,
        state_var: &StateVarMutableViewTyped<bool>,
    ) -> () {
        state_var.set_value(false);
    }
}

#[derive(Debug)]
struct Fixed {}

impl Fixed {
    pub fn new() -> Self {
        Fixed {}
    }
}

impl StateVarInterface<bool> for Fixed {
    fn calculate_state_var_from_dependencies(
        &self,
        state_var: &StateVarMutableViewTyped<bool>,
    ) -> () {
        state_var.set_value(false);
    }
}

lazy_static! {
    pub static ref GENERATE_STATE_VARS: fn() -> Vec<StateVar> = || {
        vec![
            StateVar::String(StateVarTyped::new(
                Box::new(Value::new()),
                StateVarParameters {
                    name: "value",
                    ..Default::default()
                },
            )),
            StateVar::String(StateVarTyped::new(
                Box::new(Text::new()),
                StateVarParameters {
                    name: "text",
                    for_renderer: true,
                    ..Default::default()
                },
            )),
            StateVar::Boolean(StateVarTyped::new(
                Box::new(Hidden::new()),
                StateVarParameters {
                    name: "hidden",
                    for_renderer: true,
                    ..Default::default()
                },
            )),
            StateVar::Boolean(StateVarTyped::new(
                Box::new(Disabled::new()),
                StateVarParameters {
                    name: "disabled",
                    for_renderer: true,
                    ..Default::default()
                },
            )),
            StateVar::Boolean(StateVarTyped::new(
                Box::new(Fixed::new()),
                StateVarParameters {
                    name: "fixed",
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
    pub static ref SV_MAP: HashMap<&'static str, usize> = STATE_VARIABLES_NAMES_IN_ORDER
        .iter()
        .enumerate()
        .map(|(i, v)| (*v, i))
        .collect();
    pub static ref MY_COMPONENT_DEFINITION: ComponentDefinition = ComponentDefinition {
        component_type: "text",

        state_var_index_map: SV_MAP.clone(),

        state_var_names: STATE_VARIABLES_NAMES_IN_ORDER.to_vec(),

        state_var_component_types: GENERATE_STATE_VARS()
            .iter()
            .map(|sv| sv.get_default_component_type())
            .collect(),

        generate_state_vars: *GENERATE_STATE_VARS,

        attribute_names: vec!["hide", "disabled", "fixed",],

        primary_input_state_var_ind: Some(0),

        component_profiles: vec![(ComponentProfile::Text, "value")],

        valid_children_profiles: ValidChildTypes::ValidProfiles(vec![ComponentProfile::Text]),

        ..Default::default()
    };
}
