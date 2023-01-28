use serde::{Serialize, Deserialize};

use crate::state::StateVar;
use crate::utils::{log_json, log};
use crate::{Action, ComponentName};
use crate::component::{COMPONENT_DEFINITIONS, ComponentType, ComponentDefinition,
KeyValueIgnoreCase, AttributeName, ObjectName };

use crate::ComponentChild;
use lazy_static::lazy_static;
use regex::Regex;

use std::collections::HashMap;
use std::fmt::Display;
use instant::Instant;

use crate::state_variables::*;

/// This error is caused by invalid DoenetML.
/// It is thrown only on core creation.
#[derive(Debug, PartialEq, Clone)]
pub enum DoenetMLError {

    ComponentDoesNotExist {
        comp_name: String,
        doenetml_range: RangeInDoenetML,
    },
    StateVarDoesNotExist {
        comp_name: ComponentName,
        sv_name: String,
        doenetml_range: RangeInDoenetML,
    },
    AttributeDoesNotExist {
        comp_name: ComponentName,
        attr_name: String,
        doenetml_range: RangeInDoenetML,
    },
    InvalidComponentType {
        comp_type: String,
        doenetml_range: RangeInDoenetML,
    },
    // Note: currently not used
    NonNumericalIndex {
        comp_name: ComponentName,
        invalid_index: String,
        doenetml_range: RangeInDoenetML,
    },
    // Note: currently not used
    InvalidStaticAttribute {
        comp_name: ComponentName,
        attr_name: String,
        doenetml_range: RangeInDoenetML,
    },
    CannotCopyArrayStateVar {
        // copier_comp_name: ComponentName, 
        source_comp_name: ComponentName,
        source_sv_name: StateVarName,
        doenetml_range: RangeInDoenetML,
    },
    CannotCopyIndexForStateVar {
        source_comp_name: ComponentName,
        source_sv_name: StateVarName,
        doenetml_range: RangeInDoenetML,
    },

    DuplicateName {
        name: String,
        doenetml_range: RangeInDoenetML,
    },
    InvalidComponentName {
        name: String,
        doenetml_range: RangeInDoenetML,
    },
    CyclicalDependency {
        component_chain: Vec<ComponentName>,
        doenetml_range: RangeInDoenetML,
    },
    ComponentCannotCopyOtherType {
        component_name: ComponentName,
        component_type: ComponentType,
        source_type: ComponentType,
        doenetml_range: RangeInDoenetML,
    },

    // Note: currently not used
    /// For the componentType static attr of <sources>
    CannotImplySourcesComponentType {
        component_name: ComponentName,
        doenetml_range: RangeInDoenetML,
    }
}

impl std::error::Error for DoenetMLError {}
impl Display for DoenetMLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use DoenetMLError::*;

        match self {
            ComponentDoesNotExist { comp_name, doenetml_range } => 
                write!(f, "Component '{}' does not exist. {}", comp_name, doenetml_range.to_string()),
            StateVarDoesNotExist { comp_name, sv_name, doenetml_range } =>
                write!(f, "State variable '{}' does not exist on {}. {}", sv_name, comp_name, doenetml_range.to_string()),
            AttributeDoesNotExist { comp_name, attr_name, doenetml_range } =>
                write!(f, "Attribute '{}' does not exist on {}. {}", attr_name, comp_name, doenetml_range.to_string()),
            InvalidComponentType { comp_type, doenetml_range } => 
                write!(f, "Component type {} does not exist. {}", comp_type, doenetml_range.to_string()),
            NonNumericalIndex { comp_name, invalid_index, doenetml_range } =>
                write!(f, "Component {} has non-numerical index '{}'. {}", comp_name, invalid_index, doenetml_range.to_string()),
            InvalidStaticAttribute { comp_name, attr_name, doenetml_range } =>
                write!(f, "Component {} attribute '{}' must be static. {}", comp_name, attr_name, doenetml_range.to_string()),
            CannotCopyArrayStateVar { source_comp_name, source_sv_name, doenetml_range } =>
                write!(f, "Cannot copy array state variable '{}' from component {}. {}", source_sv_name, source_comp_name, doenetml_range.to_string()),
            CannotCopyIndexForStateVar { source_comp_name, source_sv_name, doenetml_range } =>
                write!(f, "Cannot use propIndex for state variable '{}' from component {} because this state variable is not an array. {}", source_sv_name, source_comp_name, doenetml_range.to_string()),
            DuplicateName { name, doenetml_range } =>
                write!(f, "The component name {} is used multiple times. {}", name, doenetml_range.to_string()),
            InvalidComponentName { name, doenetml_range } =>
                write!(f, "The component name {} is invalid.  It must begin with a letter and can contain only letters, numbers, hyphens, and underscores. {}", name, doenetml_range.to_string()),
            CyclicalDependency { component_chain, doenetml_range } => {
                let mut msg = String::from("Cyclical dependency through components: ");
                for comp in component_chain {
                    msg.push_str(&format!("{}, ", comp));
                }
                msg.pop();
                msg.pop();

                msg.push_str(&doenetml_range.to_string());

                write!(f, "{}", msg)
            },
            ComponentCannotCopyOtherType { component_name, component_type, source_type, doenetml_range } => {
                write!(f, "The {} component '{}' cannot copy a {} component. {}", component_type, component_name, source_type, doenetml_range.to_string())
            },
            CannotImplySourcesComponentType { component_name, doenetml_range } => write!(f, "Cannot impy 'componentType' attribute of {}. {}", component_name, doenetml_range.to_string()),
        }
    }
}

/// This warning is caused by invalid DoenetML.
/// It is thrown only on core creation, but does not stop core from being created.
#[derive(Debug, PartialEq)]
pub enum DoenetMLWarning {
    PropIndexIsNotPositiveInteger {
        // Note that if there is a macro in the propIndex,
        // we can't know if it is an integer or not, so we don't throw this warning
        comp_name: ComponentName,
        invalid_index: String,
        doenetml_range: RangeInDoenetML,
    },
    InvalidChildType {
        parent_comp_name: ComponentName,
        child_comp_name: ComponentName,
        child_comp_type: ComponentType,
        doenetml_range: RangeInDoenetML,
    },
    ComponentDoesNotExist {
        comp_name: ComponentName,
        doenetml_range: RangeInDoenetML,
    },
    StateVarDoesNotExist {
        comp_name: ComponentName,
        sv_name: String,
        doenetml_range: RangeInDoenetML,
    },
    InvalidArrayIndex {
        comp_name: ComponentName,
        sv_name: Option<String>,
        array_index: String,
        doenetml_range: RangeInDoenetML,
    },
}

impl std::error::Error for DoenetMLWarning {}
impl Display for DoenetMLWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use DoenetMLWarning::*;
        match self {
            PropIndexIsNotPositiveInteger { comp_name, invalid_index, doenetml_range } => {
                write!(f, "Component {} has propIndex '{}' which is not a positive integer. {}", comp_name, invalid_index, doenetml_range.to_string())
            },
            InvalidChildType { parent_comp_name, child_comp_name: _, child_comp_type, doenetml_range } => {
                write!(f, "Component {} cannot have a child component of type {}. {}", parent_comp_name, child_comp_type, doenetml_range.to_string())
            },
            ComponentDoesNotExist { comp_name, doenetml_range } => {
                write!(f, "Component {} does not exist. {}", comp_name, doenetml_range.to_string())
            },
            StateVarDoesNotExist { comp_name, sv_name, doenetml_range } =>
                write!(f, "State variable '{}' does not exist on {}. {}", sv_name, comp_name, doenetml_range.to_string()),
            InvalidArrayIndex { comp_name, sv_name, array_index, doenetml_range } => {
                let sv_description = match sv_name {
                    Some(sv) => format!(" on state variable '{}'", sv),
                    None => String::new()
                };
                write!(f, "Invalid array index {}{} of {}. {}", array_index, sv_description, comp_name, doenetml_range.to_string())
            },
        }

    }
}


// Structures for create_components_tree_from_json
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ComponentTree {
    component_type: String,
    props: Props,
    children: Vec<ComponentOrString>,
    #[serde(default)]
    #[serde(alias = "range")]
    doenetml_range: RangeInDoenetML,
    #[serde(default)]
    allow_underscore_component_type: bool
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Props {
    name: Option<String>,
    copy_source: Option<String>,
    copy_collection: Option<String>,
    copy_prop: Option<String>,
    prop_index: Option<String>,
    component_index: Option<String>,
    #[serde(flatten)]
    attributes: HashMap<String, AttributeValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum AttributeValue {
    String(String),
    Bool(bool),
}

impl ToString for AttributeValue {
    fn to_string(&self) -> String {
        match self {
            Self::Bool(v) => v.to_string(),
            Self::String(v) => v.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OpenCloseRange {
    pub open_begin: usize,
    pub open_end: usize,
    pub close_begin: usize,
    pub close_end: usize
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SelfCloseRange {
    pub self_close_begin: usize,
    pub self_close_end: usize
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MacroRange {
    pub macro_begin: usize,
    pub macro_end: usize
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum RangeInDoenetML {
    OpenClose(OpenCloseRange),
    SelfClose(SelfCloseRange),
    FromMacro(MacroRange),
    None,
}

impl ToString for RangeInDoenetML {
    fn to_string(&self) -> String {
        match self {
            Self::OpenClose(v) => format!("Found at indices {}-{}.", v.open_begin, v.close_end),
            Self::SelfClose(v) => format!("Found at indices {}-{}.", v.self_close_begin, v.self_close_end),
            Self::FromMacro(v) => format!("Found at indices {}-{}.", v.macro_begin, v.macro_end),
            Self::None => String::new()
        }
    }
}

impl Default for RangeInDoenetML {
    fn default() -> Self {
        RangeInDoenetML::None
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum ComponentOrString {
    Component(ComponentTree),
    String(String),
}

/// This structure will get converted into `ComponentNode`;
/// that can only happen once all are created.
#[derive(Debug, Clone)]
pub struct MLComponent {
    pub name: ComponentName,
    pub parent: Option<ComponentName>,
    pub children: Vec<ComponentChild>,

    pub copy_source: Option<String>,
    pub copy_instance: Option<Vec<usize>>,
    pub copy_collection: Option<String>,
    pub copy_prop: Option<String>,
    pub static_attributes: HashMap<AttributeName, String>,

    // not filled in at first
    pub component_index: Vec<ObjectName>,
    pub prop_index: Vec<ObjectName>,

    pub definition: &'static ComponentDefinition,

    doenetml_range: RangeInDoenetML,
}

/// Convert serialized JSON of doenetML into tree of MLComponents
pub fn create_components_tree_from_json(program: &str)
    -> Result<(
            HashMap<ComponentName, MLComponent>,
            HashMap<ComponentName, HashMap<AttributeName, HashMap<usize, Vec<ObjectName>>>>,
            ComponentName,
            HashMap<String, ComponentName>,
            Vec<DoenetMLWarning>,
            Vec<DoenetMLError>
        ), DoenetMLError> {

    // log!("Parsing string for component tree: {}", program);
    let start = Instant::now();

    // This fails if there is a problem with the parser, not the input doenetML.
    // Panic - it's not a DoenetML error.
    let component_tree: Vec<ComponentOrString> = serde_json::from_str(program)
        .expect("Error extracting json");

    // TODO: if find a document child, shouldn't ignore all other children

    let component_tree = component_tree
        .iter()
        .find_map(|v| match v {
            ComponentOrString::Component(tree) => Some(tree),
            _ => None,
        })
        .and_then(|c| if c.component_type == "document" { Some(c.clone()) } else { None })
        .unwrap_or(ComponentTree {
            component_type: "document".to_string(),
            props: Props::default(),
            children: component_tree,
            doenetml_range: RangeInDoenetML::None,
            allow_underscore_component_type: false
        });

    log_json!(format!("Parsed JSON into tree"), component_tree);
    log!("parsed into tree: {:?}", start.elapsed());
    let start = Instant::now();
    
    let mut components: HashMap<ComponentName, MLComponent> = HashMap::new();
    let mut attributes: HashMap<ComponentName, HashMap<AttributeName, String>> = HashMap::new();
    let mut component_indices: HashMap<ComponentName, Option<String>> = HashMap::new();
    let mut prop_indices: HashMap<ComponentName, Option<String>> = HashMap::new();
    let mut map_sources_alias: HashMap<String, ComponentName> = HashMap::new();

    let mut component_type_counter: HashMap<String, u32> = HashMap::new();

    let mut warnings_encountered: Vec<DoenetMLWarning> = Vec::new();
    let mut errors_encountered: Vec<DoenetMLError> = Vec::new();

    let root_component_name = add_component_from_json(
        &mut components,
        &mut attributes,
        &mut component_indices,
        &mut prop_indices,
        &mut map_sources_alias,
        &component_tree,
        None,
        &mut component_type_counter,
        &mut errors_encountered
    )?;

    


    // Determine <sources>'s componentType static attribute, if not specified
    // TODO: <sources> inside <sources>
    // TODO: <sources> with copySource another <sources>
    let mut sources_component_types: HashMap<ComponentName, ComponentType> = HashMap::new();
    for (comp_name, comp) in components.iter().filter(|(_, c)|
        c.definition.component_type == "sources" && !c.static_attributes.contains_key("componentType")
    ) {
        let comp_children: Vec<&MLComponent> = comp.children.iter().filter_map(|child|
            child.as_component().and_then(|name| Some(components.get(name).unwrap()))
        ).collect();
            
        if comp_children.len() == 0 {
            // Every <sources> needs a componentType attr, so default to <number> since it doesn't matter
            sources_component_types.insert(comp_name.clone(), "number");
        } else {
            // log!("{} is <source> without componentType attr, child is {}", comp_name, comp_children[0].name);
            let first_comp_child_def = comp_children[0].definition;
            let child_type = first_comp_child_def.definition_as_replacement_children(&HashMap::new()).unwrap().component_type;
            sources_component_types.insert(comp_name.clone(), child_type);
        }
    }

    for (comp_name, child_comp_type) in sources_component_types {
        let comp = components.get_mut(&comp_name).unwrap();
        comp.static_attributes.insert("componentType", String::from(child_comp_type));
    }


    log!("created initial ML components: {:?}", start.elapsed());
    let start = Instant::now();

    let (replacement_children, macro_components, attributes_parsed, prop_indices_parsed, component_indices_parsed) =
 
        parse_attributes_and_macros(
            &components,
            attributes,
            prop_indices,
            component_indices,
            &map_sources_alias,
            &mut warnings_encountered,
        );


    
    log!("parsed attributes and macros: {:?}", start.elapsed());
    let start = Instant::now();

    // log_debug!("Components to add from macros: {:#?}", components_to_add);
    // log_debug!("Replacement children {:#?}", replacement_children);
    // log_debug!("Replacement attributes {:#?}", attributes_parsed);

    let components = components.into_iter().map(|(name, c)| {
        let mut new_children_vec: Vec<(usize, Vec<ObjectName>)> = replacement_children
            .get(&name)
            .unwrap_or(&HashMap::new())
            .clone()
            .into_iter()
            .collect();

        // sort by decending order so that splicing does not affect next iteration
        new_children_vec.sort_by(|(a,_),(b,_)| b.cmp(a));

        let mut children = c.children.clone();
        for (original_child_id, new_children) in new_children_vec.into_iter() {

            // Remove the original element, and add the new children (in order) in its place
            children.splice(
                original_child_id..=original_child_id,
                new_children
            );
        }

        (name.clone(), MLComponent {
            component_index: component_indices_parsed.get(&name).unwrap().clone(),
            prop_index: prop_indices_parsed.get(&name).unwrap().clone(),
            children,
            ..c
        })
    })
    .chain(
        macro_components.into_iter().map(|c| (c.name.clone(), c))
    ).collect();



    log!("added replacements to ML components: {:?}", start.elapsed());

    // log!("ML components: {:#?}", components);


    Ok((components, attributes_parsed, root_component_name, map_sources_alias, warnings_encountered, errors_encountered))
}

/// Recursive function
/// The return is the name of the child, if it exists
/// (it might not because of invalid doenet ml)
fn add_component_from_json(
    components: &mut HashMap<String, MLComponent>,
    attributes: &mut HashMap<ComponentName, HashMap<AttributeName, String>>,
    component_indices: &mut HashMap<ComponentName, Option<String>>,
    prop_indices: &mut HashMap<ComponentName, Option<String>>,
    map_sources_alias: &mut HashMap<String, ComponentName>,
    component_tree: &ComponentTree,
    parent: Option<String>,
    component_type_counter: &mut HashMap<String, u32>,
    errors_encountered: &mut Vec<DoenetMLError>
) -> Result<ComponentName, DoenetMLError> {

    let component_type: &str = &component_tree.component_type;

    let definition = &COMPONENT_DEFINITIONS
        .get_key_value_ignore_case(component_type)
        .ok_or(DoenetMLError::InvalidComponentType {
            comp_type: component_type.to_string(),
            doenetml_range: component_tree.doenetml_range.clone()
        }
        )?
        .1;

    let component_type = definition.component_type;

    if component_type.chars().next().unwrap() =='_' && !component_tree.allow_underscore_component_type {
        return Err(DoenetMLError::InvalidComponentType { 
            comp_type: component_type.to_string(),
            doenetml_range: component_tree.doenetml_range.clone()
         })
    }

    let count = component_type_counter.entry(component_type.to_string()).or_insert(0);
    *count += 1;

    let name = match &component_tree.props.name {
        Some(name) => {
            if regex_at(&BEGIN_LETTER, name,0).is_err() || regex_at(&CONTAINS_ONLY_NAME_CHARACTERS, name,0).is_err() {
                return Err(DoenetMLError::InvalidComponentName { 
                    name: name.clone(),
                    doenetml_range: component_tree.doenetml_range.clone()
                  });
            }
            name.clone()
            // TODO: add namespaces so default should be:
            // format!("/{}", name.clone())
        },
        None => format!("/_{}{}", component_type, count),
    };

    if components.contains_key(&name) {
        return Err(DoenetMLError::DuplicateName {
            name: name.clone(),
            doenetml_range: component_tree.doenetml_range.clone()
          });
    }

    let mut static_attributes = HashMap::new();
    let mut component_attributes = HashMap::new();

    let lower_case_attributes: HashMap<String, AttributeName> = definition
        .attribute_names
        .iter()
        .map(|n| (n.to_lowercase(), *n))
        .collect();
    let lower_case_static_attributes: HashMap<String, AttributeName> = definition
        .static_attribute_names
        .iter()
        .map(|n| (n.to_lowercase(), *n))
        .collect();

    for (attr_name, attr_value) in component_tree.props.attributes.clone().into_iter() {
        let attr_name = attr_name.to_lowercase();
        if let Some(&attribute_name) = lower_case_attributes.get(&attr_name) {
            component_attributes.insert(attribute_name, attr_value.to_string());
        } else if let Some(&attribute_name) = lower_case_static_attributes.get(&attr_name) {
            static_attributes.insert(attribute_name, attr_value.to_string());
        } else {
            return Err(DoenetMLError::AttributeDoesNotExist {
                comp_name: name.clone(),
                attr_name: attr_name.clone(),
                doenetml_range: component_tree.doenetml_range.clone()  
            });
        }
    }

    // Add alias
    if component_type == "sources" {
        if let Some(alias) = static_attributes.get("alias") {
            map_sources_alias.insert(alias.clone(), name.clone());
        }
    }

    // Recurse the children
    let mut children: Vec<ComponentChild> = Vec::new();
    for child in &component_tree.children {

        match child {
            ComponentOrString::String(child_string) => {
                children.push(ComponentChild::String(child_string.to_string()));
            },

            ComponentOrString::Component(child_tree) => {
                let child_name_or_error = add_component_from_json(
                    components,
                    attributes,
                    component_indices,
                    prop_indices,
                    map_sources_alias,
                    &child_tree,
                    Some(name.clone()),
                    component_type_counter,
                    errors_encountered
                );

                match child_name_or_error {
                    Ok(child_name) => {
                        children.push(ComponentChild::Component(child_name));
                    }
                    Err(error) => {
                        if !definition.display_errors {
                            return Err(error);
                        }

                        let mut attributes_for_props = HashMap::new();

                        let (child_begin_index, child_end_index) = match &child_tree.doenetml_range {
                            RangeInDoenetML::OpenClose(open_close) => {
                                (open_close.open_begin, open_close.close_end)
                            }
                            RangeInDoenetML::SelfClose(self_close) => {
                                (self_close.self_close_begin, self_close.self_close_end)
                            }
                            RangeInDoenetML::FromMacro(from_macro) => {
                                (from_macro.macro_begin, from_macro.macro_end)
                            }
                            RangeInDoenetML::None => (0, 0)
                        };

                        attributes_for_props.insert("start_index".to_string(), AttributeValue::String(child_begin_index.to_string()));
                        attributes_for_props.insert("end_index".to_string(), AttributeValue::String(child_end_index.to_string()));


                        // create a component of type _error to display the error in the document
                        let error_component = ComponentTree {
                            component_type: "_error".to_string(),
                            props: Props {
                                attributes: attributes_for_props,
                                ..Default::default()
                            },
                            children: vec![ComponentOrString::String(error.to_string())],
                            doenetml_range: RangeInDoenetML::None,
                            allow_underscore_component_type: true
                        };

                        let error_child_name= add_component_from_json(
                            components,
                            attributes,
                            component_indices,
                            prop_indices,
                            map_sources_alias,
                            &error_component,
                            Some(name.clone()),
                            component_type_counter,
                            errors_encountered,
                        )?;

                        children.push(ComponentChild::Component(error_child_name));

                        errors_encountered.push(error);
    
                    }
                }

            },
        }
    }

    let (copy_source, copy_instance) = convert_copy_source_name(component_tree.props.copy_source.clone());

    let component_node = MLComponent {
        name: name.clone(),
        parent,
        children,

        copy_source,
        copy_instance,
        copy_collection: component_tree.props.copy_collection.clone(),
        copy_prop: component_tree.props.copy_prop.clone(),
        prop_index: vec![],
        component_index: vec![],

        static_attributes,

        definition,

        doenetml_range: component_tree.doenetml_range.clone(),
    };

    components.insert(name.clone(), component_node);
    attributes.insert(name.clone(), component_attributes);

    // The empty component and prop index will be filled when macros are parsed.
    // Store them in separate HashMaps until they are ready.
    component_indices.insert(name.clone(), component_tree.props.component_index.clone());
    prop_indices.insert(name.clone(), component_tree.props.prop_index.clone());

    return Ok(name);
}

/// Temporary implementation to test if maps are working.
/// returns (copy source, copy instance)
/// Ex: [1 2]myname -> (myname, [1, 2])
/// TODO: The final implementation should incorporate namespaces, not just map instancing.
fn convert_copy_source_name(name: Option<String>) -> (Option<String>, Option<Vec<usize>>) {
    if let Some(name) = name {
        if name.chars().next() == Some('(') {
            let mut chars = name.chars();
            chars.next();
            let instance_indices: String = (&mut chars).take_while(|&c| c != ')').collect();
            let relative_instance = instance_indices.split(' ').map(|d| d.parse().unwrap()).collect();
            let component_name = chars.collect();
            (Some(component_name), Some(relative_instance))
        } else {
            (Some(name), None)
        }
    } else {
        (None, None)
    }
}


fn parse_attributes_and_macros(
    components: &HashMap<ComponentName, MLComponent>,
    attributes: HashMap<ComponentName, HashMap<AttributeName, String>>,
    prop_indices: HashMap<ComponentName, Option<String>>,
    component_indices: HashMap<ComponentName, Option<String>>,
    map_sources_alias: &HashMap<String, ComponentName>,
    warnings_encountered: &mut Vec<DoenetMLWarning>
) -> (
    HashMap<ComponentName, HashMap<usize, Vec<ObjectName>>>,
    Vec<MLComponent>,
    HashMap<ComponentName, HashMap<AttributeName, HashMap<usize, Vec<ObjectName>>>>,
    HashMap<ComponentName, Vec<ObjectName>>,
    HashMap<ComponentName, Vec<ObjectName>>,
    )
{
    use std::iter::repeat;

    let mut attributes_parsed = HashMap::new();
    let mut prop_indices_parsed = HashMap::new();
    let mut component_indices_parsed = HashMap::new();

    // Keyed by the component name and by the original position of the child we are replacing
    let mut replacement_children: HashMap<ComponentName, HashMap<usize, Vec<ObjectName>>> = HashMap::new();
    let mut components_to_add: Vec<MLComponent> = vec![];

    let mut macro_copy_counter: HashMap<ComponentName, usize> = HashMap::new();
    

    // This iterator gives info for every string child:
    // (original index of child, string value, component)
    let all_string_children = components.iter()
        .flat_map(|(_, comp)|
            comp.children
            .iter()
            .enumerate()
            .filter_map(|(id, child)| {
                match child {
                    ObjectName::String(string_val) => Some((id, string_val)),
                    _ => None,
                }
            })
            .zip(repeat(comp))
            .map(|((id, val), comp)| (id, val, comp))
        );

    let all_attributes = attributes.iter()
        .flat_map(|(name, attrs)|
            attrs
            .iter()
            .map(|(attr_name, val)| (*attr_name, val, components.get(name).unwrap()))
            .collect::<Vec<(AttributeName, &String, &MLComponent)>>()
        );

    // Component string children
    for (child_id, string_val, component) in all_string_children {

        let mut range_begin = None;
        if let RangeInDoenetML::OpenClose(open_close) = &component.doenetml_range {
            range_begin = Some(open_close.open_end);

            if child_id > 0 {
                if let ObjectName::Component(comp_name) = &component.children[child_id-1] {
                    let &previous_child = &components.get(comp_name).unwrap();

                    range_begin = match &previous_child.doenetml_range {
                        RangeInDoenetML::OpenClose(open_close) => Some(open_close.close_end),
                        RangeInDoenetML::SelfClose(self_close) => Some(self_close.self_close_end),
                        RangeInDoenetML::FromMacro(from_macro) => Some(from_macro.macro_end),
                        RangeInDoenetML::None => range_begin
                    }

                }
            }
        }

        let objects = apply_macro_to_string(
            string_val,
            &component.name,
            components,
            map_sources_alias,
            &mut macro_copy_counter,
            &mut components_to_add,
            range_begin,
            warnings_encountered,
        );

        // For now, replace everything in the children field
        replacement_children
            .entry(component.name.clone()).or_insert(HashMap::new())
            .entry(child_id).or_insert(objects);
    }

    // Attributes
    for (attribute_name, string_val, component) in all_attributes {

        // The reason this uses a HashMap of usizes instead of another Vec is because
        // later we might want to specify arrays of arrays in the attribute, so the key
        // might be more complicated than an integer.
        let objects: HashMap<usize, Vec<ObjectName>> = string_val.split(' ')
            .enumerate()
            .map(|(index, string_element)|

                // DoenetML is 1-indexed
                (index + 1,
                    apply_macro_to_string(
                        string_element.trim(),
                        &component.name,
                        components,
                        map_sources_alias,
                        &mut macro_copy_counter,
                        &mut components_to_add,
                        None,
                        warnings_encountered,
                    )
                )
            ).collect();

        attributes_parsed
            .entry(component.name.clone()).or_insert(HashMap::new())
            .entry(attribute_name.clone()).or_insert(objects);
    }

    // Prop indices
    for (target_name, source_index_str) in prop_indices {
        
        let index_objects = match source_index_str {
            Some(string) => apply_macro_to_string(
                &string,
                &target_name,
                components,
                map_sources_alias,
                &mut macro_copy_counter,
                &mut components_to_add,
                None,
                warnings_encountered,
            ),
            None => vec![],
        };

        prop_indices_parsed.insert(target_name, index_objects);
    }

    // Component indices
    for (target_name, source_index_str) in component_indices {
        
        let index_objects = match source_index_str {
            Some(string) => apply_macro_to_string(
                &string,
                &target_name,
                components,
                map_sources_alias,
                &mut macro_copy_counter,
                &mut components_to_add,
                None,
                warnings_encountered,
            ),
            None => vec![],
        };

        component_indices_parsed.insert(target_name,index_objects);
    }

    (
        replacement_children,
        components_to_add,
        attributes_parsed,
        prop_indices_parsed,
        component_indices_parsed,
    )
}

fn apply_macro_to_string(
    string: &str,
    component_name: &ComponentName,
    components: &HashMap<ComponentName, MLComponent>,
    map_sources_alias: &HashMap<String, ComponentName>,
    macro_copy_counter: &mut HashMap<ComponentName, usize>,
    components_to_add: &mut Vec<MLComponent>,
    start_doenetml_ind: Option<usize>,
    warnings_encountered: &mut Vec<DoenetMLWarning>

) -> Vec<ObjectName> {

    let mut objects = Vec::new();
    let mut previous_end = 0;

    loop {
        if previous_end >= string.len() {
            break;
        }
        let some_next_macro = MACRO_BEGIN.find_at(string, previous_end);
        if some_next_macro.is_none() {
            break;
        }
        let next_macro = some_next_macro.unwrap();

        // Append the regular string until start of macro
        let before = &string[previous_end..next_macro.start()];
        if !before.is_empty() {
            objects.push(ObjectName::String(before.to_string()));
        }

        match macro_comp_ref(string,
            next_macro.end(),
            component_name,
            components,
            map_sources_alias,
            macro_copy_counter,
            components_to_add,
            start_doenetml_ind,
            warnings_encountered
        ) {
            Ok((macro_name, macro_end)) => {
                previous_end = macro_end;
                objects.push(ObjectName::Component(macro_name));
            },
            Err((msg, error_end, skip_error_string)) => {
                log!("macro failed: {}", msg);
                if !skip_error_string {
                    let skipped = &string[next_macro.start()..error_end];
                    if !skipped.is_empty() {
                        objects.push(ObjectName::String(skipped.to_string()));
                    }
                }

                previous_end = error_end;
            }
        }
    }

    // Append until the end
    let last = &string[previous_end..];
    if !last.is_empty() {
        objects.push(ComponentChild::String(last.to_string()));
    }

    objects
}

fn macro_comp_ref(
    string: &str,
    start: usize,
    macro_parent: &ComponentName,
    components: &HashMap<ComponentName, MLComponent>,
    map_sources_alias: &HashMap<String, ComponentName>,
    macro_copy_counter: &mut HashMap<ComponentName, usize>,
    components_to_add: &mut Vec<MLComponent>,
    start_doenetml_ind: Option<usize>,
    warnings_encountered: &mut Vec<DoenetMLWarning>
) -> Result<(ComponentName, usize), (String, usize, bool)> {

    // log_debug!("macro at {} of {}", start, string);

    let comp_match = regex_at(&COMPONENT, string, start).map_err(|err| (err, start+1, false))?;

    let copy_source = comp_match.as_str().to_string();
    let (copy_source, copy_instance) = convert_copy_source_name(Some(copy_source.clone()));
    let copy_source = copy_source.unwrap();


    if let Some(sources_name) = map_sources_alias.get(&copy_source) {
        // Special case: the macro references a sources component

        let component_type = components.get(sources_name).unwrap()
            .static_attributes.get("componentType")
            .ok_or(("Sources did not define component type".to_string(), comp_match.end(), true))?;
        let definition = &COMPONENT_DEFINITIONS
            .get(component_type.as_str())
            .ok_or(("Sources invalid component type".to_string(), comp_match.end(), true))?;


        let macro_copy = MLComponent {
            name: name_macro_component(&copy_source, macro_parent, macro_copy_counter),
            parent: Some(macro_parent.clone()),
            children: vec![],
            copy_source: Some(copy_source),
            copy_instance,
            copy_collection: None,
            copy_prop: None,
            component_index: vec![],
            prop_index: vec![],
            static_attributes: HashMap::new(),
            definition,
            doenetml_range: RangeInDoenetML::None,
        };

        let macro_name = macro_copy.name.clone();
        components_to_add.push(macro_copy);
        return Ok((macro_name, comp_match.end()))
    }


    let mut found_error = false;
    let mut error_message = String::new();

    let source_component_option = components.get(&copy_source);

    if source_component_option.is_none() {
        found_error = true;
        error_message = format!("The component {} does not exist", copy_source);
        let doenetml_range = match start_doenetml_ind {
            Some(start_ind) => RangeInDoenetML::FromMacro(MacroRange{macro_begin: start_ind + start, macro_end: start_ind + comp_match.end()}),
            None => RangeInDoenetML::None
        };
        warnings_encountered.push(DoenetMLWarning::ComponentDoesNotExist {
            comp_name: copy_source.to_string(),
            doenetml_range,
        });
    }

    let char_at = |c: usize| string.as_bytes().get(c).map(|c| *c as char);

    let name: String;
    let definition_option: Option<&ComponentDefinition>;
    let component_index: Vec<ObjectName>;
    let copy_prop: Option<String>;
    let prop_index: Vec<ObjectName>;
 
    // Handle possible component index: brackets after the component name
    let comp_end;
    let source_def_option;

    if char_at(comp_match.end()) == Some('[') {
        // // group member
        // let index_match = regex_at(&INDEX, string, comp_match.end() + 1).map_err(|err| (err, comp_match.end(), false))?;
        // let index_str = index_match.as_str();
        // let index_end: usize;
        // if index_str == "$" {
        //     // dynamic component index
        //     unimplemented!("dynamic component index not implemented");
        // } else {
        //     // static component index
        //     component_index = vec![ObjectName::String(index_str.trim().to_string())];
        //     index_end = index_match.end();
        // }
        // let close_bracket_match = regex_at(&INDEX_END, string, index_end).map_err(|err| (err, comp_match.end(), false))?;
        // comp_end = close_bracket_match.end();

        unimplemented!("Have not implemented array index");

    } else {
        // no component index
        comp_end = comp_match.end();
        component_index = vec![];
        source_def_option = match source_component_option {
            Some(source_component) => Some(source_component.definition),
            None => None
        }
    };

    // Handle possible copy prop: dot then state variable
    let macro_end;
    if char_at(comp_end) == Some('.') {
        let prop_match = regex_at(&PROP, string, comp_end + 1).map_err(|err| (err, comp_end, false))?;
        let prop = prop_match.as_str();

        let variant_option = match source_def_option {
            Some(source_def) => match source_def.state_var_index_map.get(prop) {
                Some(v) => Some(source_def.state_var_component_types[*v]),
                None => {
                    if !found_error {
                        found_error = true;
                        error_message = format!("prop {} doesn't exist on {}", prop, source_def.component_type);

                        let doenetml_range = match start_doenetml_ind {
                            Some(start_ind) => RangeInDoenetML::FromMacro(MacroRange{macro_begin: start_ind + start, macro_end: start_ind + prop_match.end()}),
                            None => RangeInDoenetML::None
                        };
                        warnings_encountered.push(DoenetMLWarning::StateVarDoesNotExist {
                            comp_name: copy_source.to_string(),
                            sv_name: prop.to_string(),
                            doenetml_range,
                        });
                    }
                    None
                }
            }
            None => None
        };

        // Handle possible prop index: brackets after the prop name
        if string.as_bytes().get(prop_match.end()) == Some(&b'[') {

            let index_match = regex_at(&INDEX, string, prop_match.end() + 1).map_err(|err| (err, comp_end, false))?;
            let index_str = index_match.as_str().trim();
            let index_end: usize;
            if index_str == "$" {
                // dynamic index
                // TODO: multiple components in []
                let (index_name, index_macro_end) = macro_comp_ref(string,
                    index_match.end(),
                    &copy_source,
                    components,
                    map_sources_alias,
                    macro_copy_counter,
                    components_to_add,
                    start_doenetml_ind,
                    warnings_encountered
                )?;

                index_end = index_macro_end;
                prop_index = vec![ObjectName::Component(index_name.clone())];
            } else {
                // static index
                index_end = index_match.end();
                prop_index = vec![ObjectName::String(index_str.to_string())];
            }
            let close_bracket_match = regex_at(&INDEX_END, string, index_end).map_err(|err| (err, comp_end, false))?;
            macro_end = close_bracket_match.end();

            if let Some(_variant) = variant_option {
                if !found_error {
                    found_error = true;
                    error_message = format!("{}.{} cannot be indexed", copy_source, prop);

                    let doenetml_range = match start_doenetml_ind {
                        Some(start_ind) => RangeInDoenetML::FromMacro(MacroRange{macro_begin: start_ind + start, macro_end: start_ind + macro_end}),
                        None => RangeInDoenetML::None
                    };
                    warnings_encountered.push(DoenetMLWarning::InvalidArrayIndex {
                        comp_name: copy_source.to_string(),
                        sv_name: Some(prop.to_string()),
                        array_index: index_str.trim().to_string(),
                        doenetml_range,
                    });
                }
            }
        } else {
            // no index
            macro_end = prop_match.end();
            prop_index = vec![];
        }

        let source_comp_sv_name = format!("{}:{}", copy_source, prop);

        definition_option = match variant_option {
            Some(variant) => Some(&COMPONENT_DEFINITIONS
            .get(variant)
            .unwrap()),
            None => None
        };
        
        

        name = name_macro_component(
            &source_comp_sv_name,
            macro_parent,
            macro_copy_counter,
        );
        copy_prop = Some(prop.to_string());

    } else {
        // no prop
        copy_prop = None;
        prop_index = vec![];

        name = name_macro_component(
            &copy_source,
            macro_parent,
            macro_copy_counter,
        );
        definition_option = source_def_option;

        macro_end = comp_end;
    };


    if found_error {
        return Err((error_message, macro_end, true));
    }
    
    // if didn't find an error, then we have a definition
    let definition = definition_option.unwrap();

    let (copy_source, copy_instance) = convert_copy_source_name(Some(copy_source));

    let doenetml_range = match start_doenetml_ind {
        Some(start_ind) => RangeInDoenetML::FromMacro(MacroRange{macro_begin: start_ind + start, macro_end: start_ind + macro_end}),
        None => RangeInDoenetML::None
    };

    let macro_copy = MLComponent {
        name,
        parent: Some(macro_parent.clone()),
        children: vec![],

        copy_source,
        copy_instance,
        copy_collection: None,
        copy_prop,
        component_index,
        prop_index,

        static_attributes: HashMap::new(),

        definition,

        doenetml_range
    };
    let macro_name = macro_copy.name.clone();
    components_to_add.push(macro_copy);

    Ok((macro_name, macro_end))
}


fn name_macro_component(
    source_name: &str,
    component_name: &String,
    copy_counter: &mut HashMap<ComponentName, usize>,
) -> String {
    let copy_num = copy_counter.entry(source_name.to_string()).or_insert(0);
    *copy_num += 1;

    format!("__mcr:{}({})_{}", source_name, component_name, copy_num)
}

lazy_static! { static ref MACRO_BEGIN: Regex = Regex::new(r"\$").unwrap(); }
lazy_static! { static ref COMPONENT: Regex   = Regex::new(r"[a-zA-Z_]\w*").unwrap(); }
lazy_static! { static ref PROP: Regex        = Regex::new(r"[a-zA-Z]\w*").unwrap(); }
lazy_static! { static ref INDEX: Regex       = Regex::new(r"\s*(\d+|\$)").unwrap(); }
lazy_static! { static ref INDEX_END: Regex   = Regex::new(r"\s*]").unwrap(); }
lazy_static! { static ref BEGIN_LETTER: Regex= Regex::new(r"^[a-zA-Z]").unwrap(); }
lazy_static! { static ref CONTAINS_ONLY_NAME_CHARACTERS: Regex= Regex::new(r"^[a-zA-Z0-9_\-]+$").unwrap(); }


fn regex_at<'a>(regex: &Regex, string: &'a str, at: usize) -> Result<regex::Match<'a>, String> {
    regex.find_at(string, at)
        .and_then(|m| {
            if m.start() == at {Some(m)} else {None}
        })
        .ok_or(format!("regex {:?} not found at index {} of {}", regex, at, string))
}




// Structures for parse_action_from_json
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ActionStructure {
    component_name: String,
    action_name: String,
    args: HashMap<String, ArgValue>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum ArgValue {
    Bool(bool),
    Number(serde_json::Number),
    NumberArray(Vec<serde_json::Number>),
    String(String),
}

/// Returns the Action as well as the action id which the renderer sent
pub fn parse_action_from_json(action: &str) -> Result<(Action, String), String> {

    // log_debug!("Parsing string for action: {}", action);

    let action_structure: ActionStructure = serde_json::from_str(action).map_err(|e| e.to_string())?;

    let component_name = action_structure.component_name.clone();
    let action_name = action_structure.action_name.clone();
    let mut args: HashMap<String, Vec<StateVarValue>> = action_structure.args
        .into_iter()
        .map(|(k, v)| (k, v.into()))
        .collect();

    let action_id: String = args.get("actionId").unwrap().first().unwrap().clone().try_into().unwrap();
    args.remove("actionId");

    Ok((Action { component_name, action_name, args}, action_id))
}


impl From<serde_json::Number> for StateVarValue {
    fn from(v: serde_json::Number) -> Self {
        if v.is_i64() {
             StateVarValue::Integer(v.as_i64().unwrap())
         } else {
             StateVarValue::Number(v.as_f64().unwrap())
         }
    }
}

impl From<ArgValue> for Vec<StateVarValue> {
    fn from(value: ArgValue) -> Self {
         match value {
             ArgValue::Bool(v) => vec![StateVarValue::Boolean(v)],
             ArgValue::String(v) => vec![StateVarValue::String(v)],
             ArgValue::Number(v) => vec![v.into()],
             ArgValue::NumberArray(v) =>  v.into_iter().map(|v| v.into()).collect(),
         }
    }
}
