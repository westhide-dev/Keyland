use crate::scaner::{unit::Unit, Scaner};

impl<'s> Scaner<'s> {
    pub fn lookup_entity(&mut self) -> Option<Unit<'s>> {
        LOOKUP_TABLE[self.byte() as usize](self)
    }
}

type HANDLER = for<'s> fn(&mut Scaner<'s>) -> Option<Unit<'s>>;

const LOOKUP_TABLE: &[HANDLER; 256] = &[
    // 0  1    2    3    4    5    6    7    8    9    A    B    C    D    E    F
    EOF, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 0
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 1
    ___, EXL, DQT, HSH, DOL, PCT, APS, SQT, OPN, CPN, ATR, PLS, CMA, MIS, DOT, SLH, // 2
    ZRO, DIG, DIG, DIG, DIG, DIG, DIG, DIG, DIG, DIG, CLN, SMI, LST, EQL, GRT, QST, // 3
    ATS, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, // 4
    A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, OBT, BSH, CBT, CCF, UDL, // 5
    GAC, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, // 6
    A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, OBE, VLN, CBE, TID, ___, // 7
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 8
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 9
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // A
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // B
    ___, ___, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, // C
    UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, // D
    UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, // E
    UNI, UNI, UNI, UNI, UNI, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // F
];

const ___: HANDLER = |_| unreachable!("Invalid entity byte");
const UNI: HANDLER = |_| unreachable!("Invalid entity byte");

const EOF: HANDLER = |_| None;

/// [NameStartChar](https://www.w3.org/TR/xml/#NT-NameStartChar)
const NSC: HANDLER = |sn| {
    sn.skip(1);
    todo!()
};

const A2Z: HANDLER = NSC; // [a-z|A-Z]
const CLN: HANDLER = NSC; // ':'
const UDL: HANDLER = NSC; // '_'

const LIT: HANDLER = |sn| {
    sn.skip(1);
    todo!()
};

// [0-9]
const DIG: HANDLER = LIT;
