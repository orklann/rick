// -------------------------------------------------------------------------------------------------
// Rick, a Rust intercal interpreter.  Save your souls!
//
// Copyright (c) 2015 Georg Brandl
//
// This program is free software; you can redistribute it and/or modify it under the terms of the
// GNU General Public License as published by the Free Software Foundation; either version 2 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with this program;
// if not, write to the Free Software Foundation, Inc., 675 Mass Ave, Cambridge, MA 02139, USA.
// -------------------------------------------------------------------------------------------------

use std::io::{ BufRead, Read, stdin };
use std::u16;
use std::u32;
use rand::{ random, Closed01 };

use err;


/// Check statement execution chance (false -> skip).
pub fn check_chance(chance: u8) -> bool {
    if chance == 100 {
        return true;
    }
    let Closed01(val) = random::<Closed01<f32>>();
    val <= (chance as f32) / 100.
}

/// Which roman digits from the digit_tbl to put together for each
/// decimal digit.
/// These are reversed because the whole digit string is reversed
/// in the end.
const ROMAN_TRANS_TBL: [(usize, [usize; 4]); 10] = [
    // (# of digits, which digits)
    (0, [0, 0, 0, 0]),
    (1, [0, 0, 0, 0]),
    (2, [0, 0, 0, 0]),
    (3, [0, 0, 0, 0]),
    (2, [2, 1, 0, 0]),    /* or use (4, [0, 0, 0, 0]) */
    (1, [2, 0, 0, 0]),
    (2, [1, 2, 0, 0]),
    (3, [1, 1, 2, 0]),
    (4, [1, 1, 1, 2]),
    (2, [3, 1, 0, 0])];

/// Which roman digits to use for each 10^n place.
const ROMAN_DIGIT_TBL: [[(char, char); 4]; 10] = [
    // (first line - overbars, second line - characters)
    [(' ', 'I'), (' ', 'I'), (' ', 'V'), (' ', 'X')],
    [(' ', 'X'), (' ', 'X'), (' ', 'L'), (' ', 'C')],
    [(' ', 'C'), (' ', 'C'), (' ', 'D'), (' ', 'M')],
    [(' ', 'M'), ('_', 'I'), ('_', 'V'), ('_', 'X')],
    [('_', 'X'), ('_', 'X'), ('_', 'L'), ('_', 'C')],
    [('_', 'C'), ('_', 'C'), ('_', 'D'), ('_', 'M')],
    [('_', 'M'), (' ', 'i'), (' ', 'v'), (' ', 'x')],
    [(' ', 'x'), (' ', 'x'), (' ', 'l'), (' ', 'c')],
    [(' ', 'c'), (' ', 'c'), (' ', 'd'), (' ', 'm')],
    [(' ', 'm'), ('_', 'i'), ('_', 'v'), ('_', 'x')]];

pub fn to_roman(mut val: u32) -> String {
    if val == 0 {
        // zero is just a lone overbar
        return format!("_\n\n");
    }
    let mut l1 = Vec::new();
    let mut l2 = Vec::new();
    let mut place = 0;
    while val > 0 {
        let digit = (val % 10) as usize;
        for j in 0..ROMAN_TRANS_TBL[digit].0 {
            let idx = ROMAN_TRANS_TBL[digit].1[j];
            l1.push(ROMAN_DIGIT_TBL[place][idx].0);
            l2.push(ROMAN_DIGIT_TBL[place][idx].1);
        }
        place += 1;
        val /= 10;
    }
    l1.reverse();
    l2.reverse();
    format!("{}\n{}\n",
            l1.into_iter().collect::<String>(),
            l2.into_iter().collect::<String>())
}

const ENGLISH_DIGITS: [(&'static str, u8); 12] = [
    ("ZERO",  0),
    ("OH",    0),
    ("ONE",   1),
    ("TWO",   2),
    ("THREE", 3),
    ("FOUR",  4),
    ("FIVE",  5),
    ("SIX",   6),
    ("SEVEN", 7),
    ("EIGHT", 8),
    ("NINE",  9),
    ("NINER", 9)];

pub fn from_english(v: &str) -> Result<u32, err::Error> {
    let mut digits = Vec::new();
    for word in v.split_whitespace() {
        let mut found = false;
        for &(w, val) in &ENGLISH_DIGITS {
            if w == word {
                digits.push(val);
                found = true;
                break;
            }
        }
        if !found {
            return Err(err::with_str(&err::IE579, word));
        }
    }
    let mut res = 0;
    for (i, digit) in digits.iter().enumerate() {
        res += (*digit as u64) * (10 as u64).pow(digits.len() as u32 - 1 - i as u32);
    }
    if res > (u32::MAX as u64) {
        Err(err::new(&err::IE533))
    } else {
        Ok(res as u32)
    }
}

pub fn write_number(val: u32) {
    print!("{}", to_roman(val));
}

pub fn write_byte(val: u8) {
    print!("{}", val as char);
}

pub fn read_number() -> Result<u32, err::Error> {
    let stdin = stdin();
    let mut slock = stdin.lock();
    let mut buf = String::new();
    match slock.read_line(&mut buf) {
        Ok(n) if n > 0 => from_english(&buf),
        _              => Err(err::new(&err::IE562))
    }
}

pub fn read_byte() -> u16 {
    let stdin = stdin();
    let mut slock = stdin.lock();
    let mut buf = [0u8; 1];
    match slock.read(&mut buf) {
        Ok(1) => buf[0] as u16,
        _     => 256      // EOF is defined to be 256
    }
}

pub fn mingle(mut v: u32, mut w: u32) -> Result<u32, err::Error> {
    if v > (u16::MAX as u32) || w > (u16::MAX as u32) {
        return Err(err::new(&err::IE533));
    }
    v = ((v & 0x0000ff00) << 8) | (v & 0x000000ff);
    v = ((v & 0x00f000f0) << 4) | (v & 0x000f000f);
    v = ((v & 0x0c0c0c0c) << 2) | (v & 0x03030303);
    v = ((v & 0x22222222) << 1) | (v & 0x11111111);
    w = ((w & 0x0000ff00) << 8) | (w & 0x000000ff);
    w = ((w & 0x00f000f0) << 4) | (w & 0x000f000f);
    w = ((w & 0x0c0c0c0c) << 2) | (w & 0x03030303);
    w = ((w & 0x22222222) << 1) | (w & 0x11111111);
    Ok((v << 1) | w)
}

pub fn select(mut v: u32, mut w: u32) -> Result<u32, err::Error> {
    let mut i = 1;
    let mut t = 0;
    while w > 0 {
        if w & i > 0 {
            t |= v & i;
            w ^= i;
            i <<= 1;
        } else {
            w >>= 1;
            v >>= 1;
        }
    }
    Ok(t)
}

pub fn and_16(v: u16) -> u16 {
    let mut w = v >> 1;
    if v & 1 > 0 {
        w |= 0x8000;
    }
    w & v
}

pub fn and_32(v: u32) -> u32 {
    let mut w = v >> 1;
    if v & 1 > 0 {
        w |= 0x80000000;
    }
    w & v
}

pub fn or_16(v: u16) -> u16 {
    let mut w = v >> 1;
    if v & 1 > 0 {
        w |= 0x8000;
    }
    w | v
}

pub fn or_32(v: u32) -> u32 {
    let mut w = v >> 1;
    if v & 1 > 0 {
        w |= 0x80000000;
    }
    w | v
}

pub fn xor_16(v: u16) -> u16 {
    let mut w = v >> 1;
    if v & 1 > 0 {
        w |= 0x8000;
    }
    w ^ v
}

pub fn xor_32(v: u32) -> u32 {
    let mut w = v >> 1;
    if v & 1 > 0 {
        w |= 0x80000000;
    }
    w ^ v
}

pub trait FromU16: Copy {
    fn from_u16(u16) -> Self;
}

pub trait ToU16: Copy {
    fn to_u16(self) -> u16;
}

impl FromU16 for u16 {
    fn from_u16(x: u16) -> u16 { x }
}

impl FromU16 for u32 {
    fn from_u16(x: u16) -> u32 { x as u32 }
}

impl ToU16 for u16 {
    fn to_u16(self) -> u16 { self }
}

impl ToU16 for u32 {
    fn to_u16(self) -> u16 { self as u16 }
}
