use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, JoinHandle};

pub struct Intcode {
    ip: usize,
    pub ops: Vec<isize>,
    input: Receiver<isize>,
    output: Sender<isize>,
}

impl Intcode {
    pub fn new(ops: &[isize]) -> (Self, Sender<isize>, Receiver<isize>) {
        let (input_tx, input_rx) = mpsc::channel();
        let (output_tx, output_rx) = mpsc::channel();

        (
            Intcode {
                ip: 0,
                ops: ops.to_vec(),
                input: input_rx,
                output: output_tx,
            },
            input_tx,
            output_rx,
        )
    }

    pub fn new_with_io(ops: &[isize], input: Receiver<isize>, output: Sender<isize>) -> Self {
        Intcode {
            ip: 0,
            ops: ops.to_vec(),
            input,
            output,
        }
    }

    pub fn run(mut self) -> JoinHandle<Self> {
        thread::spawn(move || {
            while self.step() {}
            self
        })
    }

    fn step(&mut self) -> bool {
        let op = self.ops[self.ip];
        let arg1_mode = ((op % 1000) / 100) != 0;
        let arg2_mode = ((op % 10000) / 1000) != 0;
        let _arg3_mode = ((op % 100000) / 10000) != 0;
        let op = op % 100;

        match op {
            1 | 2 | 5 | 6 | 7 | 8 => {
                let arg1 = if arg1_mode {
                    self.ops[self.ip + 1]
                } else {
                    self.ops[self.ops[self.ip + 1] as usize]
                };

                let arg2 = if arg2_mode {
                    self.ops[self.ip + 2]
                } else {
                    self.ops[self.ops[self.ip + 2] as usize]
                };

                match op {
                    5 | 6 => {
                        self.ip = if match op {
                            5 => arg1 != 0,
                            6 => arg1 == 0,
                            _ => unreachable!(),
                        } {
                            arg2 as usize
                        } else {
                            self.ip + 3
                        };

                        true
                    }
                    1 | 2 | 7 | 8 => {
                        let reg = self.ops[self.ip + 3] as usize;

                        self.ops[reg] = match op {
                            1 => arg1 + arg2,
                            2 => arg1 * arg2,
                            7 => (arg1 < arg2) as isize,
                            8 => (arg1 == arg2) as isize,
                            _ => unreachable!(),
                        };

                        self.ip += 4;

                        true
                    }
                    _ => unreachable!(),
                }
            }
            3 => {
                let reg = self.ops[self.ip + 1] as usize;
                self.ops[reg] = self.input.recv().unwrap();
                // println!("recv {}", self.ops[reg]);
                self.ip += 2;
                true
            }
            4 => {
                let reg = self.ops[self.ip + 1] as usize;
                // println!("send {}", self.ops[reg]);
                self.output.send(self.ops[reg]).unwrap();
                self.ip += 2;
                true
            }
            99 => false,
            _ => panic!("invalid opcode: {}", self.ops[self.ip]),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_intcode() {
        let (code, input, output) = super::Intcode::new(&vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);

        code.run();

        input.send(9).unwrap();
        let result = output.recv().unwrap();

        println!("{:?}", result);
    }
}
