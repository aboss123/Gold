use crate::mir::expressions::{FunctionSig, Expression, ExpressionLevelEnvironment};
use crate::mir::types::Type;
use crate::mir::expressions::literal::Object;
use std::collections::HashMap;

pub struct CallExpression {
    pub sig: FunctionSig,
    pub args: HashMap<String, Box<dyn Expression>>,
}

impl Expression for CallExpression {
    fn get_type(&self, _: &ExpressionLevelEnvironment) -> Type {
        self.sig.return_type.clone()
    }

    fn eval(&self, environment: &ExpressionLevelEnvironment) -> Object {
        let eval_args: HashMap<String, Object> = self.args.iter()
            .map(|(s, x)| (s.to_owned(), x.eval(environment)))
            .collect();

        let new_environment = ExpressionLevelEnvironment {
            scope: eval_args,
            function_registry: environment.function_registry.clone()
        };

        let map = environment.function_registry.borrow();

        let funky_boi = map.get(&self.sig).unwrap();

        funky_boi.eval(&new_environment)
    }
}