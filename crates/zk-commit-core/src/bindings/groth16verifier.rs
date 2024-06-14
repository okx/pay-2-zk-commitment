pub use groth_16_verifier::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod groth_16_verifier {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([(
                ::std::borrow::ToOwned::to_owned("verifyProof"),
                ::std::vec![::ethers::core::abi::ethabi::Function {
                    name: ::std::borrow::ToOwned::to_owned("verifyProof"),
                    inputs: ::std::vec![
                        ::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_pA"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ),
                                2usize,
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256[2]"),
                            ),
                        },
                        ::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_pB"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                        2usize,
                                    ),
                                ),
                                2usize,
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256[2][2]"),
                            ),
                        },
                        ::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_pC"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ),
                                2usize,
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256[2]"),
                            ),
                        },
                        ::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_pubSignals"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ),
                                31usize,
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256[31]"),
                            ),
                        },
                    ],
                    outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                        name: ::std::string::String::new(),
                        kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("bool"),
                        ),
                    },],
                    constant: ::core::option::Option::None,
                    state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                },],
            )]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static GROTH16VERIFIER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x11\"\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80cD\xA1s\xDD\x14a\x000W[`\0\x80\xFD[a\0J`\x04\x806\x03\x81\x01\x90a\0E\x91\x90a\x10MV[a\0`V[`@Qa\0W\x91\x90a\x10\xD1V[`@Q\x80\x91\x03\x90\xF3[`\0a\x0E\x03V[\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x81\x10a\0\x98W`\0\x80R` `\0\xF3[PV[`\0`@Q\x83\x81R\x84` \x82\x01R\x85`@\x82\x01R`@\x81``\x83`\x07a\x07\xD0Z\x03\xFA\x91P\x81a\0\xCEW`\0\x80R` `\0\xF3[\x82Q`@\x82\x01R` \x83\x01Q``\x82\x01R`@\x83`\x80\x83`\x06a\x07\xD0Z\x03\xFA\x91P\x81a\0\xFEW`\0\x80R` `\0\xF3[PPPPPPV[`\0`\x80\x86\x01`\0\x87\x01\x7F\x17\x92\xC1\xCD\xB7\xD7\xB9\xCB\xFA\xBD{\xB6\x12\xD6\x80\xD0\x83$\xBDWe\xD8\xCC\xD0\xC34\xC1\x9Fw;\xE4\x08\x81R\x7F\x0B)\x90]\xD0\xD0\xF5\xA8\x036`\xD4\x86h\xD5\xF4~w\x1CGU}\xEDME \x17O\xE3\xF3*\x04` \x82\x01Ra\x01\xA9`\0\x88\x015\x7F,\xC1\xCE\xB1\xBA\x81\x90~\xA5L\xB2\xBD\x814\xBA\x7F\xB9^z18\x98\xA0\\m\xA2\x14\x03~\xF3\x96\xC1\x7F\x17\x81I-\x93q6M\xDB[K\xFD\"K\xE0g\xA5\x86\xBE,T\0\xF1\x7F\xA4\xD1\x07\xCC^\n\x01\xE1\x84a\0\x9BV[a\x01\xF9` \x88\x015\x7F!&k\xAD\x08\xA7@\x8DN4\xB9\xE5\xDE\xAD-D\x81\xB0\x97\xD02\xE5OJ8@R\x90\x89>j\xCF\x7F\x01\xF2#\"+C\xEA\xC8\xC81b\xBF:P\x80\r\x95&\xDF3\xDE\xA2r\x11\xEE\x82\r\xBFh_t8\x84a\0\x9BV[a\x02I`@\x88\x015\x7F*\xE3\x9B\xA7U\xF1v\xDA\x02WK\x15\x11}Y\x9CS\xA5\xCF\xFC\xA3\xD9\xDA}\xF3\n\xA2\x19\xD3B'l\x7F\x1D\xAB\x03z\xDE@\x07s\xD2~\x82\xA4\xC3\x88!\xE5\x93\x12;\xD1\x86\x01_=\x82\xF1\x8A\x15\x8C\xE0\x92\x8B\x84a\0\x9BV[a\x02\x99``\x88\x015\x7F\x0B\x0Ek\xDF\x8Cb$\x95}z>\xE0\xB1\xD8\xAF\xDC\x95\xE16#\xB5B\xA7\xBFc\xFC<\xB2\x86\xFB#\xCF\x7F\x12\xF8\xCE\xC4\xEEN\xB2\"\x80\xEC\x91H\xC9ak\xE8\x8D\"\xC7f\xA1\xF2M\x18\xC9\xD6\xF2\xF3$\xBD\xAF*\x84a\0\x9BV[a\x02\xE9`\x80\x88\x015\x7F,\x81\x82\x10\x8B\xB6\xAA\x8A\xD8\xD7p\xC8\x05\xC5PD\xF3}.d\r\xBC\xB8\xB4\xDD\xFE\xCF\xE9k\xE1(\xFC\x7F\x05\xDA:\xD1\xF1\x89\xB1\xE8\xBE\xEB\xD7\x19\x91\x08Ib\xAB\xA6\xFF\xF6\x90BE\xA4\xE7\xD2\x0C\x16\x08A\xAF\xFF\x84a\0\x9BV[a\x039`\xA0\x88\x015\x7F\x16\xB9\xB4\xD04\x9C2\x87\xDEW8z\x9D\x9F\x9B\xACp\x81$\xD1\x0C\xCF#R\xFF\xC2]Vs\x84\x815\x7F\x0C\x94^ \xE5[\xD5Ef\x9A;(/\x8D\xF7\x8F\0\xC1\xA5\xCF\x04+\x11\xA2.\x04'\xBF\x94\xC6\xE6w\x84a\0\x9BV[a\x03\x89`\xC0\x88\x015\x7F\x13[\xF0;gsv9<\xE1\xD9\x86S\x0B~\x14[\r x\xCE\xFCw\xAA*\x14\x0Bq-K\x1C\x0E\x7F.\0\x1F]\xB7\x9D`k\xA4Y\xF0j\x9A\xD2\xE0\xE7T_\xE8)\xD7\xCD\nk\x87\xAF\xE5?\x02\xF2\x1Fj\x84a\0\x9BV[a\x03\xD9`\xE0\x88\x015\x7F\x01\xB6\xBC\x8D\x8E\x8F\xE6\x0B\xBCv\xBFH\xB7\xC0\x1C/\xF4\x83Or\x83\x1Ch2[E\xBF.\x8A\xF5\xE3\xA0\x7F\x18*\x7F\x12o\xCB\xB5\xC7\x9E\xFA\xA4\xF9\xB2\x83\x91\xE5aV\xEA\xD41,;\x98\xA8|Xl\x94\x9Aw<\x84a\0\x9BV[a\x04*a\x01\0\x88\x015\x7F\x05t\xA0~\xE8\x15<C\x10C?#\x8C\xFE\xD7\xACv[\xEFMG\xAA\xAC\xD1\x9AfaJN\xD0Q\xD5\x7F\x176\xA6\xEC\xF7k\xFC\x0C\xD6\x8E\xABD\xB9\"DT\xDA\xF0\x89\xDA\xA7\xBCHp\x17eI\x81\x80\xDC(\xE9\x84a\0\x9BV[a\x04{a\x01 \x88\x015\x7F\x05\x89\x9B\xC9!\x08h`p\xA3\r\xB9\xFD\xA8\xE6\xAD@\xD2h\x10`#\xB82&\xBC\"\x122\xB5`\n\x7F\x197\xBDL\x98\xD0\x9A\xB4`\x13{\x1E>\xB2Q6\x17\x8E\x86\x8D\xD9\xA9\xB9}M>,<Q[\xCCH\x84a\0\x9BV[a\x04\xCCa\x01@\x88\x015\x7F\x02\x99\x7F\x8Ek~ \xA3U\xE60\xFF<\x9D\x9A\xA8\xD7\x17\xD9\xFF\xAA\xDC\x91\xD1\x87h\x8Bj\xDE\xF9u\xF5\x7F\x0Cd\x18.t8%k\xC3\xE8\xF8\x85\x02*N\x7F\"\xF9z3|\xE0\xE4;\xDD6\x9F\r2\xB8.\xDF\x84a\0\x9BV[a\x05\x1Da\x01`\x88\x015\x7F\x16&%9/\xE6~q\xFB?~_\x04z\xCC\xD3\xF7\xD9\x98\xDCxn-\x1A\x88\x01\x06`\x90\xCBM\xA8\x7F\x1E\xE5IM\x13I{A\x1D\xCAH\xDCadz\xCA\x85\xB5b{\x16\xC2\x92j\xD0M\xFE\xD6\xC2\xBF\x07l\x84a\0\x9BV[a\x05na\x01\x80\x88\x015\x7F\x1D\"\x02UK/\xC4\xCF\xA19\x92\xE5r\xE4\xC3{H\xEB\xA8Z\x03.\x9Cm\x1Ej\xAE\xE0\xCA\xD6\xDD\xB6\x7F\x18\x1D\x9FR\xBEk\xC21\t\x9E\x17\x0B\x11\xBA\xA5\xEA\xF5N7\x93\x08\x02\x84$_j\xB2\x85$\xD3\xC1\x02\x84a\0\x9BV[a\x05\xBFa\x01\xA0\x88\x015\x7F\x01ps\xB8\x102\xCD\xBB\xD0\xE2\xF0\\\xAA\xDB[d=\xBCn\xEC\xB6Z\x9C\xC5\xED\x8C\xA8\xAE\x19\xE5u\x8E\x7F\x19.!\xD66\xD2\xB0\xE6\x9D!\xF6\x0E\xBE\x03&\xC2f\xB9q\xCF\x8FE\x17\"\xF4/\xFAx&\x0Eke\x84a\0\x9BV[a\x06\x10a\x01\xC0\x88\x015\x7F#[\x12iR1\xC3\x9F\xFDS \x11\tP\xD6k \x9D\xD9\x11\x93\xA3i\x02@\xEF\x1FiE\x8DM\xFE\x7F)\xC9=\xF7\xD2!\x14\xFE\x90\xB8\xCB\x902\xE6Xr\x85\x8F\xE0\x10\xA6g\x83<\x17\x96\x922\xA5\xAA+\xAB\x84a\0\x9BV[a\x06aa\x01\xE0\x88\x015\x7F\x1F\xE1\x12\x96\xD6\xAC\xE8\xFA\x8A\x95/}\x19\xF6\x84\xB4\xA9\r\x817\x1A\x05?\rTq\x14\x05jhM\x15\x7F)Q\t\x01\xFB\x95\xBE\x12f\xB2\x0Fh^\x89$e\xACE9@\xBE).\xAD\x1B`3}\x9AO^\x02\x84a\0\x9BV[a\x06\xB2a\x02\0\x88\x015\x7F\x18ZH\xAF\x9E\x9Cw*\xB6t\x14,\x7F\xA8;~=i\xC8\xFB\xC5\xE4\x84/\xDB4\xB4\r\xDB\x9B\xDB\xFD\x7F%\xA8\0\x1C\xF8\xD8\xC4{<\xCB6\x91U|\xD9\tU\xD1\xD3\\1\xCAD\x92\xF3\xA3\xD6\xDE\xD1\xE8\xE7\0\x84a\0\x9BV[a\x07\x03a\x02 \x88\x015\x7F\x05O\x9E\x98cl_\x8B\r\xD1\x15\xBD^\x9A\x8E}&\xB7\xFD\xB7\x0E\x1C\xE2\x97\x0Efl\xED\x90\x1C\xA3\xF6\x7F\x0E\xD2t+f\xB2\xF3\xBEm\xB0\x7F\x8D\x91q\x8C\xE7\nO\x12 \x8D\x91\x10@\x9E\xC6\xAE\x8E\x84K,/\x84a\0\x9BV[a\x07Ta\x02@\x88\x015\x7F$`YM\xF0\xAC\x17\xE1=\xE7*\x15\\\xE3y&\xD1\x1C-\xF7S\xD8FMnT4\xE8\xFB&s\xCE\x7F\x05\xB7\n'\xED\xAC\x15e\xD1\x89\xC4e`\x9E\xCE30\xDA\xCB(\xE9)iE\xA3x\xF0|=Ph\x91\x84a\0\x9BV[a\x07\xA5a\x02`\x88\x015\x7F+\x1Bo7xo\xF3\xBC\xBE\xEA\xDBrah\x19Bh\x98`\xD3\x1E.\xB06\xEC\xB7\xE0\xBE\x07b\xBA-\x7F\x0C\x81\xAF\xC8\\\xDA.l'hv=N\x86\xA2\xC6Z\x0F\x1A\x03\x98\x05/\x11b\xD2\x19\xA9\xFB\xD9Q\xA5\x84a\0\x9BV[a\x07\xF6a\x02\x80\x88\x015\x7F\x142\x9E\xE3^\xAC\x92DZ\xDB\xFF\xBC\xD2\x1E\xF5/k\x08\xF9]Y\x03\xC8Ak\x9D\x86(l\xA3g\xE4\x7F\x07O\x03J\xE2m\xBC\xDA\xE0\x81.\xE1\x9A\xCF\xCB\xF7\x1A\xF0\x97/\x94\x92[\xB6\n\xD2U\xF2\r\xFB4>\x84a\0\x9BV[a\x08Ga\x02\xA0\x88\x015\x7F-\xDC\x86\xEC\xFD\x1A\x9E\xB7\x19@\x0C\x86\xF7\x87\x8D6Zb\xA0\x07\xA8J@AQ\xC3\x96\x86\x7F<\xD4\x11\x7F\x17\x82\xE9\xF1\xAB\xCF\xD5\x1B\t\xC6\xEC\x926\xBA_\xB1\x0B\x1A\xB7\xC5\x8FP\x82\xF2\xD9\xF9\xD3\xF0\x1E%\x14C\x84a\0\x9BV[a\x08\x98a\x02\xC0\x88\x015\x7F\x1C?\xDBtP\xBD7\x8BMI\x03.v\xDD'\xE3@\xDB\x05\x8AA\r9+>\xC4B\xBACl\xE2\x19\x7F\"\xF0\xB6Q\xE4\xE9;+aR\xB8\x80\xA9\xDEp\xBF\x1C8\x80\xB6G\xFC\xF2\xFC\xF0\x87\x92K_\x1A\x98\xED\x84a\0\x9BV[a\x08\xE9a\x02\xE0\x88\x015\x7F\x1A\xC3\xA2\xBEL\xD1k\xD2f\xCB\xDB\x88\x9Ed\x864'S'\"\xD5\x15\xC4\xF3\x1D\x9E\x08\x99\xD8\x1F\x94\x9F\x7F.\xBBH\xCE\xBC:m\xE3{V[\xCE\xE6Q\x1B\xD8B\x86\xFFK#@\xDB5?\xFA\xF3\xEA\x8D\x9C\x11\x9D\x84a\0\x9BV[a\t:a\x03\0\x88\x015\x7F\x1E\xA6\xAE$\x14\x93(R\xCA\xE6\x9C\x89J\xAD[i\xCD\xA9\xD8\xB1{@>\x04\x0E\xCD\\\xEB\xCDR\xEAP\x7F\r\xF4h0 \x11\x94\xAE\xDE#*\x08\xF8\xA0\x1E\xB9\xF0o\xF7\xE1V:\xC5\x16\xB3\xB0\x8D\xC4\x96U2:\x84a\0\x9BV[a\t\x8Ba\x03 \x88\x015\x7F'\x95\xEC\x15\xE4\xDD\xF1#q\xE8\xE5\x02U\xDC\xC6\xE3^RC5\xA4J\x901\xF1h\x13C\xA3\x9Ce\xA1\x7F\x0E\xF2\x17\x13\xFE\xAC\xAFW\xEF\xA9\xE8v9\x8E\x9D\xBCN\xA4\xC5\xFEf\x94\xD33\x16\xA4-\x92f\x07*\xAF\x84a\0\x9BV[a\t\xDBa\x03@\x88\x015\x7F\r\x05s\xA7\x16E-\xC8\x86$-\x85SF\xC0\x0B\xCC\xEEc\xC1\xE6\xC0\x06q\xCA\xE2\x9D\x10\x16\x9E\xCDQ~HG\xF6\x85,^E\xBB\x8CB\xA8-?_$'\xB9-\xAE\xE6y\xD5\x17\x91i}2\xC2\x16\x86\x9A\x84a\0\x9BV[a\n,a\x03`\x88\x015\x7F$\x93\xD3\x96\xF1M\xE0!\x97M \xE0}\xFC\xDC\r$3\xD2\x14\xDEb\x07cr\xF1\xA4\x95B_O\xF3\x7F\x1E\x9D3\x0C\x8D\x99$\xD0w/\x91\xD0q\xC3\x11\xC8\x1F\0\x8Ct\r\xAB#\xDB\xC8\xD0\x8E\x1F\x9D\x89\x19\xDF\x84a\0\x9BV[a\n}a\x03\x80\x88\x015\x7F\x0C\x03\x86\xCBo+\xC9\xD9\xD2 \xD2\x02U\xD8Xg\xEB\x89\xE7\x8De\x96\x03\x0E@\xB8\xFAN\xE9\xF2\xDCA\x7F\x10\x06\xFE{\xF5D\xD2^Vns\xA8\xA4#\xBD\x0B`xQ\x19w\xF7\x16\xD9\x8F\x15\xAD0\xBB\xF0\x0E\xC1\x84a\0\x9BV[a\n\xCEa\x03\xA0\x88\x015\x7F\r\x98^B\x87=\xCD\xDF\x1A\x82\xAF\x0B\x8C\xB6Z*\xB1\xCF~G\xB6e\x97\x0B\x8E\xC0&\t\xC2\xC6\xF2\x90\x7F0ZWge\x99`\x0F\xE29=\xFEon\x89\xE0#j9\xCC\xED\rA\x9D\x8A\xEE\xD3~\xFD\x91d\x0F\x84a\0\x9BV[a\x0B\x1Fa\x03\xC0\x88\x015\x7F/\xE4]\x8AY\xFA7\x0B|E\x19\x89\xDDH\xFB\xA2\xDC\xE1I\xE0v\xDF\xC3\x81|\x0E\x86\x887\xA6\xD2-\x7F\x1B\xAC\nD\xBB\xB0\xC3\xE0;\xF1-a\x86\x1C\xEE\x15.V\xF5+\x9A\xB45\xA0\x88[\xC7[V\xD0\xDA\xD7\x84a\0\x9BV[\x835\x82R\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG` \x85\x015\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x03\x06` \x83\x01R\x845`@\x83\x01R` \x85\x015``\x83\x01R`@\x85\x015`\x80\x83\x01R``\x85\x015`\xA0\x83\x01R\x7F-M\x9A\xA7\xE3\x02\xD9\xDFAt\x9DU\x07\x94\x9D\x05\xDB\xEA3\xFB\xB1ld;\"\xF5\x99\xA2\xBEm\xF2\xE2`\xC0\x83\x01R\x7F\x14\xBE\xDDP<7\xCE\xB0a\xD8\xEC` \x9F\xE3E\xCE\x89\x83\n\x19#\x03\x01\xF0v\xCA\xFF\0M\x19&`\xE0\x83\x01R\x7F\tg\x03/\xCB\xF7v\xD1\xAF\xC9\x85\xF8\x88w\xF1\x82\xD3\x84\x80\xA6S\xF2\xDE\xCA\xA9yL\xBC;\xF3\x06\x0Ca\x01\0\x83\x01R\x7F\x0E\x18xG\xADLy\x83t\xD0\xD6s+\xF5\x01\x84}\xD6\x8B\xC0\xE0q$\x1E\x02\x13\xBC\x7F\xC1=\xB7\xABa\x01 \x83\x01R\x7F0L\xFB\xD1\xE0\x8ApJ\x99\xF5\xE8G\xD9?\x8C<\xAA\xFD\xDE\xC4kz\r7\x9D\xA6\x9AM\x11#F\xA7a\x01@\x83\x01R\x7F\x179\xC1\xB1\xA4W\xA8\xC711#\xD2M/\x91\x92\xF8\x96\xB7\xC6>\xEA\x05\xA9\xD5\x7F\x06Tz\xD0\xCE\xC8a\x01`\x83\x01R`\0\x88\x01Qa\x01\x80\x83\x01R` `\0\x01\x88\x01Qa\x01\xA0\x83\x01R\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2a\x01\xC0\x83\x01R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xEDa\x01\xE0\x83\x01R\x7F\t\x06\x89\xD0X_\xF0u\xEC\x9E\x99\xADi\x0C3\x95\xBCK13p\xB3\x8E\xF3U\xAC\xDA\xDC\xD1\"\x97[a\x02\0\x83\x01R\x7F\x12\xC8^\xA5\xDB\x8Cm\xEBJ\xABq\x80\x8D\xCB@\x8F\xE3\xD1\xE7i\x0CC\xD3{L\xE6\xCC\x01f\xFA}\xAAa\x02 \x83\x01R\x855a\x02@\x83\x01R` \x86\x015a\x02`\x83\x01R\x7F\x18\x0C\xCEl\xAE#\xBD\xAARc\x0C\x1E>r$V\0\"7\xDF\xAD=\xA2\x80:s\x18^;\xE2O@a\x02\x80\x83\x01R\x7F\x01\x0B\xAB\xC7\t\xF9\xFD\xF2\xA1\x97\xD2\x0E\xC5\xD2\xED\x18v\xA2\xE8\xA0\x88\xFE\xE4\x07&\xB6\x1Dy8h\x90ca\x02\xA0\x83\x01R\x7F\x16\xCA?\xDDQ\x9C\x86D\x08r,\xD2_\x90\xDF>xi@t\xEEz\x1E\xD5\x81\xAF*\x10\x03/\xD7\xB3a\x02\xC0\x83\x01R\x7F\rBt\xF8R\xB0\xC1\xAA\xD2\xF6:GC\xE5\xD22\xA8d\xE1\x83\xB7\xC4\xAC\xE9\x1F\xC3:D\xD5W\xEDwa\x02\xE0\x83\x01R` \x82a\x03\0\x84`\x08a\x07\xD0Z\x03\xFA\x82Q\x81\x16\x93PPPP\x95\x94PPPPPV[`@Qa\x03\x80\x81\x01`@Ra\x0E\x1B`\0\x84\x015a\0gV[a\x0E(` \x84\x015a\0gV[a\x0E5`@\x84\x015a\0gV[a\x0EB``\x84\x015a\0gV[a\x0EO`\x80\x84\x015a\0gV[a\x0E\\`\xA0\x84\x015a\0gV[a\x0Ei`\xC0\x84\x015a\0gV[a\x0Ev`\xE0\x84\x015a\0gV[a\x0E\x84a\x01\0\x84\x015a\0gV[a\x0E\x92a\x01 \x84\x015a\0gV[a\x0E\xA0a\x01@\x84\x015a\0gV[a\x0E\xAEa\x01`\x84\x015a\0gV[a\x0E\xBCa\x01\x80\x84\x015a\0gV[a\x0E\xCAa\x01\xA0\x84\x015a\0gV[a\x0E\xD8a\x01\xC0\x84\x015a\0gV[a\x0E\xE6a\x01\xE0\x84\x015a\0gV[a\x0E\xF4a\x02\0\x84\x015a\0gV[a\x0F\x02a\x02 \x84\x015a\0gV[a\x0F\x10a\x02@\x84\x015a\0gV[a\x0F\x1Ea\x02`\x84\x015a\0gV[a\x0F,a\x02\x80\x84\x015a\0gV[a\x0F:a\x02\xA0\x84\x015a\0gV[a\x0FHa\x02\xC0\x84\x015a\0gV[a\x0FVa\x02\xE0\x84\x015a\0gV[a\x0Fda\x03\0\x84\x015a\0gV[a\x0Fra\x03 \x84\x015a\0gV[a\x0F\x80a\x03@\x84\x015a\0gV[a\x0F\x8Ea\x03`\x84\x015a\0gV[a\x0F\x9Ca\x03\x80\x84\x015a\0gV[a\x0F\xAAa\x03\xA0\x84\x015a\0gV[a\x0F\xB8a\x03\xC0\x84\x015a\0gV[a\x0F\xC6a\x03\xE0\x84\x015a\0gV[a\x0F\xD3\x81\x84\x86\x88\x8Aa\x01\x06V[\x80`\0R` `\0\xF3[`\0\x80\xFD[`\0\x80\xFD[`\0\x81\x90P\x82` `\x02\x02\x82\x01\x11\x15a\x10\x03Wa\x10\x02a\x0F\xE2V[[\x92\x91PPV[`\0\x81\x90P\x82`@`\x02\x02\x82\x01\x11\x15a\x10%Wa\x10$a\x0F\xE2V[[\x92\x91PPV[`\0\x81\x90P\x82` `\x1F\x02\x82\x01\x11\x15a\x10GWa\x10Fa\x0F\xE2V[[\x92\x91PPV[`\0\x80`\0\x80a\x04\xE0\x85\x87\x03\x12\x15a\x10hWa\x10ga\x0F\xDDV[[`\0a\x10v\x87\x82\x88\x01a\x0F\xE7V[\x94PP`@a\x10\x87\x87\x82\x88\x01a\x10\tV[\x93PP`\xC0a\x10\x98\x87\x82\x88\x01a\x0F\xE7V[\x92PPa\x01\0a\x10\xAA\x87\x82\x88\x01a\x10+V[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x81\x15\x15\x90P\x91\x90PV[a\x10\xCB\x81a\x10\xB6V[\x82RPPV[`\0` \x82\x01\x90Pa\x10\xE6`\0\x83\x01\x84a\x10\xC2V[\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xD0\xEE-\x10yq\x90\xB4 U_#l4\x1E\xD73\xD1\x1D\x8AM\"\xD4\x9F\xA6&\x05s\x81\xA6pAdsolcC\0\x08\x18\x003";
    /// The bytecode of the contract.
    pub static GROTH16VERIFIER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80cD\xA1s\xDD\x14a\x000W[`\0\x80\xFD[a\0J`\x04\x806\x03\x81\x01\x90a\0E\x91\x90a\x10MV[a\0`V[`@Qa\0W\x91\x90a\x10\xD1V[`@Q\x80\x91\x03\x90\xF3[`\0a\x0E\x03V[\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x81\x10a\0\x98W`\0\x80R` `\0\xF3[PV[`\0`@Q\x83\x81R\x84` \x82\x01R\x85`@\x82\x01R`@\x81``\x83`\x07a\x07\xD0Z\x03\xFA\x91P\x81a\0\xCEW`\0\x80R` `\0\xF3[\x82Q`@\x82\x01R` \x83\x01Q``\x82\x01R`@\x83`\x80\x83`\x06a\x07\xD0Z\x03\xFA\x91P\x81a\0\xFEW`\0\x80R` `\0\xF3[PPPPPPV[`\0`\x80\x86\x01`\0\x87\x01\x7F\x17\x92\xC1\xCD\xB7\xD7\xB9\xCB\xFA\xBD{\xB6\x12\xD6\x80\xD0\x83$\xBDWe\xD8\xCC\xD0\xC34\xC1\x9Fw;\xE4\x08\x81R\x7F\x0B)\x90]\xD0\xD0\xF5\xA8\x036`\xD4\x86h\xD5\xF4~w\x1CGU}\xEDME \x17O\xE3\xF3*\x04` \x82\x01Ra\x01\xA9`\0\x88\x015\x7F,\xC1\xCE\xB1\xBA\x81\x90~\xA5L\xB2\xBD\x814\xBA\x7F\xB9^z18\x98\xA0\\m\xA2\x14\x03~\xF3\x96\xC1\x7F\x17\x81I-\x93q6M\xDB[K\xFD\"K\xE0g\xA5\x86\xBE,T\0\xF1\x7F\xA4\xD1\x07\xCC^\n\x01\xE1\x84a\0\x9BV[a\x01\xF9` \x88\x015\x7F!&k\xAD\x08\xA7@\x8DN4\xB9\xE5\xDE\xAD-D\x81\xB0\x97\xD02\xE5OJ8@R\x90\x89>j\xCF\x7F\x01\xF2#\"+C\xEA\xC8\xC81b\xBF:P\x80\r\x95&\xDF3\xDE\xA2r\x11\xEE\x82\r\xBFh_t8\x84a\0\x9BV[a\x02I`@\x88\x015\x7F*\xE3\x9B\xA7U\xF1v\xDA\x02WK\x15\x11}Y\x9CS\xA5\xCF\xFC\xA3\xD9\xDA}\xF3\n\xA2\x19\xD3B'l\x7F\x1D\xAB\x03z\xDE@\x07s\xD2~\x82\xA4\xC3\x88!\xE5\x93\x12;\xD1\x86\x01_=\x82\xF1\x8A\x15\x8C\xE0\x92\x8B\x84a\0\x9BV[a\x02\x99``\x88\x015\x7F\x0B\x0Ek\xDF\x8Cb$\x95}z>\xE0\xB1\xD8\xAF\xDC\x95\xE16#\xB5B\xA7\xBFc\xFC<\xB2\x86\xFB#\xCF\x7F\x12\xF8\xCE\xC4\xEEN\xB2\"\x80\xEC\x91H\xC9ak\xE8\x8D\"\xC7f\xA1\xF2M\x18\xC9\xD6\xF2\xF3$\xBD\xAF*\x84a\0\x9BV[a\x02\xE9`\x80\x88\x015\x7F,\x81\x82\x10\x8B\xB6\xAA\x8A\xD8\xD7p\xC8\x05\xC5PD\xF3}.d\r\xBC\xB8\xB4\xDD\xFE\xCF\xE9k\xE1(\xFC\x7F\x05\xDA:\xD1\xF1\x89\xB1\xE8\xBE\xEB\xD7\x19\x91\x08Ib\xAB\xA6\xFF\xF6\x90BE\xA4\xE7\xD2\x0C\x16\x08A\xAF\xFF\x84a\0\x9BV[a\x039`\xA0\x88\x015\x7F\x16\xB9\xB4\xD04\x9C2\x87\xDEW8z\x9D\x9F\x9B\xACp\x81$\xD1\x0C\xCF#R\xFF\xC2]Vs\x84\x815\x7F\x0C\x94^ \xE5[\xD5Ef\x9A;(/\x8D\xF7\x8F\0\xC1\xA5\xCF\x04+\x11\xA2.\x04'\xBF\x94\xC6\xE6w\x84a\0\x9BV[a\x03\x89`\xC0\x88\x015\x7F\x13[\xF0;gsv9<\xE1\xD9\x86S\x0B~\x14[\r x\xCE\xFCw\xAA*\x14\x0Bq-K\x1C\x0E\x7F.\0\x1F]\xB7\x9D`k\xA4Y\xF0j\x9A\xD2\xE0\xE7T_\xE8)\xD7\xCD\nk\x87\xAF\xE5?\x02\xF2\x1Fj\x84a\0\x9BV[a\x03\xD9`\xE0\x88\x015\x7F\x01\xB6\xBC\x8D\x8E\x8F\xE6\x0B\xBCv\xBFH\xB7\xC0\x1C/\xF4\x83Or\x83\x1Ch2[E\xBF.\x8A\xF5\xE3\xA0\x7F\x18*\x7F\x12o\xCB\xB5\xC7\x9E\xFA\xA4\xF9\xB2\x83\x91\xE5aV\xEA\xD41,;\x98\xA8|Xl\x94\x9Aw<\x84a\0\x9BV[a\x04*a\x01\0\x88\x015\x7F\x05t\xA0~\xE8\x15<C\x10C?#\x8C\xFE\xD7\xACv[\xEFMG\xAA\xAC\xD1\x9AfaJN\xD0Q\xD5\x7F\x176\xA6\xEC\xF7k\xFC\x0C\xD6\x8E\xABD\xB9\"DT\xDA\xF0\x89\xDA\xA7\xBCHp\x17eI\x81\x80\xDC(\xE9\x84a\0\x9BV[a\x04{a\x01 \x88\x015\x7F\x05\x89\x9B\xC9!\x08h`p\xA3\r\xB9\xFD\xA8\xE6\xAD@\xD2h\x10`#\xB82&\xBC\"\x122\xB5`\n\x7F\x197\xBDL\x98\xD0\x9A\xB4`\x13{\x1E>\xB2Q6\x17\x8E\x86\x8D\xD9\xA9\xB9}M>,<Q[\xCCH\x84a\0\x9BV[a\x04\xCCa\x01@\x88\x015\x7F\x02\x99\x7F\x8Ek~ \xA3U\xE60\xFF<\x9D\x9A\xA8\xD7\x17\xD9\xFF\xAA\xDC\x91\xD1\x87h\x8Bj\xDE\xF9u\xF5\x7F\x0Cd\x18.t8%k\xC3\xE8\xF8\x85\x02*N\x7F\"\xF9z3|\xE0\xE4;\xDD6\x9F\r2\xB8.\xDF\x84a\0\x9BV[a\x05\x1Da\x01`\x88\x015\x7F\x16&%9/\xE6~q\xFB?~_\x04z\xCC\xD3\xF7\xD9\x98\xDCxn-\x1A\x88\x01\x06`\x90\xCBM\xA8\x7F\x1E\xE5IM\x13I{A\x1D\xCAH\xDCadz\xCA\x85\xB5b{\x16\xC2\x92j\xD0M\xFE\xD6\xC2\xBF\x07l\x84a\0\x9BV[a\x05na\x01\x80\x88\x015\x7F\x1D\"\x02UK/\xC4\xCF\xA19\x92\xE5r\xE4\xC3{H\xEB\xA8Z\x03.\x9Cm\x1Ej\xAE\xE0\xCA\xD6\xDD\xB6\x7F\x18\x1D\x9FR\xBEk\xC21\t\x9E\x17\x0B\x11\xBA\xA5\xEA\xF5N7\x93\x08\x02\x84$_j\xB2\x85$\xD3\xC1\x02\x84a\0\x9BV[a\x05\xBFa\x01\xA0\x88\x015\x7F\x01ps\xB8\x102\xCD\xBB\xD0\xE2\xF0\\\xAA\xDB[d=\xBCn\xEC\xB6Z\x9C\xC5\xED\x8C\xA8\xAE\x19\xE5u\x8E\x7F\x19.!\xD66\xD2\xB0\xE6\x9D!\xF6\x0E\xBE\x03&\xC2f\xB9q\xCF\x8FE\x17\"\xF4/\xFAx&\x0Eke\x84a\0\x9BV[a\x06\x10a\x01\xC0\x88\x015\x7F#[\x12iR1\xC3\x9F\xFDS \x11\tP\xD6k \x9D\xD9\x11\x93\xA3i\x02@\xEF\x1FiE\x8DM\xFE\x7F)\xC9=\xF7\xD2!\x14\xFE\x90\xB8\xCB\x902\xE6Xr\x85\x8F\xE0\x10\xA6g\x83<\x17\x96\x922\xA5\xAA+\xAB\x84a\0\x9BV[a\x06aa\x01\xE0\x88\x015\x7F\x1F\xE1\x12\x96\xD6\xAC\xE8\xFA\x8A\x95/}\x19\xF6\x84\xB4\xA9\r\x817\x1A\x05?\rTq\x14\x05jhM\x15\x7F)Q\t\x01\xFB\x95\xBE\x12f\xB2\x0Fh^\x89$e\xACE9@\xBE).\xAD\x1B`3}\x9AO^\x02\x84a\0\x9BV[a\x06\xB2a\x02\0\x88\x015\x7F\x18ZH\xAF\x9E\x9Cw*\xB6t\x14,\x7F\xA8;~=i\xC8\xFB\xC5\xE4\x84/\xDB4\xB4\r\xDB\x9B\xDB\xFD\x7F%\xA8\0\x1C\xF8\xD8\xC4{<\xCB6\x91U|\xD9\tU\xD1\xD3\\1\xCAD\x92\xF3\xA3\xD6\xDE\xD1\xE8\xE7\0\x84a\0\x9BV[a\x07\x03a\x02 \x88\x015\x7F\x05O\x9E\x98cl_\x8B\r\xD1\x15\xBD^\x9A\x8E}&\xB7\xFD\xB7\x0E\x1C\xE2\x97\x0Efl\xED\x90\x1C\xA3\xF6\x7F\x0E\xD2t+f\xB2\xF3\xBEm\xB0\x7F\x8D\x91q\x8C\xE7\nO\x12 \x8D\x91\x10@\x9E\xC6\xAE\x8E\x84K,/\x84a\0\x9BV[a\x07Ta\x02@\x88\x015\x7F$`YM\xF0\xAC\x17\xE1=\xE7*\x15\\\xE3y&\xD1\x1C-\xF7S\xD8FMnT4\xE8\xFB&s\xCE\x7F\x05\xB7\n'\xED\xAC\x15e\xD1\x89\xC4e`\x9E\xCE30\xDA\xCB(\xE9)iE\xA3x\xF0|=Ph\x91\x84a\0\x9BV[a\x07\xA5a\x02`\x88\x015\x7F+\x1Bo7xo\xF3\xBC\xBE\xEA\xDBrah\x19Bh\x98`\xD3\x1E.\xB06\xEC\xB7\xE0\xBE\x07b\xBA-\x7F\x0C\x81\xAF\xC8\\\xDA.l'hv=N\x86\xA2\xC6Z\x0F\x1A\x03\x98\x05/\x11b\xD2\x19\xA9\xFB\xD9Q\xA5\x84a\0\x9BV[a\x07\xF6a\x02\x80\x88\x015\x7F\x142\x9E\xE3^\xAC\x92DZ\xDB\xFF\xBC\xD2\x1E\xF5/k\x08\xF9]Y\x03\xC8Ak\x9D\x86(l\xA3g\xE4\x7F\x07O\x03J\xE2m\xBC\xDA\xE0\x81.\xE1\x9A\xCF\xCB\xF7\x1A\xF0\x97/\x94\x92[\xB6\n\xD2U\xF2\r\xFB4>\x84a\0\x9BV[a\x08Ga\x02\xA0\x88\x015\x7F-\xDC\x86\xEC\xFD\x1A\x9E\xB7\x19@\x0C\x86\xF7\x87\x8D6Zb\xA0\x07\xA8J@AQ\xC3\x96\x86\x7F<\xD4\x11\x7F\x17\x82\xE9\xF1\xAB\xCF\xD5\x1B\t\xC6\xEC\x926\xBA_\xB1\x0B\x1A\xB7\xC5\x8FP\x82\xF2\xD9\xF9\xD3\xF0\x1E%\x14C\x84a\0\x9BV[a\x08\x98a\x02\xC0\x88\x015\x7F\x1C?\xDBtP\xBD7\x8BMI\x03.v\xDD'\xE3@\xDB\x05\x8AA\r9+>\xC4B\xBACl\xE2\x19\x7F\"\xF0\xB6Q\xE4\xE9;+aR\xB8\x80\xA9\xDEp\xBF\x1C8\x80\xB6G\xFC\xF2\xFC\xF0\x87\x92K_\x1A\x98\xED\x84a\0\x9BV[a\x08\xE9a\x02\xE0\x88\x015\x7F\x1A\xC3\xA2\xBEL\xD1k\xD2f\xCB\xDB\x88\x9Ed\x864'S'\"\xD5\x15\xC4\xF3\x1D\x9E\x08\x99\xD8\x1F\x94\x9F\x7F.\xBBH\xCE\xBC:m\xE3{V[\xCE\xE6Q\x1B\xD8B\x86\xFFK#@\xDB5?\xFA\xF3\xEA\x8D\x9C\x11\x9D\x84a\0\x9BV[a\t:a\x03\0\x88\x015\x7F\x1E\xA6\xAE$\x14\x93(R\xCA\xE6\x9C\x89J\xAD[i\xCD\xA9\xD8\xB1{@>\x04\x0E\xCD\\\xEB\xCDR\xEAP\x7F\r\xF4h0 \x11\x94\xAE\xDE#*\x08\xF8\xA0\x1E\xB9\xF0o\xF7\xE1V:\xC5\x16\xB3\xB0\x8D\xC4\x96U2:\x84a\0\x9BV[a\t\x8Ba\x03 \x88\x015\x7F'\x95\xEC\x15\xE4\xDD\xF1#q\xE8\xE5\x02U\xDC\xC6\xE3^RC5\xA4J\x901\xF1h\x13C\xA3\x9Ce\xA1\x7F\x0E\xF2\x17\x13\xFE\xAC\xAFW\xEF\xA9\xE8v9\x8E\x9D\xBCN\xA4\xC5\xFEf\x94\xD33\x16\xA4-\x92f\x07*\xAF\x84a\0\x9BV[a\t\xDBa\x03@\x88\x015\x7F\r\x05s\xA7\x16E-\xC8\x86$-\x85SF\xC0\x0B\xCC\xEEc\xC1\xE6\xC0\x06q\xCA\xE2\x9D\x10\x16\x9E\xCDQ~HG\xF6\x85,^E\xBB\x8CB\xA8-?_$'\xB9-\xAE\xE6y\xD5\x17\x91i}2\xC2\x16\x86\x9A\x84a\0\x9BV[a\n,a\x03`\x88\x015\x7F$\x93\xD3\x96\xF1M\xE0!\x97M \xE0}\xFC\xDC\r$3\xD2\x14\xDEb\x07cr\xF1\xA4\x95B_O\xF3\x7F\x1E\x9D3\x0C\x8D\x99$\xD0w/\x91\xD0q\xC3\x11\xC8\x1F\0\x8Ct\r\xAB#\xDB\xC8\xD0\x8E\x1F\x9D\x89\x19\xDF\x84a\0\x9BV[a\n}a\x03\x80\x88\x015\x7F\x0C\x03\x86\xCBo+\xC9\xD9\xD2 \xD2\x02U\xD8Xg\xEB\x89\xE7\x8De\x96\x03\x0E@\xB8\xFAN\xE9\xF2\xDCA\x7F\x10\x06\xFE{\xF5D\xD2^Vns\xA8\xA4#\xBD\x0B`xQ\x19w\xF7\x16\xD9\x8F\x15\xAD0\xBB\xF0\x0E\xC1\x84a\0\x9BV[a\n\xCEa\x03\xA0\x88\x015\x7F\r\x98^B\x87=\xCD\xDF\x1A\x82\xAF\x0B\x8C\xB6Z*\xB1\xCF~G\xB6e\x97\x0B\x8E\xC0&\t\xC2\xC6\xF2\x90\x7F0ZWge\x99`\x0F\xE29=\xFEon\x89\xE0#j9\xCC\xED\rA\x9D\x8A\xEE\xD3~\xFD\x91d\x0F\x84a\0\x9BV[a\x0B\x1Fa\x03\xC0\x88\x015\x7F/\xE4]\x8AY\xFA7\x0B|E\x19\x89\xDDH\xFB\xA2\xDC\xE1I\xE0v\xDF\xC3\x81|\x0E\x86\x887\xA6\xD2-\x7F\x1B\xAC\nD\xBB\xB0\xC3\xE0;\xF1-a\x86\x1C\xEE\x15.V\xF5+\x9A\xB45\xA0\x88[\xC7[V\xD0\xDA\xD7\x84a\0\x9BV[\x835\x82R\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG` \x85\x015\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x03\x06` \x83\x01R\x845`@\x83\x01R` \x85\x015``\x83\x01R`@\x85\x015`\x80\x83\x01R``\x85\x015`\xA0\x83\x01R\x7F-M\x9A\xA7\xE3\x02\xD9\xDFAt\x9DU\x07\x94\x9D\x05\xDB\xEA3\xFB\xB1ld;\"\xF5\x99\xA2\xBEm\xF2\xE2`\xC0\x83\x01R\x7F\x14\xBE\xDDP<7\xCE\xB0a\xD8\xEC` \x9F\xE3E\xCE\x89\x83\n\x19#\x03\x01\xF0v\xCA\xFF\0M\x19&`\xE0\x83\x01R\x7F\tg\x03/\xCB\xF7v\xD1\xAF\xC9\x85\xF8\x88w\xF1\x82\xD3\x84\x80\xA6S\xF2\xDE\xCA\xA9yL\xBC;\xF3\x06\x0Ca\x01\0\x83\x01R\x7F\x0E\x18xG\xADLy\x83t\xD0\xD6s+\xF5\x01\x84}\xD6\x8B\xC0\xE0q$\x1E\x02\x13\xBC\x7F\xC1=\xB7\xABa\x01 \x83\x01R\x7F0L\xFB\xD1\xE0\x8ApJ\x99\xF5\xE8G\xD9?\x8C<\xAA\xFD\xDE\xC4kz\r7\x9D\xA6\x9AM\x11#F\xA7a\x01@\x83\x01R\x7F\x179\xC1\xB1\xA4W\xA8\xC711#\xD2M/\x91\x92\xF8\x96\xB7\xC6>\xEA\x05\xA9\xD5\x7F\x06Tz\xD0\xCE\xC8a\x01`\x83\x01R`\0\x88\x01Qa\x01\x80\x83\x01R` `\0\x01\x88\x01Qa\x01\xA0\x83\x01R\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2a\x01\xC0\x83\x01R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xEDa\x01\xE0\x83\x01R\x7F\t\x06\x89\xD0X_\xF0u\xEC\x9E\x99\xADi\x0C3\x95\xBCK13p\xB3\x8E\xF3U\xAC\xDA\xDC\xD1\"\x97[a\x02\0\x83\x01R\x7F\x12\xC8^\xA5\xDB\x8Cm\xEBJ\xABq\x80\x8D\xCB@\x8F\xE3\xD1\xE7i\x0CC\xD3{L\xE6\xCC\x01f\xFA}\xAAa\x02 \x83\x01R\x855a\x02@\x83\x01R` \x86\x015a\x02`\x83\x01R\x7F\x18\x0C\xCEl\xAE#\xBD\xAARc\x0C\x1E>r$V\0\"7\xDF\xAD=\xA2\x80:s\x18^;\xE2O@a\x02\x80\x83\x01R\x7F\x01\x0B\xAB\xC7\t\xF9\xFD\xF2\xA1\x97\xD2\x0E\xC5\xD2\xED\x18v\xA2\xE8\xA0\x88\xFE\xE4\x07&\xB6\x1Dy8h\x90ca\x02\xA0\x83\x01R\x7F\x16\xCA?\xDDQ\x9C\x86D\x08r,\xD2_\x90\xDF>xi@t\xEEz\x1E\xD5\x81\xAF*\x10\x03/\xD7\xB3a\x02\xC0\x83\x01R\x7F\rBt\xF8R\xB0\xC1\xAA\xD2\xF6:GC\xE5\xD22\xA8d\xE1\x83\xB7\xC4\xAC\xE9\x1F\xC3:D\xD5W\xEDwa\x02\xE0\x83\x01R` \x82a\x03\0\x84`\x08a\x07\xD0Z\x03\xFA\x82Q\x81\x16\x93PPPP\x95\x94PPPPPV[`@Qa\x03\x80\x81\x01`@Ra\x0E\x1B`\0\x84\x015a\0gV[a\x0E(` \x84\x015a\0gV[a\x0E5`@\x84\x015a\0gV[a\x0EB``\x84\x015a\0gV[a\x0EO`\x80\x84\x015a\0gV[a\x0E\\`\xA0\x84\x015a\0gV[a\x0Ei`\xC0\x84\x015a\0gV[a\x0Ev`\xE0\x84\x015a\0gV[a\x0E\x84a\x01\0\x84\x015a\0gV[a\x0E\x92a\x01 \x84\x015a\0gV[a\x0E\xA0a\x01@\x84\x015a\0gV[a\x0E\xAEa\x01`\x84\x015a\0gV[a\x0E\xBCa\x01\x80\x84\x015a\0gV[a\x0E\xCAa\x01\xA0\x84\x015a\0gV[a\x0E\xD8a\x01\xC0\x84\x015a\0gV[a\x0E\xE6a\x01\xE0\x84\x015a\0gV[a\x0E\xF4a\x02\0\x84\x015a\0gV[a\x0F\x02a\x02 \x84\x015a\0gV[a\x0F\x10a\x02@\x84\x015a\0gV[a\x0F\x1Ea\x02`\x84\x015a\0gV[a\x0F,a\x02\x80\x84\x015a\0gV[a\x0F:a\x02\xA0\x84\x015a\0gV[a\x0FHa\x02\xC0\x84\x015a\0gV[a\x0FVa\x02\xE0\x84\x015a\0gV[a\x0Fda\x03\0\x84\x015a\0gV[a\x0Fra\x03 \x84\x015a\0gV[a\x0F\x80a\x03@\x84\x015a\0gV[a\x0F\x8Ea\x03`\x84\x015a\0gV[a\x0F\x9Ca\x03\x80\x84\x015a\0gV[a\x0F\xAAa\x03\xA0\x84\x015a\0gV[a\x0F\xB8a\x03\xC0\x84\x015a\0gV[a\x0F\xC6a\x03\xE0\x84\x015a\0gV[a\x0F\xD3\x81\x84\x86\x88\x8Aa\x01\x06V[\x80`\0R` `\0\xF3[`\0\x80\xFD[`\0\x80\xFD[`\0\x81\x90P\x82` `\x02\x02\x82\x01\x11\x15a\x10\x03Wa\x10\x02a\x0F\xE2V[[\x92\x91PPV[`\0\x81\x90P\x82`@`\x02\x02\x82\x01\x11\x15a\x10%Wa\x10$a\x0F\xE2V[[\x92\x91PPV[`\0\x81\x90P\x82` `\x1F\x02\x82\x01\x11\x15a\x10GWa\x10Fa\x0F\xE2V[[\x92\x91PPV[`\0\x80`\0\x80a\x04\xE0\x85\x87\x03\x12\x15a\x10hWa\x10ga\x0F\xDDV[[`\0a\x10v\x87\x82\x88\x01a\x0F\xE7V[\x94PP`@a\x10\x87\x87\x82\x88\x01a\x10\tV[\x93PP`\xC0a\x10\x98\x87\x82\x88\x01a\x0F\xE7V[\x92PPa\x01\0a\x10\xAA\x87\x82\x88\x01a\x10+V[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x81\x15\x15\x90P\x91\x90PV[a\x10\xCB\x81a\x10\xB6V[\x82RPPV[`\0` \x82\x01\x90Pa\x10\xE6`\0\x83\x01\x84a\x10\xC2V[\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xD0\xEE-\x10yq\x90\xB4 U_#l4\x1E\xD73\xD1\x1D\x8AM\"\xD4\x9F\xA6&\x05s\x81\xA6pAdsolcC\0\x08\x18\x003";
    /// The deployed bytecode of the contract.
    pub static GROTH16VERIFIER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct Groth16Verifier<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Groth16Verifier<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Groth16Verifier<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Groth16Verifier<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Groth16Verifier<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Groth16Verifier)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Groth16Verifier<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                GROTH16VERIFIER_ABI.clone(),
                client,
            ))
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                GROTH16VERIFIER_ABI.clone(),
                GROTH16VERIFIER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `verifyProof` (0x44a173dd) function
        pub fn verify_proof(
            &self,
            p_a: [::ethers::core::types::U256; 2],
            p_b: [[::ethers::core::types::U256; 2]; 2],
            p_c: [::ethers::core::types::U256; 2],
            pub_signals: [::ethers::core::types::U256; 31],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([68, 161, 115, 221], (p_a, p_b, p_c, pub_signals))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for Groth16Verifier<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `verifyProof` function with signature `verifyProof(uint256[2],uint256[2][2],uint256[2],uint256[31])` and selector `0x44a173dd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "verifyProof",
        abi = "verifyProof(uint256[2],uint256[2][2],uint256[2],uint256[31])"
    )]
    pub struct VerifyProofCall {
        pub p_a: [::ethers::core::types::U256; 2],
        pub p_b: [[::ethers::core::types::U256; 2]; 2],
        pub p_c: [::ethers::core::types::U256; 2],
        pub pub_signals: [::ethers::core::types::U256; 31],
    }
    ///Container type for all return fields from the `verifyProof` function with signature `verifyProof(uint256[2],uint256[2][2],uint256[2],uint256[31])` and selector `0x44a173dd`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct VerifyProofReturn(pub bool);
}
