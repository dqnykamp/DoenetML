use super::*;

use crate::state::{
    StateVarInterface, StateVarMutableViewTyped, StateVarParameters, StateVarTyped,
};

use lazy_static::lazy_static;

#[derive(Debug)]
struct Hidden {}

impl Hidden {
    pub fn new() -> Self {
        Hidden {}
    }
}

impl StateVarInterface<bool> for Hidden {
    fn calculate_state_var_from_dependencies(
        &mut self,
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
        &mut self,
        state_var: &StateVarMutableViewTyped<bool>,
    ) -> () {
        state_var.set_value(false);
    }
}

lazy_static! {
    pub static ref GENERATE_STATE_VARS: fn() -> Vec<StateVar> = || {
        vec![
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
        component_type: "p",

        state_var_index_map: SV_MAP.clone(),

        state_var_names: STATE_VARIABLES_NAMES_IN_ORDER.to_vec(),

        state_var_component_types: GENERATE_STATE_VARS()
            .iter()
            .map(|sv| sv.get_default_component_type())
            .collect(),

        generate_state_vars: *GENERATE_STATE_VARS,

        attribute_names: vec!["hide", "disabled",],

        should_render_children: true,

        display_errors: true,

        action_names: || vec!["recordVisibilityChange"],

        valid_children_profiles: ValidChildTypes::AllComponents,

        ..Default::default()
    };
}
