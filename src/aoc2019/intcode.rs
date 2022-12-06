pub fn exec(
    ops: &mut [isize],
    ip: usize,
    inp: &mut impl Iterator<Item = isize>,
    out: &mut Vec<isize>,
) -> Option<usize> {
    let arg1_mode = ((ops[ip] % 1000) / 100) != 0;
    let arg2_mode = ((ops[ip] % 10000) / 1000) != 0;
    let arg3_mode = ((ops[ip] % 100000) / 10000) != 0;

    match ops[ip] % 100 {
        1 => {
            let arg1 = if arg1_mode {
                ops[ip + 1]
            } else {
                ops[ops[ip + 1] as usize]
            };
            let arg2 = if arg2_mode {
                ops[ip + 2]
            } else {
                ops[ops[ip + 2] as usize]
            };
            ops[ops[ip + 3] as usize] = arg1 + arg2;
            Some(ip + 4)
        }
        2 => {
            let arg1 = if arg1_mode {
                ops[ip + 1]
            } else {
                ops[ops[ip + 1] as usize]
            };
            let arg2 = if arg2_mode {
                ops[ip + 2]
            } else {
                ops[ops[ip + 2] as usize]
            };
            ops[ops[ip + 3] as usize] = arg1 * arg2;
            Some(ip + 4)
        }
        3 => {
            ops[ops[ip + 1] as usize] = inp.next().unwrap();
            Some(ip + 2)
        }
        4 => {
            out.push(ops[ops[ip + 1] as usize]);
            Some(ip + 2)
        }
        99 => None,
        _ => panic!("invalid opcode"),
    }
}
