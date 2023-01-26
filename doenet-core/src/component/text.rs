use super::*;
// use crate::base_definitions::*;

use crate::state::{StateVarTyped, StateVarReadOnlyTyped, StateVarReadOnly};
use crate::state_variables::*;

use crate::ComponentProfile;

use lazy_static::lazy_static;

#[derive(Debug)]
struct Value {
    val: StateVarTyped<String>,
    string_child_values: Vec<StateVarReadOnlyTyped<String>>,
}

impl StateVariable<String> for Value {
    fn return_dependency_instructions(&self) -> Vec<DependencyInstruction> {
        vec![DependencyInstruction::Child {
            desired_profiles: vec![ComponentProfile::Text],
            parse_into_expression: false,
        }]
    }

    fn set_dependencies(&mut self, dependencies: &Vec<Vec<DependencyValue>>) -> () {

        let children = &dependencies[0];

        let mut string_vals = Vec::with_capacity(children.len());

        for DependencyValue{value: child_value, ..} in children.iter() {
            if let StateVarReadOnly::String(child_string_value) = child_value {
                string_vals.push(child_string_value.create_new_read_only_view() )
            } else {
                panic!("Got a non-string value when asked for a Text component profile");
            }
        }

        self.string_child_values = string_vals;

    }

    fn calculate_state_var_from_dependencies(&self) -> () {
        
        // TODO: can we implement this without cloning the inner value?
        let value: String = self.string_child_values.iter().map(|v| v.get_value_assuming_fresh().clone()).collect();

        self.val.set_value(value);
    }

    fn get_value_assuming_fresh(&self) -> String {
        self.val.get_value_assuming_fresh().clone()
    }

    fn create_new_mutable_view(&self) -> StateVarTyped<String> {
        self.val.create_new_mutable_view()
    }

    fn get_name(&self) -> &'static str {
        "value"
    }

}



#[derive(Debug)]
struct Text {
    val: StateVarTyped<String>,
    value_sv: StateVarReadOnlyTyped<String>
}

impl StateVariable<String> for Text {
    fn return_dependency_instructions(&self) -> Vec<DependencyInstruction> {
        vec![DependencyInstruction::StateVar {
            component_name: None,
            state_var_name: "value"
        }]
    }

    fn set_dependencies(&mut self, dependencies: &Vec<Vec<DependencyValue>>) -> () {
        
        let dep_val = &dependencies[0][0].value;

        if let StateVarReadOnly::String(string_val) = dep_val {
            self.value_sv = string_val.create_new_read_only_view();
        } else {
            panic!("Something went wrong with text sv of text");
        }
    }

    fn calculate_state_var_from_dependencies(&self) -> () {
        self.val.set_value(self.value_sv.get_value_assuming_fresh().clone());
    }

    fn return_for_renderer(&self) -> bool {
        true
    }

    fn get_value_assuming_fresh(&self) -> String {
        self.val.get_value_assuming_fresh().clone()
    }

    fn create_new_mutable_view(&self) -> StateVarTyped<String> {
        self.val.create_new_mutable_view()
    }

    fn get_name(&self) -> &'static str {
        "text"
    }

}


#[derive(Debug)]
struct Hidden {
    val: StateVarTyped<bool>,
}

impl StateVariable<bool> for Hidden {
    fn calculate_state_var_from_dependencies(&self) -> () {
        self.val.set_value(false);
    }
    fn return_for_renderer(&self) -> bool {
        true
    }
    fn get_value_assuming_fresh(&self) -> bool {
        *self.val.get_value_assuming_fresh()
    }
    fn create_new_mutable_view(&self) -> StateVarTyped<bool> {
        self.val.create_new_mutable_view()
    }
    fn get_name(&self) -> &'static str {
        "hidden"
    }
}

#[derive(Debug)]
struct Disabled {
    val: StateVarTyped<bool>,
}

impl StateVariable<bool> for Disabled {
    fn calculate_state_var_from_dependencies(&self) -> () {
        self.val.set_value(false);
    }
    fn return_for_renderer(&self) -> bool {
        true
    }
    fn get_value_assuming_fresh(&self) -> bool {
        *self.val.get_value_assuming_fresh()
    }
    fn create_new_mutable_view(&self) -> StateVarTyped<bool> {
        self.val.create_new_mutable_view()
    }
    fn get_name(&self) -> &'static str {
        "disabled"
    }
}

#[derive(Debug)]
struct Fixed {
    val: StateVarTyped<bool>,
}

impl StateVariable<bool> for Fixed {
    fn calculate_state_var_from_dependencies(&self) -> () {
        self.val.set_value(false);
    }
    fn return_for_renderer(&self) -> bool {
        true
    }
    fn get_value_assuming_fresh(&self) -> bool {
        *self.val.get_value_assuming_fresh()
    }
    fn create_new_mutable_view(&self) -> StateVarTyped<bool> {
        self.val.create_new_mutable_view()
    }
    fn get_name(&self) -> &'static str {
        "fixed"
    }
}



lazy_static! {

    pub static ref GENERATE_STATE_VARS: fn () -> Vec<StateVarVariant> = || {
        vec![
            StateVarVariant::String(
                Box::new(Value {
                    val: StateVarTyped::new(),
                    string_child_values: Vec::new()
                })
            ),
            StateVarVariant::String(
                Box::new(Text {
                    val: StateVarTyped::new(),
                    value_sv: StateVarReadOnlyTyped::new()
                })
            ),
            StateVarVariant::Boolean(
                Box::new(Hidden {
                    val: StateVarTyped::new()
                })
            ),
            StateVarVariant::Boolean(
                Box::new(Disabled {
                    val: StateVarTyped::new()
                })
            ),
            StateVarVariant::Boolean(
                Box::new(Fixed {
                    val: StateVarTyped::new()
                })
            ),
        ]


    };

    pub static ref STATE_VARIABLES_NAMES_IN_ORDER: Vec<&'static str> = GENERATE_STATE_VARS().iter().map(|sv| sv.get_name()).collect();


    pub static ref MY_COMPONENT_DEFINITION: ComponentDefinition = ComponentDefinition {
        component_type: "text",

        state_var_index_map: STATE_VARIABLES_NAMES_IN_ORDER.iter().enumerate().map(|(i,v)| (*v,i) ).collect(),

        state_var_names: STATE_VARIABLES_NAMES_IN_ORDER.to_vec(),

        generate_state_vars: *GENERATE_STATE_VARS,

        attribute_names: vec![
            "hide",
            "disabled",
            "fixed",
        ],

        primary_input_state_var_ind: Some(0),

        component_profiles: vec![
            (ComponentProfile::Text, "value")
        ],

        valid_children_profiles: ValidChildTypes::ValidProfiles(vec![ComponentProfile::Text]),
        
        ..Default::default()
    };


}
