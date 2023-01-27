use lazy_static::lazy_static;

use super::*;
use crate::{state::{StateVarMutableViewTyped, StateVarReadOnlyViewTyped, StateVarReadOnlyView}, utils::log};

// use crate::base_definitions::*;

#[derive(Debug)]
struct Value {
    val: StateVarMutableViewTyped<String>,
    essential_value: StateVarReadOnlyViewTyped<String>,
    immediate_value: StateVarReadOnlyViewTyped<String>,
    sync_values: StateVarReadOnlyViewTyped<bool>,
}

impl Value {
    pub fn new() -> Self {
        Value {
            val: StateVarMutableViewTyped::new(),
            essential_value: StateVarReadOnlyViewTyped::new(),
            immediate_value: StateVarReadOnlyViewTyped::new(),
            sync_values: StateVarReadOnlyViewTyped::new(),
        }
    } 
}

impl StateVarInterface<String> for Value {
    fn return_dependency_instructions(&self) -> Vec<DependencyInstruction> {
        vec![
            DependencyInstruction::Essential { prefill: Some("prefill") },
            DependencyInstruction::StateVar {
                component_name: None,
                state_var_name: "immediateValue",
            },
            DependencyInstruction::StateVar {
                component_name: None,
                state_var_name: "syncImmediateValue",
            }
        ]
    }

    fn set_dependencies(&mut self, dependencies: &Vec<Vec<DependencyValue>>) -> () {

        if let StateVarReadOnlyView::String(essential_value) = &dependencies[0][0].value {
            self.essential_value = essential_value.create_new_read_only_view();
        } else {
            panic!("Got a non-string essential value for value of text input.");
        }

        if let StateVarReadOnlyView::String(immediate_value) = &dependencies[1][0].value {
            self.immediate_value = immediate_value.create_new_read_only_view();
        } else {
            panic!("Got a non-string immediate value for value of text input.");
        }

        if let StateVarReadOnlyView::Boolean(sync_values) = &dependencies[2][0].value {
            self.sync_values = sync_values.create_new_read_only_view();
        } else {
            panic!("Got a non-boolean sync values for value of text input.");
        }

    }

    fn calculate_state_var_from_dependencies(&self) -> () {

        log!("calc val for textinput: {}, {}, {}", *self.sync_values.get_value_assuming_fresh(), *self.immediate_value.get_value_assuming_fresh(), *self.essential_value.get_value_assuming_fresh());

    
        let value =
        if *self.sync_values.get_value_assuming_fresh() {
            self.immediate_value.get_value_assuming_fresh()
        } else {
            self.essential_value.get_value_assuming_fresh()
        };
        
        self.val.set_value(value.clone());
    }

    fn request_dependencies_to_update_value(&self) -> Result<(),()> {

        // vec![
        //     (0, Ok(vec![
        //         DependencyValue {
        //             source: sources[0][0].0.clone(),
        //             value: desired_value.clone().into(),
        //         }
        //     ])),
        //     (1, Ok(vec![
        //         DependencyValue {
        //             source: sources[1][0].0.clone(),
        //             value: desired_value.into(),
        //         }
        //     ])),
        //     (2, Ok(vec![
        //         DependencyValue {
        //             source: sources[2][0].0.clone(),
        //             value: StateVarValue::Boolean(true),
        //         }
        //     ])),
        // ]
        Err(())
    }

    fn get_value_assuming_fresh(&self) -> String {
        self.val.get_value_assuming_fresh().clone()
    }

    fn create_new_mutable_view(&self) -> StateVarMutableViewTyped<String> {
        self.val.create_new_mutable_view()
    }

    fn get_name(&self) -> &'static str {
        "value"
    }

}



#[derive(Debug)]
struct ImmediateValue {
    val: StateVarMutableViewTyped<String>,
    essential_value: StateVarReadOnlyViewTyped<String>,
}

impl ImmediateValue {
    pub fn new() -> Self {
        ImmediateValue {
            val: StateVarMutableViewTyped::new(),
            essential_value: StateVarReadOnlyViewTyped::new(),
        }
    } 
}

impl StateVarInterface<String> for ImmediateValue {
    fn return_dependency_instructions(&self) -> Vec<DependencyInstruction> {
        vec![
            DependencyInstruction::Essential { prefill: Some("prefill") },
        ]
    }

    fn return_for_renderer(&self) -> bool {
        true
    }

    fn set_dependencies(&mut self, dependencies: &Vec<Vec<DependencyValue>>) -> () {

        if let StateVarReadOnlyView::String(essential_value) = &dependencies[0][0].value {
            self.essential_value = essential_value.create_new_read_only_view();
        } else {
            panic!("Got a non-string essential value for immediate value of text input.");
        }

    }

    fn calculate_state_var_from_dependencies(&self) -> () {

        log!("calc im val for textinput: {}", *self.essential_value.get_value_assuming_fresh());


        self.val.set_value(self.essential_value.get_value_assuming_fresh().clone());

    }

    fn request_dependencies_to_update_value(&self) -> Result<(),()> {

        // vec![
        //     (0, Ok(vec![
        //         DependencyValue {
        //             source: sources[0][0].0.clone(),
        //             value: desired_value.into(),
        //         }
        //     ]))
        // ]
        Err(())
    }

    fn get_value_assuming_fresh(&self) -> String {
        self.val.get_value_assuming_fresh().clone()
    }

    fn create_new_mutable_view(&self) -> StateVarMutableViewTyped<String> {
        self.val.create_new_mutable_view()
    }

    fn get_name(&self) -> &'static str {
        "immediateValue"
    }

}


#[derive(Debug)]
struct SyncImmediateValue {
    val: StateVarMutableViewTyped<bool>,
    essential_value: StateVarReadOnlyViewTyped<bool>,
}

impl SyncImmediateValue {
    pub fn new() -> Self {
        SyncImmediateValue {
            val: StateVarMutableViewTyped::new(),
            essential_value: StateVarReadOnlyViewTyped::new(),
        }
    } 
}

impl StateVarInterface<bool> for SyncImmediateValue {
    fn return_dependency_instructions(&self) -> Vec<DependencyInstruction> {
        vec![
            DependencyInstruction::Essential { prefill: None },
        ]
    }

    fn return_initial_essential_value(&self) -> bool {
        true
    }

    fn set_dependencies(&mut self, dependencies: &Vec<Vec<DependencyValue>>) -> () {

        if let StateVarReadOnlyView::Boolean(essential_value) = &dependencies[0][0].value {
            self.essential_value = essential_value.create_new_read_only_view();
        } else {
            panic!("Got a non-booloean essential value for syncImmediate of text input.");
        }

    }

    fn calculate_state_var_from_dependencies(&self) -> () {

        self.val.set_value(self.essential_value.get_value_assuming_fresh().clone());

    }


    fn request_dependencies_to_update_value(&self) -> Result<(),()> {

        // vec![
        //     (0, Ok(vec![
        //         DependencyValue {
        //             source: sources[0][0].0.clone(),
        //             value: desired_value.into(),
        //         }
        //     ]))
        // ]
        Err(())
    }

    fn get_value_assuming_fresh(&self) -> bool {
        self.val.get_value_assuming_fresh().clone()
    }

    fn create_new_mutable_view(&self) -> StateVarMutableViewTyped<bool> {
        self.val.create_new_mutable_view()
    }

    fn get_name(&self) -> &'static str {
        "syncImmediateValue"
    }

}




#[derive(Debug)]
struct Expanded {
    val: StateVarMutableViewTyped<bool>,
}

impl Expanded {
    pub fn new() -> Self {
        Expanded {
            val: StateVarMutableViewTyped::new(),
        }
    } 
}

impl StateVarInterface<bool> for Expanded {

    fn calculate_state_var_from_dependencies(&self) -> () {

        self.val.set_value(false);

    }
    fn return_for_renderer(&self) -> bool {
        true
    }
    fn get_value_assuming_fresh(&self) -> bool {
        self.val.get_value_assuming_fresh().clone()
    }
    fn create_new_mutable_view(&self) -> StateVarMutableViewTyped<bool> {
        self.val.create_new_mutable_view()
    }
    fn get_name(&self) -> &'static str {
        "expanded"
    }
}




#[derive(Debug)]
struct Size {
    val: StateVarMutableViewTyped<f64>,
}

impl Size {
    pub fn new() -> Self {
        Size {
            val: StateVarMutableViewTyped::new(),
        }
    } 
}

impl StateVarInterface<f64> for Size {

    fn calculate_state_var_from_dependencies(&self) -> () {
        self.val.set_value(10.0);
    }
    fn return_for_renderer(&self) -> bool {
        true
    }
    fn get_value_assuming_fresh(&self) -> f64 {
        self.val.get_value_assuming_fresh().clone()
    }
    fn create_new_mutable_view(&self) -> StateVarMutableViewTyped<f64> {
        self.val.create_new_mutable_view()
    }
    fn get_name(&self) -> &'static str {
        "size"
    }
}



#[derive(Debug)]
struct Width {
    val: StateVarMutableViewTyped<f64>,
}

impl Width {
    pub fn new() -> Self {
        Width {
            val: StateVarMutableViewTyped::new(),
        }
    } 
}

impl StateVarInterface<f64> for Width {

    fn calculate_state_var_from_dependencies(&self) -> () {
        self.val.set_value(600.0);
    }
    fn return_for_renderer(&self) -> bool {
        true
    }
    fn get_value_assuming_fresh(&self) -> f64 {
        self.val.get_value_assuming_fresh().clone()
    }
    fn create_new_mutable_view(&self) -> StateVarMutableViewTyped<f64> {
        self.val.create_new_mutable_view()
    }
    fn get_name(&self) -> &'static str {
        "width"
    }
}



#[derive(Debug)]
struct Hidden {
    val: StateVarMutableViewTyped<bool>,
}

impl Hidden {
    pub fn new() -> Self {
        Hidden {
            val: StateVarMutableViewTyped::new(),
        }
    } 
}

impl StateVarInterface<bool> for Hidden {
    fn calculate_state_var_from_dependencies(&self) -> () {
        self.val.set_value(false);
    }
    fn return_for_renderer(&self) -> bool {
        true
    }
    fn get_value_assuming_fresh(&self) -> bool {
        *self.val.get_value_assuming_fresh()
    }
    fn create_new_mutable_view(&self) -> StateVarMutableViewTyped<bool> {
        self.val.create_new_mutable_view()
    }
    fn get_name(&self) -> &'static str {
        "hidden"
    }
}

#[derive(Debug)]
struct Disabled {
    val: StateVarMutableViewTyped<bool>,
}

impl Disabled {
    pub fn new() -> Self {
        Disabled {
            val: StateVarMutableViewTyped::new(),
        }
    } 
}

impl StateVarInterface<bool> for Disabled {
    fn calculate_state_var_from_dependencies(&self) -> () {
        self.val.set_value(false);
    }
    fn return_for_renderer(&self) -> bool {
        true
    }
    fn get_value_assuming_fresh(&self) -> bool {
        *self.val.get_value_assuming_fresh()
    }
    fn create_new_mutable_view(&self) -> StateVarMutableViewTyped<bool> {
        self.val.create_new_mutable_view()
    }
    fn get_name(&self) -> &'static str {
        "disabled"
    }
}






lazy_static! {

    pub static ref GENERATE_STATE_VARS: fn () -> Vec<StateVarInterfaceBoxed> = || {
        vec![
            StateVarInterfaceBoxed::String(
                Box::new(Value::new())
            ),
            StateVarInterfaceBoxed::String(
                Box::new(ImmediateValue::new())
            ),
            StateVarInterfaceBoxed::Boolean(
                Box::new(SyncImmediateValue::new())
            ),
            StateVarInterfaceBoxed::Number(
                Box::new(Width::new())
            ),
            StateVarInterfaceBoxed::Number(
                Box::new(Size::new())
            ),
            StateVarInterfaceBoxed::Boolean(
                Box::new(Hidden::new())
            ),
            StateVarInterfaceBoxed::Boolean(
                Box::new(Disabled::new())
            ),
        ]


    };

    pub static ref STATE_VARIABLES_NAMES_IN_ORDER: Vec<&'static str> = GENERATE_STATE_VARS().iter().map(|sv| sv.get_name()).collect();

    pub static ref MY_COMPONENT_DEFINITION: ComponentDefinition = ComponentDefinition {
        component_type: "textInput",

        state_var_index_map: STATE_VARIABLES_NAMES_IN_ORDER.iter().enumerate().map(|(i,v)| (*v,i) ).collect(),

        state_var_names: STATE_VARIABLES_NAMES_IN_ORDER.to_vec(),
        
        generate_state_vars: *GENERATE_STATE_VARS,

        attribute_names: vec![
            "hide",
            "disabled",
            "prefill",
        ],

        action_names: || vec!["updateImmediateValue", "updateValue"],

        on_action: |action_name, args, resolve_and_retrieve_state_var| {
            match action_name {
                "updateImmediateValue" => {
                    // Note: the key here is whatever the renderers call the new value
                    let new_val = args.get("text").expect("No text argument").first().unwrap();

                    vec![
                        (1, new_val.clone()),
                        (2, StateVarValue::Boolean(false)),
                    ]
                },

                "updateValue" => {

                    let new_val = resolve_and_retrieve_state_var(1)
                        .try_into().unwrap();
                    let new_val = StateVarValue::String(new_val);

                    vec![
                        (0, new_val),
                        (2, StateVarValue::Boolean(true)),
                    ]

                }

                _ => panic!("Unknown action '{}' called on textInput", action_name)
            }
        },

        ..Default::default()
    };
}
