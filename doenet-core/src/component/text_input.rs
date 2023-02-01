use lazy_static::lazy_static;

use super::*;
use crate::{
    state::{
        StateVarInterface, StateVarMutableViewTyped, StateVarParameters, StateVarReadOnlyView,
        StateVarReadOnlyViewTyped, StateVarTyped, UpdatesRequested,
    },
    utils::log,
};

use crate::base_definitions::*;

string_state_variable_from_attribute!(
    "bindValueTo",
    StateVarValue::String(String::from("")),
    BindValueTo
);

#[derive(Debug)]
struct Value {
    essential_value: StateVarReadOnlyViewTyped<String>,
    immediate_value: StateVarReadOnlyViewTyped<String>,
    sync_values: StateVarReadOnlyViewTyped<bool>,
    bind_value_to: StateVarReadOnlyViewTyped<String>,
}

impl Value {
    pub fn new() -> Self {
        Value {
            essential_value: StateVarReadOnlyViewTyped::new(),
            immediate_value: StateVarReadOnlyViewTyped::new(),
            sync_values: StateVarReadOnlyViewTyped::new(),
            bind_value_to: StateVarReadOnlyViewTyped::new(),
        }
    }
}

impl StateVarInterface<String> for Value {
    fn return_dependency_instructions(&self) -> Vec<DependencyInstruction> {
        vec![
            DependencyInstruction::Essential {
                prefill: Some("prefill"),
            },
            DependencyInstruction::StateVar {
                component_name: None,
                state_var_name: "immediateValue",
            },
            DependencyInstruction::StateVar {
                component_name: None,
                state_var_name: "syncImmediateValue",
            },
            DependencyInstruction::StateVar {
                component_name: None,
                state_var_name: "bindValueTo",
            },
        ]
    }

    fn set_dependencies(&mut self, dependencies: &Vec<Vec<DependencyValue>>) -> () {
        if let StateVarReadOnlyView::String(essential_value) = &dependencies[0][0].value {
            self.essential_value = essential_value.create_new_read_only_view();
        } else {
            panic!("Got a non-string essential value for value of text input.");
        }

        if let StateVarReadOnlyView::String(immediate_value) = &dependencies[1][0].value {
            self.immediate_value = immediate_value.create_new_read_only_view();
        } else {
            panic!("Got a non-string immediate value for value of text input.");
        }

        if let StateVarReadOnlyView::Boolean(sync_values) = &dependencies[2][0].value {
            self.sync_values = sync_values.create_new_read_only_view();
        } else {
            panic!("Got a non-boolean sync values for value of text input.");
        }

        if let StateVarReadOnlyView::String(bind_value_to) = &dependencies[3][0].value {
            self.bind_value_to = bind_value_to.create_new_read_only_view();
        } else {
            panic!("Got a non-string bind_value_to for value of text input.");
        }
    }

    fn calculate_state_var_from_dependencies(
        &mut self,
        state_var: &StateVarMutableViewTyped<String>,
    ) -> () {
        let bind_value_to_used_default = self.bind_value_to.get_used_default();

        let value = if *self.sync_values.get_fresh_value_record_viewed() {
            self.immediate_value.get_fresh_value_record_viewed()
        } else if bind_value_to_used_default {
            self.essential_value.get_fresh_value_record_viewed()
        } else {
            self.bind_value_to.get_fresh_value_record_viewed()
        };

        state_var.set_value(value.clone());
    }

    fn request_dependencies_to_update_value(
        &self,
        state_var: &StateVarReadOnlyViewTyped<String>,
        _is_initial_change: bool,
    ) -> Result<Vec<UpdatesRequested>, ()> {
        let desired_value = state_var.get_requested_value();
        let bind_value_to_used_default = self.bind_value_to.get_used_default();

        if bind_value_to_used_default {
            self.essential_value.request_change_value_to(desired_value.clone());
            self.immediate_value.request_change_value_to(desired_value.clone());
            self.sync_values.request_change_value_to(true);

            Ok(vec![
                UpdatesRequested {
                    instruction_ind: 0,
                    dependency_ind: 0,
                },
                UpdatesRequested {
                    instruction_ind: 1,
                    dependency_ind: 0,
                },
                UpdatesRequested {
                    instruction_ind: 2,
                    dependency_ind: 0,
                },
            ])
        } else {
            self.bind_value_to.request_change_value_to(desired_value.clone());
            self.sync_values.request_change_value_to(true);

            Ok(vec![
                UpdatesRequested {
                    instruction_ind: 3,
                    dependency_ind: 0,
                },
                UpdatesRequested {
                    instruction_ind: 2,
                    dependency_ind: 0,
                },
            ])
        }
    }
}

#[derive(Debug)]
struct ImmediateValue {
    essential_value: StateVarReadOnlyViewTyped<String>,
    sync_values: StateVarReadOnlyViewTyped<bool>,
    bind_value_to: StateVarReadOnlyViewTyped<String>,
}

impl ImmediateValue {
    pub fn new() -> Self {
        ImmediateValue {
            essential_value: StateVarReadOnlyViewTyped::new(),
            sync_values: StateVarReadOnlyViewTyped::new(),
            bind_value_to: StateVarReadOnlyViewTyped::new(),
        }
    }
}

impl StateVarInterface<String> for ImmediateValue {
    fn return_dependency_instructions(&self) -> Vec<DependencyInstruction> {
        vec![
            DependencyInstruction::Essential {
                prefill: Some("prefill"),
            },
            DependencyInstruction::StateVar {
                component_name: None,
                state_var_name: "syncImmediateValue",
            },
            DependencyInstruction::StateVar {
                component_name: None,
                state_var_name: "bindValueTo",
            },
        ]
    }

    fn set_dependencies(&mut self, dependencies: &Vec<Vec<DependencyValue>>) -> () {
        if let StateVarReadOnlyView::String(essential_value) = &dependencies[0][0].value {
            self.essential_value = essential_value.create_new_read_only_view();
        } else {
            panic!("Got a non-string essential value for immediate value of text input.");
        }

        if let StateVarReadOnlyView::Boolean(sync_values) = &dependencies[1][0].value {
            self.sync_values = sync_values.create_new_read_only_view();
        } else {
            panic!("Got a non-boolean sync values for immediate value of text input.");
        }

        if let StateVarReadOnlyView::String(bind_value_to) = &dependencies[2][0].value {
            self.bind_value_to = bind_value_to.create_new_read_only_view();
        } else {
            panic!("Got a non-string bind_value_to for immediate value of text input.");
        }
    }

    fn calculate_state_var_from_dependencies(
        &mut self,
        state_var: &StateVarMutableViewTyped<String>,
    ) -> () {
        let bind_value_to_used_default = self.bind_value_to.get_used_default();

        let immediate_value =
            if !bind_value_to_used_default && *self.sync_values.get_fresh_value_record_viewed() {
                self.bind_value_to.get_fresh_value_record_viewed()
            } else {
                self.essential_value.get_fresh_value_record_viewed()
            };

        state_var.set_value(immediate_value.clone());
    }

    fn request_dependencies_to_update_value(
        &self,
        state_var: &StateVarReadOnlyViewTyped<String>,
        is_initial_change: bool,
    ) -> Result<Vec<UpdatesRequested>, ()> {
        let desired_value = state_var.get_requested_value();

        let mut updates = Vec::with_capacity(2);
        let bind_value_to_used_default = self.bind_value_to.get_used_default();

        self.essential_value.request_change_value_to(desired_value.clone());

        updates.push(UpdatesRequested {
            instruction_ind: 0,
            dependency_ind: 0,
        });

        if !is_initial_change && !bind_value_to_used_default {
            self.bind_value_to.request_change_value_to(desired_value.clone());

            updates.push(UpdatesRequested {
                instruction_ind: 2,
                dependency_ind: 0,
            });
        }

        Ok(updates)
    }
}

#[derive(Debug)]
struct SyncImmediateValue {
    essential_value: StateVarReadOnlyViewTyped<bool>,
}

impl SyncImmediateValue {
    pub fn new() -> Self {
        SyncImmediateValue {
            essential_value: StateVarReadOnlyViewTyped::new(),
        }
    }
}

impl StateVarInterface<bool> for SyncImmediateValue {
    fn return_dependency_instructions(&self) -> Vec<DependencyInstruction> {
        vec![DependencyInstruction::Essential { prefill: None }]
    }

    fn set_dependencies(&mut self, dependencies: &Vec<Vec<DependencyValue>>) -> () {
        if let StateVarReadOnlyView::Boolean(essential_value) = &dependencies[0][0].value {
            self.essential_value = essential_value.create_new_read_only_view();
        } else {
            panic!("Got a non-booloean essential value for syncImmediate of text input.");
        }
    }

    fn calculate_state_var_from_dependencies(
        &mut self,
        state_var: &StateVarMutableViewTyped<bool>,
    ) -> () {
        state_var.set_value(self.essential_value.get_fresh_value_record_viewed().clone());
    }

    fn request_dependencies_to_update_value(
        &self,
        state_var: &StateVarReadOnlyViewTyped<bool>,
        _is_initial_change: bool,
    ) -> Result<Vec<UpdatesRequested>, ()> {
        let desired_value = state_var.get_requested_value();

        self.essential_value.request_change_value_to(*desired_value);

        Ok(vec![UpdatesRequested {
            instruction_ind: 0,
            dependency_ind: 0,
        }])
    }
}

#[derive(Debug)]
struct Expanded {}

impl Expanded {
    pub fn new() -> Self {
        Expanded {}
    }
}

impl StateVarInterface<bool> for Expanded {
    fn calculate_state_var_from_dependencies(
        &mut self,
        state_var: &StateVarMutableViewTyped<bool>,
    ) -> () {
        state_var.set_value(false);
    }
}

#[derive(Debug)]
struct Size {}

impl Size {
    pub fn new() -> Self {
        Size {}
    }
}

impl StateVarInterface<f64> for Size {
    fn calculate_state_var_from_dependencies(
        &mut self,
        state_var: &StateVarMutableViewTyped<f64>,
    ) -> () {
        state_var.set_value(10.0);
    }
}

#[derive(Debug)]
struct Width {}

impl Width {
    pub fn new() -> Self {
        Width {}
    }
}

impl StateVarInterface<f64> for Width {
    fn calculate_state_var_from_dependencies(
        &mut self,
        state_var: &StateVarMutableViewTyped<f64>,
    ) -> () {
        state_var.set_value(600.0);
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

lazy_static! {

    pub static ref GENERATE_STATE_VARS: fn () -> Vec<StateVar> = || {
        vec![
            StateVar::String(
                StateVarTyped::new(
                    Box::new(Value::new()),
                    StateVarParameters {
                        name: "value",
                        ..Default::default()
                    }
                )
            ),
            StateVar::String(
                StateVarTyped::new(
                    Box::new(ImmediateValue::new()),
                    StateVarParameters {
                        name: "immediateValue",
                        for_renderer: true,
                        ..Default::default()
                    }
                )
            ),
            StateVar::Boolean(
                StateVarTyped::new(
                    Box::new(SyncImmediateValue::new()),
                    StateVarParameters {
                        name: "syncImmediateValue",
                        initial_essential_value: true,
                        ..Default::default()
                    }
                )
            ),
            StateVar::String(
                StateVarTyped::new(
                    Box::new(BindValueTo::new()),
                    StateVarParameters {
                        name: "bindValueTo",
                        ..Default::default()
                    }
                )
            ),
            StateVar::Boolean(
                StateVarTyped::new(
                    Box::new(Expanded::new()),
                    StateVarParameters {
                        name: "expanded",
                        for_renderer: true,
                        ..Default::default()
                    }
                )
            ),
            StateVar::Number(
                StateVarTyped::new(
                    Box::new(Width::new()),
                    StateVarParameters {
                        name: "width",
                        for_renderer: true,
                        ..Default::default()
                    }
                )
            ),
            StateVar::Number(
                StateVarTyped::new(
                    Box::new(Size::new()),
                    StateVarParameters {
                        name: "size",
                        for_renderer: true,
                        ..Default::default()
                    }
                )
            ),
            StateVar::Boolean(
                StateVarTyped::new(
                    Box::new(Hidden::new()),
                    StateVarParameters {
                        name: "hidden",
                        for_renderer: true,
                        ..Default::default()
                    }
                )
            ),
            StateVar::Boolean(
                StateVarTyped::new(
                    Box::new(Disabled::new()),
                    StateVarParameters {
                        name: "disabled",
                        for_renderer: true,
                        ..Default::default()
                    }
                )
            ),
        ]


    };

    pub static ref STATE_VARIABLES_NAMES_IN_ORDER: Vec<&'static str> = GENERATE_STATE_VARS().iter().map(|sv| sv.get_name()).collect();

    pub static ref SV_MAP: HashMap<&'static str, usize> = STATE_VARIABLES_NAMES_IN_ORDER.iter().enumerate().map(|(i,v)| (*v,i) ).collect();

    pub static ref MY_COMPONENT_DEFINITION: ComponentDefinition = ComponentDefinition {
        component_type: "textInput",

        state_var_index_map: SV_MAP.clone(),

        state_var_names: STATE_VARIABLES_NAMES_IN_ORDER.to_vec(),

        state_var_component_types: GENERATE_STATE_VARS().iter().map(|sv| sv.get_default_component_type()).collect(),

        generate_state_vars: *GENERATE_STATE_VARS,

        attribute_names: vec![
            "hide",
            "disabled",
            "prefill",
            "bindValueTo"
        ],

        action_names: || vec!["updateImmediateValue", "updateValue"],

        on_action: |action_name, args, resolve_and_retrieve_state_var| {
            match action_name {
                "updateImmediateValue" => {
                    // Note: the key here is whatever the renderers call the new value
                    let new_val = args.get("text").expect("No text argument").first().unwrap();

                    vec![
                        (*SV_MAP.get("immediateValue").unwrap(), new_val.clone()),
                        (*SV_MAP.get("syncImmediateValue").unwrap(), StateVarValue::Boolean(false)),
                    ]
                },

                "updateValue" => {

                    let new_val = resolve_and_retrieve_state_var(1)
                        .try_into().unwrap();
                    let new_val = StateVarValue::String(new_val);

                    vec![
                        (*SV_MAP.get("value").unwrap(), new_val),
                    ]

                }

                _ => panic!("Unknown action '{}' called on textInput", action_name)
            }
        },

        ..Default::default()
    };
}
