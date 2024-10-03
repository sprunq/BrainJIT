use super::{MapLoopsExt, OptimizationPass};
use crate::syntax::Instruction;
use itertools::Itertools;

use crate::syntax::Instruction::*;

pub struct CombineIncrements;

impl OptimizationPass for CombineIncrements {
    fn optimize(&self, nodes: Vec<Instruction>) -> Vec<Instruction> {
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

pub struct ReplaceSet;

impl OptimizationPass for ReplaceSet {
    fn optimize(&self, nodes: Vec<Instruction>) -> Vec<Instruction> {
        nodes
            .into_iter()
            .map(|instr| {
                if let Loop { ref nodes } = instr {
                    if nodes.len() == 1 {
                        let inner = &nodes[0];
                        if let Increment { value } = inner {
                            let value = value.0;
                            if value == -1 || value == 1 {
                                return Set { value: 0 };
                            }
                        }
                    }
                }
                instr
            })
            .map_loops(Self)
            .collect()
    }
}
