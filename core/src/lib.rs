
#![allow(dead_code)] 
#![allow(unused_variables)] 

use std::cell::RefCell;
use std::rc::Rc;
use std::fmt;


/*
Why we need RefCells: the Rc does not allow mutability in the
thing it wraps. If it any point we might want to mutate a field, its value should be wrapped in a RefCell
*/

pub mod state_variable_setup;
pub mod text;
pub mod number;



use text::{Text};

use number::Number;

use state_variable_setup::*;


#[macro_export]
macro_rules! state_var_access {
    ($component_type:ident, $state_var_field:ident, $state_var_type:ty) => {

        |component: &crate::Component| -> &RefCell<$state_var_type> {

            match component {
                crate::Component::$component_type(my_component) => {
                    &my_component.$state_var_field
                },
                _ => {
                    panic!("State var access used wrong Component type argument for $component_type");
                }
            }
        }
        
    }
}




pub trait ComponentSpecificBehavior {
    fn get_trait_names(&self) -> Vec<ComponentTraitName>;

    /**
     * Capitalize names, eg, Text.
     */
    fn get_component_type(&self) -> &'static str;

    /**
     * This function should never use self in the function body.
     * Treat as if this is a constant
     */
    fn state_variable_instructions(&self) -> phf::Map<&'static str, StateVar>;

    fn state_var(&self, name: &'static str) -> Option<StateVarAccess>;


    fn set_state_var_string(&self, name: &'static str, val: String) {

        match self.state_var(name).unwrap() {
            StateVarAccess::String(sva) => { sva.replace(val); }
            _ => { unreachable!(); }
        }
    }

    fn set_state_var_integer(&self, name: &'static str, val: i64) {

        match self.state_var(name).unwrap() {
            StateVarAccess::Integer(sva) => { sva.replace(val); }
            _ => { unreachable!(); }
        }
    }

    fn set_state_var_number(&self, name: &'static str, val: f64) {

        match self.state_var(name).unwrap() {
            StateVarAccess::Number(sva) => { sva.replace(val); }
            _ => { unreachable!(); }
        }
    }

    fn set_state_var_bool(&self, name: &'static str, val: bool) {

        match self.state_var(name).unwrap() {
            StateVarAccess::Bool(sva) => { sva.replace(val); }
            _ => { unreachable!(); }
        }
    }
    
}


pub enum StateVarAccess<'a> {
    String(&'a RefCell<String>),
    Number(&'a RefCell<f64>),
    Integer(&'a RefCell<i64>),
    Bool(&'a RefCell<bool>),
}


pub trait ComponentLike: ComponentSpecificBehavior {
    fn name(&self) -> String;
    fn children(&self) -> RefCell<Vec<ComponentChild>>;
    fn parent(&self) -> RefCell<String>;
    fn parent_name(&self) -> Option<String>;
    fn add_as_child(&self, child: ComponentChild);

}





trait TextLikeComponent: ComponentLike {
    fn text_value(&self) -> String;
}
trait NumberLikeComponent: ComponentLike {
    fn add_one(&self) -> i64;
}


#[derive(Clone, PartialEq, Debug)]
pub enum ComponentTraitName {
    TextLikeComponent,
    NumberLikeComponent,
    ComponentLike,
}



#[derive(Debug, Clone)]
pub enum Component {
    Text(Rc<Text>),
    Number(Rc<Number>),
}



impl Component {

    pub fn component(&self) -> Rc<dyn ComponentLike> {
        match self {
            Component::Text(comp) => Rc::clone(comp) as Rc<dyn ComponentLike>,
            Component::Number(comp) => Rc::clone(comp) as Rc<dyn ComponentLike>,
        }
    }

}


#[derive(Debug, Clone)]
pub enum ComponentChild {
    String(String),
    Component(Rc<dyn ComponentLike>),
}




pub fn create_all_dependencies_for_component(component: &Rc<dyn ComponentLike>) -> Vec<Dependency> {
        
        let mut dependencies: Vec<Dependency> = vec![];


        let my_definitions = component.state_variable_instructions();


        for (&state_var_name, state_var_def) in my_definitions.entries() {

            //Eventually, call state_vars_to_determine_dependencies() and go calculate those values

            let dependency_instructions_hashmap = match state_var_def {
                StateVar::String(def)  => (def.return_dependency_instructions)(StateVarValuesMap::new()),
                StateVar::Bool(def)    => (def.return_dependency_instructions)(StateVarValuesMap::new()),
                StateVar::Number(def)  => (def.return_dependency_instructions)(StateVarValuesMap::new()),
                StateVar::Integer(def) => (def.return_dependency_instructions)(StateVarValuesMap::new()),
            };
            
            
            for (_, dep_instruction) in dependency_instructions_hashmap.into_iter() {


                let dependency =  create_dependency_from_instruction(component, state_var_name, dep_instruction);

                dependencies.push(dependency);
            }
        

        }

        dependencies

}


fn create_dependency_from_instruction(component: &Rc<dyn ComponentLike>, state_var: &'static str, instruction: DependencyInstruction) -> Dependency {

    let mut dependency = Dependency {
        component: component.name(),
        state_var: state_var,
        depends_on_components: vec![],
        depends_on_state_vars: vec![],
        instruction: instruction.clone(),
        variables_optional: false,
    };

    match instruction {
        DependencyInstruction::StateVar(state_var_instruction) => {

            if let Option::Some(name) = state_var_instruction.component_name {
                dependency.depends_on_components = vec![name];
            } else {
                dependency.depends_on_components = vec![component.name()];
            }
            dependency.depends_on_state_vars = vec![state_var_instruction.state_var];
        },

        DependencyInstruction::Child(child_instruction) => {
            let all_children = component.children();

            let mut depends_on_children: Vec<String> = vec![];
            for child in all_children.borrow().iter() {
                for desired_child_type in child_instruction.desired_children.iter() {
                    match child {
                        ComponentChild::Component(child_component) => {
                            if child_component.get_trait_names().contains(desired_child_type) {
                                //If not already in list, add it to the list
                                if !depends_on_children.contains(&child_component.name()) {
                                    depends_on_children.push(child_component.name());
                                }
                            }
                        },

                        ComponentChild::String(string_value) => {
                            if desired_child_type == &ComponentTraitName::TextLikeComponent {
                                //or do nothing here?
                                depends_on_children.push(format!("#{}", string_value));
                            }
                        },
                    }

                }
            }

            dependency.depends_on_components = depends_on_children;
            dependency.depends_on_state_vars = child_instruction.desired_state_vars;

        },
        DependencyInstruction::Parent(_) => {

        },
    };

    dependency
}





/** Implement Debug for trait objects **/
impl fmt::Debug for dyn ComponentLike {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name())
    }
}
impl fmt::Debug for dyn TextLikeComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let debug_text = format!("{}:{}", self.name(), self.text_value());
        f.write_str(&debug_text)
    }
}
impl fmt::Debug for dyn NumberLikeComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name())
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    


    #[test]
    fn test_core() {

        //Setup Tree
        let mut components: HashMap<String, Component> = HashMap::new();

        let text2 = Rc::new(Text {
            name: "text2".to_owned(),
            value: RefCell::new("hi there".to_owned()),
            hide: RefCell::new(false),
            parent: RefCell::new(String::new()),
            children: RefCell::new(vec![]),
        });
        components.insert(text2.name(), Component::Text(Rc::clone(&text2)));


        let text1 = Rc::new(Text {
            name: "text1".to_owned(),
            value: RefCell::new("banana".to_owned()),
            hide: RefCell::new(false),
            parent: RefCell::new(String::new()),
            children: RefCell::new(vec![]),
        });
        components.insert(text1.name(), Component::Text(Rc::clone(&text1)));


        text1.add_as_child(ComponentChild::Component(text2));


        //Create dependencies
        let dependencies = create_all_dependencies_for_component(& (text1 as Rc<dyn ComponentLike>) );

        println!("Components\n{:#?}", components);
        println!("Dependencies\n{:#?}", dependencies);

        assert!(true);


    }

}