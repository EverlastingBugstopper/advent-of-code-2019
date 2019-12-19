use super::operation::Operation;
use super::parameter::Parameter;

#[derive(Debug)]
pub struct Instruction {
    pub operation: Operation,
    pub parameters: Vec<Parameter>,
}

impl Instruction {
    pub fn new(instruction_pointer: usize, program: &mut [i32]) -> Instruction {
        let code = format!("{:0>5}", program[instruction_pointer].to_string());
        let parameter_modes: String = code[0..3].to_string().chars().rev().collect();
        let opcode = code[3..5].to_string();
        let operation = Operation::from(opcode);
        let mut parameters: Vec<Parameter> = Vec::new();
        let parse_error = "Could not parse parameter mode";
        let expected_params = operation.num_params();
        for i in 0..expected_params {
            let parameter_mode = parameter_modes.chars().nth(i);
            let parameter_mode: i32 = parameter_mode
                .unwrap_or_else(|| unreachable!(parse_error))
                .to_string()
                .parse()
                .unwrap_or_else(|_| unreachable!(parse_error));
            parameters.push(Parameter::new(
                parameter_mode,
                program[instruction_pointer + 1 + i],
            ));
        }
        let num_params = parameters.len();
        if num_params != expected_params {
            panic!(
                "operation {} expected {} parameters, got {}",
                operation, expected_params, num_params
            );
        }
        Instruction {
            operation,
            parameters,
        }
    }
}
