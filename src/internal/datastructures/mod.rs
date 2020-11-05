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

    fn get_stats(&mut self, state: &mut S, action: &mut A) -> Option<&AS> {
        let actions = self.get_actions_for_state(state);
        actions.get(action.id().as_str())
    }

    fn update_stats(&mut self, state: &mut S, action: &mut A, stats: AS) {
        self.get_actions_for_state(state)
            .insert(action.id().to_owned(), stats);
    }

    fn get_actions_for_state(&mut self, state: &mut S) -> &mut HashMap<String, AS> {
        self.data.entry(state.id()).or_insert(HashMap::new())
    }
}

#[cfg(test)]
mod tests {
    use crate::iface::*;
    use crate::internal::datastructures::QMap;

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
        assert_eq!(result.len(), 0, "state map must be empty");
    }

    #[test]
    fn get_stats_no_data() {
        let mut action: MockActioner = MockActioner::new();
        action.expect_id().times(..).return_const("X");

        let mut state: MockStater<MockActioner> = MockStater::new();
        state.expect_id().times(..).return_const("A");

        let mut qmap: QMap<MockStater<MockActioner>, MockActioner, MockActionStatter> = QMap::new();
        let result = qmap.get_stats(&mut state, &mut action);

        assert!(result.is_none(), "result should be None");
    }
}
