use crate::actions::Actioner;
use crate::states::Stater;
use crate::stats::ActionStatter;
use std::{collections::HashMap, marker};

#[derive(Clone)]
pub struct QMap<'a, S, A, AS>
where
    A: Actioner<'a>,
    S: Stater<'a, A>,
    AS: ActionStatter,
{
    #[allow(dead_code)]
    pub(crate) data: HashMap<&'a str, HashMap<&'a str, Box<AS>>>,
    _actioner: marker::PhantomData<A>,
    _stater: marker::PhantomData<S>,
}

impl<'a, S, A, AS> QMap<'a, S, A, AS>
where
    A: Actioner<'a>,
    S: Stater<'a, A>,
    AS: ActionStatter,
{
    #[allow(dead_code)]
    pub(crate) fn new() -> QMap<'a, S, A, AS> {
        QMap {
            data: HashMap::new(),
            _actioner: marker::PhantomData {},
            _stater: marker::PhantomData {},
        }
    }

    #[allow(dead_code)]
    pub(crate) fn get_stats(&mut self, state: &'a S, action: &'a A) -> Option<Box<AS>> {
        let actions = self.get_actions_for_state(state);
        if let Some(stat) = actions.get(action.id()) {
            return Some(Box::new(*stat.clone()));
        }
        None
    }

    #[allow(dead_code)]
    pub(crate) fn update_stats(&mut self, state: &'a S, action: &'a A, stats: Box<AS>) {
        self.get_actions_for_state(state).insert(action.id(), stats);
    }

    #[allow(dead_code)]
    pub(crate) fn get_actions_for_state(&mut self, state: &'a S) -> &mut HashMap<&'a str, Box<AS>> {
        self.data.entry(state.id()).or_insert_with(HashMap::new)
    }
}

#[cfg(test)]
#[allow(clippy::wildcard_imports, clippy::default_trait_access, clippy::panic)]
mod tests {
    use crate::internal::datastructures::QMap;
    use crate::mocks::*;
    use crate::stats::actionstats::Stats;

    #[test]
    /// If the qmap does not contain any entries for a state, the state
    /// should be added with an empty hashmap value.
    fn get_actions_for_state() {
        let state: MockStater<MockActioner> = MockStater {
            return_id: "A",
            return_possible_actions: vec![],
            ..Default::default()
        };

        let mut qmap: QMap<MockStater<MockActioner>, MockActioner, Stats> = QMap::new();
        let result = qmap.get_actions_for_state(&state);
        assert_eq!(result.len(), 0, "state map must be empty");
    }

    #[test]
    fn get_stats_no_data() {
        let action = MockActioner { return_id: "X" };

        let state: MockStater<MockActioner> = MockStater {
            return_id: "A",
            return_possible_actions: vec![&action],
            ..Default::default()
        };

        let mut qmap: QMap<MockStater<MockActioner>, MockActioner, Stats> = QMap::new();
        let result = qmap.get_stats(&state, &action);

        assert!(result.is_none(), "result should be None");
    }

    #[test]
    fn get_stats_state_has_data() {
        let action = MockActioner { return_id: "X" };

        let state: MockStater<MockActioner> = MockStater {
            return_id: "A",
            return_possible_actions: vec![&action],
            ..Default::default()
        };

        let stats = Box::new(Stats::default());

        let mut qmap: QMap<MockStater<MockActioner>, MockActioner, Stats> = QMap::new();
        qmap.update_stats(&state, &action, stats);
        let result = qmap.get_stats(&state, &action);

        assert!(result.is_some(), "result should be Some");
    }
}
