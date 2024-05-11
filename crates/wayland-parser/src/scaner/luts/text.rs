use crate::scaner::Scaner;

impl<'s> Scaner<'s> {
    pub fn scan_text(&mut self) {
        LOOKUP_TABLE[self.byte() as usize](self)
    }
}

type Handler = fn(&mut Scaner);

/// [XML Text](https://www.w3.org/TR/xml/#dt-text)
const LOOKUP_TABLE: &[Handler; 256] = &[
    // 0  1    2    3    4    5    6    7    8    9    A    B    C    D    E    F
    EOF, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 0
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 1
    ___, EXL, DQT, HSH, DOL, PCT, APS, SQT, OPN, CPN, ATR, PLS, CMA, MIS, DOT, SLH, // 2
    DIG, DIG, DIG, DIG, DIG, DIG, DIG, DIG, DIG, DIG, CLN, SMI, LST, EQL, GRT, QST, // 3
    ATS, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, // 4
    A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, OBT, BSH, CBT, CCF, UDL, // 5
    GAC, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, // 6
    A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, A2Z, OBE, VLN, CBE, TID, ___, // 7
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 8
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 9
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // A
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // B
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // C
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // D
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // E
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // F
];

const EOF: Handler = |_| {};
const ___: Handler = |_| {};

const TXT: Handler = |sn: &mut Scaner| {
    sn.skip(1);

    sn.scan_text();
};

const EXL: Handler = TXT; // '!'
const DQT: Handler = ___; // '"'
const HSH: Handler = TXT; // '#'
const DOL: Handler = TXT; // '$'
const PCT: Handler = TXT; // '%'
const APS: Handler = TXT; // '&'
const SQT: Handler = ___; // '''
const OPN: Handler = TXT; // '('
const CPN: Handler = TXT; // ')'
const ATR: Handler = TXT; // '*'
const PLS: Handler = TXT; // '+'
const CMA: Handler = TXT; // ','
const MIS: Handler = TXT; // '-'
const DOT: Handler = TXT; // '.'
const SLH: Handler = TXT; // '/'
const DIG: Handler = TXT; // [0-9]
const CLN: Handler = TXT; // ':'
const SMI: Handler = TXT; // ';'
const LST: Handler = ___; // '<'
const EQL: Handler = TXT; // '-'
const GRT: Handler = TXT; // '>'
const QST: Handler = TXT; // '?'
const ATS: Handler = TXT; // '@'
const A2Z: Handler = TXT; // [a-z|A-Z]
const OBT: Handler = TXT; // '['
const BSH: Handler = TXT; // '\'
const CBT: Handler = TXT; // ']'
const CCF: Handler = TXT; // '^'
const UDL: Handler = TXT; // '_'
const GAC: Handler = TXT; // '`'
const OBE: Handler = TXT; // '{'
const VLN: Handler = TXT; // '|'
const CBE: Handler = TXT; // '}'
const TID: Handler = TXT; // '~'
