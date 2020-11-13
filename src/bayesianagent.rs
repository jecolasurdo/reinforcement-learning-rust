use crate::iface::{ActionStatter, Actioner, Agenter, Stater};
use crate::internal::datastructures::QMap;
use crate::{errors::LearnerError, internal::math};
use rand::Rng;
use std::collections::HashMap;
use std::marker;

/// BayesianAgent provides facilities for 1) maintaining the learning state of
/// an environment, 2) making recommendations for actions based on the previous,
/// current, and predicted states of the system, and 3) executing actions that
/// have been recommended by the agent.
///
/// The BayesianAgent is so named because of the way it handles initial
/// conditions of the q-values associated with each of a state's actions.
/// When the agent is asked to recommend an action for some state, the agent
/// does so by choosing the action that has previously recorded a greater
/// cumulative reward than other possible actions.
///
/// This poses a dilema for initial conditions when no reward has been
/// previously recorded for one or more of the potential actions. To overcome
/// this, the BayesianAgent applies a Bayesian Average function to each
/// potential action. In essense, when an action has been called few (or zero)
/// times, it is assumed that the reward for calling that action might be
/// similar to that of calling any other action. Thus the agent weights its
/// potential reward closer to the mean of all other actions. However, as an
/// action is called more times, the agent begins to evaluate the action on its
/// observed cumulative reward moreso than the mean of all other actions.
pub struct BayesianAgent<'a, S, A, AS>
where
    A: Actioner<'a>,
    S: Stater<'a, A>,
    AS: ActionStatter,
{
    pub tie_breaker: Box<dyn Fn(usize) -> usize + 'a>,
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
    /// 'learn' updates the reinforcement model according to a transition that
    /// has occured from a previous state, through some action, to a current
    /// state. The reward value represents the positive, negative, or neutral
    /// impact that the transition has had on the environment. `previous_state`
    /// may be None if no action has been previously taken or there is no
    /// previous state (aka the system is being bootstrapped). In that case,
    /// learn becomes a no-op.
    /// See https://en.wikipedia.org/wiki/Q-learning#Algorithm
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
        let previous_state = previous_state.unwrap();
        let mut stats = match self.qmap.get_stats(previous_state, action_taken) {
            Some(s) => s,
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
        self.qmap.update_stats(&previous_state, action_taken, stats);
        self.apply_action_weights(&previous_state);
    }

    /// `transition` applies an action to a given state.
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

    /// `recommend_action` recommends an action for a given state based on
    /// behavior of the system that the agent has learned thus far.
    /// If the q-value for two or more actions are the same, the action is
    /// chosen according to a tie-breaking function. See BayesianAgent docs for
    /// more information.
    fn recommend_action(&mut self, state: &'a S) -> Result<&'a A, LearnerError> {
        struct ActionValue<'a> {
            a: &'a str,
            v: f64,
        }

        let mut best_actions: Vec<ActionValue> = Vec::new();
        let mut best_value = -1.0 * f64::MAX;

        self.apply_action_weights(state);
        for (action, stats) in self.qmap.get_actions_for_state(state) {
            let av = ActionValue {
                a: action,
                v: stats.q_value_weighted(),
            };

            if av.v > best_value {
                best_value = av.v;
                best_actions = vec![av];
            } else if (av.v - best_value).abs() < f64::EPSILON {
                best_actions.push(av);
            }
        }

        if best_actions.is_empty() {
            return Err(LearnerError::new(format!(
                "state '{}' reports no possible actions",
                state.id()
            )));
        }

        // Order of records in a hashmap is nondeterministic, so we sort
        // alphabetically by action ID to get a deterministic result.
        // Note that it is documented that it is the implementor's
        // responsibility to ensure that each action's ID is unique across all
        // possible actions within the scope of the agent, and that having
        // different actions share an ID will cause undefined behavior.
        best_actions.sort_by(|x, y| x.a.cmp(y.a));
        let tie_breaker = (self.tie_breaker)(best_actions.len());
        state.get_action(best_actions[tie_breaker].a)
    }
}

impl<'a, S, A: 'a, AS> BayesianAgent<'a, S, A, AS>
where
    S: Stater<'a, A>,
    A: Actioner<'a>,
    AS: ActionStatter,
{
    /// new returns a reference to a new BayesianAgent.
    ///
    /// priming_threshold:
    ///  The number of observations required of any action before the action's
    ///  raw q-value is trusted more than average q-value for all of a state's
    ///  actions.
    ///
    /// learning_rate:
    ///  Typically a number between 0 and 1 (though it can exceed 1)
    ///  From wikipedia: Determins to what extent newly acquired information
    ///  overrides old information.
    ///  see: https://en.wikipedia.org/wiki/Q-learning#Learning_Rate
    ///
    /// discount_factor:
    ///  From wikipedia: The discount factor determines the importance of future
    ///  rewards.
    ///  see: https://en.wikipedia.org/wiki/Q-learning#Discount_factor
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
            tie_breaker: Box::new(|n: usize| -> usize { rand::thread_rng().gen_range(0, n) }),
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
        for action in state.possible_actions() {
            match self.qmap.get_stats(state, &action) {
                Some(s) => {
                    raw_value_sum += s.q_value_raw();
                    existing_action_count += 1;
                }
                None => self
                    .qmap
                    .update_stats(state, &action, Box::new(AS::default())),
            }
        }

        let mean = math::safe_divide(raw_value_sum, existing_action_count as f64);
        let action_stats = self.qmap.get_actions_for_state(state);
        for stats in action_stats.values_mut() {
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
        for stat in self.qmap.get_actions_for_state(state).values() {
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
            ..Default::default()
        };

        let current_state = MockStater {
            return_id: "B",
            return_possible_actions: mock_actions(),
            ..Default::default()
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
                    "X" => Box::new(ActionStats {call_count: 1, q_raw: 1.0, q_weighted: 0.696_969_696_969_696_9}),
                    "Y" => Box::new(ActionStats {call_count: 1, q_raw: 1.0, q_weighted: 0.696_969_696_969_696_9}),
                    "Z" => Box::new(ActionStats {call_count: 0, q_raw: 0.0, q_weighted: 0.666_666_666_666_666_6}),
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
    fn transition_happy_path() {
        let action_x = MockActioner { return_id: "X" };
        let mock_actions = vec![&action_x];

        let applied_action_id: RefCell<Option<&str>> = RefCell::new(None);
        let current_state = MockStater {
            return_id: "A",
            return_possible_actions: mock_actions,
            return_action_is_compatible: &|_| -> bool { true },
            return_apply: &|action| -> Result<(), LearnerError> {
                applied_action_id.replace(Some(action.id()));
                Ok(())
            },
            ..Default::default()
        };

        let ba: BayesianAgent<MockStater<MockActioner>, MockActioner, ActionStats> =
            BayesianAgent::new(0, 0.0, 0.0);
        let transition_result = ba.transition(&current_state, &action_x);

        assert!(transition_result.is_ok());
        assert!(applied_action_id.borrow().is_some());
        assert_eq!(action_x.id(), applied_action_id.borrow().unwrap());
    }

    #[test]
    fn transition_action_not_compatible() {
        let unknown_action = MockActioner {
            return_id: "unknown",
        };

        let known_action = MockActioner { return_id: "known" };
        let known_actions = vec![&known_action];

        let applied_action_id: RefCell<Option<&str>> = RefCell::new(None);
        let current_state = MockStater {
            return_id: "A",
            return_possible_actions: known_actions,
            return_action_is_compatible: &|_| -> bool { false },
            return_apply: &|action| -> Result<(), LearnerError> {
                applied_action_id.replace(Some(action.id()));
                Ok(())
            },
            ..Default::default()
        };

        let ba: BayesianAgent<MockStater<MockActioner>, MockActioner, ActionStats> =
            BayesianAgent::new(0, 0.0, 0.0);
        let transition_result = ba.transition(&current_state, &unknown_action);

        assert!(transition_result.is_err());
        assert_eq!(
            format!("action {} is not compatible with state {}", "unknown", "A"),
            transition_result.unwrap_err().message()
        );
        assert!(applied_action_id.borrow().is_none());
    }

    #[test]
    fn recommend_action() {
        const TEST_STATE_ID: &str = "testStateID";
        const EXP_GET_ACTION_CALLS: i64 = 1;

        struct TestCase<'a> {
            name: &'a str,
            possible_actions: Vec<&'a MockActioner<'a>>,
            tie_break_index: usize,
            exp_result: Result<&'a str, LearnerError>,
        }

        let action_a = MockActioner { return_id: "A" };
        let action_b = MockActioner { return_id: "B" };

        let test_cases = vec![
            TestCase {
                name: "Error if no actions",
                possible_actions: vec![],
                tie_break_index: 0,
                exp_result: Err(LearnerError::new(format!(
                    "state '{}' reports no possible actions",
                    TEST_STATE_ID
                ))),
            },
            TestCase {
                name: "Action returned when bootstrapping",
                possible_actions: vec![&action_a],
                tie_break_index: 0,
                exp_result: Ok("A"),
            },
            TestCase {
                name: "Action choesn when tied",
                possible_actions: vec![&action_a, &action_b],
                tie_break_index: 1,
                exp_result: Ok("B"),
            },
        ];

        for test_case in test_cases {
            let tie_breaker_index = test_case.tie_break_index;
            let state = MockStater {
                return_id: TEST_STATE_ID,
                return_possible_actions: test_case.possible_actions,
                ..Default::default()
            };

            let mut a: BayesianAgent<MockStater<MockActioner>, MockActioner, ActionStats> =
                BayesianAgent::new(0, 0.0, 0.0);
            a.tie_breaker = Box::new(|_| tie_breaker_index);
            let act_result = a.recommend_action(&state);
            let test_name = test_case.name;

            match test_case.exp_result {
                Ok(exp_action_id) => {
                    assert!(act_result.is_ok(), "test case: {}", test_name);
                    assert_eq!(
                        RefCell::new(EXP_GET_ACTION_CALLS),
                        state.get_action_calls,
                        "test case: {}",
                        test_name
                    );
                    assert_eq!(
                        exp_action_id,
                        act_result.unwrap().id(),
                        "test case: {}",
                        test_name
                    );
                }
                Err(exp_error) => {
                    assert!(act_result.is_err(), "test case: {}", test_name);
                    assert_eq!(
                        exp_error,
                        act_result.unwrap_err(),
                        "test case: j{}",
                        test_name
                    );
                }
            }
        }
    }
}
