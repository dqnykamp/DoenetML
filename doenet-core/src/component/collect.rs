use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::base_definitions::*;

use super::*;


lazy_static!{
    pub static ref MY_STATE_VAR_DEFINITIONS: Vec<(StateVarName, StateVarVariant)> = {
        let mut state_var_definitions = HashMap::new();
        state_var_definitions.insert("hidden", HIDDEN_DEFAULT_DEFINITION());
        state_var_definitions.insert("disabled", DISABLED_DEFAULT_DEFINITION());
        return state_var_definitions
    };
}

fn member_definition(
    values: &HashMap<AttributeName, String>,
) -> &'static ComponentDefinition {
    let component_type = values.get("componentType").unwrap();
    COMPONENT_DEFINITIONS.get_key_value_ignore_case(component_type.as_str()).unwrap().1
}

fn collection_members(
    node: &ComponentNode,
    component_nodes: &HashMap<ComponentName, ComponentNode>,
) -> Vec<CollectionMembersOrCollection> {

    let my_attributes = &node.static_attributes;
    let source = my_attributes.get("source").unwrap();
    let desired_type = my_attributes.get("componentType").unwrap();
    let desired_type = COMPONENT_DEFINITIONS.get_key_value_ignore_case(desired_type).unwrap().0;
    let source_node = component_nodes.get(source).unwrap();
    members_from_children_of_type(component_nodes, source_node, desired_type)
}

lazy_static! {
    pub static ref MY_COMPONENT_DEFINITION: ComponentDefinition = ComponentDefinition {
        component_type: "collect",

        state_var_definitions: &MY_STATE_VAR_DEFINITIONS,

        attribute_names: vec![
            "hidden",
            "disabled",
        ],

        static_attribute_names: vec![
            "source",
            "componentType",
        ],

        should_render_children: true,

        replacement_components: Some(ReplacementComponents::Collection(CollectionDefinition {
            member_definition,
            collection_members,
        })),

        ..Default::default()
    };
}
