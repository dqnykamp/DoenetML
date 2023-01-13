use std::collections::HashMap;

use crate::{state_variables::*, math_expression::MathExpression, utils::{log}};



macro_rules! number_definition_from_attribute {
    ( $attribute:expr, $default:expr ) => {
        {
            StateVarVariant::Number(StateVarDefinition {
                for_renderer: true,

                initial_essential_value: $default,

                return_dependency_instructions: |_| {
                    let attribute = DependencyInstruction::Attribute{
                        attribute_name: $attribute,
                    };
                    HashMap::from([("attribute", attribute)])
                },

                determine_state_var_from_dependencies: |dependency_values| {
                    let (attribute, _) = dependency_values.dep_value("attribute")?;
                    if attribute.len() > 0 {

                        match DETERMINE_NUMBER(attribute) {
                            Ok(x) => Ok(SetValue(x)),
                            Err(msg) => {
                                crate::utils::log!("Error determing number: {}", msg);
                                Ok(SetValue(f64::NAN))
                            },
                        }
                    } else {
                        Ok ( crate::state_variables::StateVarUpdateInstruction::SetValue($default) )
                    }
                },

                request_dependencies_to_update_value: |desired_value, sources| {
                    let attribute_sources = sources.get("attribute").unwrap();
                    HashMap::from([
                        ("attribute", DETERMINE_NUMBER_DEPENDENCIES(desired_value, attribute_sources))
                    ])
                },

                ..Default::default()
            })
        }
    }
}
use evalexpr::{HashMapContext, ContextWithMutableVariables, Operator};
pub(crate) use number_definition_from_attribute;


macro_rules! integer_definition_from_attribute {
    ( $attribute:expr, $default:expr ) => {
        {
            StateVarVariant::Integer(StateVarDefinition {
                for_renderer: true,

                initial_essential_value:$default,

                dependency_instructions: vec![
                    DependencyInstruction::Attribute{
                        attribute_name: $attribute,
                    }
                ],

                determine_state_var_from_dependencies: |dependency_values| {
                    let attribute = &dependency_values[0];
                    if attribute.len() > 0 {

                        // let (expression, numerical_values) = split_dependency_values_into_math_expression_and_values(attribute)?;

                        match DETERMINE_NUMBER(attribute) {
                            Ok(x) => Ok(SetValue(x as i64)),
                            Err(msg) => Err(msg),
                        }
                    } else {
                        Ok ( crate::state_variables::StateVarUpdateInstruction::SetValue($default) )
                    }
                },

                request_dependencies_to_update_value: |desired_value, sources| {
                    let attribute_sources = &sources[0];
                    vec![
                        (0, DETERMINE_INTEGER_DEPENDENCIES(desired_value, attribute_sources))
                    ]
                },

                ..Default::default()
            })
        }
    }
}
pub(crate) use integer_definition_from_attribute;


macro_rules! boolean_definition_from_attribute {
    ( $attribute:expr, $default:expr ) => {
        {
            StateVarVariant::Boolean(StateVarDefinition {
                for_renderer: true,

                initial_essential_value: $default,

                dependency_instructions: vec![
                    DependencyInstruction::Attribute{
                        attribute_name: $attribute,
                    }
                ],

                determine_state_var_from_dependencies: |dependency_values| {
                    let attribute = &dependency_values[0];
                    if attribute.len() > 0 {

                        match DETERMINE_BOOLEAN(attribute) {
                            Ok(x) => Ok(crate::state_variables::StateVarUpdateInstruction::SetValue(x)),
                            Err(msg) => Err(msg),
                        }
                    } else {
                        Ok ( crate::state_variables::StateVarUpdateInstruction::SetValue($default) )
                    }
                },

                ..Default::default()
            })
        }
    }
}
pub(crate) use boolean_definition_from_attribute;


macro_rules! string_definition_from_attribute {
    ( $attribute:expr, $default:expr ) => {
        {
            StateVarVariant::String(StateVarDefinition {
                for_renderer: true,

                initial_essential_value: $default.to_string(),

                dependency_instructions: vec![
                    DependencyInstruction::Attribute{
                        attribute_name: $attribute,
                    }
                ],

                determine_state_var_from_dependencies: |dependency_values| {
                    let attribute = &dependency_values[0];
                    if attribute.len() > 0 {
                        DETERMINE_STRING(attribute)
                            .map(|x| crate::state_variables::StateVarUpdateInstruction::SetValue(x))
                    } else {
                        Ok ( crate::state_variables::StateVarUpdateInstruction::SetValue($default.to_string()) )
                    }
                },

                ..Default::default()
            })
        }
    }
}
pub(crate) use string_definition_from_attribute;


// ===== potentially useful macro for arrays ===== //
// macro_rules! map_array {
//     ( $variant:ident, $array:literal, $func:expr, $render:literal ) => {
//         {
//             StateVarVariant::$variant(StateVarArrayDefinition {

//                 for_renderer: $render,

//                 return_array_dependency_instructions: |_| {
//                     HashMap::from([(
//                         "corresponding_value", DependencyInstruction::CorrespondingElements {
//                             component_ref: None,
//                             array_state_var_name: $array,
//                         }
//                     )])
//                 },

//                 determine_element_from_dependencies: |_, dependency_values| {
//                     let corresponding: f64 = dependency_values.dep_value("corresponding_value")?
//                         .has_exactly_one_element()?
//                     let my_value = $func(corresponding);

//                     Ok(SetValue(my_value))
//                 },

//                 return_size_dependency_instructions: |_| {
//                     HashMap::from([
//                         ("size", DependencyInstruction::StateVar {
//                             component_ref: None,
//                             state_var: StateVarSlice::Single(StateRef::SizeOf($array)),
//                         }),
//                     ])
//                 },

//                 determine_size_from_dependencies: |dependency_values| {
//                     let size = dependency_values.dep_value("size")?
//                         .has_exactly_one_element()?
//                         .into_integer()?;
//                     Ok(SetValue(size as usize))
//                 },

//                 ..Default::default()
//             })
//         }
//     }
// }
// pub(crate) use map_array;

pub fn split_dependency_values_into_math_expression_and_values(
    dependency_values: &Vec<DependencyValue>
) -> Result<(MathExpression, Vec<StateVarValue>), String> {

    let expression = dependency_values.iter().find_map(|elem| {
        if let StateVarValue::MathExpr(ref expr) = elem.value {
            Some(expr.clone())
        } else {
            None
        }
    }).ok_or("There should have been a math expression".to_string())?;

    let values: Vec<StateVarValue> = dependency_values.iter().filter_map(|elem| {
        if let StateVarValue::MathExpr(_) = elem.value {
            None
        } else {
            Some(elem.value.clone())
        }
    }).collect();

    if values.len() != dependency_values.len() - 1 {
        return Err(format!("Invalid quantity of numerical dependency values for number-like behavior, {} out of {} were numerical but there should have been {}", values.len(), dependency_values.len(), dependency_values.len() - 1));
    }

    Ok((expression, values))
}

pub fn split_dependency_sources_into_expression_and_variables(
    dependency_sources: &Vec<(DependencySource, Option<StateVarValue>)>
) -> Result<((DependencySource, MathExpression), Vec<(DependencySource, Option<StateVarValue>)>, usize), String> {

    // For now, assume that any essential data source is the expression
    let (expr_id, expression_source, expression) = dependency_sources
        .iter()
        .enumerate()
        .find_map(|(id, attr_elem)| {
            if let DependencySource::Essential { .. } = attr_elem.0 {
                if let StateVarValue::MathExpr(expr) = attr_elem.1.as_ref().unwrap() {
                    Some((id, attr_elem.0.clone(), expr.clone()))
                } else {
                    unreachable!()
                }
            } else {
                None
            }
        }).ok_or("There should have been a math expression".to_string())?;

    let variables: Vec<(DependencySource, Option<StateVarValue>)> = dependency_sources
        .into_iter()
        .enumerate()
        .filter_map(|(id, elem)| (id != expr_id).then_some(elem.clone()))
        .collect();

    for variable in variables.iter() {
        match variable.0 {
            DependencySource::StateVar { .. } => {},
            _ => return Err("Invalid dependency sources for splitting expression and variables".to_string()),
        }
    }

    Ok(((expression_source, expression), variables, expr_id))
}



// Default functions for an essential depenency

#[allow(non_snake_case)]
pub fn USE_ESSENTIAL_DEPENDENCY_INSTRUCTION() -> Vec<DependencyInstruction> {
   vec![DependencyInstruction::Essential { prefill: None }]
}


#[allow(non_snake_case)]
pub fn DETERMINE_FROM_ESSENTIAL<T>(
    dependency_values: Vec<Vec<DependencyValue>>
) -> Result<StateVarUpdateInstruction<T>, String>
where
    T: TryFrom<StateVarValue> + Default,
    <T as TryFrom<StateVarValue>>::Error: std::fmt::Debug
{
    let essential = &dependency_values[0];
    let essential = essential.get(0);
    let set_value = match essential {
        Some(dep_value) => {
            T::try_from(dep_value.value.clone()).map_err(|e| format!("{:#?}", e))?
        },
        None => T::default(),
    };
    Ok( StateVarUpdateInstruction::SetValue( set_value ) )
}



#[allow(non_snake_case)]
pub fn REQUEST_ESSENTIAL_TO_UPDATE<T: Into<StateVarValue>>(desired_value: T, sources: Vec<Vec<(DependencySource, Option<StateVarValue>)>>)
    -> Vec<(usize, Result<Vec<DependencyValue>, String>)> {
    vec![
        (0, Ok(vec![
            DependencyValue {
                source: sources[0][0].0.clone(),
                value: desired_value.into(),
            }
        ]))
    ]
}

/// Requires that the component has a parent with 'hidden' and a bool 'hide' attribute
#[allow(non_snake_case)]
pub fn HIDDEN_DEFAULT_DEFINITION() -> StateVarVariant {
    use StateVarUpdateInstruction::*;


    StateVarVariant::Boolean(StateVarDefinition {
        
        dependency_instructions: vec![
           DependencyInstruction::Parent {
                state_var_name: "hidden",
            },
            DependencyInstruction::Attribute {
                attribute_name: "hide",
            },
        ],


        determine_state_var_from_dependencies: |dependency_values| {

            let parent_hidden = dependency_values[0][0].into_bool();

            let attribute = &dependency_values[1];

            let my_hide =
                (attribute.len() > 0)
                .then(|| DETERMINE_BOOLEAN(attribute).ok())
                .flatten();

            Ok(SetValue(parent_hidden.unwrap_or(false) || my_hide.unwrap_or(false)))
        },


        for_renderer: true,
        ..Default::default()
    })
}


/// Text (string) value of value sv
#[allow(non_snake_case)]
pub fn TEXT_DEFAULT_DEFINITION() -> StateVarVariant {
    use StateVarUpdateInstruction::*;

    StateVarVariant::String(StateVarDefinition {
        for_renderer: true,

        dependency_instructions: vec![DependencyInstruction::StateVar {
            component_name: None,
            state_var_name: "value"
        }],

        determine_state_var_from_dependencies: |dependency_values| {

            let value = &dependency_values[0][0].value;

            match &value {
                StateVarValue::String(v) => Ok(SetValue(v.to_string())),
                StateVarValue::Boolean(v) => Ok(SetValue(v.to_string())),
                StateVarValue::Integer(v) => Ok(SetValue(v.to_string())),
                StateVarValue::Number(v) => Ok(SetValue(v.to_string())),
                StateVarValue::MathExpr(_) => unreachable!(),
            }
        },

        ..Default::default()
    })
}


#[allow(non_snake_case)]
pub fn DISABLED_DEFAULT_DEFINITION() -> StateVarVariant {
    boolean_definition_from_attribute!("disabled", false)
}


#[allow(non_snake_case)]
pub fn FIXED_DEFAULT_DEFINITION() -> StateVarVariant {
    StateVarVariant::Boolean(StateVarDefinition {     
        for_renderer: true,
        determine_state_var_from_dependencies: |_| Ok(StateVarUpdateInstruction::SetValue(false)),
        ..Default::default()
    })
}


#[allow(non_snake_case)]
pub fn DETERMINE_BOOLEAN(dependency_values: &Vec<DependencyValue>)
    -> Result<bool, String> {

    if dependency_values.len() == 1
    && dependency_values[0].source != (DependencySource::Essential { value_type: "mathExpression" })  {
        
        let value = match &dependency_values[0].value {
            StateVarValue::Boolean(val) => *val,
            StateVarValue::MathExpr(e) => e.tree.eval_boolean().map_err(|e| e.to_string())?,
            _ => return Err(format!(
                    "A single dependency value must be a boolean, received {:?}",
                    dependency_values
                )),
        };

        Ok(value)

    } else {

        let (expression, variable_values) = split_dependency_values_into_math_expression_and_values(dependency_values)?;

        if variable_values.len() != expression.external_variables_count {
            log!("Tried to evalute expression with incorrect number of external variables, expected {} but found {}", expression.external_variables_count, variable_values.len());

            return Ok(false);
        }
    
        let mut context = HashMapContext::new();
    
        for (id, value) in variable_values.iter().enumerate() {
    
            let variable_num = match value {
                StateVarValue::Number(num) => (*num).into(),
                StateVarValue::Integer(num) => (*num as f64).into(),
                StateVarValue::Boolean(num) => (*num).into(),
                _ => return Err("Can't determine boolean with these values".to_string()),
            };

            let name = format!("{}{}", expression.variable_prefix, id);
            context.set_value(name, variable_num).map_err(|err| err.to_string())?;
        }

        let num =
            if expression.tree.operator() == &Operator::RootNode && expression.tree.children().is_empty() {
                // Empty expression
                false
            } else {
                expression.tree.eval_boolean_with_context(&context).unwrap_or(false)
            };

        Ok(num)
    }
}

#[allow(non_snake_case)]
pub fn DETERMINE_NUMBER(dependency_values: &Vec<DependencyValue>)
    -> Result<f64, String> {

    if dependency_values.len() == 1
    && dependency_values[0].source != (DependencySource::Essential { value_type: "mathExpression" })  {
        
        let value = match dependency_values[0].value {
            StateVarValue::Number(val) => val,
            StateVarValue::Integer(val) => val as f64,
            _ => return Err(format!(
                    "A single dependency value must be a number or integer, received {:?}",
                    dependency_values
                )),
        };

        Ok(value)

    } else {

        let (expression, variable_values) = split_dependency_values_into_math_expression_and_values(dependency_values)?;

        if variable_values.len() != expression.external_variables_count {
            log!(
                "Tried to evalute expression with {} variables but found {}",
                expression.external_variables_count,
                variable_values.len()
            );

            return Ok(f64::NAN);
        }
    
        let mut context = HashMapContext::new();
    
        for (id, value) in variable_values.iter().enumerate() {
    
            let variable_num = match value {
                StateVarValue::Number(num) => (*num).into(),
                StateVarValue::Integer(num) => (*num).into(),
                _ => return Err("Can't determine number with non-numerical variable values".to_string()),
            };
    
            let name = format!("{}{}", expression.variable_prefix, id);
            context.set_value(name, variable_num).map_err(|err| err.to_string())?;
        }

        let num =
            if expression.tree.operator() == &Operator::RootNode && expression.tree.children().is_empty() {
                // Empty expression, set to 0
                0.0
            } else {
                expression.tree.eval_number_with_context(&context).unwrap_or(f64::NAN)
            };

        Ok(num)
    }
}


#[allow(non_snake_case)]
pub fn DETERMINE_NUMBER_DEPENDENCIES(desired_value: f64, sources: &Vec<(DependencySource, Option<StateVarValue>)>)
    -> Result<Vec<DependencyValue>, String> {

    if sources.len() == 1
    && sources[0].0 != (DependencySource::Essential { value_type: "mathExpression" })  {
        let (source, _) = sources.first().unwrap().clone();
        let value = match source {
            DependencySource::Essential { value_type: "string" } =>
                StateVarValue::String(desired_value.to_string()),
            DependencySource::StateVar { .. } =>
                StateVarValue::Number(desired_value),
            _ => panic!("Base definition 'determine number' function did not expect dependency source {:?}", source),
        };
        return Ok(vec![DependencyValue {
            source,
            value,
        }])
    }
    
    let (expression, variables, expression_id) = 
        split_dependency_sources_into_expression_and_variables(sources)?;
    
    if variables.len() == 0 {
        // log_debug!("Math expression has only constants: {:?}", expression.1.tree);

        return Ok(vec![
            DependencyValue {
                source: expression.0,
                value: StateVarValue::MathExpr(MathExpression::from(desired_value))
            }
        ])

    }  else if variables.len() == 1 {

        let tree = &expression.1.tree;

        if tree.children().len() == 1 {
            let child = &tree.children()[0];
            if child.children().is_empty() && matches!(child.operator(), Operator::VariableIdentifierRead { ..}) {
                
                // syntax tree is only the one variable
                // log_debug!("Math expression has one variable and no constants: {:?}", tree);

                let sv_value = DependencyValue {
                    source: variables[0].0.clone(),
                    value: StateVarValue::Number(desired_value),
                };

                let expression_value = DependencyValue {
                    source: expression.0,
                    value: StateVarValue::MathExpr(expression.1),
                };

                if expression_id == 0 {
                    return Ok(vec![expression_value, sv_value]);
                } else {
                    return Ok(vec![sv_value, expression_value]);
                }
            }
        }
    }
        
    return Err("inverse for number not implemented with multiple dependency values or non-constant math expression".to_string());

}



#[allow(non_snake_case)]
pub fn DETERMINE_INTEGER(dependency_values: Vec<&DependencyValue>)
    -> Result<i64, String> {

    let mut concatted_children = String::new();
    for value in dependency_values {
        let str_child_val = match &value.value {
            StateVarValue::Number(num) => num.to_string(),
            StateVarValue::String(str) => str.to_string(),
            StateVarValue::Integer(num) => num.to_string(),
            _ => return Err("Invalid value for number".to_string())
        };

        concatted_children.push_str(&str_child_val);
    }

    // log!("concatted children {}", concatted_children);

    let num = if let Ok(num_result) = evalexpr::eval(&concatted_children) {
        num_result.as_int().unwrap_or(i64::default())
    } else {
        return Err(format!("Can't parse number values '{}' as math", concatted_children));
    };

    Ok(num)
}

#[allow(non_snake_case)]
pub fn DETERMINE_INTEGER_DEPENDENCIES(desired_value: i64, sources: &Vec<(DependencySource, Option<StateVarValue>)>)
    -> Result<Vec<DependencyValue>, String> {
    if sources.len() == 1 {
        let (source, _) = sources.first().unwrap().clone();
        let value = match source {
            DependencySource::Essential { value_type: "string" } =>
                StateVarValue::String(desired_value.to_string()),
            DependencySource::Essential { value_type: "integer" } =>
                StateVarValue::Integer(desired_value),
            _ => panic!("integer did not expect component type"),
        };
        Ok(vec![DependencyValue {
            source,
            value,
        }])
    } else {
        Err("inverse for number not implemented with multiple children".to_string())
    }
}


#[allow(non_snake_case)]
pub fn DETERMINE_STRING(dependency_values: &Vec<DependencyValue>)
    -> Result<String, String> {

    let mut val = String::new();
    for textlike_value_sv in dependency_values {
        
        val.push_str(& match &textlike_value_sv.value {
            StateVarValue::String(v)  => v.to_string(),
            StateVarValue::Boolean(v) => v.to_string(),
            StateVarValue::Integer(v) => v.to_string(),
            StateVarValue::Number(v)  => v.to_string(),
            StateVarValue::MathExpr(_)  => unreachable!(),
        });
    }

    Ok(val)
}



pub fn get_children_of_type<'a>(
    component_nodes: &'a HashMap<crate::ComponentName, crate::ComponentNode>,
    node: &'a crate::ComponentNode,
    component_type: crate::ComponentType,
    _include_groups: bool,
) -> impl Iterator<Item=&'a crate::ComponentNode> {
    crate::get_child_nodes_including_copy(component_nodes, node).into_iter().filter_map(move |(n, _)|
        match n {
            crate::component::ObjectName::String(_) => None,
            crate::component::ObjectName::Component(c) => {
                let comp = component_nodes.get(c).unwrap();
                let child_type = comp.definition.component_type;
                (child_type.to_lowercase() == component_type.to_lowercase())
                    .then(|| component_nodes.get(c).unwrap())
            },
        }
    )
}

