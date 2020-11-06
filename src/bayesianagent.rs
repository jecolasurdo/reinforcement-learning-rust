use crate::iface::{ActionStatter, Actioner, Agenter, Stater};
use crate::internal::datastructures::QMap;
use crate::{errors::LearnerError, internal::math};
use rand::Rng;
use std::collections::HashMap;
use std::marker;

pub struct BayesianAgent<S, A: 'static, AS>
where
    A: Actioner,
    S: Stater<A>,
    AS: ActionStatter,
{
    pub tie_breaker: Box<dyn Fn(i64) -> i64>,
    qmap: Box<QMap<S, A, AS>>,
    learning_rate: f64,
    discount_factor: f64,
    priming_threshold: i64,
    _actioner: marker::PhantomData<A>,
    _stater: marker::PhantomData<S>,
}

pub struct AgentContext<AS: ActionStatter> {
    pub learning_rate: f64,
    pub discount_factor: f64,
    pub priming_threshold: i64,
    pub q_values: HashMap<String, HashMap<String, AS>>,
}

impl<S, A: 'static, AS> Agenter<S, A> for BayesianAgent<S, A, AS>
where
    S: Stater<A>,
    A: Actioner,
    AS: ActionStatter,
{
    fn learn(
        &mut self,
        previous_state: Option<S>,
        action_taken: &mut A,
        current_state: &mut S,
        reward: f64,
    ) {
        if previous_state.is_none() {
            return;
        }
        let mut previous_state = previous_state.unwrap();
        let mut stats = match self.qmap.get_stats(&mut previous_state, action_taken) {
            Some(s) => s.clone(),
            None => AS::new(),
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

    fn transition(&self, stater: S, actioner: A) -> Result<(), LearnerError> {
        unimplemented!();
    }

    fn recommend_action(&self, stater: S) -> Result<A, LearnerError> {
        unimplemented!();
    }
}

impl<S, A: 'static, AS> BayesianAgent<S, A, AS>
where
    S: Stater<A>,
    A: Actioner,
    AS: ActionStatter,
{
    pub fn new(
        priming_threshold: i64,
        learning_rate: f64,
        discount_factor: f64,
    ) -> BayesianAgent<S, A, AS>
    where
        S: Stater<A>,
        A: Actioner,
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

    // pub fn get_agent_context(&self) -> AgentContext<AS> {
    //     AgentContext {
    //         learning_rate: self.learning_rate,
    //         discount_factor: self.discount_factor,
    //         priming_threshold: self.priming_threshold,
    //         q_values: self.qmap.data,
    //     }
    // }

    fn apply_action_weights(&mut self, state: &mut S) {
        let mut raw_value_sum = 0.0;
        let mut existing_action_count = 0;
        for mut action in state.possible_actions() {
            match self.qmap.get_stats(state, &mut action) {
                Some(s) => {
                    raw_value_sum += s.q_value_raw();
                    existing_action_count += 1;
                }
                None => self.qmap.update_stats(state, &mut action, AS::new()),
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

    fn get_best_value(&mut self, state: &mut S) -> f64 {
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
    use crate::{actionstats::ActionStats, iface::*};

    #[test]
    fn test_learn() {
        // Encapulating mock state in a function so the mocks can be created
        // and used in multiple places (since the mocks aren't Copy)
        let create_mock_actions = || -> Vec<MockActioner> {
            let mut action1 = MockActioner::new();
            action1.expect_id().times(..).return_const("X");

            let mut action2 = MockActioner::new();
            action2.expect_id().times(..).return_const("Y");

            let mut action3 = MockActioner::new();
            action3.expect_id().times(..).return_const("Z");

            vec![action1, action2, action3]
        };

        // Encapulating mock state in a function so the mocks can be created
        // and used in multiple places (since the mocks aren't Copy)
        let mock_previous_state = || -> MockStater<MockActioner> {
            let mut previous_state = MockStater::new();
            previous_state.expect_id().times(..).return_const("A");
            previous_state
                .expect_possible_actions()
                .times(..)
                .returning(create_mock_actions);
            return previous_state;
        };

        let mut current_state: MockStater<MockActioner> = MockStater::new();
        current_state.expect_id().times(..).return_const("B");
        current_state
            .expect_possible_actions()
            .times(..)
            .returning(create_mock_actions);

        let mut ba: BayesianAgent<MockStater<MockActioner>, MockActioner, ActionStats> =
            BayesianAgent::new(10, 1.0, 0.0);
        let reward = 1.0;
        let mut mock_actions = create_mock_actions();
        ba.learn(
            Some(mock_previous_state()),
            &mut mock_actions[0], // Action X
            &mut current_state,
            reward,
        );
        ba.learn(
            Some(mock_previous_state()),
            &mut mock_actions[1], // Action Y
            &mut current_state,
            reward,
        );
    }
}
