use itertools::Itertools;

use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Instruction {
    fn new(code: u8) -> Self {
        use Instruction::*;
        match code {
            0 => Adv,
            1 => Bxl,
            2 => Bst,
            3 => Jnz,
            4 => Bxc,
            5 => Out,
            6 => Bdv,
            7 => Cdv,
            _ => panic!("Invalid Code"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Operand {
    value: u8,
}

impl Operand {
    fn new(v: u8) -> Self {
        Self { value: v }
    }

    fn literal(&self) -> u64 {
        self.value as u64
    }

    fn combo(&self, registers: &[u64]) -> u64 {
        if self.value < 4 {
            self.value as u64
        } else {
            registers[(self.value - 4) as usize]
        }
    }
}

pub struct Day17;
impl Solution for Day17 {
    type Input<'a> = ([u64; 3], Vec<u8>);

    fn parse<'a>(content: &'a str) -> Self::Input<'a> {
        let re = regex::Regex::new(r"Register A: ([0-9]+)\nRegister B: ([0-9]+)\nRegister C: ([0-9]+)\n\nProgram: ([0-9,]+)").unwrap();
        let (_, [a, b, c, instr]) = re.captures(content).unwrap().extract();
        let a = a.parse::<u64>().unwrap();
        let b = b.parse::<u64>().unwrap();
        let c = c.parse::<u64>().unwrap();
        let instrs = instr
            .split(',')
            .map(|x| x.parse::<u8>().unwrap())
            .collect_vec();
        ([a, b, c], instrs)
    }

    fn part_a<'a>(input: &Self::Input<'a>) -> String {
        let registers = input.0.clone();
        let instrs = get_instructions(&input.1);

        let output = run_program(registers, &instrs).collect_vec();

        format!("{}", output.iter().map(|x| format!("{x}")).join(","))
    }

    fn part_b<'a>(input: &Self::Input<'a>) -> String {
        format!("{:?}", reverse(&input.1).iter().min().unwrap())
    }
}

fn reverse(out: &[u8]) -> Vec<u64> {
    if out.len() == 1 {
        let mut res = vec![];
        for i in 0u64..8u64 {
            if (i >> (i ^ 1)) ^ 6 ^ (i ^ 1) == out[0] as u64 {
                res.push(i)
            }
        }
        return res;
    }

    let prev = reverse(&out[1..]).iter().map(|&x| x << 3).collect_vec();

    let mut res = vec![];
    for p in prev {
        for i in 0u64..8u64 {
            if (((p | i) >> (i ^ 1)) ^ 6 ^ (i ^ 1)) & 7 == out[0] as u64 {
                res.push(p | i)
            }
        }
    }
    return res;
}

fn run_program<'a>(
    mut registers: [u64; 3],
    instrs: &'a [(Instruction, Operand)],
) -> impl Iterator<Item = u8> + 'a {
    let mut iptr = 0;
    let step = move || {
        while iptr < instrs.len() {
            let (niptr, out) = exec(instrs[iptr].0, instrs[iptr].1, &mut registers, iptr);
            iptr = niptr;
            if let Some(out) = out {
                return Some(out);
            }
        }
        return None;
    };
    std::iter::from_fn(step)
}

fn get_instructions(raw: &[u8]) -> Vec<(Instruction, Operand)> {
    raw.iter()
        .tuple_windows()
        .map(|(instr, op)| (Instruction::new(*instr), Operand::new(*op)))
        .collect_vec()
}

#[inline(always)]
fn exec(
    instr: Instruction,
    op: Operand,
    registers: &mut [u64; 3],
    iptr: usize,
) -> (usize, Option<u8>) {
    match instr {
        Instruction::Adv => registers[0] = registers[0] >> op.combo(registers),
        Instruction::Bxl => registers[1] = registers[1] ^ (op.literal() as u64),
        Instruction::Bst => registers[1] = op.combo(registers) & 0x7,
        Instruction::Jnz => {
            if registers[0] != 0 {
                return (op.literal() as usize, None);
            }
        }
        Instruction::Bxc => registers[1] = registers[1] ^ registers[2],
        Instruction::Out => return (iptr + 2, Some((op.combo(registers) % 8) as u8)),
        Instruction::Bdv => registers[1] = registers[0] >> op.combo(registers),
        Instruction::Cdv => registers[2] = registers[0] >> op.combo(registers),
    }
    return (iptr + 2, None);
}

gen_test!(
    a,
    Day17,
    r"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0",
    "4,6,3,5,6,3,5,2,1,0"
);
