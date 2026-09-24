use crate::parser::Rule;
use pest::iterators::{Pair, Pairs};
use std::{fmt::Debug, str::FromStr};

pub fn consume_one<'a>(parts: &mut Pairs<'a, Rule>) -> Pair<'a, Rule> {
    parts.next().unwrap()
}

pub fn consume_two<'a>(parts: &mut Pairs<'a, Rule>) -> (Pair<'a, Rule>, Pair<'a, Rule>) {
    (parts.next().unwrap(), parts.next().unwrap())
}

pub fn unpack_one(pair: Pair<'_, Rule>) -> Pair<'_, Rule> {
    let mut parts = pair.into_inner();
    assert!(parts.len() == 1);

    parts.next().unwrap()
}

///
/// parse Pair's str representation as integer or float number
///
pub fn num_parse<T: FromStr>(value: Pair<'_, Rule>) -> T
where
    T: FromStr,
    T::Err: Debug,
{
    value.as_str().parse().unwrap()
}

// pub fn unpack_two(pair: Pair<'_, Rule>) -> (Pair<'_, Rule>, Pair<'_, Rule>) {
//     let mut parts = pair.into_inner();
//     assert!(parts.len() == 2);

//     (parts.next().unwrap(), parts.next().unwrap())
// }

// pub fn unpack_two_strs(pair: Pair<'_, Rule>) -> (&str, &str) {
//     let (first, second) = unpack_two(pair);

//     (first.as_str(), second.as_str())
// }
