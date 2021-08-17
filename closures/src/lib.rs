use std::collections::HashMap;
use std::hash::Hash;

pub struct Cacher<T>
where T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where T: Fn(u32) -> u32,
{
    pub fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    pub fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
// Generic cacher that allows for caching multiple input values
// TIn is used as the type for the key: must implement Eq + Hash + Copy
pub struct CacheMap<TFun, TIn, TOut>
where
    TFun: Fn(TIn) -> TOut,
    TIn: Eq + Hash + Copy,
{
    calculation: TFun,
    values: HashMap<TIn, TOut>,
}

impl<TFun, TIn, TOut> CacheMap<TFun, TIn, TOut>
where
    TFun: Fn(TIn) -> TOut,
    TIn: Eq + Hash + Copy,
{
    pub fn new(calculation: TFun) -> CacheMap<TFun, TIn, TOut> {
        CacheMap{
            calculation,
            values: HashMap::new(),
        }
    }
    pub fn value(&mut self, arg: TIn) -> &TOut {
        // return the value, calculating it first if not already present
        // store reference to closure in separate variable, because it would otherwise trigger a borrow of self
        // which overlaps with the mutable borrow of self.values triggered by the call to self.values.entry().
        // If you explicitly borrow only self.calculation in the outer scope,
        // the borrow checker is smart enough to figure out that it does not overlap with self.values
        // see: https://stackoverflow.com/questions/60109843/entryor-insert-executes-despite-a-value-already-existing
        let calculation = &self.calculation;

        self.values.entry(arg).or_insert_with(|| calculation(arg))
    }
}

#[test]
fn call_with_different_values() {
    let mut c = CacheMap::new(|a| a);
    let v1 = c.value(1);
    let v2 = *c.value(2); // use the dereference operator to follow the reference to the value it’s pointing to

    assert_eq!(v2, 2);
}