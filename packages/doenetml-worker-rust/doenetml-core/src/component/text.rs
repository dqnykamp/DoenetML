use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::component::ComponentProfile;
use crate::dast::{ElementData, FlatDastElement, FlatDastElementContent, Position as DastPosition};
use crate::dependency::{Dependency, DependencyInstruction};
use crate::state::{
    StateVarInterface, StateVarMutableViewTyped, StateVarParameters, StateVarReadOnlyView,
    StateVarReadOnlyViewTyped, StateVarTyped,
};
use crate::{ComponentChild, ComponentIdx, ExtendSource};

use super::{
    ComponentEnum, ComponentNode, ComponentNodeBase, ComponentProfileStateVariables,
    RenderedComponentNode,
};

#[derive(Debug, Default, ComponentNode)]
pub struct Text {
    pub idx: ComponentIdx,
    pub parent: Option<ComponentIdx>,
    pub children: Vec<ComponentChild>,
    pub no_rendered_children: Vec<ComponentChild>,

    pub extend: Option<ExtendSource>,

    // map of descendant names to their indices
    pub descendant_names: HashMap<String, Vec<ComponentIdx>>,

    pub position: Option<DastPosition>,

    pub component_profile_state_variables: Vec<ComponentProfileStateVariables>,

    pub state_variables: TextStateVariables,

    pub rendered_state_variables: RenderedTextStateVariables,

    pub renderer_data: TextRendererData,
}

impl RenderedComponentNode for Text {
    fn to_flat_dast(&self, _: &Vec<Rc<RefCell<ComponentEnum>>>) -> FlatDastElement {
        let text_value = self
            .rendered_state_variables
            .value
            .get_fresh_value()
            .to_string();

        let rendered_children = vec![FlatDastElementContent::Text(text_value)];

        FlatDastElement {
            name: self.get_component_type().to_string(),
            attributes: HashMap::new(),
            children: rendered_children,
            data: Some(ElementData {
                id: self.get_idx(),
                ..Default::default()
            }),
            position: self.get_position().cloned(),
        }
    }
}
#[derive(Debug)]
pub struct TextStateVariables {
    pub text: StateVarTyped<String>,
}

#[derive(Debug)]
pub struct RenderedTextStateVariables {
    pub value: StateVarTyped<String>,
}

#[derive(Debug, Default)]
pub struct TextRendererData {
    pub id: ComponentIdx,
    pub value: String,
}

impl ComponentNodeBase for Text {
    fn initialize_state_variables(&mut self) {
        self.component_profile_state_variables = vec![ComponentProfileStateVariables::Text(
            self.rendered_state_variables
                .value
                .create_new_read_only_view(),
        )]
    }
}

impl Default for RenderedTextStateVariables {
    fn default() -> Self {
        RenderedTextStateVariables {
            value: StateVarTyped::new(
                Box::new(ValueStateVarInterface::new()),
                StateVarParameters {
                    name: "value",
                    ..Default::default()
                },
            ),
        }
    }
}

impl Default for TextStateVariables {
    fn default() -> Self {
        TextStateVariables {
            text: StateVarTyped::new(
                Box::new(TextStateVarInterface::new()),
                StateVarParameters {
                    name: "value",
                    ..Default::default()
                },
            ),
        }
    }
}

#[derive(Debug)]
struct ValueStateVarInterface {
    string_child_values: Vec<StateVarReadOnlyViewTyped<String>>,
}

impl ValueStateVarInterface {
    pub fn new() -> Self {
        ValueStateVarInterface {
            string_child_values: Vec::new(),
        }
    }
}

impl StateVarInterface<String> for ValueStateVarInterface {
    fn return_dependency_instructions(&self) -> Vec<DependencyInstruction> {
        vec![DependencyInstruction::Child {
            match_profiles: vec![ComponentProfile::Text],
            exclude_if_prefer_profiles: vec![],
        }]
    }

    fn save_dependencies_for_value_calculation(
        &mut self,
        dependencies: &Vec<Vec<Dependency>>,
    ) -> () {
        let children = &dependencies[0];

        let mut string_vals = Vec::with_capacity(children.len());

        for Dependency {
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
            .map(|v| v.get_fresh_value().clone())
            .collect();

        state_var.set_value(value);
    }
}

#[derive(Debug)]
struct TextStateVarInterface {
    value_sv: StateVarReadOnlyViewTyped<String>,
}

impl TextStateVarInterface {
    pub fn new() -> Self {
        TextStateVarInterface {
            value_sv: StateVarReadOnlyViewTyped::new(),
        }
    }
}

impl StateVarInterface<String> for TextStateVarInterface {
    fn return_dependency_instructions(&self) -> Vec<DependencyInstruction> {
        vec![DependencyInstruction::StateVar {
            state_var_name: "value",
        }]
    }

    fn save_dependencies_for_value_calculation(
        &mut self,
        dependencies: &Vec<Vec<Dependency>>,
    ) -> () {
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
        state_var.set_value(self.value_sv.get_fresh_value().clone());
    }
}
