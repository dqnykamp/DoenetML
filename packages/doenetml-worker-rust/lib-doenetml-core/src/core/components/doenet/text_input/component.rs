use serde::{Deserialize, Serialize};
use strum::VariantNames;
use strum_macros::{EnumDiscriminants, EnumVariantNames, IntoStaticStr};

use crate::{attribute::{attribute_type, AttributeValue}, components::prelude::*};

use super::TextInputState;

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

#[derive(Debug, Deserialize, EnumVariantNames, EnumDiscriminants)]
#[serde(rename_all = "camelCase")]
#[strum_discriminants(derive(Serialize, IntoStaticStr))]
#[strum_discriminants(serde(rename_all = "camelCase"))]
pub enum TextInputAttribute {
    BindValueTo,
    /// Whether the `<textInput>` should be hidden.
    Hide(AttributeValue<attribute_type::Boolean>),
    /// Whether the `<textInput>` should be editable.
    Disabled(AttributeValue<attribute_type::Boolean>),
    /// The content that should prefill the `<textInput>`, giving it a default value before a user has interacted with the input.
    Prefill(AttributeValue<attribute_type::String>),
}

/// Definition of the `<textInput>` DoenetML component
#[derive(Debug, Default, ComponentNode, ComponentState)]
pub struct TextInput {
    /// The common component data needed to derive the `ComponentNode` trait
    pub common: ComponentCommonData,

    /// The state variables that underlie the `<textInput>` component.
    pub state: TextInputState,
}

impl RenderedChildren for TextInput {
    fn get_rendered_children(&self) -> &Vec<ComponentPointerTextOrMacro> {
        static EMPTY_VECTOR: Vec<ComponentPointerTextOrMacro> = vec![];
        &EMPTY_VECTOR
    }
}

impl ComponentAttributes for TextInput {
    fn get_attribute_names(&self) -> Vec<AttributeName> {
        TextInputAttribute::VARIANTS.into()
    }
}

impl ComponentActions for TextInput {
    fn get_action_names(&self) -> Vec<String> {
        TextInputAction::VARIANTS
            .iter()
            .map(|s| s.to_string())
            .collect()
    }

    fn on_action(
        &self,
        action: ActionsEnum,
        resolve_and_retrieve_state_var: &mut dyn FnMut(StateVarIdx) -> StateVarValue,
    ) -> Result<Vec<UpdateFromAction>, String> {
        // The type of `action` should have already been verified, so an
        // error here is a programming logic error, not an API error.
        let action: TextInputAction = action.try_into()?;

        match action {
            TextInputAction::UpdateImmediateValue(ActionBody { args }) => Ok(vec![
                TextInputState::update_immediate_value_from_action(args.text),
                TextInputState::update_sync_immediate_value_from_action(false),
            ]),

            TextInputAction::UpdateValue => {
                let new_val = resolve_and_retrieve_state_var(
                    TextInputState::get_immediate_value_state_variable_index(),
                );

                Ok(vec![TextInputState::update_value_from_action(
                    new_val.try_into().unwrap(),
                )])
            }
        }
    }
}
