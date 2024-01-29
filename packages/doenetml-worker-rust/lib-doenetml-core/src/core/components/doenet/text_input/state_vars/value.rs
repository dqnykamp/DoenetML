use crate::components::prelude::*;

use super::TextInputState;

/// The dependencies of the value state variable of the text input component
#[add_dependency_data]
#[derive(Debug, Default, StateVariableDependencies, StateVariableGraphQueries)]
struct RequiredData {
    essential: StateVarReadOnlyView<String>,
    immediate_value: StateVarReadOnlyView<String>,
    sync_immediate_value: StateVarReadOnlyView<bool>,
    bind_value_to: StateVarReadOnlyView<String>,
    prefill: StateVarReadOnlyView<String>,
}

/// The interface for the value state variable of a text input
#[derive(Debug, Default)]
pub struct ValueStateVarInterface {
    /// The graph queries that indicate how the dependencies of this state variable will be created.
    graph_queries: RequiredDataGraphQueries,

    /// The values of the dependencies created from the graph queries
    dependency_values: RequiredData,
}

impl ValueStateVarInterface {
    pub fn new() -> Self {
        ValueStateVarInterface {
            ..Default::default()
        }
    }
}

impl From<ValueStateVarInterface> for StateVar<String> {
    fn from(interface: ValueStateVarInterface) -> Self {
        StateVar::new(Box::new(interface), Default::default())
    }
}

impl StateVarInterface<String> for ValueStateVarInterface {
    fn return_graph_queries(
        &mut self,
        _extending: Option<ExtendSource>,
        _state_var_idx: StateVarIdx,
    ) -> Vec<GraphQuery> {
        self.graph_queries = RequiredDataGraphQueries {
            essential: Some(GraphQuery::Essential),
            immediate_value: Some(TextInputState::get_immediate_value_graph_queries()),
            sync_immediate_value: Some(
                TextInputState::get_sync_immediate_value_graph_queries(),
            ),
            bind_value_to: Some(TextInputState::get_bind_value_to_graph_queries()),
            prefill: Some(TextInputState::get_prefill_graph_queries()),
        };

        (&self.graph_queries).into()
    }

    fn save_dependencies(&mut self, dependencies: &Vec<DependenciesCreatedForInstruction>) {
        self.dependency_values = dependencies.try_into().unwrap();
    }

    fn calculate_state_var_from_dependencies(&self) -> StateVarCalcResult<String> {
        let value = if *self.dependency_values.sync_immediate_value.get() {
            self.dependency_values.immediate_value.get().clone()
        } else if self.dependency_values.bind_value_to.came_from_default() {
            if self.dependency_values.essential.came_from_default() {
                self.dependency_values.prefill.get().clone()
            } else {
                self.dependency_values.essential.get().clone()
            }
        } else {
            self.dependency_values.bind_value_to.get().clone()
        };

        StateVarCalcResult::Calculated(value)
    }

    fn request_dependency_updates(
        &mut self,
        state_var: &StateVarReadOnlyView<String>,
        _is_direct_change_from_renderer: bool,
    ) -> Result<Vec<DependencyValueUpdateRequest>, RequestDependencyUpdateError> {
        let requested_value = state_var.get_requested_value();

        let dependency_values = &mut self.dependency_values;

        let bind_value_to_came_from_default = dependency_values.bind_value_to.came_from_default();

        if bind_value_to_came_from_default {
            dependency_values
                .essential
                .queue_update(requested_value.clone());
            dependency_values
                .immediate_value
                .queue_update(requested_value.clone());
            dependency_values.sync_immediate_value.queue_update(true);
        } else {
            dependency_values
                .bind_value_to
                .queue_update(requested_value.clone());
            dependency_values.sync_immediate_value.queue_update(true);
        }

        Ok(dependency_values.return_queued_updates())
    }
}
