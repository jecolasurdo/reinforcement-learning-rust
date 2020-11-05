use crate::errors::LearnerError;
use crate::iface::{ActionStatter, Actioner, Agenter, Stater};
use crate::internal::datastructures::QMap;
use rand::Rng;
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

pub fn new<S, A, AS>(
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

impl<S, A: 'static, AS> Agenter<S, A> for BayesianAgent<S, A, AS>
where
    S: Stater<A>,
    A: Actioner,
    AS: ActionStatter,
{
    fn recommend_action(&self, stater: S) -> Result<A, LearnerError> {
        unimplemented!();
    }

    fn transition(&self, stater: S, actioner: A) -> Result<(), LearnerError> {
        unimplemented!();
    }

    fn learn(&mut self, previous_state: S, action_taken: A, current_state: S, reward: f64) {
        unimplemented!();
    }
}
