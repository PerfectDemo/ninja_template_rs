#[derive(Debug)]
pub enum Instruction<'template> {
    Value(&'template str),
    Literal(&'template str),
    FormattedValue(&'template str, &'template str),
    Branch(&'template str, Vec<Instruction<'template>>, Vec<Instruction<'template>>),
    Iterate(&'template str, &'template str, Vec<Instruction<'template>>)
// 
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn testValue() {
        let c = Instruction::Value("2132432");
        match c {
            Instruction::Value(value) => {
                assert_eq!(value, "2132432");
            },
            _ => {
                println!("none thing");
            }
        }
    }

    #[test]
    fn testLiteral() {
        let c = Instruction::Literal("literal");
        match c {
            Instruction::Literal(value) => {
                assert_eq!(value, "literal");
            },
            _ => {
                println!("none thing");
            }
        }
    }

    #[test]
    fn testFormatValue() {
        let a = Instruction::FormattedValue("value", "method");
        match a {
            Instruction::FormattedValue(value, method) => {
                assert_eq!(value, "value");
                assert_eq!(method, "method");
            },
            _ => {
                println!("none thing");
            }
        }
    }

    #[test]
    fn testBranch() {
        let branch = Instruction::Branch("branch", vec![Instruction::Literal("literal")], vec![Instruction::Literal("literal")]);
        match branch {
            Instruction::Branch(value, trueChildren, falseChildren) => {
                assert_eq!(value, "branch");
                assert_eq!(trueChildren.len(), 1);
                assert_eq!(falseChildren.len(), 1);
            },
            _ => {
                println!("none thing");
            }
        }
    }

    #[test]
    fn testIterate() {
        let iterate = Instruction::Iterate("iterate", "ninja", vec![Instruction::Value("value"), Instruction::Literal("literal")]);
        match iterate {
            Instruction::Iterate(iterate, item, children) => {
                assert_eq!(iterate, "iterate");
                assert_eq!(item, "ninja");
                assert_eq!(children.len(), 2);
            },
            _ => {
                println!("none thing");
            }
        }
    }
}