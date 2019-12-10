use crate::day_2::*;
use std::io::{stdin, stdout};

pub fn solve() {
    let boost = generate_program(INPUT_BOOST);
    let stdin = stdin();
    let mut cpu = IntCodeComputer::new(boost, stdin.lock(), stdout());
    cpu.run();
}

//const INPUT_BOOST: &str = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
const INPUT_BOOST: &str = "1102,34463338,34463338,63,1007,63,34463338,63,1005,63,53,1101,0,3,1000,109,988,209,12,9,1000,209,6,209,3,203,0,1008,1000,1,63,1005,63,65,1008,1000,2,63,1005,63,902,1008,1000,0,63,1005,63,58,4,25,104,0,99,4,0,104,0,99,4,17,104,0,99,0,0,1101,0,32,1009,1101,0,842,1023,1101,0,33,1007,1101,36,0,1015,1101,35,0,1006,1101,0,0,1020,1102,1,25,1005,1101,0,34,1008,1101,0,849,1022,1102,1,1,1021,1101,22,0,1004,1102,1,26,1017,1102,286,1,1029,1101,38,0,1013,1102,20,1,1000,1102,39,1,1002,1101,0,24,1010,1101,0,30,1016,1102,1,27,1019,1102,824,1,1027,1102,216,1,1025,1102,1,28,1001,1101,295,0,1028,1102,29,1,1003,1101,31,0,1011,1102,1,827,1026,1102,1,225,1024,1101,21,0,1012,1102,1,23,1018,1102,37,1,1014,109,19,21102,40,1,-1,1008,1018,40,63,1005,63,203,4,187,1106,0,207,1001,64,1,64,1002,64,2,64,109,1,2105,1,4,4,213,1001,64,1,64,1106,0,225,1002,64,2,64,109,-8,21101,41,0,7,1008,1019,41,63,1005,63,247,4,231,1106,0,251,1001,64,1,64,1002,64,2,64,109,-1,2101,0,-2,63,1008,63,32,63,1005,63,273,4,257,1105,1,277,1001,64,1,64,1002,64,2,64,109,17,2106,0,0,4,283,1001,64,1,64,1106,0,295,1002,64,2,64,109,-13,1202,-6,1,63,1008,63,32,63,1005,63,321,4,301,1001,64,1,64,1105,1,321,1002,64,2,64,109,10,1205,-5,337,1001,64,1,64,1105,1,339,4,327,1002,64,2,64,109,-31,2102,1,9,63,1008,63,32,63,1005,63,363,1001,64,1,64,1105,1,365,4,345,1002,64,2,64,109,22,2107,34,-9,63,1005,63,385,1001,64,1,64,1106,0,387,4,371,1002,64,2,64,109,-2,21108,42,42,1,1005,1015,409,4,393,1001,64,1,64,1105,1,409,1002,64,2,64,109,-2,1208,-9,31,63,1005,63,425,1105,1,431,4,415,1001,64,1,64,1002,64,2,64,109,-3,2108,37,-1,63,1005,63,451,1001,64,1,64,1106,0,453,4,437,1002,64,2,64,109,-9,1201,6,0,63,1008,63,35,63,1005,63,475,4,459,1105,1,479,1001,64,1,64,1002,64,2,64,109,15,2107,33,-7,63,1005,63,497,4,485,1106,0,501,1001,64,1,64,1002,64,2,64,1206,6,515,1001,64,1,64,1105,1,517,4,505,1002,64,2,64,109,-2,2101,0,-7,63,1008,63,32,63,1005,63,541,1001,64,1,64,1105,1,543,4,523,1002,64,2,64,109,-6,2102,1,-2,63,1008,63,25,63,1005,63,569,4,549,1001,64,1,64,1106,0,569,1002,64,2,64,109,5,1201,-8,0,63,1008,63,19,63,1005,63,589,1106,0,595,4,575,1001,64,1,64,1002,64,2,64,109,-16,1207,10,36,63,1005,63,613,4,601,1106,0,617,1001,64,1,64,1002,64,2,64,109,25,1206,-1,631,4,623,1105,1,635,1001,64,1,64,1002,64,2,64,109,-8,21101,43,0,1,1008,1014,46,63,1005,63,655,1106,0,661,4,641,1001,64,1,64,1002,64,2,64,109,-4,2108,33,-2,63,1005,63,683,4,667,1001,64,1,64,1106,0,683,1002,64,2,64,109,1,21107,44,43,0,1005,1010,699,1105,1,705,4,689,1001,64,1,64,1002,64,2,64,109,1,21102,45,1,8,1008,1019,46,63,1005,63,729,1001,64,1,64,1106,0,731,4,711,1002,64,2,64,109,3,1207,-7,32,63,1005,63,751,1001,64,1,64,1106,0,753,4,737,1002,64,2,64,109,7,1205,0,771,4,759,1001,64,1,64,1105,1,771,1002,64,2,64,109,-18,1208,0,29,63,1005,63,789,4,777,1105,1,793,1001,64,1,64,1002,64,2,64,109,16,21107,46,47,-7,1005,1012,811,4,799,1106,0,815,1001,64,1,64,1002,64,2,64,109,17,2106,0,-9,1105,1,833,4,821,1001,64,1,64,1002,64,2,64,109,-10,2105,1,-3,1001,64,1,64,1105,1,851,4,839,1002,64,2,64,109,-16,1202,-6,1,63,1008,63,25,63,1005,63,875,1001,64,1,64,1106,0,877,4,857,1002,64,2,64,109,-1,21108,47,45,5,1005,1014,897,1001,64,1,64,1106,0,899,4,883,4,64,99,21101,27,0,1,21101,0,913,0,1106,0,920,21201,1,28853,1,204,1,99,109,3,1207,-2,3,63,1005,63,962,21201,-2,-1,1,21102,940,1,0,1105,1,920,22102,1,1,-1,21201,-2,-3,1,21101,0,955,0,1105,1,920,22201,1,-1,-2,1106,0,966,22102,1,-2,-2,109,-3,2105,1,0";