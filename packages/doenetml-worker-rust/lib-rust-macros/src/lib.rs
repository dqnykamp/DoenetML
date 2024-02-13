extern crate proc_macro2;

use component_attributes::attribute_state_var_derive;
use component_node::{
    component_actions_derive, component_attributes_derive, component_node_derive,
    rendered_children_derive, rendered_state_derive,
};
use component_state_methods::{
    add_dependency_data_impl, component_state_derive, state_variable_data_queries_derive,
    state_variable_dependencies_derive,
};
use proc_macro::TokenStream;
use state_var_methods::{
    into_state_var_enum_refs_derive, state_var_methods_derive, state_var_methods_mut_derive,
    state_var_mutable_view_methods_derive, state_var_read_only_view_methods_derive,
};

mod component_attributes;
mod component_node;
mod component_state_methods;
mod state_var_methods;
mod util;

/// Use on the Enum that lists the attributes of your component.
/// Every variant should be annotated with a `#[attribute(...)]` annotation.
///
/// The options available to `attribute(...)` are:
///  - `state_var` - The state var that will be created for this attribute. The state var **must**
///    have a `new_from_attribute(attr_name, default_value)` method.
/// - `default` - The default value for the attribute.
/// - `explicit_type` (optional) - The type of the state var that will be created for the attribute.
///    For example, if you expect a `StateVar<bool>` to be created, then `explicit_type=bool`.
///    This can be inferred if the value of `state_var` is a commonly-recognized state var type.
///
/// Example:
/// ```ignore
/// #[derive(Debug, AttributeStateVar)]
/// pub enum MyComponentAttributes {
///   #[attribute(state_var = BooleanStateVar, default = false)]
///   Foo,
///   #[attribute(state_var = CustomStateVar, default = Vec::new(), explicit_type = Vec<String>)]
///   Bar,
/// }
/// ```
///
/// Note: Enum variants are specified in PascalCase, but attribute names are always converted to camelCase.
#[proc_macro_derive(AttributeStateVar, attributes(attribute))]
pub fn attribute_state_var_derive_wrapper(input: TokenStream) -> TokenStream {
    attribute_state_var_derive(input)
}

/// Derive functions needed to be initialized as a component.
///
/// Options can be set using the `#[component(...)]` attribute.
///
/// ## Options
///  - `#[component(ref_transmutes_to = "...")]` - The name of the component that should be used to
///    render a direct reference to this component. E.g. in `<textInput name="i"/>$i`, the `$i`
///    should be rendered as a `<text extend="$i"/>` rather than a `<textInput extend="$i"/>`.
///    Setting `#[component(ref_transmutes_to = "Text")]` will do this.
#[proc_macro_derive(ComponentNode, attributes(component))]
pub fn component_node_derive_wrapper(input: TokenStream) -> TokenStream {
    component_node_derive(input)
}

#[proc_macro_derive(
    RenderedChildren,
    attributes(pass_through_children, no_rendered_children)
)]
pub fn rendered_children_derive_wrapper(input: TokenStream) -> TokenStream {
    rendered_children_derive(input)
}

#[proc_macro_derive(ComponentAttributes)]
pub fn component_attributes_derive_wrapper(input: TokenStream) -> TokenStream {
    component_attributes_derive(input)
}

#[proc_macro_derive(ComponentActions)]
pub fn component_actions_derive_wrapper(input: TokenStream) -> TokenStream {
    component_actions_derive(input)
}

#[proc_macro_derive(StateVarMethods)]
pub fn state_var_methods_derive_wrapper(input: TokenStream) -> TokenStream {
    state_var_methods_derive(input)
}

#[proc_macro_derive(StateVarMethodsMut)]
pub fn state_var_methods_mut_derive_wrapper(input: TokenStream) -> TokenStream {
    state_var_methods_mut_derive(input)
}

#[proc_macro_derive(StateVarMutableViewMethods)]
pub fn state_var_mutable_view_methods_derive_wrapper(input: TokenStream) -> TokenStream {
    state_var_mutable_view_methods_derive(input)
}

#[proc_macro_derive(StateVarViewMethods)]
pub fn state_var_read_only_view_methods_derive_wrapper(input: TokenStream) -> TokenStream {
    state_var_read_only_view_methods_derive(input)
}

#[proc_macro_derive(FromStateVarIntoStateVarEnumRefs)]
pub fn into_state_var_enum_refs_derive_wrapper(input: TokenStream) -> TokenStream {
    into_state_var_enum_refs_derive(input)
}

/// Derives an implementation of the `ComponentState` trait and auxillary functions.
///
/// The derive macro is designed to be applied to the struct defining the DoenetML component itself
/// as well as the struct defining the component's state variables.
///
/// The macro assumes that the component struct has a field `state` that contains
/// the component state variables struct.
///
/// The macro assumes all fields of the component state variables struct are state variables `StateVar<T>`.
///
/// The following attributes specify properties of state variables in the component state variables structure.
/// - #\[for_renderer\]
///
///   Designate the state variable as one that will be sent to the renderer.
///   If `for_renderer` is set, the value of the state variable will be added to the `RenderedState`
///   structure for the component that is sent to the renderer
///
/// - #\[is_public\]
///
///   Designate that the state variable is public, in the sense that it can be
///   referenced by a macro in the document.
///
/// - #\[component_profile_state_variable\]
///
///   Designate that the state variable can be used to satisfy the [`ComponentProfile`]
///   that corresponds to the state variable's type.
///
///   If a parent has a `Child` or `AttributeChild` data query, it will request
///   a particular profile type, and this state variable could be returned.
///
///   Currently, the `component_profile state_variables` does not have a mechanism for specifying
///   priority in case more than one state variable matches what a parent is requesting.
///   If there is more than one match, the state variable that appears first in the ordering of
///   the fields of the struct will be selected.
#[proc_macro_derive(
    ComponentState,
    attributes(for_renderer, is_public, component_profile_state_variable)
)]
pub fn component_state_derive_wrapper(input: TokenStream) -> TokenStream {
    component_state_derive(input)
}

/// Derives the RenderedState enum
///
/// This derive macro is designed to be applied to the `ComponentEnum` listing all component types.
///
/// It creates a parallel `RenderedState` enum whose variant names and field types
/// are based on the variant names from the `ComponentEnum`.
///
/// The variant names append `State` to the variant from `ComponentEnum`.
///
/// The field types prepend `Rendered` to the variant names. These structures
/// are created by the `ComponentState` macro applied
/// to the components state variable struct.
///
/// For example, the component type `Text` has a `TextState` struct,
/// and the `ComponentState` macro creates the `RenderedTextState` struct.
/// Since the `ComponentEnum` has a `Text` variant, the `RenderedState` macros
/// adds the variant `TextState(RenderedTextState)`
/// to the `RenderedState` enum.
#[proc_macro_derive(RenderedState)]
pub fn rendered_state_derive_wrapper(input: TokenStream) -> TokenStream {
    rendered_state_derive(input)
}

#[proc_macro_derive(StateVariableDependencies)]
pub fn state_variable_dependencies_derive_wrapper(input: TokenStream) -> TokenStream {
    state_variable_dependencies_derive(input)
}

#[proc_macro_derive(StateVariableDataQueries)]
pub fn state_variable_data_queries_derive_wrapper(input: TokenStream) -> TokenStream {
    state_variable_data_queries_derive(input)
}

#[proc_macro_attribute]
pub fn add_dependency_data(attr: TokenStream, item: TokenStream) -> TokenStream {
    add_dependency_data_impl(attr, item)
}
