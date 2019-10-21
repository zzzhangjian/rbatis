use crate::ast::NodeType::NodeType;
use crate::ast::Node::SqlNode;
use serde_json::Value;
use crate::ast::NodeConfigHolder::NodeConfigHolder;

#[derive(Clone)]
pub struct UpdateTempleteNode {
    pub id:String,
    pub childs: Vec<NodeType>,
}


impl SqlNode for UpdateTempleteNode{
    fn eval(&mut self, env: &mut Value,holder:&mut NodeConfigHolder) -> Result<String, String> {
        unimplemented!()
    }

    fn print(&self) -> String {
        let mut result="<updateTemplete ".to_string();
        result=result+"id=\""+self.id.as_str()+"\"";
        result=result+">";
        for x in &self.childs {
            result=result+x.print().as_str();
        }
        result+=" </updateTemplete>";
        return result;
    }
}