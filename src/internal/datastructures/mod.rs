use std::{collections::HashMap, marker};

use crate::iface::{ActionStatter, Actioner, Stater};

struct QMap<S, A: 'static, AS>
where
    A: Actioner,
    S: Stater<A>,
    AS: ActionStatter,
{
    #[allow(dead_code)]
    data: HashMap<String, HashMap<String, AS>>,
    _actioner: marker::PhantomData<A>,
    _stater: marker::PhantomData<S>,
}

impl<S, A: 'static, AS> QMap<S, A, AS>
where
    A: Actioner,
    S: Stater<A>,
    AS: ActionStatter,
{
    #[allow(dead_code)]
    fn new() -> QMap<S, A, AS> {
        QMap {
            data: HashMap::new(),
            _actioner: marker::PhantomData {},
            _stater: marker::PhantomData {},
        }
    }

    // fn get_stats(&mut self, state: S, action: A) -> Option<AS> {
    //     let actions = self.get_actions_for_state(state);
    //     if let Some(action) = actions.get(action.id().as_str()) {
    //         return Some(action);
    //     }
    //     None
    // }

    // fn update_stats(&mut self, state: S, action: A, stats: AS) {
    //     self.get_actions_for_state(state).insert(action.id(), stats);
    // }

    fn get_actions_for_state(&mut self, state: &mut S) -> &HashMap<String, AS> {
        if !self.data.contains_key(state.id().as_str()) {
            self.data.insert(state.id(), HashMap::new());
        }
        &self.data[state.id().as_str()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::iface::*;
    use mockall::predicate::*;
    use mockall::*;
    use std::fmt::{self, Debug};

    #[test]
    /// If the qmap does not contain any entries for a state, the state
    /// should be added with an empty hashmap value.
    fn get_actions_for_state() {
        let mut action: MockActioner = MockActioner::new();
        action.expect_id().times(0).return_const("X");

        let mut state: MockStater<MockActioner> = MockStater::new();
        state.expect_id().times(..).return_const("A");

        let mut qmap: QMap<MockStater<MockActioner>, MockActioner, MockActionStatter> = QMap::new();
        let result = qmap.get_actions_for_state(&mut state);
        assert_eq!(
            result.keys().nth(0).unwrap(),
            "A",
            "key must equal the state's id"
        );
        assert_eq!(result.values().len(), 0, "state map must be empty");
    }
}
