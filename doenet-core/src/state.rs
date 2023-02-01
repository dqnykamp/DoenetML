use enum_as_inner::EnumAsInner;
use serde::Serialize;

use crate::state_variables::*;
use std::{
    cell::{Ref, RefCell},
    fmt,
    ops::Deref,
    rc::Rc,
};
// use self::Freshness::*;

use crate::math_expression::MathExpression;

#[derive(Debug, Clone, Copy, PartialEq, EnumAsInner)]
pub enum Freshness {
    Fresh,
    Stale,
    Unresolved,
}

pub enum StateVar {
    Number(StateVarTyped<f64>),
    Integer(StateVarTyped<i64>),
    String(StateVarTyped<String>),
    Boolean(StateVarTyped<bool>),
    MathExpr(StateVarTyped<MathExpression>),
}

pub enum StateVarMutableView {
    Number(StateVarMutableViewTyped<f64>),
    Integer(StateVarMutableViewTyped<i64>),
    String(StateVarMutableViewTyped<String>),
    Boolean(StateVarMutableViewTyped<bool>),
    MathExpr(StateVarMutableViewTyped<MathExpression>),
}

pub enum StateVarReadOnlyView {
    Number(StateVarReadOnlyViewTyped<f64>),
    Integer(StateVarReadOnlyViewTyped<i64>),
    String(StateVarReadOnlyViewTyped<String>),
    Boolean(StateVarReadOnlyViewTyped<bool>),
    MathExpr(StateVarReadOnlyViewTyped<MathExpression>),
}

#[derive(Debug)]
pub struct StateVarTyped<T: Default + Clone> {
    value: StateVarMutableViewTyped<T>,
    immutable_view_of_value: StateVarReadOnlyViewTyped<T>,
    interface: Box<dyn StateVarInterface<T>>,
    parameters: StateVarParameters<T>,
    change_counter_when_last_rendered: u32,
}

pub struct UpdatesRequested {
    pub instruction_ind: usize,
    pub dependency_ind: usize,
}

pub trait StateVarInterface<T: Default + Clone>: std::fmt::Debug {
    fn return_dependency_instructions(&self) -> Vec<DependencyInstruction> {
        Vec::new()
    }

    fn set_dependencies(&mut self, _dependencies: &Vec<Vec<DependencyValue>>) -> () {}

    fn calculate_state_var_from_dependencies(
        &mut self,
        state_var: &StateVarMutableViewTyped<T>,
    ) -> ();

    fn request_dependencies_to_update_value(
        &self,
        _state_var: &StateVarReadOnlyViewTyped<T>,
        _is_initial_change: bool,
    ) -> Result<Vec<UpdatesRequested>, ()> {
        Err(())
    }
}

#[derive(Debug, Default)]
pub struct StateVarParameters<T> {
    pub for_renderer: bool,
    pub initial_essential_value: T,
    pub name: &'static str,
}

#[derive(Debug)]
pub struct StateVarMutableViewTyped<T: Default + Clone> {
    inner: Rc<RefCell<StateVarInner<T>>>,
    change_counter_when_last_viewed: u32,
}

#[derive(Debug)]
struct StateVarInner<T: Default + Clone> {
    value: T,
    freshness: Freshness,
    requested_value: T,
    used_default: bool,
    change_counter: u32,
}

impl<T: Default + Clone> StateVarInner<T> {
    pub fn get_fresh_value<'a>(&'a self) -> &'a T {
        if self.freshness != Freshness::Fresh {
            panic!("State variable is not fresh, cannot get its fresh value");
        }
        &self.value
    }

    pub fn get_last_value_even_if_stale<'a>(&'a self) -> &'a T {
        if self.freshness == Freshness::Unresolved {
            panic!("State variable is unresolved, cannot get its value");
        }
        &self.value
    }

    pub fn mark_stale(&mut self) {
        self.freshness = Freshness::Stale;
    }

    pub fn set_value(&mut self, new_val: T) {
        self.value = new_val;
        self.freshness = Freshness::Fresh;
        self.change_counter += 1;
    }

    pub fn set_value_and_used_default(&mut self, new_val: T, used_default: bool) {
        self.value = new_val;
        self.freshness = Freshness::Fresh;
        self.used_default = used_default;
        self.change_counter += 1;
    }

    pub fn restore_previous_value(&mut self) {
        match self.freshness {
            Freshness::Stale => {
                self.freshness = Freshness::Fresh;
            }
            Freshness::Fresh => (),
            Freshness::Unresolved => {
                panic!("Cannot restore previous value to an unresolved state variable");
            }
        }
    }

    pub fn get_used_default(&self) -> bool {
        self.used_default
    }

    pub fn request_change_value_to(&mut self, requested_val: T) {
        self.requested_value = requested_val;
    }

    pub fn get_requested_value<'a>(&'a self) -> &'a T {
        &self.requested_value
    }

    pub fn get_change_counter(&self) -> u32 {
        self.change_counter
    }
}

impl<T: Default + Clone> StateVarMutableViewTyped<T> {
    pub fn new() -> Self {
        StateVarMutableViewTyped {
            inner: Rc::new(RefCell::new(StateVarInner {
                value: T::default(),
                freshness: Freshness::Unresolved,
                requested_value: T::default(),
                used_default: false,
                change_counter: 1, // Note: start at 1 so starts out indicating it changed
            })),
            change_counter_when_last_viewed: 0,
        }
    }

    pub fn new_with_value(val: T, used_default: bool) -> Self {
        StateVarMutableViewTyped {
            inner: Rc::new(RefCell::new(StateVarInner {
                value: val,
                freshness: Freshness::Fresh,
                requested_value: T::default(),
                used_default,
                change_counter: 1, // Note: start at 1 so starts out indicating it changed
            })),
            change_counter_when_last_viewed: 0,
        }
    }

    // use get_read_only_view to main a reference to the value
    // that you can repeatedly access its value
    // but allow the value to be modified when not accessing the value
    pub fn create_new_read_only_view(&self) -> StateVarReadOnlyViewTyped<T> {
        StateVarReadOnlyViewTyped {
            inner: self.inner.clone(),
            change_counter_when_last_viewed: 0,
        }
    }

    pub fn check_if_changed_since_last_viewed(&self) -> bool {
        self.inner.borrow().get_change_counter() > self.change_counter_when_last_viewed
    }

    pub fn get_fresh_value_record_viewed<'a>(&'a mut self) -> impl Deref<Target = T> + 'a {
        let inner = self.inner.borrow();
        self.change_counter_when_last_viewed = inner.get_change_counter();
        Ref::map(inner, |v| v.get_fresh_value())
    }

    pub fn get_fresh_value<'a>(&'a self) -> impl Deref<Target = T> + 'a {
        Ref::map(self.inner.borrow(), |v| v.get_fresh_value())
    }

    pub fn record_viewed(&mut self) {
        let inner = self.inner.borrow();
        self.change_counter_when_last_viewed = inner.get_change_counter();
    }

    // Note: getting last value does not count as a view so don't update change_counter_when_last_viewed
    pub fn get_last_value_even_if_stale<'a>(&'a self) -> impl Deref<Target = T> + 'a {
        Ref::map(self.inner.borrow(), |v| v.get_last_value_even_if_stale())
    }

    pub fn set_value(&self, new_val: T) {
        self.inner.borrow_mut().set_value(new_val);
    }

    pub fn set_value_and_used_default(&self, new_val: T, used_default: bool) {
        self.inner
            .borrow_mut()
            .set_value_and_used_default(new_val, used_default);
    }

    pub fn restore_previous_value(&self) {
        self.inner.borrow_mut().restore_previous_value();
    }

    pub fn get_used_default(&self) -> bool {
        self.inner.borrow().get_used_default()
    }

    // pub fn create_new_mutable_view(&self) -> StateVarTyped<T> {
    //     StateVarTyped {
    //         inner: self.inner.clone(),
    //     }
    // }

    pub fn mark_stale(&self) {
        self.inner.borrow_mut().mark_stale()
    }

    pub fn get_freshness(&self) -> Freshness {
        self.inner.borrow().freshness
    }

    pub fn request_change_value_to(&self, requested_val: T) {
        self.inner
            .borrow_mut()
            .request_change_value_to(requested_val);
    }

    pub fn get_requested_value<'a>(&'a self) -> impl Deref<Target = T> + 'a {
        Ref::map(self.inner.borrow(), |v| v.get_requested_value())
    }

    pub fn set_value_to_requested_value(&self) {
        let mut inner = self.inner.borrow_mut();
        inner.value = inner.requested_value.clone();
    }

    pub fn get_change_counter(&self) -> u32 {
        let inner = self.inner.borrow_mut();
        inner.get_change_counter()
    }
}

#[derive(Debug)]
pub struct StateVarReadOnlyViewTyped<T: Default + Clone> {
    // this must remain private to ensure read only
    inner: Rc<RefCell<StateVarInner<T>>>,
    change_counter_when_last_viewed: u32,
}

impl<T: Default + Clone> StateVarReadOnlyViewTyped<T> {
    pub fn new() -> Self {
        StateVarReadOnlyViewTyped {
            inner: Rc::new(RefCell::new(StateVarInner {
                value: T::default(),
                freshness: Freshness::Unresolved,
                requested_value: T::default(),
                used_default: false,
                change_counter: 1, // Note: start at 1 so starts out indicating it changed
            })),
            change_counter_when_last_viewed: 0,
        }
    }

    pub fn check_if_changed_since_last_viewed(&self) -> bool {
        self.inner.borrow().get_change_counter() > self.change_counter_when_last_viewed
    }

    pub fn get_fresh_value_record_viewed<'a>(&'a mut self) -> impl Deref<Target = T> + 'a {
        let inner = self.inner.borrow();
        self.change_counter_when_last_viewed = inner.get_change_counter();
        Ref::map(inner, |v| v.get_fresh_value())
    }

    pub fn get_fresh_value<'a>(&'a self) -> impl Deref<Target = T> + 'a {
        Ref::map(self.inner.borrow(), |v| v.get_fresh_value())
    }

    pub fn record_viewed(&mut self) {
        let inner = self.inner.borrow();
        self.change_counter_when_last_viewed = inner.get_change_counter();
    }

    // Note: getting last value does not count as a view so don't update change_counter_when_last_viewed
    pub fn get_last_value_even_if_stale<'a>(&'a self) -> impl Deref<Target = T> + 'a {
        Ref::map(self.inner.borrow(), |v| v.get_last_value_even_if_stale())
    }

    pub fn get_freshness(&self) -> Freshness {
        self.inner.borrow().freshness
    }

    pub fn get_used_default(&self) -> bool {
        self.inner.borrow().get_used_default()
    }

    pub fn create_new_read_only_view(&self) -> StateVarReadOnlyViewTyped<T> {
        StateVarReadOnlyViewTyped {
            inner: self.inner.clone(),
            change_counter_when_last_viewed: 0,
        }
    }

    pub fn request_change_value_to(&self, requested_val: T) {
        self.inner
            .borrow_mut()
            .request_change_value_to(requested_val);
    }

    pub fn get_requested_value<'a>(&'a self) -> impl Deref<Target = T> + 'a {
        Ref::map(self.inner.borrow(), |v| v.get_requested_value())
    }
}

impl From<&StateVarMutableView> for serde_json::Value {
    fn from(state_var: &StateVarMutableView) -> serde_json::Value {
        match state_var.get_freshness() {
            Freshness::Fresh => match state_var {
                // Note: use last value so that don't increment change_counter_when_last_viewed
                StateVarMutableView::Number(inner_sv) => {
                    serde_json::json!(*inner_sv.get_last_value_even_if_stale())
                }
                StateVarMutableView::Integer(inner_sv) => {
                    serde_json::json!(*inner_sv.get_last_value_even_if_stale())
                }
                StateVarMutableView::String(inner_sv) => {
                    serde_json::json!(inner_sv.get_last_value_even_if_stale().clone())
                }
                StateVarMutableView::Boolean(inner_sv) => {
                    serde_json::json!(*inner_sv.get_last_value_even_if_stale())
                }
                StateVarMutableView::MathExpr(inner_sv) => {
                    serde_json::json!(*inner_sv.get_last_value_even_if_stale())
                }
            },
            Freshness::Stale => serde_json::Value::Null,
            Freshness::Unresolved => serde_json::Value::Null,
        }
    }
}

impl StateVarMutableView {
    pub fn new_with_value(sv_val: StateVarValue, used_default: bool) -> Self {
        match sv_val {
            StateVarValue::Number(val) => StateVarMutableView::Number(
                StateVarMutableViewTyped::new_with_value(val, used_default),
            ),
            StateVarValue::Integer(val) => StateVarMutableView::Integer(
                StateVarMutableViewTyped::new_with_value(val, used_default),
            ),
            StateVarValue::String(val) => StateVarMutableView::String(
                StateVarMutableViewTyped::new_with_value(val, used_default),
            ),
            StateVarValue::Boolean(val) => StateVarMutableView::Boolean(
                StateVarMutableViewTyped::new_with_value(val, used_default),
            ),
            StateVarValue::MathExpr(val) => StateVarMutableView::MathExpr(
                StateVarMutableViewTyped::new_with_value(val, used_default),
            ),
        }
    }

    pub fn mark_stale(&mut self) {
        match self {
            StateVarMutableView::Number(sv_typed) => sv_typed.mark_stale(),
            StateVarMutableView::Integer(sv_typed) => sv_typed.mark_stale(),
            StateVarMutableView::String(sv_typed) => sv_typed.mark_stale(),
            StateVarMutableView::Boolean(sv_typed) => sv_typed.mark_stale(),
            StateVarMutableView::MathExpr(sv_typed) => sv_typed.mark_stale(),
        };
    }

    pub fn get_freshness(&self) -> Freshness {
        match self {
            StateVarMutableView::Number(sv_typed) => sv_typed.get_freshness(),
            StateVarMutableView::Integer(sv_typed) => sv_typed.get_freshness(),
            StateVarMutableView::String(sv_typed) => sv_typed.get_freshness(),
            StateVarMutableView::Boolean(sv_typed) => sv_typed.get_freshness(),
            StateVarMutableView::MathExpr(sv_typed) => sv_typed.get_freshness(),
        }
    }

    pub fn get_used_default(&self) -> bool {
        match self {
            StateVarMutableView::Number(sv_typed) => sv_typed.get_used_default(),
            StateVarMutableView::Integer(sv_typed) => sv_typed.get_used_default(),
            StateVarMutableView::String(sv_typed) => sv_typed.get_used_default(),
            StateVarMutableView::Boolean(sv_typed) => sv_typed.get_used_default(),
            StateVarMutableView::MathExpr(sv_typed) => sv_typed.get_used_default(),
        }
    }

    // use get_read_only_view to main a reference to the value
    // that you can repeatedly access its value
    // but allow the value to be modified when not accessing the value
    pub fn get_read_only_view(&self) -> StateVarReadOnlyView {
        match self {
            StateVarMutableView::Number(sv_inner) => {
                StateVarReadOnlyView::Number(sv_inner.create_new_read_only_view())
            }
            StateVarMutableView::Integer(sv_inner) => {
                StateVarReadOnlyView::Integer(sv_inner.create_new_read_only_view())
            }
            StateVarMutableView::String(sv_inner) => {
                StateVarReadOnlyView::String(sv_inner.create_new_read_only_view())
            }
            StateVarMutableView::Boolean(sv_inner) => {
                StateVarReadOnlyView::Boolean(sv_inner.create_new_read_only_view())
            }
            StateVarMutableView::MathExpr(sv_inner) => {
                StateVarReadOnlyView::MathExpr(sv_inner.create_new_read_only_view())
            }
        }
    }

    pub fn get_fresh_value(&self) -> StateVarValue {
        match self {
            StateVarMutableView::Number(sv_inner) => {
                StateVarValue::Number(*sv_inner.get_fresh_value())
            }
            StateVarMutableView::Integer(sv_inner) => {
                StateVarValue::Integer(*sv_inner.get_fresh_value())
            }
            StateVarMutableView::String(sv_inner) => {
                StateVarValue::String(sv_inner.get_fresh_value().clone())
            }
            StateVarMutableView::Boolean(sv_inner) => {
                StateVarValue::Boolean(*sv_inner.get_fresh_value())
            }
            StateVarMutableView::MathExpr(sv_inner) => {
                StateVarValue::MathExpr(sv_inner.get_fresh_value().clone())
            }
        }
    }

    pub fn set_value_to_requested_value(&self) {
        match self {
            StateVarMutableView::Number(sv_typed) => sv_typed.set_value_to_requested_value(),
            StateVarMutableView::Integer(sv_typed) => sv_typed.set_value_to_requested_value(),
            StateVarMutableView::String(sv_typed) => sv_typed.set_value_to_requested_value(),
            StateVarMutableView::Boolean(sv_typed) => sv_typed.set_value_to_requested_value(),
            StateVarMutableView::MathExpr(sv_typed) => sv_typed.set_value_to_requested_value(),
        }
    }

    pub fn get_type_as_str(&self) -> &'static str {
        match self {
            Self::String(_) => "string",
            Self::Boolean(_) => "boolean",
            Self::Integer(_) => "integer",
            Self::Number(_) => "number",
            Self::MathExpr(_) => "mathExpression",
        }
    }
}

impl StateVarReadOnlyView {
    pub fn get_freshness(&self) -> Freshness {
        match self {
            StateVarReadOnlyView::Number(sv_typed) => sv_typed.get_freshness(),
            StateVarReadOnlyView::Integer(sv_typed) => sv_typed.get_freshness(),
            StateVarReadOnlyView::String(sv_typed) => sv_typed.get_freshness(),
            StateVarReadOnlyView::Boolean(sv_typed) => sv_typed.get_freshness(),
            StateVarReadOnlyView::MathExpr(sv_typed) => sv_typed.get_freshness(),
        }
    }

    pub fn get_used_default(&self) -> bool {
        match self {
            StateVarReadOnlyView::Number(sv_typed) => sv_typed.get_used_default(),
            StateVarReadOnlyView::Integer(sv_typed) => sv_typed.get_used_default(),
            StateVarReadOnlyView::String(sv_typed) => sv_typed.get_used_default(),
            StateVarReadOnlyView::Boolean(sv_typed) => sv_typed.get_used_default(),
            StateVarReadOnlyView::MathExpr(sv_typed) => sv_typed.get_used_default(),
        }
    }

    pub fn get_fresh_value(&self) -> StateVarValue {
        match self {
            StateVarReadOnlyView::Number(sv_inner) => {
                StateVarValue::Number(*sv_inner.get_fresh_value())
            }
            StateVarReadOnlyView::Integer(sv_inner) => {
                StateVarValue::Integer(*sv_inner.get_fresh_value())
            }
            StateVarReadOnlyView::String(sv_inner) => {
                StateVarValue::String(sv_inner.get_fresh_value().clone())
            }
            StateVarReadOnlyView::Boolean(sv_inner) => {
                StateVarValue::Boolean(*sv_inner.get_fresh_value())
            }
            StateVarReadOnlyView::MathExpr(sv_inner) => {
                StateVarValue::MathExpr(sv_inner.get_fresh_value().clone())
            }
        }
    }

    pub fn get_type_as_str(&self) -> &'static str {
        match self {
            Self::String(_) => "string",
            Self::Boolean(_) => "boolean",
            Self::Integer(_) => "integer",
            Self::Number(_) => "number",
            Self::MathExpr(_) => "mathExpression",
        }
    }
}

impl<T: Default + Clone> StateVarTyped<T> {
    pub fn new(
        interface: Box<dyn StateVarInterface<T>>,
        parameters: StateVarParameters<T>,
    ) -> Self {
        let value = StateVarMutableViewTyped::new();
        StateVarTyped {
            immutable_view_of_value: value.create_new_read_only_view(),
            value,
            interface,
            parameters,
            change_counter_when_last_rendered: 0,
        }
    }

    // use get_read_only_view to main a reference to the value
    // that you can repeatedly access its value
    // but allow the value to be modified when not accessing the value
    pub fn create_new_read_only_view(&self) -> StateVarReadOnlyViewTyped<T> {
        StateVarReadOnlyViewTyped {
            inner: self.value.inner.clone(),
            change_counter_when_last_viewed: 0,
        }
    }

    pub fn get_fresh_value<'a>(&'a self) -> impl Deref<Target = T> + 'a {
        Ref::map(self.value.inner.borrow(), |v| v.get_fresh_value())
    }

    pub fn set_value(&self, new_val: T) {
        self.value.inner.borrow_mut().set_value(new_val);
    }

    pub fn set_value_and_used_default(&self, new_val: T, used_default: bool) {
        self.value
            .inner
            .borrow_mut()
            .set_value_and_used_default(new_val, used_default);
    }

    pub fn get_used_default(&self) -> bool {
        self.value.inner.borrow().get_used_default()
    }

    pub fn create_new_mutable_view(&self) -> StateVarMutableViewTyped<T> {
        StateVarMutableViewTyped {
            inner: self.value.inner.clone(),
            change_counter_when_last_viewed: 0,
        }
    }

    pub fn mark_stale(&self) {
        self.value.inner.borrow_mut().mark_stale()
    }

    pub fn get_freshness(&self) -> Freshness {
        self.value.inner.borrow().freshness
    }

    pub fn request_change_value_to(&self, requested_val: T) {
        self.value
            .inner
            .borrow_mut()
            .request_change_value_to(requested_val);
    }

    pub fn get_requested_value<'a>(&'a self) -> impl Deref<Target = T> + 'a {
        Ref::map(self.value.inner.borrow(), |v| v.get_requested_value())
    }

    pub fn return_dependency_instructions(&self) -> Vec<DependencyInstruction> {
        self.interface.return_dependency_instructions()
    }

    pub fn set_dependencies(&mut self, dependencies: &Vec<Vec<DependencyValue>>) -> () {
        self.interface.set_dependencies(dependencies)
    }

    fn calculate_state_var_from_dependencies(&mut self) -> () {
        self.interface
            .calculate_state_var_from_dependencies(&self.value)
    }

    fn request_dependencies_to_update_value(
        &self,
        is_initial_change: bool,
    ) -> Result<Vec<UpdatesRequested>, ()> {
        self.interface
            .request_dependencies_to_update_value(&self.immutable_view_of_value, is_initial_change)
    }

    fn get_name(&self) -> &'static str {
        self.parameters.name
    }

    fn return_for_renderer(&self) -> bool {
        self.parameters.for_renderer
    }

    fn return_initial_essential_value(&self) -> T {
        self.parameters.initial_essential_value.clone()
    }

    pub fn record_rendered(&mut self) {
        let inner = self.value.inner.borrow();
        self.change_counter_when_last_rendered = inner.get_change_counter();
    }

    pub fn check_if_changed_since_last_rendered(&self) -> bool {
        self.value.inner.borrow().get_change_counter() > self.change_counter_when_last_rendered
    }
}

impl StateVar {
    pub fn mark_stale(&mut self) {
        match self {
            StateVar::Number(sv_typed) => sv_typed.mark_stale(),
            StateVar::Integer(sv_typed) => sv_typed.mark_stale(),
            StateVar::String(sv_typed) => sv_typed.mark_stale(),
            StateVar::Boolean(sv_typed) => sv_typed.mark_stale(),
            StateVar::MathExpr(sv_typed) => sv_typed.mark_stale(),
        };
    }

    pub fn get_freshness(&self) -> Freshness {
        match self {
            StateVar::Number(sv_typed) => sv_typed.get_freshness(),
            StateVar::Integer(sv_typed) => sv_typed.get_freshness(),
            StateVar::String(sv_typed) => sv_typed.get_freshness(),
            StateVar::Boolean(sv_typed) => sv_typed.get_freshness(),
            StateVar::MathExpr(sv_typed) => sv_typed.get_freshness(),
        }
    }

    pub fn get_used_default(&self) -> bool {
        match self {
            StateVar::Number(sv_typed) => sv_typed.get_used_default(),
            StateVar::Integer(sv_typed) => sv_typed.get_used_default(),
            StateVar::String(sv_typed) => sv_typed.get_used_default(),
            StateVar::Boolean(sv_typed) => sv_typed.get_used_default(),
            StateVar::MathExpr(sv_typed) => sv_typed.get_used_default(),
        }
    }

    // use get_read_only_view to main a reference to the value
    // that you can repeatedly access its value
    // but allow the value to be modified when not accessing the value
    pub fn get_read_only_view(&self) -> StateVarReadOnlyView {
        match self {
            StateVar::Number(sv_inner) => {
                StateVarReadOnlyView::Number(sv_inner.create_new_read_only_view())
            }
            StateVar::Integer(sv_inner) => {
                StateVarReadOnlyView::Integer(sv_inner.create_new_read_only_view())
            }
            StateVar::String(sv_inner) => {
                StateVarReadOnlyView::String(sv_inner.create_new_read_only_view())
            }
            StateVar::Boolean(sv_inner) => {
                StateVarReadOnlyView::Boolean(sv_inner.create_new_read_only_view())
            }
            StateVar::MathExpr(sv_inner) => {
                StateVarReadOnlyView::MathExpr(sv_inner.create_new_read_only_view())
            }
        }
    }

    pub fn get_fresh_value(&self) -> StateVarValue {
        match self {
            StateVar::Number(sv_inner) => StateVarValue::Number(*sv_inner.get_fresh_value()),
            StateVar::Integer(sv_inner) => StateVarValue::Integer(*sv_inner.get_fresh_value()),
            StateVar::String(sv_inner) => StateVarValue::String(sv_inner.get_fresh_value().clone()),
            StateVar::Boolean(sv_inner) => StateVarValue::Boolean(*sv_inner.get_fresh_value()),
            StateVar::MathExpr(sv_inner) => {
                StateVarValue::MathExpr(sv_inner.get_fresh_value().clone())
            }
        }
    }

    pub fn request_change_value_to(&self, requested_val: StateVarValue) {
        match self {
            StateVar::Number(sv_typed) => {
                sv_typed.request_change_value_to(requested_val.try_into().unwrap())
            }
            StateVar::Integer(sv_typed) => {
                sv_typed.request_change_value_to(requested_val.try_into().unwrap())
            }
            StateVar::String(sv_typed) => {
                sv_typed.request_change_value_to(requested_val.try_into().unwrap())
            }
            StateVar::Boolean(sv_typed) => {
                sv_typed.request_change_value_to(requested_val.try_into().unwrap())
            }
            StateVar::MathExpr(sv_typed) => {
                sv_typed.request_change_value_to(requested_val.try_into().unwrap())
            }
        }
    }

    pub fn record_rendered(&mut self) {
        match self {
            StateVar::Number(sv_typed) => sv_typed.record_rendered(),
            StateVar::Integer(sv_typed) => sv_typed.record_rendered(),
            StateVar::String(sv_typed) => sv_typed.record_rendered(),
            StateVar::Boolean(sv_typed) => sv_typed.record_rendered(),
            StateVar::MathExpr(sv_typed) => sv_typed.record_rendered(),
        }
    }

    pub fn check_if_changed_since_last_rendered(&self) -> bool {
        match self {
            StateVar::Number(sv_typed) => sv_typed.check_if_changed_since_last_rendered(),
            StateVar::Integer(sv_typed) => sv_typed.check_if_changed_since_last_rendered(),
            StateVar::String(sv_typed) => sv_typed.check_if_changed_since_last_rendered(),
            StateVar::Boolean(sv_typed) => sv_typed.check_if_changed_since_last_rendered(),
            StateVar::MathExpr(sv_typed) => sv_typed.check_if_changed_since_last_rendered(),
        }
    }

    pub fn get_type_as_str(&self) -> &'static str {
        match self {
            Self::String(_) => "string",
            Self::Boolean(_) => "boolean",
            Self::Integer(_) => "integer",
            Self::Number(_) => "number",
            Self::MathExpr(_) => "mathExpression",
        }
    }

    pub fn return_dependency_instructions(&self) -> Vec<DependencyInstruction> {
        match self {
            StateVar::Number(sv_typed) => sv_typed.return_dependency_instructions(),
            StateVar::Integer(sv_typed) => sv_typed.return_dependency_instructions(),
            StateVar::String(sv_typed) => sv_typed.return_dependency_instructions(),
            StateVar::Boolean(sv_typed) => sv_typed.return_dependency_instructions(),
            StateVar::MathExpr(sv_typed) => sv_typed.return_dependency_instructions(),
        }
    }

    pub fn set_dependencies(&mut self, dependencies: &Vec<Vec<DependencyValue>>) {
        match self {
            StateVar::Number(sv_typed) => sv_typed.set_dependencies(dependencies),
            StateVar::Integer(sv_typed) => sv_typed.set_dependencies(dependencies),
            StateVar::String(sv_typed) => sv_typed.set_dependencies(dependencies),
            StateVar::Boolean(sv_typed) => sv_typed.set_dependencies(dependencies),
            StateVar::MathExpr(sv_typed) => sv_typed.set_dependencies(dependencies),
        }
    }

    pub fn calculate_state_var_from_dependencies(&mut self) {
        match self {
            StateVar::Number(sv_typed) => sv_typed.calculate_state_var_from_dependencies(),
            StateVar::Integer(sv_typed) => sv_typed.calculate_state_var_from_dependencies(),
            StateVar::String(sv_typed) => sv_typed.calculate_state_var_from_dependencies(),
            StateVar::Boolean(sv_typed) => sv_typed.calculate_state_var_from_dependencies(),
            StateVar::MathExpr(sv_typed) => sv_typed.calculate_state_var_from_dependencies(),
        }
    }

    pub fn request_dependencies_to_update_value(
        &self,
        is_initial_change: bool,
    ) -> Result<Vec<UpdatesRequested>, ()> {
        match self {
            StateVar::Number(sv_typed) => {
                sv_typed.request_dependencies_to_update_value(is_initial_change)
            }
            StateVar::Integer(sv_typed) => {
                sv_typed.request_dependencies_to_update_value(is_initial_change)
            }
            StateVar::String(sv_typed) => {
                sv_typed.request_dependencies_to_update_value(is_initial_change)
            }
            StateVar::Boolean(sv_typed) => {
                sv_typed.request_dependencies_to_update_value(is_initial_change)
            }
            StateVar::MathExpr(sv_typed) => {
                sv_typed.request_dependencies_to_update_value(is_initial_change)
            }
        }
    }

    pub fn get_name(&self) -> &'static str {
        match self {
            StateVar::Number(sv_typed) => sv_typed.get_name(),
            StateVar::Integer(sv_typed) => sv_typed.get_name(),
            StateVar::String(sv_typed) => sv_typed.get_name(),
            StateVar::Boolean(sv_typed) => sv_typed.get_name(),
            StateVar::MathExpr(sv_typed) => sv_typed.get_name(),
        }
    }

    pub fn return_for_renderer(&self) -> bool {
        match self {
            StateVar::Number(sv_typed) => sv_typed.return_for_renderer(),
            StateVar::Integer(sv_typed) => sv_typed.return_for_renderer(),
            StateVar::String(sv_typed) => sv_typed.return_for_renderer(),
            StateVar::Boolean(sv_typed) => sv_typed.return_for_renderer(),
            StateVar::MathExpr(sv_typed) => sv_typed.return_for_renderer(),
        }
    }

    pub fn return_initial_essential_value(&self) -> StateVarValue {
        match self {
            StateVar::Number(sv_typed) => {
                StateVarValue::Number(sv_typed.return_initial_essential_value())
            }
            StateVar::Integer(sv_typed) => {
                StateVarValue::Integer(sv_typed.return_initial_essential_value())
            }
            StateVar::String(sv_typed) => {
                StateVarValue::String(sv_typed.return_initial_essential_value())
            }
            StateVar::Boolean(sv_typed) => {
                StateVarValue::Boolean(sv_typed.return_initial_essential_value())
            }
            StateVar::MathExpr(sv_typed) => {
                StateVarValue::MathExpr(sv_typed.return_initial_essential_value())
            }
        }
    }

    pub fn get_default_component_type(&self) -> &'static str {
        match self {
            StateVar::Number(_) => "number",
            StateVar::Integer(_) => "number",
            StateVar::String(_) => "text",
            StateVar::Boolean(_) => "boolean",
            StateVar::MathExpr(_) => {
                unimplemented!("Should not have math expression state variable")
            }
        }
    }
}

// Boilerplate to display EssentialStateVar and StateVar better

impl fmt::Debug for StateVar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.get_freshness() {
            Freshness::Fresh => self.get_fresh_value().fmt(f),
            Freshness::Stale => f.write_str("Stale"),
            Freshness::Unresolved => f.write_str("Unresolved"),
        }
    }
}

impl fmt::Debug for StateVarMutableView {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.get_freshness() {
            Freshness::Fresh => self.get_fresh_value().fmt(f),
            Freshness::Stale => f.write_str("Stale"),
            Freshness::Unresolved => f.write_str("Unresolved"),
        }
    }
}

impl fmt::Debug for StateVarReadOnlyView {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.get_freshness() {
            Freshness::Fresh => self.get_fresh_value().fmt(f),
            Freshness::Stale => f.write_str("Stale"),
            Freshness::Unresolved => f.write_str("Unresolved"),
        }
    }
}

pub type EssentialStateVar = StateVarMutableView;

// /// A special endpoint on the dependency graph which is associated with a
// /// particular state var. Actions often update these.
// /// An EssentialStateVar cannot be stale

// #[derive(Debug)]
// pub enum EssentialStateVar {
//     Number(EssentialStateVarTyped<f64>),
//     Integer(EssentialStateVarTyped<i64>),
//     String(EssentialStateVarTyped<String>),
//     Boolean(EssentialStateVarTyped<bool>),
//     MathExpr(EssentialStateVarTyped<MathExpression>),
// }

// #[derive(Debug)]
// pub struct EssentialStateVarTyped<T: Default + Clone> {
//     inner: Rc<RefCell<EssentialStateVarInner<T>>>,
// }

// #[derive(Debug)]
// struct EssentialStateVarInner<T: Default + Clone> {
//     value: T,
// }

// impl<T: Default + Clone> EssentialStateVarInner<T> {
//     pub fn get_value<'a>(&'a self) -> &'a T {
//         &self.value
//     }

//     pub fn set_value(&mut self, new_val: T) {
//         self.value = new_val;
//     }
// }

// impl<T: Default + Clone> EssentialStateVarTyped<T> {
//     pub fn new(val: T) -> Self {
//         EssentialStateVarTyped {
//             inner: Rc::new(RefCell::new(EssentialStateVarInner {
//                 value: val,
//             })),
//         }
//     }

//     pub fn get_value<'a>(&'a self) -> impl Deref<Target = T> + 'a {
//         Ref::map(self.inner.borrow(), |v| v.get_value())
//     }

//     pub fn set_value(&self, new_val: T) {
//         self.inner.borrow_mut().set_value(new_val);
//     }

//     pub fn create_new_mutable_view(&self) -> EssentialStateVarTyped<T> {
//         EssentialStateVarTyped {
//             inner: self.inner.clone(),
//         }
//     }

// }

// impl From<&EssentialStateVar> for serde_json::Value {
//     fn from(state_var: &EssentialStateVar) -> serde_json::Value {
//         match state_var {
//             EssentialStateVar::Number(inner_sv) => {
//                 serde_json::json!(*inner_sv.get_value())
//             }
//             EssentialStateVar::Integer(inner_sv) => {
//                 serde_json::json!(*inner_sv.get_value())
//             }
//             EssentialStateVar::String(inner_sv) => {
//                 serde_json::json!(inner_sv.get_value().clone())
//             }
//             EssentialStateVar::Boolean(inner_sv) => {
//                 serde_json::json!(*inner_sv.get_value())
//             }
//             EssentialStateVar::MathExpr(inner_sv) => {
//                 serde_json::json!(*inner_sv.get_value())
//             }
//         }
//     }
// }

// impl EssentialStateVar {

//     pub fn new(value: StateVarValue) -> Self {
//         match value {
//             StateVarValue::Number(typed_val) => EssentialStateVar::Number(
//                 EssentialStateVarTyped::new(typed_val)
//             ),
//             StateVarValue::Integer(typed_val) => EssentialStateVar::Integer(
//                 EssentialStateVarTyped::new(typed_val)
//             ),
//             StateVarValue::String(typed_val) => EssentialStateVar::String(
//                 EssentialStateVarTyped::new(typed_val)
//             ),
//             StateVarValue::Boolean(typed_val) => EssentialStateVar::Boolean(
//                 EssentialStateVarTyped::new(typed_val)
//             ),
//             StateVarValue::MathExpr(typed_val) => EssentialStateVar::MathExpr(
//                 EssentialStateVarTyped::new(typed_val)
//             ),
//         }
//     }

//     pub fn get_value(&self) -> StateVarValue {
//         match self {
//             EssentialStateVar::Number(sv_inner) => {
//                 StateVarValue::Number(*sv_inner.get_value())
//             }
//             EssentialStateVar::Integer(sv_inner) => {
//                 StateVarValue::Integer(*sv_inner.get_value())
//             }
//             EssentialStateVar::String(sv_inner) => {
//                 StateVarValue::String(sv_inner.get_value().clone())
//             }
//             EssentialStateVar::Boolean(sv_inner) => {
//                 StateVarValue::Boolean(*sv_inner.get_value())
//             }
//             EssentialStateVar::MathExpr(sv_inner) => {
//                 StateVarValue::MathExpr(sv_inner.get_value().clone())
//             }
//         }
//     }

//     pub fn get_type_as_str(&self) -> &'static str {
//         match self {
//             Self::String(_) => "string",
//             Self::Boolean(_) => "boolean",
//             Self::Integer(_) => "integer",
//             Self::Number(_) => "number",
//             Self::MathExpr(_) => "mathExpression",
//         }
//     }
// }
