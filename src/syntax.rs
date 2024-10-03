use std::num::Wrapping;

#[derive(Debug, Clone)]
pub enum Instruction {
    Increment { value: Wrapping<i8> },
    CellIncrement { value: i32 },
    Loop { nodes: Vec<Instruction> },
    Write,
    Read,
}

pub fn parse(str: &str) -> Result<Vec<Instruction>, ()> {
    let mut nodes = Vec::new();
    let mut stack = Vec::new();

    for char in str.chars() {
        match char {
            '+' => {
                nodes.push(Instruction::Increment { value: Wrapping(1) });
            }
            '-' => {
                nodes.push(Instruction::Increment {
                    value: Wrapping(-1),
                });
            }
            '>' => {
                nodes.push(Instruction::CellIncrement { value: 1 });
            }
            '<' => {
                nodes.push(Instruction::CellIncrement { value: -1 });
            }
            '.' => {
                nodes.push(Instruction::Write);
            }
            ',' => {
                nodes.push(Instruction::Read);
            }
            '[' => {
                stack.push(nodes);
                nodes = vec![]
            }
            ']' => {
                match stack.pop() {
                    Some(mut parent) => {
                        parent.push(Instruction::Loop { nodes });
                        nodes = parent;
                    }
                    None => panic!("No matching ["),
                };
            }
            _ => {
                // Comment
            }
        }
    }

    Ok(nodes)
}
