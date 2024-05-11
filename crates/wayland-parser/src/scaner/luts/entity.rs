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
    ZRO, DIG, DIG, DIG, DIG, DIG, DIG, DIG, DIG, DIG, ___, SMI, LST, EQL, GRT, QST, // 3
    ATS, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 4
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, OBT, BSH, CBT, CCF, ___, // 5
    GAC, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 6
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, OBE, VLN, CBE, TID, ___, // 7
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 8
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 9
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // A
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // B
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // C
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // D
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // E
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // F
];

const EOF: HANDLER = |_| None;
const ___: HANDLER = |_| unreachable!("Invalid Byte");

/// [NameStartChar](https://www.w3.org/TR/xml/#NT-NameStartChar)
const ELM: HANDLER = |sn| {
    sn.mklo();
    sn.skip(1);
    sn.scan_name();
    sn.mkhi();

    todo!()
};

const TXT: HANDLER = |sn| {
    sn.skip(1);
    todo!()
};

const LST: HANDLER = |sn| {
    sn.skip(1);
    todo!()
};

const EQL: HANDLER = |sn| {
    sn.skip(1);
    todo!()
};

const GRT: HANDLER = |sn| {
    sn.skip(1);
    todo!()
};

const DQT: HANDLER = |sn| {
    sn.skip(1);
    todo!()
};
