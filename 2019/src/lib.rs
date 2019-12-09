use std::sync::mpsc::{self, SyncSender, Receiver};
use std::thread::{self, JoinHandle};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Parameter {
    Position(usize),
    Value(isize),
}
impl Parameter {
    fn value(self, mem: &[isize]) -> isize {
        match self {
            Parameter::Position(idx) => mem[idx],
            Parameter::Value(v) => v
        }
    }
    // fn as_ref<'a>(&'a self, mem: &'a [isize]) -> &'a isize {
    //     match self {
    //         Parameter::Position(idx) => &mem[*idx],
    //         Parameter::Value(v) => v
    //     }
    // }
    fn as_mut<'a>(&'a mut self, mem: &'a mut [isize]) -> &'a mut isize {
        match self {
            Parameter::Position(idx) => &mut mem[*idx],
            Parameter::Value(v) => v
        }
    }
}
#[derive(Copy, Clone, Debug)]
pub enum Instruction {
    Add(Parameter, Parameter, Parameter),
    Mlt(Parameter, Parameter, Parameter),
    Inp(Parameter),
    Otp(Parameter),
    Jit(Parameter, Parameter),
    Jif(Parameter, Parameter),
    Lth(Parameter, Parameter, Parameter),
    Equ(Parameter, Parameter, Parameter),
    Hlt,
}
impl Instruction {
    pub fn len(&self) -> usize {
        use Instruction::*;
        match self {
            Add(..) => 4,
            Mlt(..) => 4,
            Inp(..) => 2,
            Otp(..) => 2,
            Jit(..) => 3,
            Jif(..) => 3,
            Lth(..) => 4,
            Equ(..) => 4,
            Hlt     => 1,
        }
    }
}
pub struct Cpu {
    mem: Vec<isize>,
    input: Receiver<isize>,
    output: SyncSender<isize>,
    iptr: usize,
    hlt: bool,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    InputBroken,
    OutputBroken,
    InstructionOverflow,
    InvalidInstruction(isize),
    InvalidParameterMode(usize),
    _non_exhaustive,
}

impl Cpu {
    pub fn new(mem: Vec<isize>) -> (Self, SyncSender<isize>, Receiver<isize>) {
        let (i_tx, i_rx) = mpsc::sync_channel(1024);
        let (o_tx, o_rx) = mpsc::sync_channel(1024);
        let ret = Self {
            mem,
            input: i_rx,
            output: o_tx,
            iptr: 0,
            hlt: true
        };
        (ret, i_tx, o_rx)
    }
    pub fn from_input(input: &str) -> (Self, SyncSender<isize>, Receiver<isize>) {
        use std::str::FromStr;
        let mem = input.lines().nth(0).unwrap().split(",")
            .map(isize::from_str)
            .map(Result::unwrap)
            .collect();
        Self::new(mem)
    }
    pub fn run(&mut self) -> Result<(), Error> {
        self.hlt = false;
        self.iptr = 0;
        while !self.hlt {
            let ix = self.parse_instruction()?;
            self.exec_instruction(ix)?;
        }
        Ok(())
    }
    pub fn run_parallel(mut self) -> JoinHandle<Result<Self, Error>> {
        thread::spawn(move|| {
            self.run()?;
            Ok(self)
        })
    }
    pub fn run_parallel_and_drop(mut self) -> JoinHandle<Result<(), Error>> {
        thread::spawn(move|| {
            self.run()
        })
    }
    pub fn mem(&self) -> &[isize] {
        &self.mem
    }
    fn parse_instruction(&mut self) -> Result<Instruction, Error> {
        let full_op = self.instruction_offset(0)?;
        // println!("Parsing {}", full_op);
        let op = full_op % 100;
        let mut pmodes = full_op / 100;
        let pmodes = &mut pmodes;
        let param: &mut dyn FnMut(&Cpu, usize) -> Result<Parameter, Error> =
        &mut move|this, offset| {
            let mode = (*pmodes % 10) as usize;
            *pmodes /= 10;
            match mode {
                0 => Ok(Parameter::Position(this.instruction_offset(offset)? as usize)),
                1 => Ok(Parameter::Value(this.instruction_offset(offset)?)),
                _ => Err(Error::InvalidParameterMode(mode)),
            }
        };
        use Instruction::*;
        let ix = match op {
            1 => Add(param(self, 1)?, param(self, 2)?, param(self, 3)?),
            2 => Mlt(param(self, 1)?, param(self, 2)?, param(self, 3)?),
            3 => Inp(param(self, 1)?),
            4 => Otp(param(self, 1)?),
            5 => Jit(param(self, 1)?, param(self, 2)?),
            6 => Jif(param(self, 1)?, param(self, 2)?),
            7 => Lth(param(self, 1)?, param(self, 2)?, param(self, 3)?),
            8 => Equ(param(self, 1)?, param(self, 2)?, param(self, 3)?),
            99 => Hlt,
            _ => Err(Error::InvalidInstruction(op))?,
        };

        Ok(ix)
    }
    fn instruction_offset(&self, offset: usize) -> Result<isize, Error> {
        self.mem.get(self.iptr + offset).copied().ok_or(Error::InstructionOverflow)
    }
    fn exec_instruction(&mut self, ix: Instruction) -> Result<(), Error> {
        // println!("Executing: {:?}", ix);
        use Instruction::*;
        let mut next_iptr = None;
        match ix {
            Add(a, b, mut out) => *out.as_mut(&mut self.mem) = a.value(&self.mem) + b.value(&self.mem),
            Mlt(a, b, mut out) => *out.as_mut(&mut self.mem) = a.value(&self.mem) * b.value(&self.mem),
            Inp(mut a) => *a.as_mut(&mut self.mem) = self.input.recv().ok().ok_or(Error::InputBroken)?,
            Otp(a) => self.output.send(a.value(&self.mem)).ok().ok_or(Error::OutputBroken)?,
            Jit(x, p) => if 0 != x.value(&self.mem) { next_iptr = Some(p.value(&self.mem) as usize) },
            Jif(x, p) => if 0 == x.value(&self.mem) { next_iptr = Some(p.value(&self.mem) as usize) },
            Lth(a, b, mut out) => *out.as_mut(&mut self.mem) = if a.value(&self.mem) < b.value(&self.mem) { 1 } else { 0 },
            Equ(a, b, mut out) => *out.as_mut(&mut self.mem) = if a.value(&self.mem) == b.value(&self.mem) { 1 } else { 0 },
            Hlt => self.hlt = true,
        }
        self.iptr = next_iptr.unwrap_or_else(||self.iptr + ix.len());
        // println!("Instruction ptr: {}", self.iptr);

        Ok(())
    }
}
