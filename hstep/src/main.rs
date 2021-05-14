use std::fs::File;
use std::io;
// use std::io::BufRead;
use std::io::BufReader;
// use std::io::Read;
// use std::str;
// use std::collections::HashMap;
// use std::path::Path;

use hstep::generated::{parse_data_func, read_tuple_0, read_tuple_1, read_tuple_2, read_tuple_3,read_tuple_4, read_tuple_5, read_tuple_7};
use hstep::hparser::{Parser, PeekableBufReader};
use std::time::{SystemTime};

fn parse_data_tuple_func(iden: &str, parser: &mut Parser) {
    match iden {
        "B_SPLINE_CURVE" => {
            read_tuple_5(parser, 
                Parser::read_int,
                Parser::read_id_vector,
                Parser::read_literal,
                Parser::read_bool,
                Parser::read_bool,
            );
        }
        "B_SPLINE_CURVE_WITH_KNOTS" => {
            read_tuple_3(parser,
                Parser::read_int_vector,
                Parser::read_float_vector,
                Parser::read_literal,
            );
        }
        "B_SPLINE_SURFACE" => {
            read_tuple_7(parser,
                Parser::read_int,
                Parser::read_int,
                Parser::read_id_vector_vector,
                Parser::read_literal,
                Parser::read_bool,
                Parser::read_bool,
                Parser::read_bool,
            );
        }
        "B_SPLINE_SURFACE_WITH_KNOTS" => {
            read_tuple_5(parser,
                Parser::read_int_vector,
                Parser::read_int_vector,
                Parser::read_float_vector,
                Parser::read_float_vector,
                Parser::read_literal,
            );
        }
        "BOUNDED_CURVE" => {
            read_tuple_0(parser);
        }
        "BOUNDED_SURFACE" => {
            read_tuple_0(parser);
        }
        "CURVE" => {
            read_tuple_0(parser);
        }
        "GEOMETRIC_REPRESENTATION_CONTEXT" => {
            read_tuple_1(parser,Parser::read_int);
        }
        "GEOMETRIC_REPRESENTATION_ITEM" => {
            read_tuple_0(parser);
        }
        "GLOBAL_UNCERTAINTY_ASSIGNED_CONTEXT" => {
            read_tuple_1(parser,Parser::read_id_vector);
        }
        "GLOBAL_UNIT_ASSIGNED_CONTEXT" => {
            read_tuple_1(parser,Parser::read_id_vector);
        }
        "LENGTH_UNIT" => {
            read_tuple_0(parser);
        }
        "NAMED_UNIT" => {
            read_tuple_1(parser,Parser::read_star);
        }
        "PLANE_ANGLE_UNIT" => {
            read_tuple_0(parser);
        }
        "RATIONAL_B_SPLINE_CURVE" => {
            read_tuple_1(parser,Parser::read_float_vector);
        }
        "RATIONAL_B_SPLINE_SURFACE" => {
            read_tuple_1(parser,Parser::read_float_vector_vector);
        }
        "REPRESENTATION_CONTEXT" => {
            read_tuple_2(parser,Parser::read_string, Parser::read_string);
        }
        "REPRESENTATION_ITEM" => {
            read_tuple_1(parser,Parser::read_string);
        }
        "REPRESENTATION_RELATIONSHIP" => {
            read_tuple_4(parser,
                Parser::read_string,
                Parser::read_string,
                Parser::read_id,
                Parser::read_id,
            );
        }
        "REPRESENTATION_RELATIONSHIP_WITH_TRANSFORMATION" => {
            read_tuple_1(parser,Parser::read_id);
        }
        "SI_UNIT" => {
            read_tuple_2(parser,Parser::read_literal_or_dollar, Parser::read_literal);
        }
        "SHAPE_REPRESENTATION_RELATIONSHIP" => {
            read_tuple_0(parser);
        }
        "SOLID_ANGLE_UNIT" => {
            read_tuple_0(parser);
        }
        "SURFACE" => {
            read_tuple_0(parser);
        }
        _ => panic!("Unkown data tuple function >>{}<<", iden),
    };
}
fn parse_data_tuple(parser: &mut Parser) {
    parser.read_open_paren();
    parser.skip_whitespace();
    loop {
        if parser.peek_char() == ')' {
            parser.read_close_paren();
            break;
        }
        let iden = parser.read_identifier();
        parser.skip_whitespace();
        parse_data_tuple_func(&iden, parser);
        if parser.peek_char() == ')' {
            parser.read_close_paren();
            break;
        }
        parser.skip_expect_whitespace()
    }
}

fn main() {

    let start = SystemTime::now();
    
    let filename = "/Users/Henry Heffan/Desktop/foxtrot/Kondo_only_data.step";
    let mut parser = Parser {
        stream: PeekableBufReader::new(io::BufReader::new(File::open(filename).expect("file opens"))),
    };

    // READ IN DATA FUNCTIONS
    loop {
        match parser.peek_char() {
            '#' => {
                let _idx = parser.read_id();
                parser.skip_whitespace();
                parser.read_equal_sign();
                parser.skip_whitespace();
                match parser.peek_char() {
                    '(' => parse_data_tuple(&mut parser),
                    'A'..='Z' | 'a'..='z' => {
                        let iden = parser.read_identifier();
                        let _entity = parse_data_func(&iden, &mut parser);
                        // mp.insert(idx, entity);
                    }
                    _ => panic!("unexpected character >>{}<<", parser.peek_char()),
                };
                parser.read_semicolon();
                parser.skip_whitespace();
            }
            _ => {
                let iden = parser.read_identifier();
                if iden != "ENDSEC" {
                    panic!("unexpected line in data section >>{}<<", iden)
                }
                parser.read_semicolon();
                parser.skip_whitespace();
                break;
            }
        }
    }


    let end = SystemTime::now();
    let since_the_epoch = end.duration_since(start).expect("Time went backwards");
    println!("time {:?}", since_the_epoch);
}

