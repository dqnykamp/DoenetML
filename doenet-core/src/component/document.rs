use lazy_static::lazy_static;

use crate::state::{
    StateVarInterface, StateVarMutableViewTyped, StateVarParameters, StateVarTyped,
};

use super::*;

#[derive(Debug)]
struct SubmitLabel {}

impl SubmitLabel {
    pub fn new() -> Self {
        SubmitLabel {}
    }
}

impl StateVarInterface<String> for SubmitLabel {
    fn calculate_state_var_from_dependencies(
        &mut self,
        state_var: &StateVarMutableViewTyped<String>,
    ) -> () {
        state_var.set_value(String::from("Check Work"));
    }
}

#[derive(Debug)]
struct SubmitLabelNoCorrectness {}

impl SubmitLabelNoCorrectness {
    pub fn new() -> Self {
        SubmitLabelNoCorrectness {}
    }
}

impl StateVarInterface<String> for SubmitLabelNoCorrectness {
    fn calculate_state_var_from_dependencies(
        &mut self,
        state_var: &StateVarMutableViewTyped<String>,
    ) -> () {
        state_var.set_value(String::from("Submit Response"));
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

#[derive(Debug)]
struct Fixed {}

impl Fixed {
    pub fn new() -> Self {
        Fixed {}
    }
}

impl StateVarInterface<bool> for Fixed {
    fn calculate_state_var_from_dependencies(
        &mut self,
        state_var: &StateVarMutableViewTyped<bool>,
    ) -> () {
        state_var.set_value(false);
    }
}

#[derive(Debug)]
struct Title {}

impl Title {
    pub fn new() -> Self {
        Title {}
    }
}

impl StateVarInterface<String> for Title {
    fn calculate_state_var_from_dependencies(
        &mut self,
        state_var: &StateVarMutableViewTyped<String>,
    ) -> () {
        state_var.set_value(String::from(""));
    }
}

#[derive(Debug)]
struct Level {}

impl Level {
    pub fn new() -> Self {
        Level {}
    }
}

impl StateVarInterface<i64> for Level {
    fn calculate_state_var_from_dependencies(
        &mut self,
        state_var: &StateVarMutableViewTyped<i64>,
    ) -> () {
        state_var.set_value(0);
    }
}

#[derive(Debug)]
struct JustSubmitted {}

impl JustSubmitted {
    pub fn new() -> Self {
        JustSubmitted {}
    }
}

impl StateVarInterface<bool> for JustSubmitted {
    fn calculate_state_var_from_dependencies(
        &mut self,
        state_var: &StateVarMutableViewTyped<bool>,
    ) -> () {
        state_var.set_value(true);
    }
}

#[derive(Debug)]
struct ShowCorrectness {}

impl ShowCorrectness {
    pub fn new() -> Self {
        ShowCorrectness {}
    }
}

impl StateVarInterface<bool> for ShowCorrectness {
    fn calculate_state_var_from_dependencies(
        &mut self,
        state_var: &StateVarMutableViewTyped<bool>,
    ) -> () {
        state_var.set_value(true);
    }
}

#[derive(Debug)]
struct CreditAchieved {}

impl CreditAchieved {
    pub fn new() -> Self {
        CreditAchieved {}
    }
}

impl StateVarInterface<f64> for CreditAchieved {
    fn calculate_state_var_from_dependencies(
        &mut self,
        state_var: &StateVarMutableViewTyped<f64>,
    ) -> () {
        state_var.set_value(1.0);
    }
}

#[derive(Debug)]
struct CreateSubmitAllButton {}

impl CreateSubmitAllButton {
    pub fn new() -> Self {
        CreateSubmitAllButton {}
    }
}

impl StateVarInterface<bool> for CreateSubmitAllButton {
    fn calculate_state_var_from_dependencies(
        &mut self,
        state_var: &StateVarMutableViewTyped<bool>,
    ) -> () {
        state_var.set_value(false);
    }
}

#[derive(Debug)]
struct SuppressAnswerSubmitButtons {}

impl SuppressAnswerSubmitButtons {
    pub fn new() -> Self {
        SuppressAnswerSubmitButtons {}
    }
}

impl StateVarInterface<bool> for SuppressAnswerSubmitButtons {
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
            StateVar::String(StateVarTyped::new(
                Box::new(SubmitLabel::new()),
                StateVarParameters {
                    name: "submitLabel",
                    for_renderer: true,
                    ..Default::default()
                },
            )),
            StateVar::String(StateVarTyped::new(
                Box::new(SubmitLabelNoCorrectness::new()),
                StateVarParameters {
                    name: "submitLabelNoCorrectness",
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
            StateVar::String(StateVarTyped::new(
                Box::new(Title::new()),
                StateVarParameters {
                    name: "title",
                    for_renderer: true,
                    ..Default::default()
                },
            )),
            StateVar::Integer(StateVarTyped::new(
                Box::new(Level::new()),
                StateVarParameters {
                    name: "level",
                    for_renderer: true,
                    ..Default::default()
                },
            )),
            StateVar::Boolean(StateVarTyped::new(
                Box::new(JustSubmitted::new()),
                StateVarParameters {
                    name: "justSubmitted",
                    for_renderer: true,
                    ..Default::default()
                },
            )),
            StateVar::Boolean(StateVarTyped::new(
                Box::new(ShowCorrectness::new()),
                StateVarParameters {
                    name: "showCorrectness",
                    for_renderer: true,
                    ..Default::default()
                },
            )),
            StateVar::Number(StateVarTyped::new(
                Box::new(CreditAchieved::new()),
                StateVarParameters {
                    name: "creditAchieved",
                    for_renderer: true,
                    ..Default::default()
                },
            )),
            StateVar::Boolean(StateVarTyped::new(
                Box::new(CreateSubmitAllButton::new()),
                StateVarParameters {
                    name: "createSubmitAllButton",
                    for_renderer: true,
                    ..Default::default()
                },
            )),
            StateVar::Boolean(StateVarTyped::new(
                Box::new(SuppressAnswerSubmitButtons::new()),
                StateVarParameters {
                    name: "suppressAnswerSubmitButtons",
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
        component_type: "document",

        state_var_index_map: SV_MAP.clone(),

        state_var_names: STATE_VARIABLES_NAMES_IN_ORDER.to_vec(),

        state_var_component_types: GENERATE_STATE_VARS()
            .iter()
            .map(|sv| sv.get_default_component_type())
            .collect(),

        generate_state_vars: *GENERATE_STATE_VARS,

        attribute_names: vec![],

        should_render_children: true,

        display_errors: true,

        valid_children_profiles: ValidChildTypes::AllComponents,

        ..Default::default()
    };
}
