use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::utils::{log};


use super::*;
use crate::base_definitions::*;


use crate::ComponentProfile;



lazy_static! {

    pub static ref MY_STATE_VAR_DEFINITIONS: Vec<(StateVarName, StateVarVariant)> = {

        use StateVarUpdateInstruction::*;

        vec![

        ("value", StateVarVariant::Number(StateVarDefinition {
            initial_essential_value: f64::NAN,
            dependency_instructions: vec![
                DependencyInstruction::StateVar {
                    component_name: None,
                    state_var_name: "lastValue",
                },
                DependencyInstruction::StateVar {
                    component_name: None,
                    state_var_name: "immediateValue",
                },
                DependencyInstruction::StateVar {
                    component_name: None,
                    state_var_name: "syncImmediateValue",
                },
            ],
            determine_state_var_from_dependencies: |dependency_values| {
                let essential_value = dependency_values[0][0]
                    .into_number()?;
                let immediate_value = dependency_values[1][0]
                    .into_number()?;
                let sync_values = dependency_values[2][0]
                    .into_bool()?;

                let value: f64 =
                    if sync_values {
                        immediate_value
                    } else {
                        essential_value
                    };
                Ok(SetValue(value))
            } ,
            request_dependencies_to_update_value: |desired_value, sources| {
                vec![
                    (0, Ok(vec![
                        DependencyValue {
                            source: sources[0][0].0.clone(),
                            value: desired_value.clone().into(),
                        }
                    ])),
                    (1, Ok(vec![
                        DependencyValue {
                            source: sources[1][0].0.clone(),
                            value: desired_value.clone().into(),
                        }
                    ])),
                    (2, Ok(vec![
                        DependencyValue {
                            source: sources[2][0].0.clone(),
                            value: StateVarValue::Boolean(true),
                        }
                    ])),
                ]
            },
            ..Default::default()
        })),

        ("immediateValue", StateVarVariant::Number(StateVarDefinition {
            dependency_instructions: vec![
                DependencyInstruction::StateVar {
                    component_name: None,
                    state_var_name: "rawRendererValue",
                },
                // ("sync", DependencyInstruction::StateVar {
                //     component_ref: None,
                //     state_var: StateVarSlice::Single(StateRef::Basic("syncImmediateValue")),
                // }),
            ],
            determine_state_var_from_dependencies: |dependency_values| {
                let string_value = dependency_values[0][0]
                    .into_string()?;
                let value: f64 = string_value.parse().unwrap_or(f64::NAN);

                Ok(SetValue(value))
            },
            request_dependencies_to_update_value: |desired_value, sources| {
                vec![
                    (0, Ok(vec![
                        DependencyValue {
                            source: sources[0][0].0.clone(),
                            value: desired_value.to_string().into(),
                        }
                    ])),
                    // ("sync", Ok(vec![
                    //     DependencyValue {
                    //         source: sources.get("sync").unwrap().first().unwrap().0.clone(),
                    //         value: StateVarValue::Boolean(false),
                    //     }
                    // ])),
                ]
            },
            ..Default::default()
        })),

        ("lastValue", StateVarVariant::Number(StateVarDefinition {
            dependency_instructions: vec![
                DependencyInstruction::Essential { prefill: Some("prefill") }
            ],
            determine_state_var_from_dependencies: DETERMINE_FROM_ESSENTIAL,
            request_dependencies_to_update_value: REQUEST_ESSENTIAL_TO_UPDATE,
            initial_essential_value: f64::NAN,
            ..Default::default()
        })),

        ("syncImmediateValue", StateVarVariant::Boolean(StateVarDefinition {
            dependency_instructions: USE_ESSENTIAL_DEPENDENCY_INSTRUCTION(),
            determine_state_var_from_dependencies: DETERMINE_FROM_ESSENTIAL,
            request_dependencies_to_update_value: REQUEST_ESSENTIAL_TO_UPDATE,
            initial_essential_value: true,
            ..Default::default()
        })),

         ("rawRendererValue", StateVarVariant::String(StateVarDefinition {
             for_renderer: true,
             dependency_instructions: vec![
                DependencyInstruction::Essential { prefill: Some("prefill") }
            ],
             determine_state_var_from_dependencies: DETERMINE_FROM_ESSENTIAL,
             request_dependencies_to_update_value: REQUEST_ESSENTIAL_TO_UPDATE,
             ..Default::default()
         })),


        ("expanded", StateVarVariant::Boolean(StateVarDefinition {
            for_renderer: true,
            determine_state_var_from_dependencies: |_| Ok(SetValue(false)),
            ..Default::default()
            
        })),

        ("size", StateVarVariant::Number(StateVarDefinition {
            determine_state_var_from_dependencies: |_| {
                Ok(SetValue(10.0))
            },
            for_renderer: true,
            ..Default::default()
        })),

        ("width", StateVarVariant::Number(StateVarDefinition {
            for_renderer: true,
            determine_state_var_from_dependencies: |_| Ok(SetValue(600.0)),
            ..Default::default()
        })),

        ("hidden", HIDDEN_DEFAULT_DEFINITION()),
        ("disabled", DISABLED_DEFAULT_DEFINITION()),

        ]
    };


}



lazy_static! {
    pub static ref MY_COMPONENT_DEFINITION: ComponentDefinition = ComponentDefinition {
        component_type: "numberInput",

        state_var_definitions: &MY_STATE_VAR_DEFINITIONS,
        
        state_var_index_map: MY_STATE_VAR_DEFINITIONS.iter().enumerate().map(|(i,v)| (v.0,i) ).collect(),

        attribute_names: vec![
            "hide",
            "disabled",
            "prefill",
        ],

        renderer_type: RendererType::Special{
            component_type: "textInput",
            state_var_aliases: HashMap::from([("rawRendererValue", "immediateValue")]),
        },

        component_profiles: vec![
            (ComponentProfile::Number, "value"),
            // (ComponentProfile::Text, "value"),
        ],
        
        action_names: || vec!["updateImmediateValue", "updateValue"],

        on_action: |action_name, args, resolve_and_retrieve_state_var| {
            match action_name {
                "updateImmediateValue" => {
                    // Note: the key here is whatever the renderers call the new value
                    let new_val = args.get("text").expect("No text argument").first().unwrap();

                    vec![
                        (4, new_val.clone()),
                        (3, StateVarValue::Boolean(false)),
                    ]
                },

                "updateValue" => {

                    let new_val = resolve_and_retrieve_state_var(1)
                        .unwrap().try_into().unwrap();
                    let new_val = StateVarValue::Number(new_val);

                    vec![
                        (2, new_val),
                        (3, StateVarValue::Boolean(true)),
                    ]
                }

                _ => panic!("Unknown action '{}' called on numberInput", action_name)
            }
        },

        ..Default::default()
    };
}
