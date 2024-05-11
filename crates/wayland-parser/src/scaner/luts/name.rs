use crate::scaner::Scaner;

impl<'s> Scaner<'s> {
    pub fn scan_name(&mut self) {
        LOOKUP_TABLE[self.byte() as usize](self)
    }
}

type HANDLER = fn(&mut Scaner);

/// [XML Name](https://www.w3.org/TR/xml/#NT-Name)
const LOOKUP_TABLE: &[HANDLER; 256] = &[
    // 0  1    2    3    4    5    6    7    8    9    A    B    C    D    E    F
    EOF, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 0
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 1
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, MIS, DOT, ___, // 2
    ___, DIG, DIG, DIG, DIG, DIG, DIG, DIG, DIG, DIG, CLN, ___, ___, ___, ___, ___, // 3
    ___, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, // 4
    A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, ___, ___, ___, ___, UDL, // 5
    ___, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, // 6
    A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, ___, ___, ___, ___, ___, // 7
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 8
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 9
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // A
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // B
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // C
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // D
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // E
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // F
];

const EOF: HANDLER = |_| {};
const ___: HANDLER = |_| {};

const _N_: HANDLER = |sn| {
    sn.skip(1);

    sn.scan_name();
};

/// [NameStartChar](https://www.w3.org/TR/xml/#NT-NameStartChar)
const A2Z: HANDLER = _N_; // [a-z|A-Z]
const CLN: HANDLER = _N_; // ':'
const UDL: HANDLER = _N_; // '_'

/// [NameChar](https://www.w3.org/TR/xml/#NT-NameChar)
const DIG: HANDLER = _N_; // [0-9]
const MIS: HANDLER = _N_; // '-'
const DOT: HANDLER = _N_; // '.'
