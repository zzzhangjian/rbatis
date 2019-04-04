use crate::ast::NodeType::NodeType;
use crate::ast::Node::{SqlNode, DoChildNodes};
use serde_json::{Value, Map};
use crate::utils;
use std::collections::HashMap;

#[derive(Clone)]
pub struct ForEachNode {
    pub childs: Vec<NodeType>,
    pub collection: String,
    pub index: String,
    pub item: String,
    pub open: String,
    pub close: String,
    pub separator: String,
}

impl SqlNode for ForEachNode {
    fn eval(&mut self, env: &mut Value) -> Result<String, String> {
        let mut result = String::new();

        let collectionValue = utils::value_util::GetDeepValue(self.collection.as_str(), env);
        if collectionValue.is_null() {
            return Result::Err("[RustMybatis] collection name:".to_owned() + self.collection.as_str() + " is none value!");
        }
        if !&collectionValue.is_array() {
            return Result::Err("[RustMybatis] collection name:".to_owned() + self.collection.as_str() + " is not a array value!");
        }
        let collection = collectionValue.as_array().unwrap();

        let mut index = -1;
        for item in collection {
            index = index + 1;
            //build temp arg
            let mut objMap = serde_json::Map::new();
            objMap.insert("item".to_string(), item.clone());
            objMap.insert("index".to_string(), Value::Number(serde_json::Number::from_f64(index as f64).unwrap()));
            let mut tempArg: Value = Value::Object(objMap);
            let itemResult = DoChildNodes(&mut self.childs, &mut tempArg);
            if itemResult.is_err() {
                return itemResult;
            }
            result = result + itemResult.unwrap().as_str();
        }
        return Result::Ok(result);
    }
}