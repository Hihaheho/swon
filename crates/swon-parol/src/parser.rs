// ---------------------------------------------------------
// This file was generated by parol.
// It is not intended for manual editing and changes will be
// lost after next build.
// ---------------------------------------------------------

use parol_runtime::once_cell::sync::Lazy;
#[allow(unused_imports)]
use parol_runtime::parser::{LLKParser, LookaheadDFA, ParseTreeType, ParseType, Production, Trans};
use parol_runtime::{ParolError, ParseTree, TerminalIndex};
use parol_runtime::{ScannerConfig, TokenStream, Tokenizer};
use std::path::Path;

use crate::grammar::Grammar;
use crate::grammar_trait::GrammarAuto;

use parol_runtime::lexer::tokenizer::{
    ERROR_TOKEN, NEW_LINE_TOKEN, UNMATCHABLE_TOKEN, WHITESPACE_TOKEN,
};

pub const TERMINALS: &[&str; 29] = &[
    /*  0 */ UNMATCHABLE_TOKEN,
    /*  1 */ UNMATCHABLE_TOKEN,
    /*  2 */ UNMATCHABLE_TOKEN,
    /*  3 */ UNMATCHABLE_TOKEN,
    /*  4 */ UNMATCHABLE_TOKEN,
    /*  5 */ r"\d[\d_]*",
    /*  6 */ r"true",
    /*  7 */ r"false",
    /*  8 */ r"null",
    /*  9 */ r"!",
    /* 10 */ r#"""#,
    /* 11 */ r#"\p{XID_Start}\p{XID_Continue}*""#,
    /* 12 */
    r#"(\\[nrt\\"0]|\p{Letter}|\p{Mark}|\p{Number}|[\p{Punctuation}--\\"]|\p{Symbol}|\p{Space_Separator})*"#,
    /* 13 */
    r"(\p{Letter}|\p{Mark}|\p{Number}|\p{Punctuation}|\p{Symbol}|\p{Space_Separator})*",
    /* 14 */ r"\r\n|\r|\n",
    /* 15 */ r"[\s--\r\n]+",
    /* 16 */ r"@",
    /* 17 */ r"\$",
    /* 18 */ r"\.",
    /* 19 */ r"\{",
    /* 20 */ r"\}",
    /* 21 */ r"\[",
    /* 22 */ r"\]",
    /* 23 */ r"=",
    /* 24 */ r",",
    /* 25 */ r"\\\\",
    /* 26 */ r":",
    /* 27 */ r"\p{XID_Start}\p{XID_Continue}*",
    /* 28 */ ERROR_TOKEN,
];

pub const TERMINAL_NAMES: &[&str; 29] = &[
    /*  0 */ "EndOfInput",
    /*  1 */ "Newline",
    /*  2 */ "Whitespace",
    /*  3 */ "LineComment",
    /*  4 */ "BlockComment",
    /*  5 */ "Integer",
    /*  6 */ "True",
    /*  7 */ "False",
    /*  8 */ "Null",
    /*  9 */ "Hole",
    /* 10 */ "Quote",
    /* 11 */ "TypedQuote",
    /* 12 */ "InString",
    /* 13 */ "Text",
    /* 14 */ "Newline0",
    /* 15 */ "Ws",
    /* 16 */ "At",
    /* 17 */ "Ext",
    /* 18 */ "Dot",
    /* 19 */ "Begin",
    /* 20 */ "End",
    /* 21 */ "ArrayBegin",
    /* 22 */ "ArrayEnd",
    /* 23 */ "Bind",
    /* 24 */ "Comma",
    /* 25 */ "Continue",
    /* 26 */ "TextStart",
    /* 27 */ "Ident",
    /* 28 */ "Error",
];

/* SCANNER_0: "INITIAL" */
const SCANNER_0: (&[&str; 5], &[TerminalIndex; 19]) = (
    &[
        /*  0 */ UNMATCHABLE_TOKEN,
        /*  1 */ NEW_LINE_TOKEN,
        /*  2 */ WHITESPACE_TOKEN,
        /*  3 */ r"(#.*(\r\n|\r|\n|$))",
        /*  4 */ UNMATCHABLE_TOKEN,
    ],
    &[
        5,  /* Integer */
        6,  /* True */
        7,  /* False */
        8,  /* Null */
        9,  /* Hole */
        10, /* Quote */
        11, /* TypedQuote */
        16, /* At */
        17, /* Ext */
        18, /* Dot */
        19, /* Begin */
        20, /* End */
        21, /* ArrayBegin */
        22, /* ArrayEnd */
        23, /* Bind */
        24, /* Comma */
        25, /* Continue */
        26, /* TextStart */
        27, /* Ident */
    ],
);

/* SCANNER_1: "String" */
const SCANNER_1: (&[&str; 5], &[TerminalIndex; 4]) = (
    &[
        /*  0 */ UNMATCHABLE_TOKEN,
        /*  1 */ UNMATCHABLE_TOKEN,
        /*  2 */ UNMATCHABLE_TOKEN,
        /*  3 */ UNMATCHABLE_TOKEN,
        /*  4 */ UNMATCHABLE_TOKEN,
    ],
    &[
        10, /* Quote */
        11, /* TypedQuote */
        12, /* InString */
        15, /* Ws */
    ],
);

/* SCANNER_2: "Text" */
const SCANNER_2: (&[&str; 5], &[TerminalIndex; 3]) = (
    &[
        /*  0 */ UNMATCHABLE_TOKEN,
        /*  1 */ UNMATCHABLE_TOKEN,
        /*  2 */ UNMATCHABLE_TOKEN,
        /*  3 */ UNMATCHABLE_TOKEN,
        /*  4 */ UNMATCHABLE_TOKEN,
    ],
    &[13 /* Text */, 14 /* Newline0 */, 15 /* Ws */],
);

const MAX_K: usize = 1;

pub const NON_TERMINALS: &[&str; 54] = &[
    /*  0 */ "Array",
    /*  1 */ "ArrayBegin",
    /*  2 */ "ArrayEnd",
    /*  3 */ "ArrayList",
    /*  4 */ "ArrayMarker",
    /*  5 */ "ArrayMarkerOpt",
    /*  6 */ "ArrayOpt",
    /*  7 */ "At",
    /*  8 */ "Begin",
    /*  9 */ "Bind",
    /* 10 */ "Binding",
    /* 11 */ "Bindings",
    /* 12 */ "Boolean",
    /* 13 */ "Comma",
    /* 14 */ "Continue",
    /* 15 */ "Dot",
    /* 16 */ "End",
    /* 17 */ "Ext",
    /* 18 */ "ExtensionNameSpace",
    /* 19 */ "False",
    /* 20 */ "Hole",
    /* 21 */ "Ident",
    /* 22 */ "InString",
    /* 23 */ "Integer",
    /* 24 */ "Key",
    /* 25 */ "KeyBase",
    /* 26 */ "KeyOpt",
    /* 27 */ "Keys",
    /* 28 */ "KeysList",
    /* 29 */ "Newline",
    /* 30 */ "Null",
    /* 31 */ "Object",
    /* 32 */ "ObjectList",
    /* 33 */ "ObjectOpt",
    /* 34 */ "Quote",
    /* 35 */ "Section",
    /* 36 */ "SectionBinding",
    /* 37 */ "SectionList",
    /* 38 */ "String",
    /* 39 */ "StringContinues",
    /* 40 */ "StringContinuesList",
    /* 41 */ "Swon",
    /* 42 */ "SwonList",
    /* 43 */ "SwonList0",
    /* 44 */ "Text",
    /* 45 */ "TextBinding",
    /* 46 */ "TextBindingOpt",
    /* 47 */ "TextStart",
    /* 48 */ "True",
    /* 49 */ "TypedQuote",
    /* 50 */ "TypedString",
    /* 51 */ "Value",
    /* 52 */ "ValueBinding",
    /* 53 */ "Ws",
];

pub const LOOKAHEAD_AUTOMATA: &[LookaheadDFA; 54] = &[
    /* 0 - "Array" */
    LookaheadDFA {
        prod0: 43,
        transitions: &[],
        k: 0,
    },
    /* 1 - "ArrayBegin" */
    LookaheadDFA {
        prod0: 71,
        transitions: &[],
        k: 0,
    },
    /* 2 - "ArrayEnd" */
    LookaheadDFA {
        prod0: 72,
        transitions: &[],
        k: 0,
    },
    /* 3 - "ArrayList" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 5, 1, 44),
            Trans(0, 6, 1, 44),
            Trans(0, 7, 1, 44),
            Trans(0, 8, 1, 44),
            Trans(0, 9, 1, 44),
            Trans(0, 10, 1, 44),
            Trans(0, 11, 1, 44),
            Trans(0, 19, 1, 44),
            Trans(0, 21, 1, 44),
            Trans(0, 22, 2, 45),
        ],
        k: 1,
    },
    /* 4 - "ArrayMarker" */
    LookaheadDFA {
        prod0: 23,
        transitions: &[],
        k: 0,
    },
    /* 5 - "ArrayMarkerOpt" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[Trans(0, 5, 1, 24), Trans(0, 22, 2, 25)],
        k: 1,
    },
    /* 6 - "ArrayOpt" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 5, 2, 47),
            Trans(0, 6, 2, 47),
            Trans(0, 7, 2, 47),
            Trans(0, 8, 2, 47),
            Trans(0, 9, 2, 47),
            Trans(0, 10, 2, 47),
            Trans(0, 11, 2, 47),
            Trans(0, 19, 2, 47),
            Trans(0, 21, 2, 47),
            Trans(0, 22, 2, 47),
            Trans(0, 24, 1, 46),
        ],
        k: 1,
    },
    /* 7 - "At" */
    LookaheadDFA {
        prod0: 66,
        transitions: &[],
        k: 0,
    },
    /* 8 - "Begin" */
    LookaheadDFA {
        prod0: 69,
        transitions: &[],
        k: 0,
    },
    /* 9 - "Bind" */
    LookaheadDFA {
        prod0: 73,
        transitions: &[],
        k: 0,
    },
    /* 10 - "Binding" */
    LookaheadDFA {
        prod0: 5,
        transitions: &[],
        k: 0,
    },
    /* 11 - "Bindings" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[Trans(0, 19, 2, 7), Trans(0, 23, 1, 6), Trans(0, 26, 3, 8)],
        k: 1,
    },
    /* 12 - "Boolean" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[Trans(0, 6, 1, 49), Trans(0, 7, 2, 50)],
        k: 1,
    },
    /* 13 - "Comma" */
    LookaheadDFA {
        prod0: 74,
        transitions: &[],
        k: 0,
    },
    /* 14 - "Continue" */
    LookaheadDFA {
        prod0: 75,
        transitions: &[],
        k: 0,
    },
    /* 15 - "Dot" */
    LookaheadDFA {
        prod0: 68,
        transitions: &[],
        k: 0,
    },
    /* 16 - "End" */
    LookaheadDFA {
        prod0: 70,
        transitions: &[],
        k: 0,
    },
    /* 17 - "Ext" */
    LookaheadDFA {
        prod0: 67,
        transitions: &[],
        k: 0,
    },
    /* 18 - "ExtensionNameSpace" */
    LookaheadDFA {
        prod0: 29,
        transitions: &[],
        k: 0,
    },
    /* 19 - "False" */
    LookaheadDFA {
        prod0: 52,
        transitions: &[],
        k: 0,
    },
    /* 20 - "Hole" */
    LookaheadDFA {
        prod0: 54,
        transitions: &[],
        k: 0,
    },
    /* 21 - "Ident" */
    LookaheadDFA {
        prod0: 77,
        transitions: &[],
        k: 0,
    },
    /* 22 - "InString" */
    LookaheadDFA {
        prod0: 62,
        transitions: &[],
        k: 0,
    },
    /* 23 - "Integer" */
    LookaheadDFA {
        prod0: 48,
        transitions: &[],
        k: 0,
    },
    /* 24 - "Key" */
    LookaheadDFA {
        prod0: 20,
        transitions: &[],
        k: 0,
    },
    /* 25 - "KeyBase" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 10, 3, 28),
            Trans(0, 17, 2, 27),
            Trans(0, 27, 1, 26),
        ],
        k: 1,
    },
    /* 26 - "KeyOpt" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 0, 2, 22),
            Trans(0, 10, 2, 22),
            Trans(0, 16, 2, 22),
            Trans(0, 17, 2, 22),
            Trans(0, 18, 2, 22),
            Trans(0, 19, 2, 22),
            Trans(0, 20, 2, 22),
            Trans(0, 21, 1, 21),
            Trans(0, 23, 2, 22),
            Trans(0, 26, 2, 22),
            Trans(0, 27, 2, 22),
        ],
        k: 1,
    },
    /* 27 - "Keys" */
    LookaheadDFA {
        prod0: 17,
        transitions: &[],
        k: 0,
    },
    /* 28 - "KeysList" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 0, 2, 19),
            Trans(0, 10, 2, 19),
            Trans(0, 16, 2, 19),
            Trans(0, 17, 2, 19),
            Trans(0, 18, 1, 18),
            Trans(0, 19, 2, 19),
            Trans(0, 20, 2, 19),
            Trans(0, 23, 2, 19),
            Trans(0, 26, 2, 19),
            Trans(0, 27, 2, 19),
        ],
        k: 1,
    },
    /* 29 - "Newline" */
    LookaheadDFA {
        prod0: 64,
        transitions: &[],
        k: 0,
    },
    /* 30 - "Null" */
    LookaheadDFA {
        prod0: 53,
        transitions: &[],
        k: 0,
    },
    /* 31 - "Object" */
    LookaheadDFA {
        prod0: 38,
        transitions: &[],
        k: 0,
    },
    /* 32 - "ObjectList" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 10, 1, 39),
            Trans(0, 17, 1, 39),
            Trans(0, 20, 2, 40),
            Trans(0, 27, 1, 39),
        ],
        k: 1,
    },
    /* 33 - "ObjectOpt" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 10, 2, 42),
            Trans(0, 17, 2, 42),
            Trans(0, 20, 2, 42),
            Trans(0, 24, 1, 41),
            Trans(0, 27, 2, 42),
        ],
        k: 1,
    },
    /* 34 - "Quote" */
    LookaheadDFA {
        prod0: 60,
        transitions: &[],
        k: 0,
    },
    /* 35 - "Section" */
    LookaheadDFA {
        prod0: 14,
        transitions: &[],
        k: 0,
    },
    /* 36 - "SectionBinding" */
    LookaheadDFA {
        prod0: 10,
        transitions: &[],
        k: 0,
    },
    /* 37 - "SectionList" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 0, 2, 16),
            Trans(0, 10, 1, 15),
            Trans(0, 16, 2, 16),
            Trans(0, 17, 1, 15),
            Trans(0, 20, 2, 16),
            Trans(0, 27, 1, 15),
        ],
        k: 1,
    },
    /* 38 - "String" */
    LookaheadDFA {
        prod0: 58,
        transitions: &[],
        k: 0,
    },
    /* 39 - "StringContinues" */
    LookaheadDFA {
        prod0: 55,
        transitions: &[],
        k: 0,
    },
    /* 40 - "StringContinuesList" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 0, 2, 57),
            Trans(0, 5, 2, 57),
            Trans(0, 6, 2, 57),
            Trans(0, 7, 2, 57),
            Trans(0, 8, 2, 57),
            Trans(0, 9, 2, 57),
            Trans(0, 10, 2, 57),
            Trans(0, 11, 2, 57),
            Trans(0, 16, 2, 57),
            Trans(0, 17, 2, 57),
            Trans(0, 19, 2, 57),
            Trans(0, 20, 2, 57),
            Trans(0, 21, 2, 57),
            Trans(0, 22, 2, 57),
            Trans(0, 24, 2, 57),
            Trans(0, 25, 1, 56),
            Trans(0, 27, 2, 57),
        ],
        k: 1,
    },
    /* 41 - "Swon" */
    LookaheadDFA {
        prod0: 0,
        transitions: &[],
        k: 0,
    },
    /* 42 - "SwonList" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 0, 2, 4),
            Trans(0, 10, 1, 3),
            Trans(0, 16, 2, 4),
            Trans(0, 17, 1, 3),
            Trans(0, 20, 2, 4),
            Trans(0, 27, 1, 3),
        ],
        k: 1,
    },
    /* 43 - "SwonList0" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[Trans(0, 0, 2, 2), Trans(0, 16, 1, 1), Trans(0, 20, 2, 2)],
        k: 1,
    },
    /* 44 - "Text" */
    LookaheadDFA {
        prod0: 63,
        transitions: &[],
        k: 0,
    },
    /* 45 - "TextBinding" */
    LookaheadDFA {
        prod0: 11,
        transitions: &[],
        k: 0,
    },
    /* 46 - "TextBindingOpt" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[Trans(0, 13, 2, 13), Trans(0, 15, 1, 12)],
        k: 1,
    },
    /* 47 - "TextStart" */
    LookaheadDFA {
        prod0: 76,
        transitions: &[],
        k: 0,
    },
    /* 48 - "True" */
    LookaheadDFA {
        prod0: 51,
        transitions: &[],
        k: 0,
    },
    /* 49 - "TypedQuote" */
    LookaheadDFA {
        prod0: 61,
        transitions: &[],
        k: 0,
    },
    /* 50 - "TypedString" */
    LookaheadDFA {
        prod0: 59,
        transitions: &[],
        k: 0,
    },
    /* 51 - "Value" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 5, 3, 32),
            Trans(0, 6, 4, 33),
            Trans(0, 7, 4, 33),
            Trans(0, 8, 5, 34),
            Trans(0, 9, 8, 37),
            Trans(0, 10, 6, 35),
            Trans(0, 11, 7, 36),
            Trans(0, 19, 1, 30),
            Trans(0, 21, 2, 31),
        ],
        k: 1,
    },
    /* 52 - "ValueBinding" */
    LookaheadDFA {
        prod0: 9,
        transitions: &[],
        k: 0,
    },
    /* 53 - "Ws" */
    LookaheadDFA {
        prod0: 65,
        transitions: &[],
        k: 0,
    },
];

pub const PRODUCTIONS: &[Production; 78] = &[
    // 0 - Swon: SwonList /* Vec */ SwonList0 /* Vec */;
    Production {
        lhs: 41,
        production: &[ParseType::N(43), ParseType::N(42)],
    },
    // 1 - SwonList0: Section SwonList0;
    Production {
        lhs: 43,
        production: &[ParseType::N(43), ParseType::N(35)],
    },
    // 2 - SwonList0: ;
    Production {
        lhs: 43,
        production: &[],
    },
    // 3 - SwonList: Binding SwonList;
    Production {
        lhs: 42,
        production: &[ParseType::N(42), ParseType::N(10)],
    },
    // 4 - SwonList: ;
    Production {
        lhs: 42,
        production: &[],
    },
    // 5 - Binding: Keys Bindings;
    Production {
        lhs: 10,
        production: &[ParseType::N(11), ParseType::N(27)],
    },
    // 6 - Bindings: ValueBinding;
    Production {
        lhs: 11,
        production: &[ParseType::N(52)],
    },
    // 7 - Bindings: SectionBinding;
    Production {
        lhs: 11,
        production: &[ParseType::N(36)],
    },
    // 8 - Bindings: TextBinding;
    Production {
        lhs: 11,
        production: &[ParseType::N(45)],
    },
    // 9 - ValueBinding: Bind Value;
    Production {
        lhs: 52,
        production: &[ParseType::N(51), ParseType::N(9)],
    },
    // 10 - SectionBinding: Begin Swon End;
    Production {
        lhs: 36,
        production: &[ParseType::N(16), ParseType::N(41), ParseType::N(8)],
    },
    // 11 - TextBinding: TextStart TextBindingOpt /* Option */ Text Newline;
    Production {
        lhs: 45,
        production: &[
            ParseType::N(29),
            ParseType::N(44),
            ParseType::N(46),
            ParseType::N(47),
        ],
    },
    // 12 - TextBindingOpt: Ws^ /* Clipped */;
    Production {
        lhs: 46,
        production: &[ParseType::N(53)],
    },
    // 13 - TextBindingOpt: ;
    Production {
        lhs: 46,
        production: &[],
    },
    // 14 - Section: At Keys SectionList /* Vec */;
    Production {
        lhs: 35,
        production: &[ParseType::N(37), ParseType::N(27), ParseType::N(7)],
    },
    // 15 - SectionList: Binding SectionList;
    Production {
        lhs: 37,
        production: &[ParseType::N(37), ParseType::N(10)],
    },
    // 16 - SectionList: ;
    Production {
        lhs: 37,
        production: &[],
    },
    // 17 - Keys: Key KeysList /* Vec */;
    Production {
        lhs: 27,
        production: &[ParseType::N(28), ParseType::N(24)],
    },
    // 18 - KeysList: Dot Key KeysList;
    Production {
        lhs: 28,
        production: &[ParseType::N(28), ParseType::N(24), ParseType::N(15)],
    },
    // 19 - KeysList: ;
    Production {
        lhs: 28,
        production: &[],
    },
    // 20 - Key: KeyBase KeyOpt /* Option */;
    Production {
        lhs: 24,
        production: &[ParseType::N(26), ParseType::N(25)],
    },
    // 21 - KeyOpt: ArrayMarker;
    Production {
        lhs: 26,
        production: &[ParseType::N(4)],
    },
    // 22 - KeyOpt: ;
    Production {
        lhs: 26,
        production: &[],
    },
    // 23 - ArrayMarker: ArrayBegin ArrayMarkerOpt /* Option */ ArrayEnd;
    Production {
        lhs: 4,
        production: &[ParseType::N(2), ParseType::N(5), ParseType::N(1)],
    },
    // 24 - ArrayMarkerOpt: Integer;
    Production {
        lhs: 5,
        production: &[ParseType::N(23)],
    },
    // 25 - ArrayMarkerOpt: ;
    Production {
        lhs: 5,
        production: &[],
    },
    // 26 - KeyBase: Ident;
    Production {
        lhs: 25,
        production: &[ParseType::N(21)],
    },
    // 27 - KeyBase: ExtensionNameSpace;
    Production {
        lhs: 25,
        production: &[ParseType::N(18)],
    },
    // 28 - KeyBase: String;
    Production {
        lhs: 25,
        production: &[ParseType::N(38)],
    },
    // 29 - ExtensionNameSpace: Ext Ident;
    Production {
        lhs: 18,
        production: &[ParseType::N(21), ParseType::N(17)],
    },
    // 30 - Value: Object;
    Production {
        lhs: 51,
        production: &[ParseType::N(31)],
    },
    // 31 - Value: Array;
    Production {
        lhs: 51,
        production: &[ParseType::N(0)],
    },
    // 32 - Value: Integer;
    Production {
        lhs: 51,
        production: &[ParseType::N(23)],
    },
    // 33 - Value: Boolean;
    Production {
        lhs: 51,
        production: &[ParseType::N(12)],
    },
    // 34 - Value: Null;
    Production {
        lhs: 51,
        production: &[ParseType::N(30)],
    },
    // 35 - Value: StringContinues;
    Production {
        lhs: 51,
        production: &[ParseType::N(39)],
    },
    // 36 - Value: TypedString;
    Production {
        lhs: 51,
        production: &[ParseType::N(50)],
    },
    // 37 - Value: Hole;
    Production {
        lhs: 51,
        production: &[ParseType::N(20)],
    },
    // 38 - Object: Begin ObjectList /* Vec */ End;
    Production {
        lhs: 31,
        production: &[ParseType::N(16), ParseType::N(32), ParseType::N(8)],
    },
    // 39 - ObjectList: Key Bind Value ObjectOpt /* Option */ ObjectList;
    Production {
        lhs: 32,
        production: &[
            ParseType::N(32),
            ParseType::N(33),
            ParseType::N(51),
            ParseType::N(9),
            ParseType::N(24),
        ],
    },
    // 40 - ObjectList: ;
    Production {
        lhs: 32,
        production: &[],
    },
    // 41 - ObjectOpt: Comma;
    Production {
        lhs: 33,
        production: &[ParseType::N(13)],
    },
    // 42 - ObjectOpt: ;
    Production {
        lhs: 33,
        production: &[],
    },
    // 43 - Array: ArrayBegin ArrayList /* Vec */ ArrayEnd;
    Production {
        lhs: 0,
        production: &[ParseType::N(2), ParseType::N(3), ParseType::N(1)],
    },
    // 44 - ArrayList: Value ArrayOpt /* Option */ ArrayList;
    Production {
        lhs: 3,
        production: &[ParseType::N(3), ParseType::N(6), ParseType::N(51)],
    },
    // 45 - ArrayList: ;
    Production {
        lhs: 3,
        production: &[],
    },
    // 46 - ArrayOpt: Comma;
    Production {
        lhs: 6,
        production: &[ParseType::N(13)],
    },
    // 47 - ArrayOpt: ;
    Production {
        lhs: 6,
        production: &[],
    },
    // 48 - Integer: /\d[\d_]*/;
    Production {
        lhs: 23,
        production: &[ParseType::T(5)],
    },
    // 49 - Boolean: True;
    Production {
        lhs: 12,
        production: &[ParseType::N(48)],
    },
    // 50 - Boolean: False;
    Production {
        lhs: 12,
        production: &[ParseType::N(19)],
    },
    // 51 - True: 'true';
    Production {
        lhs: 48,
        production: &[ParseType::T(6)],
    },
    // 52 - False: 'false';
    Production {
        lhs: 19,
        production: &[ParseType::T(7)],
    },
    // 53 - Null: 'null';
    Production {
        lhs: 30,
        production: &[ParseType::T(8)],
    },
    // 54 - Hole: '!';
    Production {
        lhs: 20,
        production: &[ParseType::T(9)],
    },
    // 55 - StringContinues: String StringContinuesList /* Vec */;
    Production {
        lhs: 39,
        production: &[ParseType::N(40), ParseType::N(38)],
    },
    // 56 - StringContinuesList: Continue String StringContinuesList;
    Production {
        lhs: 40,
        production: &[ParseType::N(40), ParseType::N(38), ParseType::N(14)],
    },
    // 57 - StringContinuesList: ;
    Production {
        lhs: 40,
        production: &[],
    },
    // 58 - String: Quote InString Quote;
    Production {
        lhs: 38,
        production: &[ParseType::N(34), ParseType::N(22), ParseType::N(34)],
    },
    // 59 - TypedString: TypedQuote InString Quote;
    Production {
        lhs: 50,
        production: &[ParseType::N(34), ParseType::N(22), ParseType::N(49)],
    },
    // 60 - Quote: '"';
    Production {
        lhs: 34,
        production: &[ParseType::T(10)],
    },
    // 61 - TypedQuote: /\p{XID_Start}\p{XID_Continue}*"/;
    Production {
        lhs: 49,
        production: &[ParseType::T(11)],
    },
    // 62 - InString: /(\\[nrt\\"0]|\p{Letter}|\p{Mark}|\p{Number}|[\p{Punctuation}--\\"]|\p{Symbol}|\p{Space_Separator})*/;
    Production {
        lhs: 22,
        production: &[ParseType::T(12)],
    },
    // 63 - Text: /(\p{Letter}|\p{Mark}|\p{Number}|\p{Punctuation}|\p{Symbol}|\p{Space_Separator})*/;
    Production {
        lhs: 44,
        production: &[ParseType::T(13)],
    },
    // 64 - Newline: /\r\n|\r|\n/;
    Production {
        lhs: 29,
        production: &[ParseType::T(14)],
    },
    // 65 - Ws: /[\s--\r\n]+/;
    Production {
        lhs: 53,
        production: &[ParseType::T(15)],
    },
    // 66 - At: '@';
    Production {
        lhs: 7,
        production: &[ParseType::T(16)],
    },
    // 67 - Ext: '$';
    Production {
        lhs: 17,
        production: &[ParseType::T(17)],
    },
    // 68 - Dot: '.';
    Production {
        lhs: 15,
        production: &[ParseType::T(18)],
    },
    // 69 - Begin: '{';
    Production {
        lhs: 8,
        production: &[ParseType::T(19)],
    },
    // 70 - End: '}';
    Production {
        lhs: 16,
        production: &[ParseType::T(20)],
    },
    // 71 - ArrayBegin: '[';
    Production {
        lhs: 1,
        production: &[ParseType::T(21)],
    },
    // 72 - ArrayEnd: ']';
    Production {
        lhs: 2,
        production: &[ParseType::T(22)],
    },
    // 73 - Bind: '=';
    Production {
        lhs: 9,
        production: &[ParseType::T(23)],
    },
    // 74 - Comma: ',';
    Production {
        lhs: 13,
        production: &[ParseType::T(24)],
    },
    // 75 - Continue: '\\';
    Production {
        lhs: 14,
        production: &[ParseType::T(25)],
    },
    // 76 - TextStart: ":";
    Production {
        lhs: 47,
        production: &[ParseType::T(26)],
    },
    // 77 - Ident: /\p{XID_Start}\p{XID_Continue}*/;
    Production {
        lhs: 21,
        production: &[ParseType::T(27)],
    },
];

static SCANNERS: Lazy<Vec<ScannerConfig>> = Lazy::new(|| {
    vec![
        ScannerConfig::new(
            "INITIAL",
            Tokenizer::build(TERMINALS, SCANNER_0.0, SCANNER_0.1).unwrap(),
            &[
                (10 /* Quote */, 1 /* String */),
                (11 /* TypedQuote */, 1 /* String */),
                (26 /* TextStart */, 2 /* Text */),
            ],
        ),
        ScannerConfig::new(
            "String",
            Tokenizer::build(TERMINALS, SCANNER_1.0, SCANNER_1.1).unwrap(),
            &[(10 /* Quote */, 0 /* INITIAL */)],
        ),
        ScannerConfig::new(
            "Text",
            Tokenizer::build(TERMINALS, SCANNER_2.0, SCANNER_2.1).unwrap(),
            &[(14 /* Newline */, 0 /* INITIAL */)],
        ),
    ]
});

pub fn parse<'t, T>(
    input: &'t str,
    file_name: T,
    user_actions: &mut Grammar<'t>,
) -> Result<ParseTree<'t>, ParolError>
where
    T: AsRef<Path>,
{
    let mut llk_parser = LLKParser::new(
        41,
        LOOKAHEAD_AUTOMATA,
        PRODUCTIONS,
        TERMINAL_NAMES,
        NON_TERMINALS,
    );
    llk_parser.trim_parse_tree();

    // Initialize wrapper
    let mut user_actions = GrammarAuto::new(user_actions);
    llk_parser.parse(
        TokenStream::new(input, file_name, &SCANNERS, MAX_K).unwrap(),
        &mut user_actions,
    )
}
