use std::{collections::HashMap, marker};

use crate::iface::{ActionStatter, Actioner, Stater};

struct QMap<S, A, AS>
where
    A: Actioner,
    S: Stater<A>,
    AS: ActionStatter + Copy,
{
    #[allow(dead_code)]
    data: HashMap<String, HashMap<String, AS>>,
    _actioner: marker::PhantomData<A>,
    _stater: marker::PhantomData<S>,
}

impl<S, A, AS> QMap<S, A, AS>
where
    A: Actioner,
    S: Stater<A> + Copy,
    AS: ActionStatter + Copy,
{
    #[allow(dead_code)]
    fn new() -> QMap<S, A, AS> {
        QMap {
            data: HashMap::new(),
            _actioner: marker::PhantomData {},
            _stater: marker::PhantomData {},
        }
    }

    fn get_stats(&mut self, state: S, action: A) -> Option<AS> {
        let actions = self.get_actions_for_state(state);
        if let Some(action) = actions.get(action.id().as_str()) {
            return Some(*action);
        }
        None
    }

    fn update_stats(&mut self, state: S, action: A, stats: AS) {
        self.get_actions_for_state(state).insert(action.id(), stats);
    }

    fn get_actions_for_state(&mut self, state: S) -> HashMap<String, AS> {
        if !self.data.contains_key(state.id().as_str()) {
            self.data.insert(state.id(), HashMap::new());
        }
        self.data[state.id().as_str()].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::iface::*;
    use mockall::predicate::*;
    use mockall::*;

    #[test]
    fn get_stats_no_data() {
        // let mut mock_stater = MockStater::new();
        let mut mock_actioner = MockActioner::new();
        mock_actioner.expect_id().times(1).return_const("test");
        assert_eq!("test", mock_actioner.id());

        // let qmap = QMap::new();
        // let result = qmap.get_stats(state, action);
        // assert_eq!(None, result);
    }
}
