use pest::Parser;
use pest::iterators::Pairs;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "usda.pest"]
struct UsdaParser;

pub fn parse(input: &str) -> Pairs<'_, Rule> {
    UsdaParser::parse(Rule::layer_spec, input).unwrap()
}

pub fn dbg_parse(input: &str, rule: Rule) -> Pairs<'_, Rule> {
    UsdaParser::parse(rule, input).unwrap()
}
