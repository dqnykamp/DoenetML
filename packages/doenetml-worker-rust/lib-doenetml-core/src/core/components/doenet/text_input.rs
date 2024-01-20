use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use strum::VariantNames;
use strum_macros::EnumVariantNames;

use crate::{
    components::{actions::ActionBody, prelude::*, ActionsEnum},
    state_var_interfaces::{
        boolean_state_var_interfaces::GeneralBooleanStateVarInterface,
        text_state_var_interfaces::GeneralStringStateVarInterface,
    },
};

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(from_wasm_abi))]
#[serde(expecting = "`text` must be a string")]
pub struct TextInputActionArgs {
    pub text: String,
}

#[derive(Debug, Deserialize, Serialize, EnumVariantNames)]
#[serde(tag = "actionName", rename_all = "camelCase")]
#[strum(serialize_all = "camelCase")]
#[cfg_attr(feature = "web", derive(tsify::Tsify))]
#[cfg_attr(feature = "web", tsify(from_wasm_abi))]
pub enum TextInputAction {
    UpdateImmediateValue(ActionBody<TextInputActionArgs>),
    UpdateValue,
}

#[derive(Debug, Default, ComponentNode, ComponentStateVariables)]
pub struct TextInput {
    pub common: ComponentCommonData,

    pub state: TextInputStateVariables,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TextInputRenderedState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub immediate_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
}

#[derive(Debug)]
pub struct TextInputStateVariables {
    value: StateVar<String>,
    immediate_value: StateVar<String>,
    sync_immediate_value: StateVar<bool>,
    bind_value_to: StateVar<String>,
    prefill: StateVar<String>,
    hidden: StateVar<bool>,
    disabled: StateVar<bool>,
}

impl TextInputStateVariables {
    fn new() -> Self {
        TextInputStateVariables {
            value: StateVar::new(
                Box::<ValueStateVarInterface>::default(),
                StateVarParameters {
                    ..Default::default()
                },
                Default::default(),
            ),
            immediate_value: StateVar::new(
                Box::<ImmediateValueStateVarInterface>::default(),
                StateVarParameters {
                    ..Default::default()
                },
                Default::default(),
            ),
            sync_immediate_value: StateVar::new(
                Box::<SyncImmediateValueStateVarInterface>::default(),
                StateVarParameters {
                    ..Default::default()
                },
                true,
            ),
            bind_value_to: StateVar::new(
                Box::<GeneralStringStateVarInterface>::default(),
                StateVarParameters {
                    dependency_instruction_hint: Some(DependencyInstruction::AttributeChild {
                        attribute_name: "bindValueTo",
                        match_profiles: vec![ComponentProfile::Text],
                    }),
                    ..Default::default()
                },
                Default::default(),
            ),
            prefill: StateVar::new(
                Box::<GeneralStringStateVarInterface>::default(),
                StateVarParameters {
                    dependency_instruction_hint: Some(DependencyInstruction::AttributeChild {
                        attribute_name: "prefill",
                        match_profiles: vec![ComponentProfile::Text],
                    }),
                    ..Default::default()
                },
                Default::default(),
            ),
            hidden: StateVar::new(
                Box::<GeneralBooleanStateVarInterface>::default(),
                StateVarParameters {
                    dependency_instruction_hint: Some(DependencyInstruction::AttributeChild {
                        attribute_name: "hide",
                        match_profiles: vec![ComponentProfile::Text, ComponentProfile::Boolean],
                    }),
                    ..Default::default()
                },
                false,
            ),
            disabled: StateVar::new(
                Box::<GeneralBooleanStateVarInterface>::default(),
                StateVarParameters {
                    dependency_instruction_hint: Some(DependencyInstruction::AttributeChild {
                        attribute_name: "disabled",
                        match_profiles: vec![ComponentProfile::Text, ComponentProfile::Boolean],
                    }),
                    ..Default::default()
                },
                false,
            ),
        }
    }
}

impl Default for TextInputStateVariables {
    fn default() -> Self {
        TextInputStateVariables::new()
    }
}

// TODO: derive via macros
impl ComponentStateVariables for TextInputStateVariables {
    fn get_num_state_variables(&self) -> StateVarIdx {
        7
    }
    fn get_state_variable(&self, state_var_idx: StateVarIdx) -> Option<StateVarEnumRef> {
        match state_var_idx {
            0 => Some((&self.value).into()),
            1 => Some((&self.immediate_value).into()),
            2 => Some((&self.sync_immediate_value).into()),
            3 => Some((&self.bind_value_to).into()),
            4 => Some((&self.prefill).into()),
            5 => Some((&self.hidden).into()),
            6 => Some((&self.disabled).into()),
            _ => None,
        }
    }

    fn get_state_variable_mut(&mut self, state_var_idx: StateVarIdx) -> Option<StateVarEnumRefMut> {
        match state_var_idx {
            0 => Some((&mut self.value).into()),
            1 => Some((&mut self.immediate_value).into()),
            2 => Some((&mut self.sync_immediate_value).into()),
            3 => Some((&mut self.bind_value_to).into()),
            4 => Some((&mut self.prefill).into()),
            5 => Some((&mut self.hidden).into()),
            6 => Some((&mut self.disabled).into()),
            _ => None,
        }
    }

    fn get_state_variable_index_from_name(&self, name: &str) -> Option<StateVarIdx> {
        match name {
            "value" => Some(0),
            "immediateValue" => Some(1),
            "syncImmediateValue" => Some(2),
            "bindValueTo" => Some(3),
            "prefill" => Some(4),
            "hidden" => Some(5),
            "disabled" => Some(6),
            _ => None,
        }
    }

    fn get_state_variable_index_from_name_case_insensitive(
        &self,
        name: &str,
    ) -> Option<StateVarIdx> {
        match name {
            x if x.eq_ignore_ascii_case("value") => Some(0),
            x if x.eq_ignore_ascii_case("immediateValue") => Some(1),
            x if x.eq_ignore_ascii_case("syncImmediateValue") => Some(2),
            x if x.eq_ignore_ascii_case("bindValueTo") => Some(3),
            x if x.eq_ignore_ascii_case("prefill") => Some(4),
            x if x.eq_ignore_ascii_case("hidden") => Some(5),
            x if x.eq_ignore_ascii_case("disabled") => Some(6),
            _ => None,
        }
    }

    fn get_component_profile_state_variables(&self) -> Vec<ComponentProfileStateVariable> {
        vec![ComponentProfileStateVariable::Text(
            self.value.create_new_read_only_view(),
            0,
        )]
    }

    fn get_public_state_variable_index_from_name_case_insensitive(
        &self,
        name: &str,
    ) -> Option<StateVarIdx> {
        match name {
            x if x.eq_ignore_ascii_case("value") => Some(0),
            x if x.eq_ignore_ascii_case("immediateValue") => Some(1),
            x if x.eq_ignore_ascii_case("prefill") => Some(4),
            x if x.eq_ignore_ascii_case("hidden") => Some(5),
            x if x.eq_ignore_ascii_case("disabled") => Some(6),
            _ => None,
        }
    }

    fn get_for_renderer_state_variable_indices(&self) -> Vec<StateVarIdx> {
        vec![1, 6]
    }

    fn check_if_state_variable_is_for_renderer(&self, state_var_idx: StateVarIdx) -> bool {
        match state_var_idx {
            1 => true,
            6 => true,
            _ => false,
        }
    }

    fn return_rendered_state(&mut self) -> Option<RenderedState> {
        Some(RenderedState::TextInput(TextInputRenderedState {
            immediate_value: Some(self.immediate_value.get_fresh_value_record_viewed().clone()),
            disabled: Some(*self.disabled.get_fresh_value_record_viewed()),
        }))
    }

    fn return_rendered_state_update(&mut self) -> Option<RenderedState> {
        let value_changed = self.immediate_value.check_if_changed_since_last_viewed();

        let disabled_changed = self.disabled.check_if_changed_since_last_viewed();

        if value_changed || disabled_changed {
            let mut updated_variables = TextInputRenderedState::default();

            if value_changed {
                updated_variables.immediate_value =
                    Some(self.immediate_value.get_fresh_value_record_viewed().clone());
            }

            if disabled_changed {
                updated_variables.disabled = Some(*self.disabled.get_fresh_value_record_viewed());
            }

            Some(RenderedState::TextInput(updated_variables))
        } else {
            None
        }
    }
}

// TODO via macro
impl TextInputStateVariables {
    fn get_value_state_variable_index() -> StateVarIdx {
        0
    }
    fn get_immediate_value_state_variable_index() -> StateVarIdx {
        1
    }
    fn get_sync_immediate_value_state_variable_index() -> StateVarIdx {
        2
    }
    fn get_bind_value_to_state_variable_index() -> StateVarIdx {
        3
    }
    fn get_prefill_state_variable_index() -> StateVarIdx {
        4
    }
    fn get_hidden_state_variable_index() -> StateVarIdx {
        5
    }
    fn get_disabled_state_variable_index() -> StateVarIdx {
        6
    }

    fn get_value_dependency_instructions() -> DependencyInstruction {
        DependencyInstruction::StateVar {
            component_idx: None,
            state_var_idx: 0,
        }
    }
    fn get_immediate_value_dependency_instructions() -> DependencyInstruction {
        DependencyInstruction::StateVar {
            component_idx: None,
            state_var_idx: 1,
        }
    }
    fn get_sync_immediate_value_dependency_instructions() -> DependencyInstruction {
        DependencyInstruction::StateVar {
            component_idx: None,
            state_var_idx: 2,
        }
    }
    fn get_bind_value_to_dependency_instructions() -> DependencyInstruction {
        DependencyInstruction::StateVar {
            component_idx: None,
            state_var_idx: 3,
        }
    }
    fn get_prefill_dependency_instructions() -> DependencyInstruction {
        DependencyInstruction::StateVar {
            component_idx: None,
            state_var_idx: 4,
        }
    }
}

impl RenderedComponentNode for TextInput {
    fn get_attribute_names(&self) -> Vec<AttributeName> {
        vec!["bindValueTo", "hide", "disabled", "prefill"]
    }

    fn get_action_names(&self) -> Vec<String> {
        TextInputAction::VARIANTS
            .iter()
            .map(|s| s.to_string())
            .collect()
    }

    fn on_action(
        &self,
        action: ActionsEnum,
        resolve_and_retrieve_state_var: &mut dyn FnMut(StateVarIdx) -> StateVarValueEnum,
    ) -> Result<Vec<(StateVarIdx, StateVarValueEnum)>, String> {
        // The type of `action` should have already been verified, so an
        // error here is a programming logic error, not an API error.
        let action: TextInputAction = action.try_into()?;

        match action {
            TextInputAction::UpdateImmediateValue(ActionBody { args }) => Ok(vec![
                (
                    TextInputStateVariables::get_immediate_value_state_variable_index(),
                    StateVarValueEnum::String(args.text),
                ),
                (
                    TextInputStateVariables::get_sync_immediate_value_state_variable_index(),
                    StateVarValueEnum::Boolean(false),
                ),
            ]),

            TextInputAction::UpdateValue => {
                let new_val = resolve_and_retrieve_state_var(
                    TextInputStateVariables::get_immediate_value_state_variable_index(),
                );

                Ok(vec![(
                    TextInputStateVariables::get_value_state_variable_index(),
                    new_val,
                )])
            }
        }
    }
}

#[derive(Debug, Default)]
struct ValueStateVarInterface {
    essential_value: StateVarReadOnlyView<String>,
    immediate_value: StateVarReadOnlyView<String>,
    sync_immediate_value: StateVarReadOnlyView<bool>,
    bind_value_to: StateVarReadOnlyView<String>,
    prefill: StateVarReadOnlyView<String>,
}

impl StateVarInterface<String> for ValueStateVarInterface {
    fn return_dependency_instructions(
        &self,
        _extend_source: Option<&ExtendSource>,
        _parameters: &StateVarParameters,
        _state_var_idx: StateVarIdx,
    ) -> Vec<DependencyInstruction> {
        vec![
            DependencyInstruction::Essential,
            TextInputStateVariables::get_immediate_value_dependency_instructions(),
            TextInputStateVariables::get_sync_immediate_value_dependency_instructions(),
            TextInputStateVariables::get_bind_value_to_dependency_instructions(),
            TextInputStateVariables::get_prefill_dependency_instructions(),
        ]
    }

    fn save_dependencies_for_value_calculation(&mut self, dependencies: &Vec<Vec<Dependency>>) {
        self.essential_value = (&dependencies[0][0].value).try_into().unwrap();
        self.immediate_value = (&dependencies[1][0].value).try_into().unwrap();
        self.sync_immediate_value = (&dependencies[2][0].value).try_into().unwrap();
        self.bind_value_to = (&dependencies[3][0].value).try_into().unwrap();
        self.prefill = (&dependencies[4][0].value).try_into().unwrap();
    }

    fn calculate_state_var_from_dependencies_and_mark_fresh(
        &self,
        state_var: &StateVarMutableView<String>,
    ) {
        let bind_value_to_used_default = self.bind_value_to.get_used_default();

        let value = if *self.sync_immediate_value.get_fresh_value() {
            self.immediate_value.get_fresh_value().clone()
        } else if bind_value_to_used_default {
            if self.essential_value.get_used_default() {
                self.prefill.get_fresh_value().clone()
            } else {
                self.essential_value.get_fresh_value().clone()
            }
        } else {
            self.bind_value_to.get_fresh_value().clone()
        };

        let value_changed = if let Some(old_value) = state_var.try_get_last_value() {
            value != *old_value
        } else {
            true
        };

        if value_changed {
            state_var.set_value(value);
        } else {
            state_var.restore_previous_value();
        }
    }

    fn request_dependencies_to_update_value(
        &self,
        state_var: &StateVarReadOnlyView<String>,
        _is_direct_change_from_renderer: bool,
    ) -> Result<Vec<DependencyValueUpdateRequest>, RequestDependencyUpdateError> {
        let desired_value = state_var.get_requested_value();
        let bind_value_to_used_default = self.bind_value_to.get_used_default();

        if bind_value_to_used_default {
            self.essential_value
                .request_change_value_to(desired_value.clone());
            self.immediate_value
                .request_change_value_to(desired_value.clone());
            self.sync_immediate_value.request_change_value_to(true);

            Ok(vec![
                DependencyValueUpdateRequest {
                    instruction_idx: 0,
                    dependency_idx: 0,
                },
                DependencyValueUpdateRequest {
                    instruction_idx: 1,
                    dependency_idx: 0,
                },
                DependencyValueUpdateRequest {
                    instruction_idx: 2,
                    dependency_idx: 0,
                },
            ])
        } else {
            self.bind_value_to
                .request_change_value_to(desired_value.clone());
            self.sync_immediate_value.request_change_value_to(true);

            Ok(vec![
                DependencyValueUpdateRequest {
                    instruction_idx: 3,
                    dependency_idx: 0,
                },
                DependencyValueUpdateRequest {
                    instruction_idx: 2,
                    dependency_idx: 0,
                },
            ])
        }
    }
}

#[derive(Debug, Default)]
struct ImmediateValueStateVarInterface {
    essential_value: StateVarReadOnlyView<String>,
    sync_immediate_value: StateVarReadOnlyView<bool>,
    bind_value_to: StateVarReadOnlyView<String>,
    prefill: StateVarReadOnlyView<String>,
}

impl StateVarInterface<String> for ImmediateValueStateVarInterface {
    fn return_dependency_instructions(
        &self,
        _extend_source: Option<&ExtendSource>,
        _parameters: &StateVarParameters,
        _state_var_idx: StateVarIdx,
    ) -> Vec<DependencyInstruction> {
        vec![
            DependencyInstruction::Essential,
            TextInputStateVariables::get_sync_immediate_value_dependency_instructions(),
            TextInputStateVariables::get_bind_value_to_dependency_instructions(),
            TextInputStateVariables::get_prefill_dependency_instructions(),
        ]
    }

    fn save_dependencies_for_value_calculation(&mut self, dependencies: &Vec<Vec<Dependency>>) {
        self.essential_value = (&dependencies[0][0].value).try_into().unwrap();
        self.sync_immediate_value = (&dependencies[1][0].value).try_into().unwrap();
        self.bind_value_to = (&dependencies[2][0].value).try_into().unwrap();
        self.prefill = (&dependencies[3][0].value).try_into().unwrap();
    }

    fn calculate_state_var_from_dependencies_and_mark_fresh(
        &self,
        state_var: &StateVarMutableView<String>,
    ) {
        let bind_value_to_used_default = self.bind_value_to.get_used_default();

        let immediate_value =
            if !bind_value_to_used_default && *self.sync_immediate_value.get_fresh_value() {
                self.bind_value_to.get_fresh_value().clone()
            } else if self.essential_value.get_used_default() {
                self.prefill.get_fresh_value().clone()
            } else {
                self.essential_value.get_fresh_value().clone()
            };

        let value_changed = if let Some(old_value) = state_var.try_get_last_value() {
            immediate_value != *old_value
        } else {
            true
        };

        if value_changed {
            state_var.set_value(immediate_value);
        } else {
            state_var.restore_previous_value();
        }
    }

    fn request_dependencies_to_update_value(
        &self,
        state_var: &StateVarReadOnlyView<String>,
        is_direct_change_from_renderer: bool,
    ) -> Result<Vec<DependencyValueUpdateRequest>, RequestDependencyUpdateError> {
        let desired_value = state_var.get_requested_value();

        let mut updates = Vec::with_capacity(2);
        let bind_value_to_used_default = self.bind_value_to.get_used_default();

        self.essential_value
            .request_change_value_to(desired_value.clone());

        updates.push(DependencyValueUpdateRequest {
            instruction_idx: 0,
            dependency_idx: 0,
        });

        if !is_direct_change_from_renderer && !bind_value_to_used_default {
            self.bind_value_to
                .request_change_value_to(desired_value.clone());

            updates.push(DependencyValueUpdateRequest {
                instruction_idx: 2,
                dependency_idx: 0,
            });
        }

        Ok(updates)
    }
}

#[derive(Debug, Default)]
struct SyncImmediateValueStateVarInterface {
    essential_value: StateVarReadOnlyView<bool>,
}

impl StateVarInterface<bool> for SyncImmediateValueStateVarInterface {
    fn return_dependency_instructions(
        &self,
        _extend_source: Option<&ExtendSource>,
        _parameters: &StateVarParameters,
        _state_var_idx: StateVarIdx,
    ) -> Vec<DependencyInstruction> {
        vec![DependencyInstruction::Essential]
    }

    fn save_dependencies_for_value_calculation(&mut self, dependencies: &Vec<Vec<Dependency>>) {
        self.essential_value = (&dependencies[0][0].value).try_into().unwrap();
    }

    fn calculate_state_var_from_dependencies_and_mark_fresh(
        &self,
        state_var: &StateVarMutableView<bool>,
    ) {
        state_var.set_value(*self.essential_value.get_fresh_value());
    }

    fn request_dependencies_to_update_value(
        &self,
        state_var: &StateVarReadOnlyView<bool>,
        _is_direct_change_from_renderer: bool,
    ) -> Result<Vec<DependencyValueUpdateRequest>, RequestDependencyUpdateError> {
        let desired_value = state_var.get_requested_value();

        self.essential_value.request_change_value_to(*desired_value);

        Ok(vec![DependencyValueUpdateRequest {
            instruction_idx: 0,
            dependency_idx: 0,
        }])
    }
}
