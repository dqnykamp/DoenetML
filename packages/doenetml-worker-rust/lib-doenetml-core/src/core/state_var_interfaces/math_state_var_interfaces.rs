use crate::{
    components::prelude::{
        Dependency, DependencyInstruction, DependencyValueUpdateRequest, StateVarInterface,
        StateVarMutableViewTyped, StateVarParameters, StateVarReadOnlyView,
        StateVarReadOnlyViewTyped,
    },
    state::MathExpression,
    ExtendSource,
};

use super::common::create_dependency_instruction_from_extend_source;

#[derive(Debug)]
enum MathOrTextValues {
    Math(StateVarReadOnlyViewTyped<MathExpression>),
    Text(StateVarReadOnlyViewTyped<String>),
}

impl Default for MathOrTextValues {
    fn default() -> Self {
        MathOrTextValues::Text(StateVarReadOnlyViewTyped::default())
    }
}

#[derive(Debug, Default)]
pub struct GeneralMathStateVarInterface {
    math_text_dependency_values: Vec<MathOrTextValues>,
}

impl StateVarInterface<MathExpression> for GeneralMathStateVarInterface {
    fn return_dependency_instructions(
        &self,
        extend_source: Option<&ExtendSource>,
        parameters: &StateVarParameters,
    ) -> Vec<DependencyInstruction> {
        let mut dep_instructs: Vec<DependencyInstruction> = Vec::with_capacity(2);

        if parameters.create_dependency_from_extend_source {
            if let Some(dep_inst) =
                create_dependency_instruction_from_extend_source(extend_source, parameters)
            {
                dep_instructs.push(dep_inst)
            }
        }

        if let Some(dependency_instruction) = &parameters.dependency_instruction_hint {
            dep_instructs.push(dependency_instruction.clone());
        }

        dep_instructs
    }

    fn save_dependencies_for_value_calculation(
        &mut self,
        dependencies: &Vec<Vec<Dependency>>,
    ) -> () {
        let num_dependencies = dependencies.iter().fold(0, |a, c| a + c.len());

        let mut math_or_text_vals = Vec::with_capacity(num_dependencies);

        for instruction in dependencies.iter() {
            for Dependency {
                value: dep_value, ..
            } in instruction.iter()
            {
                match dep_value {
                    StateVarReadOnlyView::Math(dep_math_value) => math_or_text_vals.push(
                        MathOrTextValues::Math(dep_math_value.create_new_read_only_view()),
                    ),
                    StateVarReadOnlyView::String(dep_string_value) => math_or_text_vals.push(
                        MathOrTextValues::Text(dep_string_value.create_new_read_only_view()),
                    ),
                    _ => {
                        panic!(
                            "Got a non-math or text value for a dependency for a GeneralMathStateVarInterface"
                        );
                    }
                }
            }
        }

        self.math_text_dependency_values = math_or_text_vals;
    }

    fn calculate_state_var_from_dependencies_and_mark_fresh(
        &self,
        state_var: &StateVarMutableViewTyped<MathExpression>,
    ) -> () {
        // TODO: implement an algorithm that concatenates the strings along with codes for the math expressions,
        // parses the resulting string with math-expressions parser (text or latex, depending on parameters that don't have yet)
        // then substitutes the values of the math components into the variables corresponding to the codes.

        // Setting value to 0 for now!!!
        state_var.set_value(MathExpression(0.0));
    }
}
