use crate::errors::LearnerError;
use crate::iface::*;

pub(crate) struct MockStater<'a, A> {
    pub(crate) return_id: &'a str,
    pub(crate) return_possible_actions: Vec<&'a A>,
    pub(crate) return_action_is_compatible: &'a dyn Fn(&'a A) -> bool,
    pub(crate) return_apply: &'a dyn Fn(&'a A) -> Result<(), LearnerError>,
}

impl<'a, A> Stater<'a, A> for MockStater<'a, A>
where
    A: Actioner<'a>,
{
    fn possible_actions(&self) -> Vec<&'a A> {
        self.return_possible_actions.as_slice().into()
    }

    fn action_is_compatible(&self, action: &'a A) -> bool {
        (self.return_action_is_compatible)(action)
    }

    fn get_action(&self, action_name: &str) -> Result<&'a A, LearnerError> {
        for action in &self.return_possible_actions {
            if action.id() == action_name {
                return Ok(action);
            }
        }
        panic!(format!(
            "Action '{}' not found in MockStater '{}'",
            action_name,
            self.id()
        ))
    }

    fn id(&self) -> &str {
        self.return_id
    }

    fn apply(&self, action: &'a A) -> Result<(), LearnerError> {
        (self.return_apply)(action)
    }
}

pub(crate) struct MockActioner<'a> {
    pub(crate) return_id: &'a str,
}

impl<'a> Actioner<'a> for MockActioner<'a> {
    fn id(&self) -> &'a str {
        self.return_id
    }
}
