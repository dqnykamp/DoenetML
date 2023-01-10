pub mod state_variables;
pub mod component;

pub mod state;
pub mod parse_json;
pub mod utils;
pub mod base_definitions;
pub mod math_expression;

use lazy_static::lazy_static;
use parse_json::{DoenetMLError, DoenetMLWarning, MLComponent, RangeInDoenetML};
use state::StateVar;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;

use state::{State, EssentialStateVar};
use component::*;
use state_variables::*;

use crate::math_expression::MathExpression;
use crate::utils::{log_json, log_debug, log};
use serde::Serialize;

use instant::Instant;

/// A static DoenetCore is created from parsed DoenetML at the beginning.
/// While `component_states` and `essential_data` can update using
/// internal mutability (the RefCell), the over-arching HashMaps are static.
#[derive(Debug)]
pub struct DoenetCore {
    /// The component tree has almost the same structure as the tree of elements
    /// typed into DoenetML, except macros are converted into their own components.
    pub component_nodes: HashMap<ComponentName, ComponentNode>,

    /// State variables
    pub component_states: HashMap<ComponentName, HashMap<StateVarName, StateVar>>,

    /// This should always be the name of a <document> component
    pub root_component_name: ComponentName,

    /// **The Dependency Graph**
    /// A DAG whose vertices are the state variables and attributes
    /// of every component, and whose endpoint vertices are essential data.
    ///
    /// Used for
    /// - producing values when determining a state variable
    /// - tracking when a change affects other state variables
    pub dependencies: HashMap<DependencyKey, Vec<Dependency>>,

    /// Endpoints of the dependency graph.
    /// Every update instruction will lead to these.
    pub essential_data: HashMap<ComponentName, HashMap<EssentialDataOrigin, EssentialStateVar>>,
}



// ==== Five levels: ComponentNode to RenderedComponent ====

/// Created by the DoenetML: an xml component or a macro
#[derive(Debug, Clone)]
pub struct ComponentNode {

    pub name: ComponentName,
    pub parent: Option<ComponentName>,
    pub children: Vec<ComponentChild>,

    pub copy_source: Option<CopySource>,
    pub static_attributes: HashMap<AttributeName, String>,

    pub definition: &'static ComponentDefinition,
}

/// Refers to a ComponentNode
/// A ComponentName is not static because it cannot be known at compile time.
pub type ComponentName = String;

/// A superset of ComponentRefs that includes children of duplicates:
/// copies and collection members.
/// This corresponds to one component in the component tree sent to the renderer.
#[derive(Debug)]
struct RenderedComponent<'a> {
    component_node: &'a ComponentNode,
    child_of_copy: Option<&'a ComponentNode>,
}



/// Dependencies keyed by:
/// 1. the name of the component
/// 2. the name of a state variable slice
///    which allows for two kinds of dependencies:
///      - direct dependency: when a single state var depends on something
///      - indirect dependency: when a group depends on something,
///        and members of the group inherit the dependency.
///        The motivation for indirect dependencies is that
///        the size of groups can change (e.g. an array changes size).
///        To keep the dependency graph static, we do not update
///        individual dependencies but simply apply the group dependency.
/// 3. the instruction name, given by the state variable to track where
///    dependecy values came from.
#[derive(Debug, Hash, PartialEq, Eq, Serialize)]
pub struct DependencyKey (ComponentName, StateVarName, InstructionName);

/// A collection of edges on the dependency graph
/// - Groups and array state var slices get converted into multiple DependencyValues
/// - A dependency applies to every instance, so it refers to instances relatively.
/// For example:
/// If A, a component inside a map, depends on B, a component inside a map
/// in the map, then each instance of A depends on a different instance of B.
/// But their relative instance is the same, and that is what to store
/// in the dependency graph.
#[derive(Debug, Serialize, PartialEq, Eq)]
pub enum Dependency {
    Essential {
        component_name: ComponentName,
        origin: EssentialDataOrigin,
    },
    StateVar {
        component_name: ComponentName,
        state_var_name: StateVarName,
    },
}



pub fn create_doenet_core(
    program: &str,
    existing_essential_data: Option<HashMap<ComponentName, HashMap<EssentialDataOrigin, EssentialStateVar>>>,
) -> Result<(DoenetCore, Vec<DoenetMLWarning>, Vec<DoenetMLError>), DoenetMLError> {

    log!("===== DoenetCore creation =====");

    let start = Instant::now();

    // Create component nodes and attributes
    let (ml_components, component_attributes, root_component_name, _map_sources_alias, warnings_encountered, errors_encountered) =
        parse_json::create_components_tree_from_json(program)?;

    let mut doenet_ml_warnings = vec![];
    let mut doenet_ml_errors = vec![];

    doenet_ml_warnings.extend(warnings_encountered);
    doenet_ml_errors.extend(errors_encountered);

    let component_nodes = convert_ml_components_into_component_nodes(ml_components, &mut doenet_ml_warnings, &mut doenet_ml_errors)?;

    doenet_ml_warnings.extend(check_for_invalid_childen_component_profiles(&component_nodes));
    check_for_cyclical_copy_sources(&component_nodes)?;
    check_for_invalid_component_names(&component_nodes, &component_attributes)?;


    log!("create component nodes: {:?}", start.elapsed());
    let start = Instant::now();

    let (dependencies, essential_data) = create_dependencies_and_essential_data(
        &component_nodes,
        &component_attributes,
        existing_essential_data
    );

    log!("create dependencies: {:?}", start.elapsed());
    let start = Instant::now();

    check_for_cyclical_dependencies(&dependencies)?;

    let component_states = create_stale_component_states(&component_nodes);

    log!("create stale states: {:?}", start.elapsed());
    let start = Instant::now();


    // log_json!("Component tree upon core creation",
    //     utils::json_components(&component_nodes, &component_states));
    // log_json!("Dependencies",
    //     utils::json_dependencies(&dependencies));
    // log_json!("Essential data upon core creation",
    //     utils::json_essential_data(&essential_data));
    // log_debug!("DoenetCore creation warnings, {:?}", doenet_ml_warnings);

    log!("create json objects: {:?}", start.elapsed());

    Ok((DoenetCore {
        component_nodes,
        component_states,
        root_component_name,
        dependencies,
        essential_data,
    }, doenet_ml_warnings, doenet_ml_errors))
}


/// Add CopySource info
fn convert_ml_components_into_component_nodes(
    ml_components: HashMap<ComponentName, MLComponent>,
    doenet_ml_warnings: &mut Vec<DoenetMLWarning>,
    _doenet_ml_errors: &mut Vec<DoenetMLError>,
) -> Result<HashMap<ComponentName, ComponentNode>, DoenetMLError> {
    let mut component_nodes = HashMap::new();
    for (name, ml_component) in ml_components.iter() {
        
        let copy_source = copy_source_for_ml_component(
            &ml_components,
            ml_component,
            doenet_ml_warnings,
        )?;

        let component_node = ComponentNode {
            name: name.clone(),
            parent: ml_component.parent.clone(),
            children: ml_component.children.clone(),
            copy_source,
            static_attributes: ml_component.static_attributes.clone(),
            definition: ml_component.definition,
        };

        component_nodes.insert(name.clone(), component_node);
    }

    Ok(component_nodes)
}

fn copy_source_for_ml_component(
    ml_components: &HashMap<ComponentName, MLComponent>,
    ml_component: &MLComponent,
    doenet_ml_warnings: &mut Vec<DoenetMLWarning>,
) -> Result<Option<CopySource>, DoenetMLError> {

    let source_comp_name = ml_component.copy_source.as_ref();
    if source_comp_name.is_none() {
        return Ok(None);
    }
    let source_comp_name = source_comp_name.unwrap();

    let source_comp = ml_components
        .get(source_comp_name);

    if source_comp.is_none() {
        doenet_ml_warnings.push(DoenetMLWarning::ComponentDoesNotExist {
            comp_name: source_comp_name.to_string(),
            doenetml_range: RangeInDoenetML::None,
        });
        return Ok(None);
    }

    let source_comp = source_comp.unwrap();

    let source_def = source_comp.definition;


    let copy_prop = ml_component.copy_prop.as_ref();
    if copy_prop.is_none() {
        if !std::ptr::eq(ml_component.definition, source_def) {
            return Err(DoenetMLError::ComponentCannotCopyOtherType {
                component_name: ml_component.name.clone(),
                component_type: ml_component.definition.component_type,
                source_type: &source_def.component_type,
                doenetml_range: RangeInDoenetML::None,
            });
        }

        return Ok(Some(CopySource::Component(source_comp_name.clone())));
    }
    let copy_prop = copy_prop.unwrap();


    let source_sv_name = source_def
        .state_var_definitions
        .get_key_value_ignore_case(copy_prop.as_str());

    if source_sv_name.is_none() {
        doenet_ml_warnings.push(DoenetMLWarning::StateVarDoesNotExist {
            comp_name: source_comp.name.clone(),
            sv_name: copy_prop.clone(),
            doenetml_range: RangeInDoenetML::None,
        });
        return Ok(None);
    }
    
    let source_sv_name = source_sv_name.unwrap().0;

    Ok(Some(CopySource::StateVar(ComponentRefState(
        source_comp_name.clone(),
        source_sv_name
    ))))

}


fn create_dependencies_and_essential_data(
    component_nodes: &HashMap<ComponentName, ComponentNode>,
    component_attributes: &HashMap<ComponentName, HashMap<AttributeName, HashMap<usize, Vec<ObjectName>>>>,
    existing_essential_data: Option<HashMap<ComponentName, HashMap<EssentialDataOrigin, EssentialStateVar>>>,
) -> (HashMap<DependencyKey, Vec<Dependency>>, HashMap<ComponentName, HashMap<EssentialDataOrigin, EssentialStateVar>>) {

    let mut all_state_var_defs: Vec<(&ComponentName, StateVarName, &StateVarVariant)> = Vec::new();
    for (_, comp) in component_nodes.iter() {
        for (sv_name, sv_def) in comp.definition.state_var_definitions {
            all_state_var_defs.push((&comp.name, sv_name, sv_def));
        }
    }


    // Fill in component_states and dependencies HashMaps for every component
    // and supply any essential_data required by dependencies.
    let should_initialize_essential_data = existing_essential_data.is_none();
    let mut essential_data = existing_essential_data.unwrap_or(HashMap::new());

    let mut dependencies = HashMap::new();

    for component in component_nodes.values() {

        let dependencies_for_this_component = create_all_dependencies_for_component(
            &component_nodes,
            component,
            component_attributes.get(&component.name).unwrap_or(&HashMap::new()),
            // copy_index_flags.get(component_name).as_deref(),
            &mut essential_data,
            should_initialize_essential_data,
        );
        dependencies.extend(dependencies_for_this_component);



    }
    (dependencies, essential_data)
}

fn create_all_dependencies_for_component<'a>(
    components: &'a HashMap<ComponentName, ComponentNode>,
    component: &'a ComponentNode,
    component_attributes: &'a HashMap<AttributeName, HashMap<usize, Vec<ObjectName>>>,
    // copy_index_flag: Option<&(ComponentName, StateVarName, Vec<ObjectName>)>,
    essential_data: &'a mut HashMap<ComponentName, HashMap<EssentialDataOrigin, EssentialStateVar>>,
    should_initialize_essential_data: bool,
) -> HashMap<DependencyKey, Vec<Dependency>> {

    // log_debug!("Creating dependencies for {}", component.name);
    let mut dependencies: HashMap<DependencyKey, Vec<Dependency>> = HashMap::new();
    let my_definitions = component.definition.state_var_definitions;


    for (&state_var_name, state_var_variant) in my_definitions {

        let dependency_instructions = state_var_variant.return_dependency_instructions(HashMap::new());

        let component_state = ComponentState(
            component,
            state_var_name,
        );
        for (instruct_name, ref dep_instruction) in dependency_instructions.into_iter() {
            let instruct_dependencies = create_dependencies_from_instruction(
                &components,
                &component_state,
                component_attributes,
                dep_instruction,
                instruct_name,
                essential_data,
                should_initialize_essential_data
            );

            dependencies.insert(
                DependencyKey(component.name.clone(), component_state.1.clone(), instruct_name),
                instruct_dependencies   
            );
        }

    }

    dependencies

}

/// This function also creates essential data when a DependencyInstruction asks for it.
/// The second return is element specific dependencies.
fn create_dependencies_from_instruction(
    components: &HashMap<ComponentName, ComponentNode>,
    component_state: &ComponentState,
    component_attributes: &HashMap<AttributeName, HashMap<usize, Vec<ObjectName>>>,
    instruction: &DependencyInstruction,
    instruction_name: InstructionName,
    essential_data: &mut HashMap<ComponentName, HashMap<EssentialDataOrigin, EssentialStateVar>>,
    should_initialize_essential_data: bool,
) -> Vec<Dependency> {

    // log_debug!("Creating dependency {}:{} from instruction {:?}", component_state, instruction_name, instruction);

    let component = component_state.0;
    let state_var_name = &component_state.1;

    match &instruction {

        DependencyInstruction::Essential { prefill } => {

            let source_name = get_recursive_copy_source_component_when_exists(components, component);
            let essential_origin = EssentialDataOrigin::StateVar(state_var_name);

            if should_initialize_essential_data && source_name == component.name {
                // Components only create their own essential data

                let sv_def = component.definition.state_var_definitions.get(state_var_name).unwrap();

                let initial_data: StateVarValue = prefill
                    .and_then(|prefill_attr_name| component_attributes
                        .get(prefill_attr_name)
                        .and_then(|attr| {
                            attr.get(&1).unwrap()
                                .first().unwrap()
                                .as_string().and_then(|actual_str|
                                    package_string_as_state_var_value(actual_str.to_string(), sv_def).ok(),
                                )
                            })
                        )
                    .unwrap_or(sv_def.initial_essential_value());

                let initial_data = InitialEssentialData::Single(initial_data);
    
                create_essential_data_for(
                    &source_name,
                    essential_origin.clone(),
                    initial_data,
                    essential_data
                );
            }

            vec![Dependency::Essential {
                component_name: source_name ,
                origin: essential_origin,
            }]
        },

        DependencyInstruction::StateVar { component_name, state_var_name } => {

            let component_name = component_name.clone()
                .unwrap_or(component.name.clone());

            vec![Dependency::StateVar { 
                component_name,
                state_var_name }]
        },

        DependencyInstruction::Parent { state_var_name } => {

            let parent_name = component.parent.clone().expect(&format!(
                "Component {}:{} asks for a parent but there is none.",
                component.name, instruction_name
            ));


            vec![Dependency::StateVar { 
                component_name: parent_name,
                state_var_name }]
        },

        DependencyInstruction::Child { desired_profiles, parse_into_expression } => {

            enum RelevantChild<'a> {
                StateVar(Dependency),
                String(&'a String, &'a ComponentNode), // value, parent name
            }

            let mut relevant_children: Vec<RelevantChild> = Vec::new();
            let can_parse_into_expression = *parse_into_expression;
            
            let source_relative =
                get_recursive_copy_source_component_when_exists(components, component);
            let source = components.get(&source_relative).unwrap();
            
            if let Some(CopySource::StateVar(ref component_state)) = source.copy_source {
                // copying a state var means we don't inheret its children,
                // so we depend on it directly
                relevant_children.push(
                    RelevantChild::StateVar(Dependency::StateVar { 
                        component_name: component_state.0.clone(),
                        state_var_name: component_state.1.clone()
                     })
                );
            }


            let children = get_child_nodes_including_copy(components, component);

            for child in children.iter() {

                match child {
                    (ComponentChild::Component(child_name), _) => {

                        let child_node = components.get(child_name).unwrap();
                        let child_def = definition_as_replacement_child(child_node);


                        if let Some(profile_sv) = child_def.component_profile_match(desired_profiles) {
                            relevant_children.push(
                                RelevantChild::StateVar(Dependency::StateVar { 
                                    component_name: child_node.name.clone(),
                                    state_var_name: profile_sv
                                 })
                            );
                        }
                    },
                    (ComponentChild::String(string_value), actual_parent) => {
                        if desired_profiles.contains(&ComponentProfile::Text)
                            || desired_profiles.contains(&ComponentProfile::Number) {
                            relevant_children.push(
                                RelevantChild::String(string_value, actual_parent)
                            );
                        }
                    },
                }
            }

            let mut dependencies = Vec::new();

            if can_parse_into_expression {

                // Assuming for now that expression is math expression
                let expression = MathExpression::new(
                    &relevant_children.iter().map(|child| match child {
                        // The component name doesn't matter, the expression just needs to know there is
                        // an external variable at that location
                        RelevantChild::StateVar(_) => ObjectName::Component(String::new()),
                        RelevantChild::String(string_value, _) => ObjectName::String(string_value.to_string()),
                    }).collect()
                );

                // Assuming that no other child instruction exists which has already filled
                // up the child essential data
                let essential_origin = EssentialDataOrigin::ComponentChild(0);

                if should_initialize_essential_data {
                    create_essential_data_for(
                        &component.name,
                        essential_origin.clone(),
                        InitialEssentialData::Single(
                            StateVarValue::MathExpr(expression),
                        ),
                        essential_data
                    );    
                }

                dependencies.push(Dependency::Essential {
                    component_name: component.name.clone(),
                    origin: essential_origin,
                });

                // We already dealt with the essential data, so now only retain the component children
                relevant_children.retain(|child| matches!(child, RelevantChild::StateVar(_)));
                
            }

            // Stores how many string children added per parent.
            let mut essential_data_numbering: HashMap<ComponentName, usize> = HashMap::new();

            for relevant_child in relevant_children {
                match relevant_child {

                    RelevantChild::StateVar(child_dep) => {
                        dependencies.push(child_dep);
                    },

                    RelevantChild::String(string_value, actual_parent) => {
                        let index = essential_data_numbering
                            .entry(actual_parent.name.clone()).or_insert(0 as usize);

                        let essential_origin = EssentialDataOrigin::ComponentChild(*index);

                        if should_initialize_essential_data && std::ptr::eq(component, actual_parent) {
                            // Components create their own essential data

                            let value = StateVarValue::String(string_value.clone());
                            create_essential_data_for(
                                &actual_parent.name,
                                essential_origin.clone(),
                                InitialEssentialData::Single(value),
                                essential_data
                            );
                        }

                        dependencies.push(Dependency::Essential {
                            component_name: actual_parent.name.clone(),
                            origin: essential_origin,
                        });

                        *index += 1;
                    },
                }
            }
            
            dependencies
        },

        DependencyInstruction::Attribute { attribute_name } => {

            // log_debug!("Getting attribute {} for {}", attribute_name, component_slice);
            let sv_def = component.definition.state_var_definitions.get(state_var_name).unwrap();
            let essential_origin = EssentialDataOrigin::StateVar(state_var_name);


            let default_value = match sv_def {

                StateVarVariant::Number(_) | 
                StateVarVariant::Integer(_) => {
                    StateVarValue::MathExpr(MathExpression::new(
                        &vec![ObjectName::String(match sv_def.initial_essential_value() {
                            StateVarValue::Number(v) => v.to_string(),
                            StateVarValue::Integer(v) => v.to_string(),
                            _ => unreachable!(),
                        })]
                    ))
                },
                _ => sv_def.initial_essential_value(),
            };

            let attribute = component_attributes.get(*attribute_name);
            if attribute.is_none() {
                if let Some(CopySource::Component(component_name)) = &component.copy_source {

                    // inherit attribute from copy source

                    return vec![Dependency::StateVar {
                        component_name: component_name.clone(),
                        state_var_name
                     }]
                }

                if should_initialize_essential_data {
                    create_essential_data_for(
                        &component.name,
                        EssentialDataOrigin::StateVar(state_var_name),
                        InitialEssentialData::Single(default_value),
                        essential_data
                    );    
                }

                return vec![Dependency::Essential {
                    component_name: component.name.clone(),
                    origin: essential_origin,
                }]
            }

            // attribute specified
            let attribute = attribute.unwrap();

            // log_debug!("attribute {:?}", attribute);

            // Create the essential data if it does not exist yet
            if should_initialize_essential_data && !essential_data_exists_for(&component.name, &essential_origin, essential_data) {

                let get_value_from_object_list = |obj_list: &Vec<ObjectName>| -> StateVarValue {

                    if matches!(sv_def, StateVarVariant::Number(_)
                        | StateVarVariant::Integer(_)
                        | StateVarVariant::Boolean(_)
                    ) {
                        StateVarValue::MathExpr(
                            MathExpression::new(obj_list)
                        )
                    } else if obj_list.len() > 0 {

                        let first_obj = obj_list.get(0).unwrap();
                        if obj_list.len() > 1 {
                            unimplemented!("Multiple objects for non mathexpression state var");
                        }
                        match first_obj {
                            ObjectName::String(str_val) => {
                                package_string_as_state_var_value(str_val.to_string(), &sv_def).unwrap()
                            }
                            _ => default_value.clone()
                        }
                    } else {
                        default_value.clone()
                    }
                };

                let initial_essential_data;


                assert_eq!(attribute.keys().len(), 1);
                let obj_list = attribute.get(&1).unwrap();

                // log_debug!("Initializing non-array essential data for {} from attribute data {:?}", component_slice, obj_list);

                let value = get_value_from_object_list(obj_list);
                initial_essential_data = InitialEssentialData::Single(value);                    

                create_essential_data_for(
                    &component.name,
                    essential_origin.clone(),
                    initial_essential_data,
                    essential_data,
                );
            }


            let attribute_index = 1;
            let attr_objects = attribute.get(&attribute_index)
                .expect(&format!("attribute {} does not have index {}. Attribute: {:?}",
                    component.name, &attribute_index, attribute));

            let mut dependencies = Vec::new();

            let relevant_attr_objects = match sv_def {
                StateVarVariant::Number(_) |
                StateVarVariant::Integer(_) => {
                    // First add an essential dependency to the expression
                    dependencies.push(Dependency::Essential {
                        component_name: component.name.clone(),
                        origin: essential_origin.clone(),
                    });

                    attr_objects.into_iter().filter_map(|obj|
                        matches!(obj, ObjectName::Component(_)).then(|| obj.clone())
                    ).collect()
                },
                _ => attr_objects.clone(),
            };

            for attr_object in relevant_attr_objects {

                let dependency = match attr_object {
                    ObjectName::String(_) => Dependency::Essential {
                        component_name: component.name.clone(),
                        origin: essential_origin.clone(),
                    },
                    ObjectName::Component(comp_name) => {
                        let comp = components.get(&comp_name).unwrap();
                        let primary_input_sv = comp.definition.primary_input_state_var.expect(
                            &format!("An attribute cannot depend on a non-primitive component. Try adding '.value' to the macro.")
                        );

                        Dependency::StateVar { 
                            component_name: comp_name,
                            state_var_name: primary_input_sv
                         }
                    },
                };

                dependencies.push(dependency);
            }

            dependencies
        },
    }
}


fn package_string_as_state_var_value(input_string: String, state_var_variant: &StateVarVariant)
    -> Result<StateVarValue, String> {

    match state_var_variant {
        StateVarVariant::String(_) => {
            Ok(StateVarValue::String(input_string))
        },

        StateVarVariant::Boolean(_) => {

            if input_string == "true" {
                Ok(StateVarValue::Boolean(true))
            } else if input_string == "false" {
                Ok(StateVarValue::Boolean(false))
            } else {
                Err(format!("Cannot evaluate string '{}' as boolean", input_string))
            }
        },

        StateVarVariant::Integer(_) => {
            if let Ok(val) = evalexpr::eval_int(&input_string) {
                Ok(StateVarValue::Integer(val))
            } else {
                Err(format!("Cannot package string '{}' as integer", input_string))
        }
        },

        StateVarVariant::Number(_) => {
            if let Ok(val) = evalexpr::eval_number(&input_string) {
                Ok(StateVarValue::Number(val))
            } else {
                Err(format!("Cannot package string '{}' as number", input_string))
            }
        },
    }
}

/// Recurse until the name of the original source is found.
/// This allows copies to share essential data.
fn get_recursive_copy_source_component_when_exists(
    components: &HashMap<ComponentName, ComponentNode>,
    component: &ComponentNode,
) -> ComponentName {
    match &component.copy_source {
        Some(CopySource::Component(source_name)) => {
            get_recursive_copy_source_component_when_exists(
                components,
                &components.get(&source_name.clone()).unwrap(),
            )
        },
        _ => component.name.clone(),
    }
}



#[derive(Debug, Clone)]
struct EssentialState (ComponentName, EssentialDataOrigin);

/// Essential data can be generated by
/// - a state variable requesting it
/// - a string child, converted into essential data
///   so that it can change when requested
/// - a string in an attribute
#[derive(Serialize, Debug, Clone, Eq, Hash, PartialEq)]
pub enum EssentialDataOrigin {
    StateVar(StateVarName),
    ComponentChild(usize),
    // AttributeString(usize),
}

/// A single essential state
enum InitialEssentialData {
    Single(StateVarValue),
}

/// Add essential data for a state variable or string child
fn create_essential_data_for(
    component_name: &ComponentName,
    origin: EssentialDataOrigin,
    initial_values: InitialEssentialData,
    essential_data: &mut HashMap<ComponentName, HashMap<EssentialDataOrigin, EssentialStateVar>>,
) {

    if let Some(comp_essential_data) = essential_data.get(component_name) {
        assert!( !comp_essential_data.contains_key(&origin) );
    }

    let essential_state = match initial_values {
        InitialEssentialData::Single(value) => EssentialStateVar(value)
    };

    // log_debug!("New essential data for {} {:?} {:?}", component_name, origin, essential_state);

    essential_data
        .entry(component_name.clone())
        .or_insert(HashMap::new())
        .entry(origin.clone())
        .or_insert(essential_state);
}

fn essential_data_exists_for(
    component_name: &ComponentName,
    origin: &EssentialDataOrigin,
    essential_data: &HashMap<ComponentName, HashMap<EssentialDataOrigin, EssentialStateVar>>
) -> bool {

    if let Some(comp_essen) = essential_data.get(component_name) {
        comp_essen.contains_key(origin)
    } else {
        false
    }
}



fn create_stale_component_states(component_nodes: &HashMap<ComponentName, ComponentNode>)
    -> HashMap<ComponentName, HashMap<StateVarName, StateVar>> {

    let mut component_states = HashMap::new();
    for  component in component_nodes.values() {

        let state_for_this_component: HashMap<StateVarName, StateVar> =
            component.definition.state_var_definitions.iter()
            .map(|(&sv_name, sv_variant)| (sv_name, StateVar::new(&sv_variant)))
            .collect();
            
        component_states.insert(
            component.name.clone(),
            state_for_this_component,
        );
    }
    component_states
}



/// A single state variable
#[derive(Debug, Clone)]
struct ComponentState<'a> (&'a ComponentNode, StateVarName);


fn resolve_state_variable(
    core: &DoenetCore,
    component_state: &ComponentState,
) -> Option<StateVarValue> {


    // No need to continue if the state var is already resolved or if the element does not exist
    let current_state = component_state.get_value(&core.component_states);
    if let Some(State::Resolved(current_value)) = current_state {
        return Some(current_value);
    } else if current_state.is_none() {
        // There is nothing to resolve
        // log_debug!("{} does not exist", component_state);
        return None
    }

    let my_dependencies = dependencies_of_state_var(&core.dependencies, component_state);
    // log_debug!(">> Resolving {} \nIt has dependencies {:?}", component_state, my_dependencies);

    let mut dependency_values: HashMap<InstructionName, Vec<DependencyValue>> = HashMap::new();
    for (dep_name, deps) in my_dependencies {
        let mut values_for_this_dep: Vec<DependencyValue> = Vec::new();

        for dep in deps {
            let dependency_source = get_source_for_dependency(&core.component_nodes, &core.essential_data, &dep);

            match dep {
                Dependency::StateVar { component_name, state_var_name } => {
                    let new_node = core.component_nodes.get(component_name).unwrap();
                    let new_component_state = ComponentState(&new_node, state_var_name);
                    if let Some(state_var_value) = resolve_state_variable(core, &new_component_state) {
                        values_for_this_dep.push(DependencyValue { source: dependency_source.clone(), value: state_var_value });
                    }
                },

                Dependency::Essential { component_name, origin } => {


                    let value = core.essential_data
                        .get(&component_name.clone()).unwrap()
                        .get(&origin).unwrap()
                        .clone();
    
                    values_for_this_dep.push(DependencyValue {
                        source: dependency_source,
                        value: value.0,
                    })
                },
            }
        }

        dependency_values.insert(dep_name, values_for_this_dep);
    }


    // log_debug!("Dependency values for {}: {:#?}", component_state, dependency_values);

    let node = &component_state.0;

    let update_instruction = generate_update_instruction_for_state(
        component_state,
        dependency_values,
    ).expect(&format!("Can't resolve {} (a {} component type)",
        component_state, node.definition.component_type)
    );

    let updated_value: Option<StateVarValue>;

    match update_instruction {
        StateVarUpdateInstruction::NoChange => {
            match current_state {
                Some(State::Stale) => 
                    panic!("Cannot use NoChange update instruction on a stale value"),
                Some(State::Resolved(current_resolved_value)) => {
                    // Do nothing. It's resolved, so we can use it as is
                    updated_value = Some(current_resolved_value);
                },
                None => {
                    updated_value = None;
                },
            }
        },
        StateVarUpdateInstruction::SetValue(new_value) => {

            updated_value = Some(component_state.set_value(&core.component_states, new_value));
        }

    };

    // log_debug!("Updated {} to {:?}", component_state, updated_value);

    return updated_value;
}


/// This determines the state var given its dependency values.
fn generate_update_instruction_for_state(
    component_state: &ComponentState,
    dependency_values: HashMap<InstructionName, Vec<DependencyValue>>

) -> Result<StateVarUpdateInstruction<StateVarValue>, String> {


    let state_var_def = &component_state.0.definition
        .state_var_definitions.get(component_state.1).unwrap();


    state_var_def.determine_state_var_from_dependencies(dependency_values)

}



// TODO: Use &Dependency instead of cloning
fn dependencies_of_state_var<'a>(
    dependencies: &'a HashMap<DependencyKey, Vec<Dependency>>,
    component_state: &ComponentState,
) -> HashMap<InstructionName, Vec<&'a Dependency>> {
    let component_node = &component_state.0;
    let state_var_name = &component_state.1;

    // TODO: this is inefficient!!!
    let deps = dependencies.iter().filter_map(| (key, deps) | {

        let key_is_me = key.0 == component_node.name && (
            key.1 == state_var_name.clone()
        );

        key_is_me.then(|| (key.2, deps))
    });

    let mut combined: HashMap<InstructionName, Vec<&Dependency>> = HashMap::new();
    for (k, v) in deps {
        if let Some(accum) = combined.get_mut(k) {
            let dedup: Vec<&Dependency> = v.iter().filter(|&x| !contains_ptr(accum, &x)).collect();
            accum.extend(dedup);
        } else {
            combined.insert(k, v.iter().collect());
        }
    }
    
    combined
}

fn contains_ptr<T>(v: &Vec<&T>, e: &T) -> bool {
    v.iter().any(|&x| std::ptr::eq(x, e))
}

fn get_source_for_dependency(
    component_nodes: &HashMap<ComponentName, ComponentNode>,
    essential_data: &HashMap<ComponentName, HashMap<EssentialDataOrigin, EssentialStateVar>>,
    dependency: &Dependency,
) -> DependencySource {

    match dependency {
        Dependency::Essential { component_name, origin } => {

            let data = essential_data.get(&component_name.clone()).unwrap().get(origin).unwrap();

            DependencySource::Essential {
                value_type: data.0.type_as_str()
            }

        },

        Dependency::StateVar { component_name, state_var_name } => {
            let component_type = component_nodes.get(component_name).unwrap().definition
                .component_type;

            DependencySource::StateVar {
                component_type,
                state_var_name
            }
        },

    }
}

/// Also includes the values of essential data
fn get_dependency_sources_for_state_var<'a>(
    component_nodes: &'a HashMap<ComponentName, ComponentNode>,
    dependencies: &'a HashMap<DependencyKey, Vec<Dependency>>,
    essential_data: &'a mut HashMap<ComponentName, HashMap<EssentialDataOrigin, EssentialStateVar>>,
    component_state: &ComponentState,
) -> HashMap<InstructionName, Vec<(DependencySource, Option<StateVarValue>)>> {

    let my_dependencies = dependencies_of_state_var(dependencies, component_state);
    let mut dependency_sources: HashMap<InstructionName, Vec<(DependencySource, Option<StateVarValue>)>> = HashMap::new();

    for (instruction_name, new_dependencies) in my_dependencies {
        let instruction_sources: Vec<(DependencySource, Option<StateVarValue>)> = new_dependencies.iter().map(|dependency| {
            let source = get_source_for_dependency(component_nodes, essential_data, &dependency);

            let essential_value = if let Dependency::Essential { origin, .. } = dependency {
                let data = essential_data
                    .get(&component_state.0.name).unwrap()
                    .get(origin).unwrap();
                let value = data.0.clone();
                Some(value)

            } else {
                None
            };

            (source, essential_value)
        }).collect();

        dependency_sources.insert(instruction_name, instruction_sources);
    }

    dependency_sources
}


fn mark_stale_state_var_and_dependencies<'a>(
    dependencies: &'a HashMap<DependencyKey, Vec<Dependency>>,
    component_nodes: &'a HashMap<ComponentName, ComponentNode>,
    component_states: &'a mut HashMap<ComponentName, HashMap<StateVarName, StateVar>>,
    component_state: &ComponentState,
) {
    // log_debug!("Check stale {:?}", state);

    let state = component_states
        .get(&component_state.0.name.clone())
        .expect(&format!("Error accessing state of {:?}", component_state.0))
        .get(component_state.1)
        .expect(&format!("Error accessing state variable {} of {:?}", component_state.1, component_state.0));
        


    state.mark_stale();

    let depending_on_me = get_state_variables_depending_on_me(dependencies, component_nodes, &component_state);

    for new_component_state in depending_on_me {
        mark_stale_state_var_and_dependencies(dependencies, component_nodes, component_states, &new_component_state);
    }
}

fn mark_stale_essential_datum_dependencies<'a>(
    component_nodes: &'a HashMap<ComponentName, ComponentNode>,
    dependencies: &'a HashMap<DependencyKey, Vec<Dependency>>,
    component_states: &'a mut HashMap<ComponentName, HashMap<StateVarName, StateVar>>,
    essential_state: &EssentialState,
) {
    let component_name = essential_state.0.clone();
    let origin = essential_state.1.clone();

    // log_debug!("Marking stale essential {}:{}", component_name, state_var);

    let search_dep = Dependency::Essential {
        component_name,
        origin,
    };

    let my_dependencies = dependencies.iter().filter_map( |(key, deps) | {
        if deps.contains(&search_dep) {
            Some(ComponentState(component_nodes.get(&key.0).unwrap(), key.1))
        } else {
            None
        }
    });

    for component_state in my_dependencies {
        mark_stale_state_var_and_dependencies(dependencies, component_nodes, component_states, &component_state);
    }
}

/// Calculate all the state vars that depend on the given state var
fn get_state_variables_depending_on_me<'a>(
    dependencies: &'a HashMap<DependencyKey, Vec<Dependency>>,
    component_nodes: &'a HashMap<ComponentName, ComponentNode>,
    component_state: &'a ComponentState,
) -> Vec<ComponentState<'a>> {

    let sv_component = &component_state.0;
    let sv_name = &component_state.1;

    let mut depending_on_me = vec![];

    for (_, dependencies) in dependencies.iter() {
        for dependency in dependencies {

            match dependency {
                Dependency::StateVar { component_name, state_var_name } => {
                    if sv_component.name == *component_name && sv_name == state_var_name {
                        let node = component_nodes.get(&component_name.clone()).unwrap();
                        depending_on_me.push(ComponentState(node, state_var_name));
                    }
                },

                // Essential dependencies are endpoints
                Dependency::Essential { .. } => {},

            }
        }
    }


    depending_on_me
}



pub fn update_renderers(core: &DoenetCore) -> String {
    let json_obj = generate_render_tree(core);

    log_json!("Component tree after renderer update", utils::json_components(&core.component_nodes, &core.component_states));

    log_json!("Essential data after renderer update",
    utils::json_essential_data(&core.essential_data));

    serde_json::to_string(&json_obj).unwrap()
}

fn generate_render_tree(core: &DoenetCore) -> serde_json::Value {
    let start = Instant::now();

    let root_node = core.component_nodes.get(&core.root_component_name).unwrap();

    let root_comp_rendered = RenderedComponent {
        component_node: root_node,
        child_of_copy: None
    };
    let mut json_obj: Vec<serde_json::Value> = vec![];

    log!("===== Render tree ======");
    generate_render_tree_internal(core, root_comp_rendered, &mut json_obj);


    log!("generated renderer tree: {:?}", start.elapsed());

    serde_json::Value::Array(json_obj)
}

fn generate_render_tree_internal(
    core: &DoenetCore,
    component: RenderedComponent,
    json_obj: &mut Vec<serde_json::Value>,
) {
    use serde_json::{Map, Value, json};

    // log_debug!("generating render tree for {}", component);

    let component_definition = component.component_node.definition;

    let renderered_state_vars = component_definition
        .state_var_definitions
        .iter()
        .filter_map(|(k, v)| {
            v.for_renderer().then(|| k)
        });

    let state_var_aliases = match &component_definition.renderer_type {
        RendererType::Special { state_var_aliases, .. } => state_var_aliases.clone(),
        RendererType::Myself => HashMap::new(),
    };

    let mut state_values = serde_json::Map::new();
    for state_var_name in renderered_state_vars {

        let value = resolve_state_variable(core, &ComponentState(component.component_node, &state_var_name)).unwrap();

        let sv_renderer_name = state_var_aliases
            .get(state_var_name)
            .map(|x| *x)
            .unwrap_or(state_var_name)
            .to_string();


        let json_value = json!(value);


        state_values.insert(sv_renderer_name, json_value);
    }

    let name_to_render = name_rendered_component(&component, component_definition.component_type);

    let mut children_instructions = Vec::new();
    if component_definition.should_render_children {
        for (child, actual_parent) in get_child_refs_including_copy_and_members(core, component.component_node) {
            match child {
                ObjectRef::String(string) => {
                    children_instructions.push(json!(string));
                },
                ObjectRef::Component(comp_node) => {
                    let child_component = RenderedComponent {
                        component_node: comp_node,
                        child_of_copy: component.child_of_copy.clone().or(
                            (actual_parent.name != component.component_node.name).then(|| component.component_node)
                        ),
                    };

                    let child_definition = child_component.component_node.definition;

                    let child_name = name_rendered_component(&child_component, child_definition.component_type);

                    let action_component_name = child_component.component_node.name.clone();

                    let child_actions: Map<String, Value> =
                        (child_definition.action_names)()
                        .iter()
                        .map(|action_name| (action_name.to_string(), json!({
                            "actionName": action_name,
                            "componentName": action_component_name,
                        }))).collect();

                    let renderer_type = match &child_definition.renderer_type {
                        RendererType::Special{ component_type, .. } => *component_type,
                        RendererType::Myself => child_definition.component_type,
                    };

                    children_instructions.push(json!({
                        "actions": child_actions,
                        "componentName": child_name,
                        "componentType": child_definition.component_type,
                        "effectiveName": child_name,
                        "rendererType": renderer_type,
                    }));

                    generate_render_tree_internal(core, child_component, json_obj); 
                },
            }
        }
    }

    json_obj.push(json!({
        "componentName": name_to_render,
        "stateValues": serde_json::Value::Object(state_values),
        "childrenInstructions": json!(children_instructions),
    }));

}

fn name_rendered_component(component: &RenderedComponent, _component_type: &str) -> String {
    let name_to_render = component.component_node.name.clone();
    
    let name_to_render = match &component.child_of_copy {
        Some(copy_name) => format!("__cp:{}({})", name_to_render, copy_name.name),
        None => name_to_render,
    };

    name_to_render
}




#[derive(Debug)]
pub struct Action {
    pub component_name: ComponentName,
    pub action_name: String,

    /// The keys are not state variable names.
    /// They are whatever name the renderer calls the new value.
    pub args: HashMap<String, Vec<StateVarValue>>,
}

/// Internal structure used to track changes
#[derive(Debug, Clone)]
enum UpdateRequest<'a> {
    SetEssentialValue(EssentialState, StateVarValue),
    SetStateVar(ComponentState<'a>, StateVarValue),
}

pub fn handle_action_from_json(core: &mut DoenetCore, action: &str) -> String {

    log!("handle action {}", action);

    let (action, action_id) = parse_json::parse_action_from_json(action)
        .expect(&format!("Error parsing json action: {}", action));

    if action.action_name != "recordVisibilityChange" {
        log!("actually handling action");
        handle_action(core, action);
    }

    action_id
}

pub fn handle_action(core: &mut DoenetCore, action: Action) {

    // log_debug!("Handling action {:#?}", action);


    let component = core.component_nodes.get(&action.component_name).unwrap();

    let state_var_resolver = | state_var_name: &StateVarName | {
        let component_state = ComponentState(&component, state_var_name);
        resolve_state_variable(&core, &component_state)
    };

    let state_vars_to_update = (component.definition.on_action)(
        &action.action_name,
        action.args,
        &state_var_resolver,
    );

    for (state_var_name, requested_value) in state_vars_to_update {

        let component_state = ComponentState(&component, state_var_name);
        let request = UpdateRequest::SetStateVar(component_state, requested_value);
        process_update_request(&core.component_nodes, &core.dependencies, &mut core.component_states, &mut core.essential_data, &request);
    }

    // log_json!("Component tree after action", utils::json_components(&core.component_nodes, &core.component_states));

}


/// Convert the results of `request_dependencies_to_update_value`
/// into UpdateRequest struct.
fn convert_dependency_values_to_update_request<'a>(
    component_nodes: &'a HashMap<ComponentName, ComponentNode>,
    dependencies: &'a HashMap<DependencyKey, Vec<Dependency>>,
    component_state: &'a ComponentState,
    requests: HashMap<InstructionName, Result<Vec<DependencyValue>, String>>,
) -> Vec<UpdateRequest<'a>> {

    let component = component_state.0;
    let state_var = &component_state.1;

    let my_dependencies = dependencies_of_state_var(dependencies, &component_state);

    let mut update_requests = Vec::new();

    for (instruction_name, instruction_requests) in requests {

        let valid_requests = match instruction_requests {
            Err(_e) => {
                // log_debug!("Inverse definition for {} failed with: {}", component_state, _e);
                break;
            },
            Ok(result) => result,
        };



        let instruct_dependencies = my_dependencies.get(instruction_name).expect(
            &format!("{}:{} has the wrong instruction name to determine dependencies",
                component.definition.component_type, state_var)
        );

        assert_eq!(valid_requests.len(), instruct_dependencies.len());

        for (request, dependency) in valid_requests.into_iter().zip(instruct_dependencies.iter()) {

            match dependency {
                Dependency::Essential { component_name, origin } => {
                    update_requests.push(UpdateRequest::SetEssentialValue(
                        EssentialState(component_name.clone(), origin.clone()),
                        request.value.clone(),
                    ))
                },
                Dependency::StateVar { component_name, state_var_name } => {
                    // TODO: receiving multiple dependencies because of multiple instances

                    let component_node = component_nodes.get(&component_name.clone()).unwrap();

                    let component_state = ComponentState(component_node, state_var_name);
                    update_requests.push(UpdateRequest::SetStateVar(component_state, request.value.clone()));

                },
            }
        }

    }

    update_requests

}

fn process_update_request(
    component_nodes: &HashMap<ComponentName, ComponentNode>,
    dependencies: &HashMap<DependencyKey, Vec<Dependency>>,
    component_states: &mut HashMap<ComponentName, HashMap<StateVarName, StateVar>>,
    essential_data: &mut HashMap<ComponentName, HashMap<EssentialDataOrigin, EssentialStateVar>>,
    update_request: &UpdateRequest
) {

    // log_debug!("Processing update request {:?}", update_request);

    match update_request {
        UpdateRequest::SetEssentialValue(essential_state, requested_value) => {

            let essential_var = essential_data
                .get_mut(&essential_state.0).unwrap()
                .get_mut(&essential_state.1).unwrap();

            essential_var.set_value(
                requested_value.clone(),
            ).expect(
                &format!("Failed to set essential value for {:?}", essential_state)
            );

            // log_debug!("Updated essential data {:?}", core.essential_data);

            mark_stale_essential_datum_dependencies(component_nodes, dependencies, component_states, essential_state);
        },

        UpdateRequest::SetStateVar(component_state, requested_value) => {

            let dep_update_requests = request_dependencies_to_update_value_including_shadow(
                component_nodes,
                dependencies,
                essential_data,
                component_state,
                requested_value.clone(),
            );

            for dep_update_request in dep_update_requests {
                process_update_request(component_nodes, dependencies, component_states, essential_data, &dep_update_request);
            }

            // needed?
            // mark_stale_state_var_and_dependencies(core, component_name, &map, &StateVarSlice::Single(state_var_ref.clone()));
        }
    }

}

fn request_dependencies_to_update_value_including_shadow<'a, 'b>(
    component_nodes: &'a HashMap<ComponentName, ComponentNode>,
    dependencies: &'a HashMap<DependencyKey, Vec<Dependency>>,
    essential_data: &'b mut HashMap<ComponentName, HashMap<EssentialDataOrigin, EssentialStateVar>>,
    component_state: &'a ComponentState,
    new_value: StateVarValue,
) -> Vec<UpdateRequest<'a>> {

    let component = component_state.0;
    let state_var_name = &component_state.1;

    if let Some(component_ref_state) = state_var_is_shadowing(&component_state) {

        let source_component = component_nodes.get(&component_ref_state.0).unwrap();
        let source_state_var =component_ref_state.1;
        let source_state = ComponentState(source_component, source_state_var);
        vec![UpdateRequest::SetStateVar(source_state, new_value)]

    } else {

        let dependency_sources = get_dependency_sources_for_state_var(component_nodes, dependencies, essential_data, component_state);

        // log_debug!("Dependency sources for {}, {:?}", component_state, dependency_sources);

        let requests = component.definition.state_var_definitions.get(state_var_name).unwrap()
            .request_dependencies_to_update_value(new_value, dependency_sources)
            .expect(&format!("Failed requesting dependencies for {}", component_state));

        // log_debug!("{} wants its dependency to update to: {:?}", component_state, requests);

        let update_requests = convert_dependency_values_to_update_request(component_nodes, dependencies, component_state, requests);

        // log_debug!("{} generated update requests: {:#?}", component_state, update_requests);

        update_requests
    }
}

/// Detect if a state var is shadowing because of a CopySource
/// and has a primary input state variable, which is needed.
fn state_var_is_shadowing<'a>(component_state: &'a ComponentState)
    -> Option<ComponentRefState> {

    let component = component_state.0;
    let state_var = &component_state.1;
    if let Some(CopySource::StateVar(ref component_ref_state)) = component.copy_source {
        if let Some(primary_input_state_var) = component.definition.primary_input_state_var {

            if state_var == &primary_input_state_var {
                Some(component_ref_state.clone())
            } else {
                None
            }
        } else {
            panic!("{} component type doesn't have a primary input state var", component.definition.component_type);
        }

    } else {
        None
    }
}





#[derive(Debug)]
enum ObjectRef<'a> {
    Component(&'a ComponentNode),
    String(String),
}

fn get_child_refs_including_copy_and_members<'a>(
    core: &'a DoenetCore,
    component_node: &'a ComponentNode,
) -> Vec<(ObjectRef<'a>, &'a ComponentNode)> {

    let mut children_vec: Vec<(ObjectRef, &ComponentNode)> = Vec::new();

    match &component_node.copy_source {
        Some(CopySource::Component(source_name)) => {
            let source_component = core.component_nodes.get(source_name).unwrap();
            children_vec = get_child_refs_including_copy_and_members(core, source_component);
        },
        _ => {},
    }

    children_vec.extend(
        component_node.children
        .iter()
        .flat_map(|c| match c {
            ComponentChild::String(s) => vec![(ObjectRef::String(s.clone()), component_node)],
            ComponentChild::Component(c) => {
                let node = core.component_nodes.get(c).unwrap();
                vec![(ObjectRef::Component(node), component_node)]
            }
        })
    );

    children_vec
}

fn get_child_nodes_including_copy<'a>(
    components: &'a HashMap<ComponentName, ComponentNode>,
    component: &'a ComponentNode,
) -> Vec<(&'a ComponentChild, &'a ComponentNode)> {

    let mut children_vec: Vec<(&ComponentChild, &ComponentNode)> = Vec::new();
    if let Some(CopySource::Component(ref source_name)) = component.copy_source {

        let source_comp = components.get(&source_name.clone()).unwrap();

        children_vec = get_child_nodes_including_copy(components, source_comp);
    }

    children_vec.extend(
        component.children
        .iter()
        .map(|c| (c, component))
    );

    children_vec
}



// ==== Type Implementations ====


impl<'a> ComponentState<'a> {

    fn get_value(&self, component_states: &HashMap<ComponentName, HashMap<StateVarName, StateVar>>)
        -> Option<State<StateVarValue>> {
        Some(component_states.get(&self.0.name).unwrap()
            .get(&self.1)
            .expect(&format!("Component {} has no state var '{}'", self.0.name, self.1))
            .get_state()
        )
    }

    fn set_value(&self, component_states: &HashMap<ComponentName, HashMap<StateVarName, StateVar>>, new_value: StateVarValue)
        -> StateVarValue {
            component_states.get(&self.0.name).unwrap()
                .get(&self.1)
                .expect(&format!("Component {} has no state var '{}'", self.0.name, self.1))
                .set_value(new_value)
                .unwrap()
    }
}



impl Display for ComponentState<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.0.name, self.1)
    }
}

impl Display for RenderedComponent<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(child_of_copy) = &self.child_of_copy {
            write!(f, "{}(child of {})", self.component_node.name, child_of_copy.name)
        } else {
            write!(f, "{}", self.component_node.name)
        }
    }
}


// ==== Groups (Batches/Collections) ====


fn definition_as_replacement_child(component: &ComponentNode) -> &'static ComponentDefinition {
    component.definition.definition_as_replacement_children(&component.static_attributes)
        .unwrap_or(component.definition)
}



// ==== Relative Instance ====
// Find instances of dependencies relative to the component.

#[derive(Debug, Clone)]
pub struct ComponentRefState (ComponentName, StateVarName);



// ==== Error and warning checks during core creating ====

fn check_for_invalid_childen_component_profiles(component_nodes: &HashMap<ComponentName, ComponentNode>) -> Vec<DoenetMLWarning> {
    let mut doenet_ml_warnings = vec![];
    for (_, component) in component_nodes.iter() {
        if let ValidChildTypes::ValidProfiles(ref valid_profiles) = component.definition.valid_children_profiles {

            for child in component.children.iter().filter_map(|child| child.as_component()) {
                let child_comp = component_nodes.get(child).unwrap();
                let mut has_valid_profile = false;
                let child_member_def = child_comp.definition.definition_as_replacement_children(&child_comp.static_attributes).unwrap();
                for (child_profile, _) in child_member_def.component_profiles.iter() {
                    if valid_profiles.contains(child_profile) {
                        has_valid_profile = true;
                        break;
                    }
                }
                if matches!(child_member_def.replacement_components, Some(ReplacementComponents::Children)) {
                    has_valid_profile = true;
                }

                if has_valid_profile == false {
                    doenet_ml_warnings.push(DoenetMLWarning::InvalidChildType {
                        parent_comp_name: component.name.clone(),
                        child_comp_name: child_comp.name.clone(),
                        child_comp_type: child_member_def.component_type,
                        doenetml_range: RangeInDoenetML::None,
                    });
                }
            }
    
        }
    }
    doenet_ml_warnings
}

/// Do this before dependency generation so it doesn't crash
fn check_for_cyclical_copy_sources(_component_nodes: &HashMap<ComponentName, ComponentNode>) -> Result<(), DoenetMLError> {
    // All the components that copy another component, along with the name of the component they copy
    // let copy_comp_targets: Vec<(&ComponentNode, &ComponentRefRelative)> = component_nodes.iter().filter_map(|(_, c)|
    //     match c.copy_source {
    //         Some(CopySource::Component(ref component_ref_relative)) => Some((c, component_ref_relative)),
    //         _ => None,
    //     }
    // ).collect();

    // for (copy_component, _) in copy_comp_targets.iter() {
    //     if let Some(cyclic_error) = check_cyclic_copy_source_component(&component_nodes, copy_component) {
    //         return Err(cyclic_error);
    //     }
    // }
    return Ok(())
}

// fn check_cyclic_copy_source_component(
//     components: &HashMap<ComponentName, ComponentNode>,
//     component: &ComponentNode,

// ) -> Option<DoenetMLError> {

//     let mut current_comp = component;
//     let mut chain = vec![];
//     while let Some(CopySource::Component(ref component_ref_relative)) = current_comp.copy_source {

//         if chain.contains(&current_comp.name) {
//             // Cyclical dependency
//             chain.push(current_comp.name.clone());

//             let start_index = chain.iter().enumerate().find_map(|(index, name)| {
//                 if name == &current_comp.name {
//                     Some(index)
//                 } else {
//                     None
//                 }
//             }).unwrap();

//             let (_, relevant_chain) = chain.split_at(start_index);

//             return Some(DoenetMLError::CyclicalDependency {
//                 component_chain: Vec::from(relevant_chain),
//                 doenetml_range: RangeInDoenetML::None,
//             });


//         } else {

//             chain.push(current_comp.name.clone());
//             current_comp = components.get(&component_ref_relative.of_node_relative().name).unwrap();
//         }
//     }

//     None
// }

fn check_for_invalid_component_names(
    component_nodes: &HashMap<ComponentName, ComponentNode>,
    component_attributes: &HashMap<ComponentName, HashMap<AttributeName, HashMap<usize, Vec<ObjectName>>>>,
) -> Result<(), DoenetMLError> {

    for attributes_for_comp in component_attributes.values() {
        for attributes in attributes_for_comp.values() {
            for attribute_list in attributes.values() {
                for attr_object in attribute_list {

                    if let ObjectName::Component(comp_obj) = attr_object {
                        if !component_nodes.contains_key(comp_obj) {
                            // The component tried to copy a non-existent component.
                            return Err(DoenetMLError::ComponentDoesNotExist {
                                comp_name: comp_obj.to_owned(),
                                doenetml_range: RangeInDoenetML::None,
                            });
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn check_for_cyclical_dependencies(_dependencies: &HashMap<DependencyKey, Vec<Dependency>>) -> Result<(), DoenetMLError> {
   // Now that the dependency graph has been created, use it to check for cyclical dependencies
    // for all the components
    // for (dep_key, _) in dependencies.iter() {
    //     let mut chain = vec![(dep_key.0.clone(), dep_key.1.clone())];
    //     let possible_error = check_for_cyclical_dependency_chain(&dependencies, &mut chain);

    //     if let Some(error) = possible_error {
    //         return Err(error);
    //     }
    // }
    Ok(())
}


