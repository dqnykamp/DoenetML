use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::components::prelude::*;
use crate::state::MathExpression;
use crate::state_var_interfaces::math_state_var_interfaces::GeneralMathStateVarInterface;

#[derive(Debug, Default, ComponentNode, RenderedComponentNode)]
pub struct Math {
    pub common: ComponentCommonData,

    pub value_state_var_view: StateVarReadOnlyViewTyped<MathExpression>,

    pub renderer_data: TextRendererData,
}

// TODO: implement RenderedComponentNode rather than deriving it

#[derive(Debug, Default)]
pub struct TextRendererData {
    pub id: ComponentIdx,
    pub value: String,
}

impl ComponentNodeStateVariables for Math {
    fn initialize_state_variables(&mut self) {
        self.common.state_variables = Vec::new();

        ///////////////////////
        // Value state variable
        ///////////////////////

        let value_state_variable = StateVarTyped::new(
            Box::new(GeneralMathStateVarInterface::default()),
            StateVarParameters {
                for_renderer: true,
                name: "value",
                dependency_instruction_hint: Some(DependencyInstruction::Child {
                    match_profiles: vec![ComponentProfile::Text],
                    exclude_if_prefer_profiles: vec![],
                }),
                create_dependency_from_extend_source: true,
                is_primary_state_variable_for_shadowing_extend_source: true,
                is_public: true,
                ..Default::default()
            },
            Default::default(),
        );

        // save a view to field for easy access when create flat dast
        self.value_state_var_view = value_state_variable.create_new_read_only_view();

        // Use the value state variable for fulling the text component profile
        self.common.component_profile_state_variables = vec![ComponentProfileStateVariable::Math(
            value_state_variable.create_new_read_only_view(),
            "value",
        )];
        self.common
            .state_variables
            .push(StateVar::Math(value_state_variable));
    }
}
