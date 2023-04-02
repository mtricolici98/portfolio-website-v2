use std::rc::Rc;

use yew::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NavContext {
    pub active_submenu: Option<String>,
}

impl Reducible for NavContext {
    type Action = Option<String>;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        NavContext { active_submenu: action }.into()
    }
}

pub type NavigationContext = UseReducerHandle<NavContext>;

#[derive(Properties, Debug, PartialEq)]
pub struct NavContextProviderProps {
    #[prop_or_default]
    pub children: Children,
}
