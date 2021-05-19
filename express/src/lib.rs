use memchr::{memchr, memchr2, memchr_iter};
use nom::{
    branch::{alt},
    bytes::complete::{tag},
    character::complete::{alpha1, alphanumeric0, alphanumeric1, digit1, char},
    multi::{fold_many1, many0, many1},
    combinator::{recognize},
    sequence::{pair, preceded},
};

enum Parse {
    KeywordAbs,
    KeywordAbstract,
    KeywordAcos,
    KeywordAggregate,
    KeywordAlias,
    KeywordAnd,
    KeywordAndor,
    KeywordArray,
    KeywordAs,
    KeywordAsin,
    KeywordAtan,
    KeywordBag,
    KeywordBasedOn,
    KeywordBegin,
    KeywordBinary,
    KeywordBlength,
    KeywordBoolean,
    KeywordBy,
    KeywordCase,
    KeywordConstant,
    KeywordConstE,
    KeywordCos,
    KeywordDerive,
    KeywordDiv,
    KeywordElse,
    KeywordEnd,
    KeywordEndAlias,
    KeywordEndCase,
    KeywordEndConstant,
    KeywordEndEntity,
    KeywordEndFunction,
    KeywordEndIf,
    KeywordEndLocal,
    KeywordEndProcedure,
    KeywordEndRepeat,
    KeywordEndRule,
    KeywordEndSchema,
    KeywordEndSubtypeConstraint,
    KeywordEndType,
    KeywordEntity,
    KeywordEnumeration,
    KeywordEscape,
    KeywordExists,
    KeywordExtensible,
    KeywordExp,
    KeywordFalse,
    KeywordFixed,
    KeywordFor,
    KeywordFormat,
    KeywordFrom,
    KeywordFunction,
    KeywordGeneric,
    KeywordGenericEntity,
    KeywordHibound,
    KeywordHiindex,
    KeywordIf,
    KeywordIn,
    KeywordInsert,
    KeywordInteger,
    KeywordInverse,
    KeywordLength,
    KeywordLike,
    KeywordList,
    KeywordLobound,
    KeywordLocal,
    KeywordLog,
    KeywordLog10,
    KeywordLog2,
    KeywordLogical,
    KeywordLoindex,
    KeywordMod,
    KeywordNot,
    KeywordNumber,
    KeywordNvl,
    KeywordOdd,
    KeywordOf,
    KeywordOneof,
    KeywordOptional,
    KeywordOr,
    KeywordOtherwise,
    KeywordPi,
    KeywordProcedure,
    KeywordQuery,
    KeywordReal,
    KeywordReference,
    KeywordRemove,
    KeywordRenamed,
    KeywordRepeat,
    KeywordReturn,
    KeywordRolesof,
    KeywordRule,
    KeywordSchema,
    KeywordSelect,
    KeywordSelf,
    KeywordSet,
    KeywordSin,
    KeywordSizeof,
    KeywordSkip,
    KeywordSqrt,
    KeywordString,
    KeywordSubtype,
    KeywordSubtypeConstraint,
    KeywordSupertype,
    KeywordTan,
    KeywordThen,
    KeywordTo,
    KeywordTotalOver,
    KeywordTrue,
    KeywordType,
    KeywordTypeof,
    KeywordUnique,
    KeywordUnknown,
    KeywordUntil,
    KeywordUse,
    KeywordUsedin,
    KeywordValue,
    KeywordValueIn,
    KeywordValueUnique,
    KeywordVar,
    KeywordWhere,
    KeywordWhile,
    KeywordWith,
    KeywordXor,
}
pub type IResult<'a, U> = nom::IResult<&'a str, U, nom::error::VerboseError<&'a str>>;

/// Keyword parser (A.1.1)
fn keyword(s: &str) -> IResult<Parse> {
    let (rest, s) = alpha1(s)?;
    use Parse::*;
    Ok((rest, match s {
        "abs" => KeywordAbs,
        "abstract" => KeywordAbstract,
        "acos" => KeywordAcos,
        "aggregate" => KeywordAggregate,
        "alias" => KeywordAlias,
        "and" => KeywordAnd,
        "andor" => KeywordAndor,
        "array" => KeywordArray,
        "as" => KeywordAs,
        "asin" => KeywordAsin,
        "atan" => KeywordAtan,
        "bag" => KeywordBag,
        "based_on" => KeywordBasedOn,
        "begin" => KeywordBegin,
        "binary" => KeywordBinary,
        "blength" => KeywordBlength,
        "boolean" => KeywordBoolean,
        "by" => KeywordBy,
        "case" => KeywordCase,
        "constant" => KeywordConstant,
        "const_e" => KeywordConstE,
        "cos" => KeywordCos,
        "derive" => KeywordDerive,
        "div" => KeywordDiv,
        "else" => KeywordElse,
        "end" => KeywordEnd,
        "end_alias" => KeywordEndAlias,
        "end_case" => KeywordEndCase,
        "end_constant" => KeywordEndConstant,
        "end_entity" => KeywordEndEntity,
        "end_function" => KeywordEndFunction,
        "end_if" => KeywordEndIf,
        "end_local" => KeywordEndLocal,
        "end_procedure" => KeywordEndProcedure,
        "end_repeat" => KeywordEndRepeat,
        "end_rule" => KeywordEndRule,
        "end_schema" => KeywordEndSchema,
        "end_subtype_constraint" => KeywordEndSubtypeConstraint,
        "end_type" => KeywordEndType,
        "entity" => KeywordEntity,
        "enumeration" => KeywordEnumeration,
        "escape" => KeywordEscape,
        "exists" => KeywordExists,
        "extensible" => KeywordExtensible,
        "exp" => KeywordExp,
        "false" => KeywordFalse,
        "fixed" => KeywordFixed,
        "for" => KeywordFor,
        "format" => KeywordFormat,
        "from" => KeywordFrom,
        "function" => KeywordFunction,
        "generic" => KeywordGeneric,
        "generic_entity" => KeywordGenericEntity,
        "hibound" => KeywordHibound,
        "hiindex" => KeywordHiindex,
        "if" => KeywordIf,
        "in" => KeywordIn,
        "insert" => KeywordInsert,
        "integer" => KeywordInteger,
        "inverse" => KeywordInverse,
        "length" => KeywordLength,
        "like" => KeywordLike,
        "list" => KeywordList,
        "lobound" => KeywordLobound,
        "local" => KeywordLocal,
        "log" => KeywordLog,
        "log10" => KeywordLog10,
        "log2" => KeywordLog2,
        "logical" => KeywordLogical,
        "loindex" => KeywordLoindex,
        "mod" => KeywordMod,
        "not" => KeywordNot,
        "number" => KeywordNumber,
        "nvl" => KeywordNvl,
        "odd" => KeywordOdd,
        "of" => KeywordOf,
        "oneof" => KeywordOneof,
        "optional" => KeywordOptional,
        "or" => KeywordOr,
        "otherwise" => KeywordOtherwise,
        "pi" => KeywordPi,
        "procedure" => KeywordProcedure,
        "query" => KeywordQuery,
        "real" => KeywordReal,
        "reference" => KeywordReference,
        "remove" => KeywordRemove,
        "renamed" => KeywordRenamed,
        "repeat" => KeywordRepeat,
        "return" => KeywordReturn,
        "rolesof" => KeywordRolesof,
        "rule" => KeywordRule,
        "schema" => KeywordSchema,
        "select" => KeywordSelect,
        "self" => KeywordSelf,
        "set" => KeywordSet,
        "sin" => KeywordSin,
        "sizeof" => KeywordSizeof,
        "skip" => KeywordSkip,
        "sqrt" => KeywordSqrt,
        "string" => KeywordString,
        "subtype" => KeywordSubtype,
        "subtype_constraint" => KeywordSubtypeConstraint,
        "supertype" => KeywordSupertype,
        "tan" => KeywordTan,
        "then" => KeywordThen,
        "to" => KeywordTo,
        "total_over" => KeywordTotalOver,
        "true" => KeywordTrue,
        "type" => KeywordType,
        "typeof" => KeywordTypeof,
        "unique" => KeywordUnique,
        "unknown" => KeywordUnknown,
        "until" => KeywordUntil,
        "use" => KeywordUse,
        "usedin" => KeywordUsedin,
        "value" => KeywordValue,
        "value_in" => KeywordValueIn,
        "value_unique" => KeywordValueUnique,
        "var" => KeywordVar,
        "where" => KeywordWhere,
        "while" => KeywordWhile,
        "with" => KeywordWith,
        "xor" => KeywordXor,
        _ => {
            use nom::error::*;
            return Err(nom::Err::Failure(VerboseError {
                errors: vec![("Missing keyword",
                              VerboseErrorKind::Nom(ErrorKind::Alt))]
            }));
        },
    }))
}

fn digits(s: &str) -> IResult<usize> {
    digit1(s).map(|v| (v.0, v.1.parse().unwrap()))
}

fn simple_id(s: &str) -> IResult<&str> {
    recognize(pair(alpha1, alt((alphanumeric0, tag("_")))))(s)
}

fn bits(s: &str) -> IResult<usize> {
    fold_many1(alt((char('0'), char('1'))), 0, |mut acc: usize, item| acc * 2 + item.to_digit(10).unwrap() as usize)(s)
}

fn binary_literal(s: &str) -> IResult<usize> {
    preceded(char('%'), bits)(s)
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

////////////////////////////////////////////////////////////////////////////////

/// Remove comments from an EXPRESS file and converts to lower-case
pub fn strip_flatten(data: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(data.len());
    let mut i = 0;
    while i < data.len() {
        match data[i] {
            // Block comments
            b'(' => if i + 1 < data.len() && data[i + 1] == b'*' {
                for j in memchr_iter(b')', &data[i + 2..]) {
                    if data[i + j + 1] == b'*' {
                        i += j + 2;
                        break;
                    }
                }
            },
            // Single-line comments
            b'-' if i + 1 < data.len() && data[i + 1] == b'-' => {
                let newline = memchr(b'\n', &data[i + 2..]);
                i += newline.unwrap_or(0) + 3;
            },
            c => out.push(c.to_ascii_lowercase())
        }
        i += 1;
    }
    out
}

