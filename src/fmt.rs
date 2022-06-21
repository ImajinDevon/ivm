use crate::compile;

pub fn format_instruction_set(bytecode: &[u8]) -> String {
    let mut string = String::new();
    let mut pos = 0;

    while pos < bytecode.len() {
        let byte = bytecode[pos];
        let instruc_name = compile::b_instruc::get_instruction_name(byte);

        string += &format!(
            "  b{:02x} {instruc_name}{}",
            byte,
            " ".repeat(17 - instruc_name.len())
        );
        pos += 1;

        let bytes = match byte {
            compile::b_instruc::INVOKE
            | compile::b_instruc::STATIC_VAR_LOAD
            | compile::b_instruc::STATIC_VAR_STORE => {
                let bytes = &bytecode[pos..][..compile::b_instruc::PTR_LEN];
                pos += compile::b_instruc::PTR_LEN;

                bytes.to_vec()
            }

            compile::b_instruc::PUSH_BYTES => {
                let len = u32::from_be_bytes(
                    bytecode[pos..][..compile::b_instruc::PTR_LEN]
                        .try_into()
                        .unwrap(),
                );
                pos += compile::b_instruc::PTR_LEN;

                let bytes = bytecode[pos..pos + len as usize].to_vec();
                pos += len as usize;

                bytes.to_vec()
            }
            _ => unreachable!("compile::b_instruc::get_instruction_name() would fail before this"),
        };
        string += &format!(
            "{:?};\n",
            bytes
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<Vec<_>>()
        );
    }
    string
}
