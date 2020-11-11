use crate::iface::{ActionStatter, Actioner, Agenter, Stater};
use crate::internal::datastructures::QMap;
use crate::{errors::LearnerError, internal::math};
use rand::Rng;
use std::collections::HashMap;
use std::marker;

pub struct BayesianAgent<'a, S, A, AS>
where
    A: Actioner<'a>,
    S: Stater<'a, A>,
    AS: ActionStatter,
{
    pub tie_breaker: Box<dyn Fn(i64) -> i64>,
    qmap: Box<QMap<'a, S, A, AS>>,
    learning_rate: f64,
    discount_factor: f64,
    priming_threshold: i64,
    _actioner: marker::PhantomData<A>,
    _stater: marker::PhantomData<S>,
}

#[derive(Debug, PartialEq)]
pub struct AgentContext<'a, AS: ActionStatter> {
    pub learning_rate: f64,
    pub discount_factor: f64,
    pub priming_threshold: i64,
    pub q_values: HashMap<&'a str, HashMap<&'a str, Box<AS>>>,
}

impl<'a, S, A, AS> Agenter<'a, S, A> for BayesianAgent<'a, S, A, AS>
where
    S: Stater<'a, A>,
    A: Actioner<'a>,
    AS: ActionStatter,
{
    fn learn(
        &mut self,
        previous_state: Option<&'a S>,
        action_taken: &'a A,
        current_state: &'a S,
        reward: f64,
    ) {
        if previous_state.is_none() {
            return;
        }
        let mut previous_state = previous_state.unwrap();
        let mut stats = match self.qmap.get_stats(previous_state, action_taken) {
            Some(s) => s.clone(),
            None => Box::new(AS::default()),
        };

        self.apply_action_weights(current_state);
        let new_value = math::bellman(
            stats.q_value_weighted(),
            self.learning_rate,
            reward,
            self.discount_factor,
            self.get_best_value(current_state),
        );
        stats.set_calls(stats.calls() + 1);
        stats.set_q_value_raw(new_value);
        self.qmap
            .update_stats(&mut previous_state, action_taken, stats);
        self.apply_action_weights(&mut previous_state);
    }

    fn transition(&self, current_state: &'a S, action: &'a A) -> Result<(), LearnerError> {
        if !current_state.action_is_compatible(action) {
            return Err(LearnerError::new(format!(
                "action {} is not compatible with state {}",
                action.id().to_string(),
                current_state.id().to_string()
            )));
        }
        current_state.apply(action)
    }

    fn recommend_action(&self, stater: &'a S) -> Result<&'a A, LearnerError> {
        unimplemented!();
    }
}

impl<'a, S, A: 'a, AS> BayesianAgent<'a, S, A, AS>
where
    S: Stater<'a, A>,
    A: Actioner<'a>,
    AS: ActionStatter,
{
    pub fn new(
        priming_threshold: i64,
        learning_rate: f64,
        discount_factor: f64,
    ) -> BayesianAgent<'a, S, A, AS>
    where
        S: Stater<'a, A>,
        A: Actioner<'a>,
        AS: ActionStatter,
    {
        BayesianAgent {
            tie_breaker: Box::new(|n: i64| -> i64 { rand::thread_rng().gen_range(0, n) }),
            qmap: Box::new(QMap::new()),
            learning_rate,
            discount_factor,
            priming_threshold,
            _actioner: marker::PhantomData {},
            _stater: marker::PhantomData {},
        }
    }

    pub fn get_agent_context(&self) -> AgentContext<AS> {
        AgentContext {
            learning_rate: self.learning_rate,
            discount_factor: self.discount_factor,
            priming_threshold: self.priming_threshold,
            q_values: self.qmap.data.clone(),
        }
    }

    fn apply_action_weights(&mut self, state: &'a S) {
        let mut raw_value_sum = 0.0;
        let mut existing_action_count = 0;
        for mut action in state.possible_actions() {
            match self.qmap.get_stats(state, &mut action) {
                Some(s) => {
                    raw_value_sum += s.q_value_raw();
                    existing_action_count += 1;
                }
                None => self
                    .qmap
                    .update_stats(state, &mut action, Box::new(AS::default())),
            }
        }

        let mean = math::safe_divide(raw_value_sum, existing_action_count as f64);
        let action_stats = self.qmap.get_actions_for_state(state);
        for (_, stats) in action_stats {
            let weighted_mean = math::bayesian_average(
                self.priming_threshold as f64,
                stats.calls() as f64,
                mean,
                stats.q_value_raw(),
            );
            stats.set_q_value_weighted(weighted_mean);
        }
    }

    fn get_best_value(&mut self, state: &'a S) -> f64 {
        let mut best_q_value = 0.0;
        for (_, stat) in self.qmap.get_actions_for_state(state) {
            let q = stat.q_value_weighted();
            if q > best_q_value {
                best_q_value = q;
            }
        }
        best_q_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actionstats::ActionStats;
    use crate::mocks::*;
    use maplit::hashmap;
    use std::cell::RefCell;

    #[test]
    fn learn() {
        let action_x = MockActioner { return_id: "X" };
        let action_y = MockActioner { return_id: "Y" };
        let action_z = MockActioner { return_id: "Z" };
        let mock_actions = || -> Vec<&MockActioner> { vec![&action_x, &action_y, &action_z] };

        let previous_state = MockStater {
            return_id: "A",
            return_possible_actions: mock_actions(),
            return_action_is_compatible: &|_| -> bool { unimplemented!() },
            return_apply: &|_| -> Result<(), LearnerError> { unimplemented!() },
        };

        let current_state = MockStater {
            return_id: "B",
            return_possible_actions: mock_actions(),
            return_action_is_compatible: &|_| -> bool { unimplemented!() },
            return_apply: &|_| -> Result<(), LearnerError> { unimplemented!() },
        };

        let mut ba: BayesianAgent<MockStater<MockActioner>, MockActioner, ActionStats> =
            BayesianAgent::new(10, 1.0, 0.0);
        let reward = 1.0;
        ba.learn(Some(&previous_state), &action_x, &current_state, reward);
        ba.learn(Some(&previous_state), &action_y, &current_state, reward);

        let actual = ba.get_agent_context();

        let expected = AgentContext {
            learning_rate: 1.0,
            discount_factor: 0.0,
            priming_threshold: 10,
            q_values: hashmap! {
                "A" => hashmap! {
                    "X" => Box::new(ActionStats {call_count: 1, q_raw: 1.0, q_weighted: 0.6969696969696969}),
                    "Y" => Box::new(ActionStats {call_count: 1, q_raw: 1.0, q_weighted: 0.6969696969696969}),
                    "Z" => Box::new(ActionStats {call_count: 0, q_raw: 0.0, q_weighted: 0.66666666666666666}),
                },
                "B" => hashmap! {
                    "X" => Box::new(ActionStats {call_count: 0, q_raw: 0.0, q_weighted: 0.0}),
                    "Y" => Box::new(ActionStats {call_count: 0, q_raw: 0.0, q_weighted: 0.0}),
                    "Z" => Box::new(ActionStats {call_count: 0, q_raw: 0.0, q_weighted: 0.0}),
                },
            },
        };
        assert_eq!(expected, actual);
    }

    #[test]
    fn transition() {
        // returns error if action not compatible
        // applies action otherwise

        let action_x = MockActioner { return_id: "X" };
        let mock_actions = vec![&action_x];

        let applied_action_id: RefCell<Option<&str>> = RefCell::new(None);
        let current_state = MockStater {
            return_id: "A",
            return_possible_actions: mock_actions,
            return_action_is_compatible: &|_| -> bool {
                return true;
            },
            return_apply: &|action| -> Result<(), LearnerError> {
                applied_action_id.replace(Some(action.id()));
                Ok(())
            },
        };

        let ba: BayesianAgent<MockStater<MockActioner>, MockActioner, ActionStats> =
            BayesianAgent::new(0, 0.0, 0.0);
        let transition_result = ba.transition(&current_state, &action_x);
        assert!(transition_result.is_ok());
        assert!(!applied_action_id.borrow().is_none());
        assert_eq!(action_x.id(), applied_action_id.borrow().unwrap());
    }
}
