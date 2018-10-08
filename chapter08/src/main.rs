use std::cmp::*;
use std::ops::*;
use std::default::Default;

#[derive(Clone)]
pub enum CloneExample {
    Good,
    Bad,
}

#[derive(Copy, Clone)]
pub enum CopyExample {
    Good,
    Bad,
}

#[derive(Debug)]
pub enum DebugExample {
    Good,
    Bad,
}

#[derive(PartialEq)]
pub enum PartialEqSelf {
    Good,
    Bad,
}

pub enum PartialEqU32 {
    Good,
    Bad,
}

impl PartialEq<u32> for PartialEqU32 {
    fn eq(&self, other: &u32) -> bool {
        match self {
            PartialEqU32::Good => other % 2 == 0,
            PartialEqU32::Bad => other % 2 == 1,
        }
    }
}

#[derive(Eq, PartialEq)]
pub enum EqExample {
    Good,
    Bad,
}

#[derive(PartialOrd, PartialEq)]
pub enum PartialOrdSelf {
    Good,
    Bad,
}

pub enum PartialOrdU32 {
    Good,
    Bad,
}

impl PartialEq<u32> for PartialOrdU32 {
    fn eq(&self, _other: &u32) -> bool {
        false
    }
}

impl PartialOrd<u32> for PartialOrdU32 {
    fn partial_cmp(&self, _other: &u32) -> Option<Ordering> {
        match self {
            PartialOrdU32::Good => Some(Ordering::Greater),
            PartialOrdU32::Bad => None,
        }
    }
}

#[derive(Ord, Eq, PartialOrd, PartialEq)]
pub enum OrdExample {
    Good,
    Bad,
}

#[derive(Hash)]
pub enum HashExample {
    Good,
    Bad,
}

#[derive(Default)]
pub struct DefaultExample {
    name: String,
    value: i32,
}

pub enum AddExample {
    One,
    Two,
    Three,
    Many,
}

impl Add for AddExample {
    type Output = AddExample;

    fn add(self, other: AddExample) -> AddExample {
        match (self, other) {
            (AddExample::One, AddExample::One) => AddExample::Two,
            (AddExample::One, AddExample::Two) => AddExample::Three,
            (AddExample::Two, AddExample::One) => AddExample::Three,
            _ => AddExample::Many,
        }
    }
}

impl AddAssign for AddExample {
    fn add_assign(&mut self, other: AddExample) {
        *self = match (&self, other) {
            (AddExample::One, AddExample::One) => AddExample::Two,
            (AddExample::One, AddExample::Two) => AddExample::Three,
            (AddExample::Two, AddExample::One) => AddExample::Three,
            _ => AddExample::Many,
        };
    }
}

pub enum BitExample {
    Yes,
    No,
}

impl BitAnd for BitExample {
    type Output = BitExample;

    fn bitand(self, other: BitExample) -> BitExample {
        match (self, other) {
            (BitExample::Yes, BitExample::Yes) => BitExample::Yes,
            (BitExample::No, BitExample::Yes) => BitExample::No,
            (BitExample::Yes, BitExample::No) => BitExample::No,
            (BitExample::No, BitExample::No) => BitExample::No,
        }
    }
}

impl BitAndAssign for BitExample {
    fn bitand_assign(&mut self, other: BitExample) {
        *self = match (&self, other) {
            (BitExample::Yes, BitExample::Yes) => BitExample::Yes,
            (BitExample::No, BitExample::Yes) => BitExample::No,
            (BitExample::Yes, BitExample::No) => BitExample::No,
            (BitExample::No, BitExample::No) => BitExample::No,
        };
    }
}

impl BitOr for BitExample {
    type Output = BitExample;

    fn bitor(self, other: BitExample) -> BitExample {
        match (self, other) {
            (BitExample::Yes, BitExample::Yes) => BitExample::Yes,
            (BitExample::No, BitExample::Yes) => BitExample::Yes,
            (BitExample::Yes, BitExample::No) => BitExample::Yes,
            (BitExample::No, BitExample::No) => BitExample::No,
        }
    }
}

impl BitOrAssign for BitExample {
    fn bitor_assign(&mut self, other: BitExample) {
        *self = match (&self, other) {
            (BitExample::Yes, BitExample::Yes) => BitExample::Yes,
            (BitExample::No, BitExample::Yes) => BitExample::Yes,
            (BitExample::Yes, BitExample::No) => BitExample::Yes,
            (BitExample::No, BitExample::No) => BitExample::No,
        };
    }
}

impl BitXor for BitExample {
    type Output = BitExample;

    fn bitxor(self, other: BitExample) -> BitExample {
        match (self, other) {
            (BitExample::Yes, BitExample::Yes) => BitExample::No,
            (BitExample::No, BitExample::Yes) => BitExample::Yes,
            (BitExample::Yes, BitExample::No) => BitExample::Yes,
            (BitExample::No, BitExample::No) => BitExample::No,
        }
    }
}

impl BitXorAssign for BitExample {
    fn bitxor_assign(&mut self, other: BitExample) {
        *self = match (&self, other) {
            (BitExample::Yes, BitExample::Yes) => BitExample::No,
            (BitExample::No, BitExample::Yes) => BitExample::Yes,
            (BitExample::Yes, BitExample::No) => BitExample::Yes,
            (BitExample::No, BitExample::No) => BitExample::No,
        };
    }
}

pub struct DerefExample {
    val: u32,
}

impl Deref for DerefExample {
    type Target = u32;

    fn deref(&self) -> &u32 {
        return &self.val;
    }
}

impl DerefMut for DerefExample {
    fn deref_mut(&mut self) -> &mut u32 {
        return &mut self.val;
    }
}

pub enum DropExample {
    Good,
    Bad,
}

impl Drop for DropExample {
    fn drop(&mut self) {
        match self {
            DropExample::Good => println!("Good DropExample dropped"),
            DropExample::Bad => println!("Bad DropExample dropped"),
        };
    }
}

pub struct IndexExample {
    first: u32,
    second: u32,
    third: u32,
    junk: u32,
}

impl<'a> Index<&'a str> for IndexExample {
    type Output = u32;

    fn index(&self, index: &'a str) -> &u32 {
        match index {
            "first" => &self.first,
            "second" => &self.second,
            "third" => &self.third,
            _ => &0,
        }
    }
}

impl<'a> IndexMut<&'a str> for IndexExample {
    fn index_mut(&mut self, index: &'a str) -> &mut u32 {
        match index {
            "first" => &mut self.first,
            "second" => &mut self.second,
            "third" => &mut self.third,
            _ => &mut self.junk,
        }
    }
}

pub enum NegExample {
    Yes,
    No,
}

impl Neg for NegExample {
    type Output = NegExample;

    fn neg(self) -> NegExample {
        match self {
            NegExample::Yes => NegExample::No,
            NegExample::No => NegExample::Yes,
        }
    }
}

pub enum NotExample {
    True,
    False,
}

impl Not for NotExample {
    type Output = NotExample;

    fn not(self) -> NotExample {
        match self {
            NotExample::True => NotExample::False,
            NotExample::False => NotExample::True,
        }
    }
}

pub enum NotSyncExample {
    Good,
    Bad,
}

impl !Sync for NotSyncExample {}

fn main() {
    let x: DefaultExample = Default::default();
    println!("Default String is {:?}, default i32 is {:?}", x.name, x.value);
}
