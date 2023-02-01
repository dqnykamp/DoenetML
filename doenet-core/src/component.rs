use crate::state::StateVar;
use crate::state_variables::*;
use crate::{ComponentInd, ComponentState};
use enum_as_inner::EnumAsInner;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::{self, Debug};

use crate::lazy_static;

pub mod document;
pub mod number;
pub mod text;
pub mod text_input;
// pub mod boolean;
// pub mod p;
pub mod number_input;
// pub mod boolean_input;
// pub mod sequence;
// pub mod graph;
// pub mod point;
// pub mod collect;
// pub mod section;
// pub mod line;
// pub mod map;
// pub mod template;
// pub mod sources;
// pub mod conditional_content;
// pub mod case;
pub mod _error;

lazy_static! {
    pub static ref COMPONENT_DEFINITIONS: HashMap<ComponentType, &'static ComponentDefinition> = {

        let defs: Vec<&'static ComponentDefinition> = vec![
            &crate::text               ::MY_COMPONENT_DEFINITION,
            &crate::number             ::MY_COMPONENT_DEFINITION,
            &crate::text_input         ::MY_COMPONENT_DEFINITION,
            &crate::document           ::MY_COMPONENT_DEFINITION,
            // &crate::boolean            ::MY_COMPONENT_DEFINITION,
            // &crate::p                  ::MY_COMPONENT_DEFINITION,
            &crate::number_input       ::MY_COMPONENT_DEFINITION,
            // &crate::boolean_input      ::MY_COMPONENT_DEFINITION,
            // &crate::sequence           ::MY_COMPONENT_DEFINITION,
            // &crate::graph              ::MY_COMPONENT_DEFINITION,
            // &crate::point              ::MY_COMPONENT_DEFINITION,
            // &crate::collect            ::MY_COMPONENT_DEFINITION,
            // &crate::section            ::MY_COMPONENT_DEFINITION,
            // &crate::line               ::MY_COMPONENT_DEFINITION,
            // &crate::map                ::MY_COMPONENT_DEFINITION,
            // &crate::template           ::MY_COMPONENT_DEFINITION,
            // &crate::sources            ::MY_COMPONENT_DEFINITION,
            // &crate::conditional_content::MY_COMPONENT_DEFINITION,
            // &crate::case               ::MY_COMPONENT_DEFINITION,
            &crate::_error               ::MY_COMPONENT_DEFINITION,
        ];

        defs.into_iter().map(|def| (def.component_type, def)).collect()
    };
}

/// camelCase
pub type ComponentType = &'static str;

/// camelCase
pub type AttributeName = &'static str;

/// How a CopySource affects its component
///
/// Component:
/// - This only works if the source component is the same type.
/// - In a `ChildUpdateInstruction`, the source's children are included before
///   including its own. So without its own children, many of component'struct
///   state variables become exactly the same as the source's.
/// - For the renderer, these 'inherited' children are copied but need
///   a different name supplied by core's `aliases` HashMap. When the renderer
///   sends an action that involves an alias, it is redirected
///   to the source's existing child.
/// - An `EssentialDependencyInstruction` will point to the source's
///   essential variables. The component does not have essential data for itself.s
/// - Attributes are inherited from the source but are overridden when specified.
///
/// StateVar:
/// - Three `StateVariableDefinition` functions are overridden for the component's
///   'primary input' state variable (usually called 'value'):
///   - `return_dependency_instructions`
///   - `determine_state_var_from_dependencies`
///   - `request_dependencies_to_update_value`
///   These overrides cause the primary variable to depend on and copy the source.
/// - If the component type has no primary input, a StateVar CopySource will not work.
#[derive(Debug, Clone)]
pub enum CopySource {
    Component(ComponentInd),
    StateVar(ComponentState),
}

#[derive(Clone, PartialEq, Eq, Debug, Hash, Serialize)]
pub enum ComponentProfile {
    Text,
    Number,
    Boolean,
    Math,
    // Graphical,
}

/// The definition of a component type.
pub struct ComponentDefinition {
    // pub state_var_definitions: &'static Vec<(StateVarName, StateVarVariant)>,
    pub state_var_index_map: HashMap<StateVarName, usize>,

    pub state_var_names: Vec<&'static str>,

    pub state_var_component_types: Vec<&'static str>,

    pub generate_state_vars: fn() -> Vec<StateVar>,

    /// An ordered list of which profiles this component fulfills, along with the name of the
    /// state variable that fulfills it.
    /// The first element in the list is the profile most preferred by this component
    pub component_profiles: Vec<(ComponentProfile, StateVarName)>,

    pub valid_children_profiles: ValidChildTypes,

    pub attribute_names: Vec<AttributeName>,

    pub static_attribute_names: Vec<AttributeName>,

    pub array_aliases: HashMap<&'static str, StateVarName>,

    /// Process an action and return the state variables to change.
    /// The update requests will be processed in the order returned.
    pub on_action: for<'a> fn(
        action_name: &str,
        args: HashMap<String, Vec<StateVarValue>>,
        resolve_and_retrieve_state_var: &'a mut dyn FnMut(usize) -> StateVarValue,
    ) -> Vec<(usize, StateVarValue)>,

    pub should_render_children: bool,

    pub display_errors: bool,

    /// These have to match `on_action` and with what the renderers have
    pub action_names: fn() -> Vec<&'static str>,

    /// The primary input is a state variable, except it gets overridden if
    /// the component is being copied from another state var
    pub primary_input_state_var_ind: Option<usize>,

    pub renderer_type: RendererType,

    /// If specified, the component's parent will treat this as multiple components.
    pub replacement_components: Option<ReplacementComponents>,

    pub component_type: ComponentType,
}

pub enum ReplacementComponents {
    // Unlike the previous, Children cannot form a component group
    // because they may be of different types.
    Children,
}

impl ComponentDefinition {
    /// Returns component definition of members, or itself if there are no replacement components
    /// Pass the static_attributes as a parameter
    pub fn definition_as_replacement_children(
        &self,
        _static_attributes: &HashMap<AttributeName, String>,
    ) -> Option<&ComponentDefinition> {
        match &self.replacement_components {
            Some(ReplacementComponents::Children) => None,
            None => Some(self),
        }
    }

    pub fn component_profile_match(
        &self,
        desired_profiles: &Vec<ComponentProfile>,
    ) -> Option<StateVarName> {
        for profile in self.component_profiles.iter() {
            if desired_profiles.contains(&profile.0) {
                let profile_state_var = profile.1;

                return Some(profile_state_var);
            }
        }
        None
    }

    pub fn get_renderer_type_as_str(&self) -> &'static str {
        match &self.renderer_type {
            RendererType::Myself => self.component_type,
            RendererType::Special { component_type, .. } => component_type,
        }
    }
}

// lazy_static! {
//     static ref EMPTY_STATE_VARS: Vec<(StateVarName, StateVarVariant)> = {
//         Vec::new()
//     };
// }

fn empty_state_vars() -> Vec<StateVar> {
    Vec::new()
}

impl Default for ComponentDefinition {
    fn default() -> Self {
        ComponentDefinition {
            // state_var_definitions: &EMPTY_STATE_VARS,
            state_var_index_map: HashMap::new(),
            state_var_names: Vec::new(),
            state_var_component_types: Vec::new(),
            generate_state_vars: empty_state_vars,
            attribute_names: Vec::new(),
            static_attribute_names: Vec::new(),
            array_aliases: HashMap::new(),
            should_render_children: false,
            display_errors: false,
            renderer_type: RendererType::Myself,
            primary_input_state_var_ind: None,
            component_profiles: vec![],
            valid_children_profiles: ValidChildTypes::ValidProfiles(vec![]),
            action_names: || Vec::new(),
            on_action: |_, _, _| vec![],
            replacement_components: None,
            component_type: "default_invalid",
        }
    }
}

impl Debug for ComponentDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ComponentDefinition")
            // .field("state_var_definitions", &self.state_var_definitions)
            .field("should_render_children", &self.should_render_children)
            .field("component_type", &self.component_type)
            .field("renderer_type", &self.renderer_type)
            .field(
                "primary_input_state_var_ind",
                &self.primary_input_state_var_ind,
            )
            .field("primary_output_traits", &self.component_profiles)
            .field("action_names", &(self.action_names)())
            .finish()
    }
}

pub type ComponentChild = ObjectName;

/// An object refers to either a component or a string child.
#[derive(Debug, PartialEq, Eq, Hash, Clone, serde::Serialize, EnumAsInner)]
pub enum ObjectName {
    Component(ComponentInd),
    String(String),
}

pub enum ValidChildTypes {
    AllComponents,
    /// All children need to match one of the given profiles
    ValidProfiles(Vec<ComponentProfile>),
}

#[derive(Debug)]
pub enum RendererType {
    Myself,
    Special {
        component_type: &'static str,
        state_var_aliases: HashMap<StateVarName, StateVarName>,
    },
    // DoNotRender,
}

pub trait KeyValueIgnoreCase<K, V> {
    fn get_key_value_ignore_case<'a>(&'a self, key: &str) -> Option<(&'a K, &'a V)>;
}

impl<K, V> KeyValueIgnoreCase<K, V> for HashMap<K, V>
where
    K: ToString + std::cmp::Eq + std::hash::Hash,
{
    fn get_key_value_ignore_case<'a>(&'a self, key: &str) -> Option<(&'a K, &'a V)> {
        let lowercase_to_normalized: HashMap<String, &K> = self
            .keys()
            .into_iter()
            .map(|k| (k.to_string().to_lowercase(), k))
            .collect();

        lowercase_to_normalized
            .get(&key.to_string().to_lowercase())
            .and_then(|k| self.get_key_value(k))
    }
}
