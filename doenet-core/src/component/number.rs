use lazy_static::lazy_static;

use evalexpr::{ContextWithMutableVariables, HashMapContext, Operator};

use crate::math_expression::MathExpression;
use crate::state::{
    StateVarInterface, StateVarMutableViewTyped, StateVarParameters, StateVarReadOnlyView,
    StateVarReadOnlyViewTyped, StateVarTyped, UpdatesRequested,
};
use crate::state_variables::*;
use crate::utils::log;
// use crate::base_definitions::*;

use super::*;

use crate::ComponentProfile;

#[derive(Debug)]
struct Value {
    single_numeric_child: Option<StateVarReadOnlyViewTyped<f64>>,
    math_expression: Option<StateVarReadOnlyViewTyped<MathExpression>>,
    numerical_children: Vec<StateVarReadOnlyViewTyped<f64>>,
    math_expression_is_single_variable: bool,
}

impl Value {
    pub fn new() -> Self {
        Value {
            single_numeric_child: None,
            math_expression: None,
            numerical_children: Vec::new(),
            math_expression_is_single_variable: false,
        }
    }
}

impl StateVarInterface<f64> for Value {
    fn return_dependency_instructions(&self) -> Vec<DependencyInstruction> {
        vec![DependencyInstruction::Child {
            desired_profiles: vec![ComponentProfile::Number],
            parse_into_expression: true,
        }]
    }

    fn set_dependencies(&mut self, dependencies: &Vec<Vec<DependencyValue>>) -> () {
        // log!("deps of num value: {:#?}", dependencies);

        let children = &dependencies[0];

        if children[0].source
            != (DependencySource::Essential {
                value_type: "mathExpression",
            })
        {
            // TODO: what's the situation where wouldn't get a math expression?

            if let StateVarReadOnlyView::Number(val) = &children[0].value {
                self.single_numeric_child = Some(val.create_new_read_only_view());
            } else {
                panic!(
                    "A number had a single child that wasn't a number: {:?}",
                    children[0].value
                );
            }

            if children.len() > 1 {
                panic!("Number's children should have been parsed into an expression");
            }
        } else {
            if let StateVarReadOnlyView::MathExpr(val) = &children[0].value {
                self.math_expression = Some(val.create_new_read_only_view());
            } else {
                panic!("Number's children should have been parsed into an expression");
            }

            let mut numerical_children = Vec::with_capacity(children.len() - 1);

            for child in children.iter().skip(1) {
                if let StateVarReadOnlyView::Number(val) = &child.value {
                    numerical_children.push(val.create_new_read_only_view());
                } else {
                    panic!("Number should have number children");
                }
            }

            if numerical_children.len()
                != self
                    .math_expression
                    .as_ref()
                    .unwrap()
                    .get_value_assuming_fresh()
                    .external_variables_count
            {
                panic!("Number not parsed correctly");
            }

            let mut math_expression_is_single_variable = false;

            if numerical_children.len() == 1 {
                let expression = &self
                    .math_expression
                    .as_ref()
                    .unwrap()
                    .get_value_assuming_fresh();

                let tree = &expression.tree;

                if tree.children().len() == 1 {
                    let child = &tree.children()[0];
                    if child.children().is_empty()
                        && matches!(child.operator(), Operator::VariableIdentifierRead { .. })
                    {
                        math_expression_is_single_variable = true;
                    }
                }
            }

            self.math_expression_is_single_variable = math_expression_is_single_variable;

            self.numerical_children = numerical_children;
        }
    }

    fn calculate_state_var_from_dependencies(
        &self,
        state_var: &StateVarMutableViewTyped<f64>,
    ) -> () {
        if let Some(single_child) = &self.single_numeric_child {
            state_var.set_value(*single_child.get_value_assuming_fresh())
        } else {
            let expression = &self
                .math_expression
                .as_ref()
                .unwrap()
                .get_value_assuming_fresh();

            let mut context = HashMapContext::new();

            for (id, value) in self.numerical_children.iter().enumerate() {
                let variable_num = *value.get_value_assuming_fresh();

                let name = format!("{}{}", expression.variable_prefix, id);
                context.set_value(name, variable_num.into()).unwrap();
            }

            let num = if expression.tree.operator() == &Operator::RootNode
                && expression.tree.children().is_empty()
            {
                // Empty expression, set to 0
                0.0
            } else {
                expression
                    .tree
                    .eval_number_with_context(&context)
                    .unwrap_or(f64::NAN)
            };

            state_var.set_value(num);
        }
    }

    fn request_dependencies_to_update_value(
        &self,
        state_var: &StateVarReadOnlyViewTyped<f64>,
    ) -> Result<Vec<UpdatesRequested>, ()> {
        let desired_value = state_var.get_requested_value();

        if let Some(single_child) = &self.single_numeric_child {
            single_child.request_value(*desired_value);

            Ok(vec![UpdatesRequested {
                instruction_ind: 0,
                dependency_ind: 0,
            }])
        } else if self.numerical_children.len() == 0 {
            // have a constant math expression

            self.math_expression
                .as_ref()
                .unwrap()
                .request_value(MathExpression::from(*desired_value));

            Ok(vec![UpdatesRequested {
                instruction_ind: 0,
                dependency_ind: 0,
            }])
        } else if self.math_expression_is_single_variable {
            self.numerical_children[0].request_value(*desired_value);

            Ok(vec![UpdatesRequested {
                instruction_ind: 0,
                dependency_ind: 1,
            }])
        } else {
            // have not implemented other cases
            Err(())
        }
    }
}

#[derive(Debug)]
struct Text {
    value_sv: StateVarReadOnlyViewTyped<f64>,
}

impl Text {
    pub fn new() -> Self {
        Text {
            value_sv: StateVarReadOnlyViewTyped::new(),
        }
    }
}

impl StateVarInterface<String> for Text {
    fn return_dependency_instructions(&self) -> Vec<DependencyInstruction> {
        vec![DependencyInstruction::StateVar {
            component_name: None,
            state_var_name: "value",
        }]
    }

    fn set_dependencies(&mut self, dependencies: &Vec<Vec<DependencyValue>>) -> () {
        let dep_val = &dependencies[0][0].value;

        if let StateVarReadOnlyView::Number(number_val) = dep_val {
            self.value_sv = number_val.create_new_read_only_view();
        } else {
            panic!("Something wrong with text sv of number");
        }
    }

    fn calculate_state_var_from_dependencies(
        &self,
        state_var: &StateVarMutableViewTyped<String>,
    ) -> () {
        state_var.set_value(self.value_sv.get_value_assuming_fresh().to_string());
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
        &self,
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
        &self,
        state_var: &StateVarMutableViewTyped<bool>,
    ) -> () {
        state_var.set_value(false);
    }
}

#[derive(Debug)]
struct Fixed {}

impl Fixed {
    pub fn new() -> Self {
        Fixed {}
    }
}

impl StateVarInterface<bool> for Fixed {
    fn calculate_state_var_from_dependencies(
        &self,
        state_var: &StateVarMutableViewTyped<bool>,
    ) -> () {
        state_var.set_value(false);
    }
}

lazy_static! {
    pub static ref GENERATE_STATE_VARS: fn() -> Vec<StateVar> = || {
        vec![
            StateVar::Number(StateVarTyped::new(
                Box::new(Value::new()),
                StateVarParameters {
                    name: "value",
                    ..Default::default()
                },
            )),
            StateVar::String(StateVarTyped::new(
                Box::new(Text::new()),
                StateVarParameters {
                    name: "text",
                    for_renderer: true,
                    ..Default::default()
                },
            )),
            StateVar::Boolean(StateVarTyped::new(
                Box::new(Hidden::new()),
                StateVarParameters {
                    name: "hidden",
                    for_renderer: true,
                    ..Default::default()
                },
            )),
            StateVar::Boolean(StateVarTyped::new(
                Box::new(Disabled::new()),
                StateVarParameters {
                    name: "disabled",
                    for_renderer: true,
                    ..Default::default()
                },
            )),
            StateVar::Boolean(StateVarTyped::new(
                Box::new(Fixed::new()),
                StateVarParameters {
                    name: "fixed",
                    for_renderer: true,
                    ..Default::default()
                },
            )),
        ]
    };
    pub static ref STATE_VARIABLES_NAMES_IN_ORDER: Vec<&'static str> = GENERATE_STATE_VARS()
        .iter()
        .map(|sv| sv.get_name())
        .collect();
    pub static ref MY_COMPONENT_DEFINITION: ComponentDefinition = ComponentDefinition {
        component_type: "number",

        state_var_index_map: STATE_VARIABLES_NAMES_IN_ORDER
            .iter()
            .enumerate()
            .map(|(i, v)| (*v, i))
            .collect(),

        state_var_names: STATE_VARIABLES_NAMES_IN_ORDER.to_vec(),

        state_var_component_types: GENERATE_STATE_VARS()
            .iter()
            .map(|sv| sv.get_default_component_type())
            .collect(),

        generate_state_vars: *GENERATE_STATE_VARS,

        attribute_names: vec!["hide", "disabled",],

        primary_input_state_var_ind: Some(0),

        component_profiles: vec![
            (ComponentProfile::Number, "value"),
            (ComponentProfile::Text, "text"),
        ],

        valid_children_profiles: ValidChildTypes::ValidProfiles(vec![ComponentProfile::Number]),

        ..Default::default()
    };
}
