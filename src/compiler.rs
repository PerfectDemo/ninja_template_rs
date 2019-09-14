use crate::instruction::Instruction;
use std::usize::MAX;

pub fn compile(source: &str) -> Vec<Instruction> {
    let mut current = 0;
    let mut instructions = vec![];
    let length = source.len();

    while current < length {
        let currentStr = &source[current..];
        let literal = findLiteral(currentStr).unwrap();
        println!("literal: {:?}", literal);

        instructions.push(literal.instruction);
        current += literal.index;

        let other = findOther(&source[current..]);
        match other {
            None => {
                current = length;
            },
            Some(mut Other) => {
                instructions.append(&mut Other.instructions);
                current += Other.index;
            }
        }
    }
    return instructions;
}

fn findLiteral(target: &str) -> Option<LiteralInstruction> {
    let mut beginIndex = target.find("{").unwrap_or(MAX);
    
    if beginIndex == MAX {
        beginIndex = target.len();
    }

    Some(LiteralInstruction {
        index: beginIndex,
        instruction: Instruction::Literal(&target[0..beginIndex])
    })
}

#[derive(Debug)]
struct LiteralInstruction<'a> {
    index: usize,
    instruction: Instruction<'a>
}

#[derive(Debug)]
struct Other<'a> {
    index: usize,
    instructions: Vec<Instruction<'a>>
}

fn findOther(target: &str) -> Option<Other> {
    let mut end = 0;

    println!("xine");
    if target.find("{").unwrap_or(MAX) != 0 {
        println!("none");
        return None;
    }

    let mut instructions = vec![];
    if target.find("{{").unwrap_or(MAX) == 0 { // value or formated value
        let endValueSymbol = "}}";
        let endIndex = target.find(endValueSymbol).unwrap();
            println!("endIndex: {}", endIndex);

        if endIndex != 0 {
            let index = target.find('|').unwrap_or(0);
            if index != 0 {
                let result: Vec<&str> = target[2..endIndex - 2].split('|').collect();
                let value = result[0].trim();
                let formatter = result[1].trim();
                instructions.push(Instruction::FormattedValue(&value, &formatter));
            } else {
                let value = target[2..endIndex - 2].trim();
                instructions.push(Instruction::Value(&value));
            }
            end = endIndex + 2;
        }
    } else {
        let forIndex = target.find("for").unwrap_or(MAX);
        let ifIndex = target.find("if").unwrap_or(MAX);

        if forIndex == ifIndex {
            end = target.len();
        }

        if forIndex < ifIndex { // for parser
            println!("for: {}", forIndex);
            let endForTag = target.find("}").unwrap();
            let splitList: Vec<&str> = target[0..endForTag].split("of").collect();
            let list = splitList[1].trim();

            println!("target: {}", target);
            let itemSplit: Vec<&str> = target[forIndex + 3..endForTag].trim().split(",").collect();
            println!("split:{:?}", itemSplit);
            let item = itemSplit[0].trim();
            let children = vec![];

            instructions.push(Instruction::Iterate(item, list, children));
            
            let endForClose = target.find("endfor").unwrap();
            end = target[endForClose..].find("}").unwrap() + endForClose + 1;
        } else { // if parser
            println!("if");
            println!("ifIndex: {}", ifIndex);
            let endIfTag = target.find("}").unwrap();
            let branch = target[ifIndex + 2..endIfTag - 1].trim();
            let mut ifChilren: Vec<Instruction> = vec![];
            let mut elseChildren: Vec<Instruction> = vec![];

            instructions.push(Instruction::Branch(branch, ifChilren, elseChildren));
            let endIfClose = target.find("endif").unwrap();
            end = target[endIfClose..].find("}").unwrap() + endIfClose + 1;
        }
    }

    Some(Other {
        index: end,
        instructions: instructions
    })
    
}

#[cfg(test)]
mod test {
    use crate::compiler::*;

    #[test]
    fn testCompile() {
        let target = "{ if ninja.hello } ninja { endif }77777 {{ nija | 3r234 }} 3742482374289 { for item, index of list } ninja { endfor } 2342";
        let instructions = compile(target);
        println!("[INSTRUTIONS]: {:?}", instructions);
        assert!(instructions.len() > 0);
    }

    #[test]
    fn testOther() {
        let target = "{{ ninja | hello }}";
        let other = findOther(&target).unwrap();
        println!("{:?}", other);
        assert!(other.instructions.len() > 0);
    }

    #[test]
    fn testFor() {
        let target = "{ for item, index of list } ninja { endfor }";
        let other = findOther(&target).unwrap();
        println!("{:?}", other);
        assert!(other.instructions.len() > 0);
    }

    #[test] 
    fn testBranch() {
        let target = "{ if ninja.hello } ninja { endif }";
        let other = findOther(&target).unwrap();
        println!("{:?}", other);
        assert!(other.instructions.len() > 0);
    }
}

