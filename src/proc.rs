use crate::{
    layer::{Attribute, AttributeType, AttributeValue, Prim},
    parser::Rule,
    utils::{consume_one, consume_two, num_parse, unpack_one},
};
use pest::iterators::{Pair, Pairs};

// // for debugging
// fn print_inner(node: &Pair<'_, Rule>) {
//     println!("-------");
//     for pair in node.clone().into_inner() {
//         println!(
//             "{:?} {:?} '{} '",
//             node.as_rule(),
//             pair.as_rule(),
//             pair.as_str()
//         );
//     }
//     println!("-------");
// }

fn singleline_double_quote_string(node: Pair<'_, Rule>) -> String {
    assert!(node.as_rule() == Rule::singleline_double_quote_string);

    let text = node.as_str();
    String::from(&text[1..text.len() - 1])
}

//
// rule processors
//

fn double_quote_string(node: Pair<'_, Rule>) -> String {
    assert!(node.as_rule() == Rule::double_quote_string);

    let value = unpack_one(node);
    match value.as_rule() {
        Rule::singleline_double_quote_string => singleline_double_quote_string(value),
        _ => unreachable!(),
    }
}

fn string(node: Pair<'_, Rule>) -> String {
    assert!(node.as_rule() == Rule::string);

    let value = unpack_one(node);
    match value.as_rule() {
        Rule::double_quote_string => double_quote_string(value),
        _ => unreachable!(),
    }
}

fn atomic_value(val_type: &AttributeType, node: Pair<'_, Rule>) -> AttributeValue {
    assert!(node.as_rule() == Rule::atomic_value);

    let value = unpack_one(node);

    match value.as_rule() {
        Rule::number => match val_type {
            AttributeType::Float => AttributeValue::Float(num_parse(value)),
            AttributeType::Int => AttributeValue::Int(num_parse(value)),
            _ => panic!("unexpected number type {val_type:?}"),
        },
        Rule::identifier => todo!(),
        Rule::string => {
            let val = string(value);
            match val_type {
                AttributeType::Token => AttributeValue::Token(val),
                AttributeType::String => AttributeValue::String(val),
                _ => panic!("unexpected string type {val_type:?}"),
            }
        }
        _ => unreachable!(),
    }
}

fn tuple_item(val_type: &AttributeType, node: Pair<'_, Rule>) -> AttributeValue {
    assert!(node.as_rule() == Rule::tuple_item);

    let value = unpack_one(node);

    match value.as_rule() {
        Rule::atomic_value => atomic_value(val_type, value),
        Rule::tuple_value => tuple_value(val_type, value),
        _ => unreachable!(),
    }
}

fn list_tuple_item(val_type: &AttributeType, node: Pair<'_, Rule>) -> Vec<AttributeValue> {
    assert!(node.as_rule() == Rule::list_tuple_item);

    let base_type = val_type.base_type();

    node.into_inner()
        .map(|pair| tuple_item(base_type, pair))
        .collect()
}

fn tuple_value(val_type: &AttributeType, node: Pair<'_, Rule>) -> AttributeValue {
    assert!(node.as_rule() == Rule::tuple_value);

    let items = list_tuple_item(val_type, unpack_one(node));

    match val_type {
        AttributeType::Float3 => {
            assert!(items.len() == 3);
            AttributeValue::Float3(
                (&items[0]).try_into().unwrap(),
                (&items[1]).try_into().unwrap(),
                (&items[2]).try_into().unwrap(),
            )
        }
        AttributeType::Point3f => {
            assert!(items.len() == 3);
            AttributeValue::Point3f(
                (&items[0]).try_into().unwrap(),
                (&items[1]).try_into().unwrap(),
                (&items[2]).try_into().unwrap(),
            )
        }
        _ => panic!("unexpected tuple type {val_type:?}"),
    }
}

fn list_item(val_type: &AttributeType, node: Pair<'_, Rule>) -> AttributeValue {
    assert!(node.as_rule() == Rule::list_item);

    let base_type = val_type.base_type();
    let value = unpack_one(node);

    match value.as_rule() {
        Rule::atomic_value => atomic_value(base_type, value),
        Rule::list_value => list_value(base_type, value),
        Rule::tuple_value => tuple_value(base_type, value),
        _ => unreachable!(),
    }
}

fn list_list_item(val_type: &AttributeType, node: Pair<'_, Rule>) -> Vec<AttributeValue> {
    assert!(node.as_rule() == Rule::list_list_item);

    node.into_inner()
        .map(|pair| list_item(&val_type, pair))
        .collect()
}

fn list_value(val_type: &AttributeType, node: Pair<'_, Rule>) -> AttributeValue {
    assert!(node.as_rule() == Rule::list_value);

    let items = list_list_item(val_type, unpack_one(node));
    AttributeValue::new_array(items)
}

fn typed_value(val_type: AttributeType, node: Pair<'_, Rule>) -> AttributeValue {
    assert!(node.as_rule() == Rule::typed_value);

    let value = unpack_one(node);

    match value.as_rule() {
        Rule::atomic_value => atomic_value(&val_type, value),
        Rule::list_value => list_value(&val_type, value),
        Rule::array_type => todo!(),
        _ => unreachable!(),
    }
}

fn attribute_type(node: Pair<'_, Rule>) -> AttributeType {
    assert!(node.as_rule() == Rule::attribute_type);

    let mut parts = node.into_inner();
    let mut part = consume_one(&mut parts);

    if part.as_rule() == Rule::attribute_variability {
        // attribute variability is ignored for now
        part = consume_one(&mut parts);
    }

    let base_type = match part.as_str() {
        "token" => AttributeType::Token,
        "int" => AttributeType::Int,
        "float3" => AttributeType::Float3,
        "point3f" => AttributeType::Point3f,
        _ => panic!("unexpected attribute type '{}'", part.as_str()),
    };

    if let Some(array_type) = parts.next() {
        assert!(array_type.as_rule() == Rule::array_type);

        return AttributeType::new_array(base_type);
    }

    base_type
}

/// # Returns
///
/// tuple of attributes type and name, as &str
fn attribute_declaration(node: Pair<'_, Rule>) -> (AttributeType, &str) {
    assert!(node.as_rule() == Rule::attribute_declaration);

    let mut parts = node.into_inner();

    let attr_type = attribute_type(consume_one(&mut parts));
    let attr_name = consume_one(&mut parts).as_str();

    (attr_type, attr_name)
}

fn attribute_assignment(attr_type: AttributeType, name: &str, node: Pair<'_, Rule>) -> Attribute {
    assert!(node.as_rule() == Rule::attribute_assignment);

    let value = typed_value(attr_type, unpack_one(node));
    Attribute::new(name, value)
}

fn attribute_spec(node: Pair<'_, Rule>) -> Attribute {
    assert!(node.as_rule() == Rule::attribute_spec);

    let (declaration, assignment) = consume_two(&mut node.into_inner());
    let (attr_type, attr_name) = attribute_declaration(declaration);

    attribute_assignment(attr_type, attr_name, assignment)
}

fn property_spec(node: Pair<'_, Rule>) -> Attribute {
    assert!(node.as_rule() == Rule::property_spec);

    attribute_spec(consume_one(&mut node.into_inner()))
}

fn prim_item(node: Pair<'_, Rule>, child_prims: &mut Vec<Prim>, attributes: &mut Vec<Attribute>) {
    assert!(node.as_rule() == Rule::prim_item);

    let inner = consume_one(&mut node.into_inner());
    match inner.as_rule() {
        Rule::property_spec => attributes.push(property_spec(inner)),
        Rule::prim_spec => child_prims.push(prim_spec(inner)),
        _ => unreachable!(),
    }
}

fn prim_contents(node: Pair<'_, Rule>) -> (Vec<Attribute>, Vec<Prim>) {
    assert!(node.as_rule() == Rule::prim_contents);

    let mut attributes = vec![];
    let mut child_prims = vec![];

    for pair in node.into_inner() {
        prim_item(pair, &mut child_prims, &mut attributes);
    }

    (attributes, child_prims)
}

fn prim_specifier(node: Pair<'_, Rule>) {
    assert!(node.as_rule() == Rule::prim_specifier);
    // only 'def' prim specifier supported
    assert!(node.as_str() == "def");
}

fn prim_spec(node: Pair<'_, Rule>) -> Prim {
    fn get_prim_type_and_name<'a>(mut parts: &mut Pairs<'a, Rule>) -> (Option<&'a str>, &'a str) {
        let mut type_name = None;
        let mut next = consume_one(&mut parts);

        // prim_type_name is optional
        if next.as_rule() == Rule::prim_type_name {
            type_name = Some(next.as_str());
            next = consume_one(&mut parts);
        }

        let prim_name = next.as_str();

        (type_name, prim_name)
    }

    assert!(node.as_rule() == Rule::prim_spec);

    let mut parts = node.into_inner();

    prim_specifier(consume_one(&mut parts));
    let (type_name, name) = get_prim_type_and_name(&mut parts);

    let mut next = consume_one(&mut parts);
    if next.as_rule() == Rule::prim_metadata {
        // prim metadata is ignored
        next = consume_one(&mut parts);
    }

    let (attributes, children) = prim_contents(next);

    Prim::new(name, type_name, attributes, children)
}

fn layer_spec(root: Pair<'_, Rule>) -> Vec<Prim> {
    assert!(root.as_rule() == Rule::layer_spec);

    let mut prims = vec![];

    for pair in root.into_inner() {
        match pair.as_rule() {
            Rule::layer_header | Rule::layer_metadata => { /* ignored */ }
            Rule::layer_item => {
                // current grammer only have 'prime_spec' option for 'layer_item'
                let prim = prim_spec(pair.into_inner().next().unwrap());
                prims.push(prim);
            }
            _ => unreachable!(),
        }
    }

    prims
}

pub fn process(pairs: Pairs<'_, Rule>) -> Vec<Prim> {
    layer_spec(
        pairs
            .into_iter()
            .next()
            .unwrap()
            .into_inner()
            .next()
            .unwrap(),
    )
}
