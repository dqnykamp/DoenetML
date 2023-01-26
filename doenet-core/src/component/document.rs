use lazy_static::lazy_static;

// use crate::base_definitions::*;
use crate::state::StateVarTyped;
use crate::state_variables::*;

use super::*;

#[derive(Debug)]
struct SubmitLabel {
    val: StateVarTyped<String>,
}

impl StateVariable<String> for SubmitLabel {
    fn calculate_state_var_from_dependencies(&self) -> () {
        self.val.set_value(String::from("Check work"));
    }
    fn get_value_assuming_fresh(&self) -> String {
        self.val.get_value_assuming_fresh().clone()
    }
    fn create_new_mutable_view(&self) -> StateVarTyped<String> {
        self.val.create_new_mutable_view()
    }
    fn get_name(&self) -> &'static str {
        "submitLabel"
    }
}

#[derive(Debug)]
struct SubmitLabelNoCorrectness {
    val: StateVarTyped<String>,
}

impl StateVariable<String> for SubmitLabelNoCorrectness {
    fn calculate_state_var_from_dependencies(&self) -> () {
        self.val.set_value(String::from("Submit Response"));
    }
    fn return_for_renderer(&self) -> bool {
        true
    }
    fn get_value_assuming_fresh(&self) -> String {
        self.val.get_value_assuming_fresh().clone()
    }
    fn create_new_mutable_view(&self) -> StateVarTyped<String> {
        self.val.create_new_mutable_view()
    }
    fn get_name(&self) -> &'static str {
        "submitLabelNoCorrectness"
    }
}

#[derive(Debug)]
struct Hidden {
    val: StateVarTyped<bool>,
}

impl StateVariable<bool> for Hidden {
    fn calculate_state_var_from_dependencies(&self) -> () {
        self.val.set_value(false);
    }
    fn return_for_renderer(&self) -> bool {
        true
    }
    fn get_value_assuming_fresh(&self) -> bool {
        *self.val.get_value_assuming_fresh()
    }
    fn create_new_mutable_view(&self) -> StateVarTyped<bool> {
        self.val.create_new_mutable_view()
    }
    fn get_name(&self) -> &'static str {
        "hidden"
    }
}

#[derive(Debug)]
struct Disabled {
    val: StateVarTyped<bool>,
}

impl StateVariable<bool> for Disabled {
    fn calculate_state_var_from_dependencies(&self) -> () {
        self.val.set_value(false);
    }
    fn return_for_renderer(&self) -> bool {
        true
    }
    fn get_value_assuming_fresh(&self) -> bool {
        *self.val.get_value_assuming_fresh()
    }
    fn create_new_mutable_view(&self) -> StateVarTyped<bool> {
        self.val.create_new_mutable_view()
    }
    fn get_name(&self) -> &'static str {
        "disabled"
    }
}

#[derive(Debug)]
struct Fixed {
    val: StateVarTyped<bool>,
}

impl StateVariable<bool> for Fixed {
    fn calculate_state_var_from_dependencies(&self) -> () {
        self.val.set_value(false);
    }
    fn return_for_renderer(&self) -> bool {
        true
    }
    fn get_value_assuming_fresh(&self) -> bool {
        *self.val.get_value_assuming_fresh()
    }
    fn create_new_mutable_view(&self) -> StateVarTyped<bool> {
        self.val.create_new_mutable_view()
    }
    fn get_name(&self) -> &'static str {
        "fixed"
    }
}

#[derive(Debug)]
struct Title {
    val: StateVarTyped<String>,
}

impl StateVariable<String> for Title {
    fn calculate_state_var_from_dependencies(&self) -> () {
        self.val.set_value(String::from(""));
    }
    fn return_for_renderer(&self) -> bool {
        true
    }
    fn get_value_assuming_fresh(&self) -> String {
        self.val.get_value_assuming_fresh().clone()
    }
    fn create_new_mutable_view(&self) -> StateVarTyped<String> {
        self.val.create_new_mutable_view()
    }
    fn get_name(&self) -> &'static str {
        "title"
    }
}

#[derive(Debug)]
struct Level {
    val: StateVarTyped<i64>,
}

impl StateVariable<i64> for Level {
    fn calculate_state_var_from_dependencies(&self) -> () {
        self.val.set_value(0)
    }
    fn return_for_renderer(&self) -> bool {
        true
    }
    fn get_value_assuming_fresh(&self) -> i64 {
        *self.val.get_value_assuming_fresh()
    }
    fn create_new_mutable_view(&self) -> StateVarTyped<i64> {
        self.val.create_new_mutable_view()
    }
    fn get_name(&self) -> &'static str {
        "level"
    }
}

#[derive(Debug)]
struct JustSubmitted {
    val: StateVarTyped<bool>,
}

impl StateVariable<bool> for JustSubmitted {
    fn calculate_state_var_from_dependencies(&self) -> () {
        self.val.set_value(true);
    }
    fn return_for_renderer(&self) -> bool {
        true
    }
    fn get_value_assuming_fresh(&self) -> bool {
        *self.val.get_value_assuming_fresh()
    }
    fn create_new_mutable_view(&self) -> StateVarTyped<bool> {
        self.val.create_new_mutable_view()
    }
    fn get_name(&self) -> &'static str {
        "justSubmitted"
    }
}

#[derive(Debug)]
struct ShowCorrectness {
    val: StateVarTyped<bool>,
}

impl StateVariable<bool> for ShowCorrectness {
    fn calculate_state_var_from_dependencies(&self) -> () {
        self.val.set_value(true);
    }
    fn return_for_renderer(&self) -> bool {
        true
    }
    fn get_value_assuming_fresh(&self) -> bool {
        *self.val.get_value_assuming_fresh()
    }
    fn create_new_mutable_view(&self) -> StateVarTyped<bool> {
        self.val.create_new_mutable_view()
    }
    fn get_name(&self) -> &'static str {
        "showCorrectness"
    }
}

#[derive(Debug)]
struct CreditAchieved {
    val: StateVarTyped<f64>,
}

impl StateVariable<f64> for CreditAchieved {
    fn calculate_state_var_from_dependencies(&self) -> () {
        self.val.set_value(1.0);
    }
    fn return_for_renderer(&self) -> bool {
        true
    }
    fn get_value_assuming_fresh(&self) -> f64 {
        *self.val.get_value_assuming_fresh()
    }
    fn create_new_mutable_view(&self) -> StateVarTyped<f64> {
        self.val.create_new_mutable_view()
    }
    fn get_name(&self) -> &'static str {
        "creditAchieved"
    }
}

#[derive(Debug)]
struct CreateSubmitAllButton {
    val: StateVarTyped<bool>,
}

impl StateVariable<bool> for CreateSubmitAllButton {
    fn calculate_state_var_from_dependencies(&self) -> () {
        self.val.set_value(false);
    }
    fn return_for_renderer(&self) -> bool {
        true
    }
    fn get_value_assuming_fresh(&self) -> bool {
        *self.val.get_value_assuming_fresh()
    }
    fn create_new_mutable_view(&self) -> StateVarTyped<bool> {
        self.val.create_new_mutable_view()
    }
    fn get_name(&self) -> &'static str {
        "createSubmitAllButton"
    }
}

#[derive(Debug)]
struct SuppressAnswerSubmitButtons {
    val: StateVarTyped<bool>,
}

impl StateVariable<bool> for SuppressAnswerSubmitButtons {
    fn calculate_state_var_from_dependencies(&self) -> () {
        self.val.set_value(true);
    }
    fn return_for_renderer(&self) -> bool {
        true
    }
    fn get_value_assuming_fresh(&self) -> bool {
        *self.val.get_value_assuming_fresh()
    }
    fn create_new_mutable_view(&self) -> StateVarTyped<bool> {
        self.val.create_new_mutable_view()
    }
    fn get_name(&self) -> &'static str {
        "suppressAnswerSubmitButtons"
    }
}


lazy_static! {

    pub static ref GENERATE_STATE_VARS: fn () -> Vec<StateVarVariant> = || {
        vec![
            StateVarVariant::String(
                Box::new(SubmitLabel {
                    val: StateVarTyped::new()
                })
            ),
            StateVarVariant::String(
                Box::new(SubmitLabelNoCorrectness {
                    val: StateVarTyped::new()
                })
            ),
            StateVarVariant::Boolean(
                Box::new(Hidden {
                    val: StateVarTyped::new()
                })
            ),
            StateVarVariant::Boolean(
                Box::new(Disabled {
                    val: StateVarTyped::new()
                })
            ),
            StateVarVariant::Boolean(
                Box::new(Fixed {
                    val: StateVarTyped::new()
                })
            ),
            StateVarVariant::String(
                Box::new(Title {
                    val: StateVarTyped::new()
                })
            ),
            StateVarVariant::Integer(
                Box::new(Level {
                    val: StateVarTyped::new()
                })
            ),
            StateVarVariant::Boolean(
                Box::new(JustSubmitted {
                    val: StateVarTyped::new()
                })
            ),
            StateVarVariant::Boolean(
                Box::new(ShowCorrectness {
                    val: StateVarTyped::new()
                })
            ),
            StateVarVariant::Number(
                Box::new(CreditAchieved {
                    val: StateVarTyped::new()
                })
            ),
            StateVarVariant::Boolean(
                Box::new(CreateSubmitAllButton {
                    val: StateVarTyped::new()
                })
            ),
            StateVarVariant::Boolean(
                Box::new(SuppressAnswerSubmitButtons {
                    val: StateVarTyped::new()
                })
            )
        ]


    };

    pub static ref STATE_VARIABLES_NAMES_IN_ORDER: Vec<&'static str> = GENERATE_STATE_VARS().iter().map(|sv| sv.get_name()).collect();

    pub static ref MY_COMPONENT_DEFINITION: ComponentDefinition = ComponentDefinition {
        component_type: "document",

        // state_var_definitions: &MY_STATE_VAR_DEFINITIONS,

        state_var_index_map: STATE_VARIABLES_NAMES_IN_ORDER.iter().enumerate().map(|(i,v)| (*v,i) ).collect(),

        state_var_names: STATE_VARIABLES_NAMES_IN_ORDER.to_vec(),

        generate_state_vars: *GENERATE_STATE_VARS,

        attribute_names: vec![],

        should_render_children: true,

        display_errors: true,

        valid_children_profiles: ValidChildTypes::AllComponents,

        ..Default::default()
    };


}
