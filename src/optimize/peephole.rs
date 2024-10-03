use super::{MapLoopsExt, OptimizationPass};
use crate::syntax::Instruction;
use itertools::Itertools;

pub struct CombineIncrements;

impl OptimizationPass for CombineIncrements {
    fn optimize(&self, nodes: Vec<Instruction>) -> Vec<Instruction> {
        use crate::syntax::Instruction::*;
        nodes
            .into_iter()
            .coalesce(|prev, current| match (prev, current) {
                (Increment { value: a }, Increment { value: b }) => Ok(Increment { value: a + b }),
                (CellIncrement { value: a }, CellIncrement { value: b }) => {
                    Ok(CellIncrement { value: a + b })
                }
                (a, b) => Err((a, b)),
            })
            .map_loops(Self)
            .collect()
    }
}
