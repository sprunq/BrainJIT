pub mod peephole;

use crate::syntax::Instruction;

pub trait OptimizationPass {
    fn optimize(&self, nodes: Vec<Instruction>) -> Vec<Instruction>;
}

trait MapLoopsExt: Iterator<Item = Instruction> {
    fn map_loops<O>(self, optimizer: O) -> MapLoops<Self, O>
    where
        O: OptimizationPass,
        Self: Sized,
    {
        MapLoops {
            iter: self,
            optimizer,
        }
    }
}

struct MapLoops<I, O> {
    iter: I,
    optimizer: O,
}

impl<I, O> Iterator for MapLoops<I, O>
where
    I: Iterator<Item = Instruction>,
    O: OptimizationPass,
{
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|instr| match instr {
            Instruction::Loop { nodes } => {
                let mapped_nodes = self.optimizer.optimize(nodes);
                Instruction::Loop {
                    nodes: mapped_nodes,
                }
            }
            other => other,
        })
    }
}

impl<I> MapLoopsExt for I where I: Iterator<Item = Instruction> {}
