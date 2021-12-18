use hex::decode;
use nom::{
    bits::complete::{tag, take},
    branch::alt,
    combinator::{map, map_opt},
    multi::length_count,
    sequence::{preceded, tuple},
    IResult,
};
use num::FromPrimitive;
use std::io::{self, BufRead};

type BitSlice<'a> = (&'a [u8], usize);

fn bit_slice_diff(a: BitSlice, b: BitSlice) -> usize {
    (a.0.len() * 8 - a.1) - (b.0.len() * 8 - b.1)
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Operator {
    Sum = 0,
    Mult = 1,
    Min = 2,
    Max = 3,
    GreaterThan = 5,
    LessThan = 6,
    EqualTo = 7,
}

impl FromPrimitive for Operator {
    fn from_i64(n: i64) -> Option<Self> {
        FromPrimitive::from_u64(n as u64)
    }

    fn from_u64(n: u64) -> Option<Self> {
        match n {
            0 => Some(Operator::Sum),
            1 => Some(Operator::Mult),
            2 => Some(Operator::Min),
            3 => Some(Operator::Max),
            5 => Some(Operator::GreaterThan),
            6 => Some(Operator::LessThan),
            7 => Some(Operator::EqualTo),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Packet {
    Literal {
        version: u8,
        value: i64,
    },
    Operator {
        version: u8,
        operator: Operator,
        data: Vec<Packet>,
    },
}

impl Packet {
    fn version_sum(&self) -> u64 {
        match self {
            Packet::Literal { version, .. } => *version as u64,
            Packet::Operator { version, data, .. } => {
                *version as u64 + data.into_iter().map(|p| p.version_sum()).sum::<u64>()
            }
        }
    }

    fn value(&self) -> i64 {
        match self {
            Packet::Literal { value, .. } => *value,
            Packet::Operator { operator, data, .. } => match *operator {
                Operator::Sum => data.into_iter().map(|p| p.value()).sum::<i64>(),
                Operator::Mult => data.into_iter().map(|p| p.value()).product::<i64>(),
                Operator::Min => data.into_iter().map(|p| p.value()).min().unwrap(),
                Operator::Max => data.into_iter().map(|p| p.value()).max().unwrap(),
                Operator::GreaterThan => (data[0].value() > data[1].value()) as i64,
                Operator::LessThan => (data[0].value() < data[1].value()) as i64,
                Operator::EqualTo => (data[0].value() == data[1].value()) as i64,
            },
        }
    }

    fn parse_literal(mut input: BitSlice) -> IResult<BitSlice, i64> {
        let mut value: i64 = 0;
        let mut chunk: i64;

        loop {
            (input, chunk) = take(5usize)(input)?;
            value = (16 * value) + (chunk & 15);
            if (chunk & 1 << 4) == 0 {
                break;
            }
        }

        Ok((input, value))
    }

    fn parse_bit_length_packets(mut input: BitSlice) -> IResult<BitSlice, Vec<Packet>> {
        let total;
        let mut packets = vec![];
        let mut length = 0;
        (input, total) = take(15u8)(input)?;

        while length < total {
            let (remaining, packet) = Packet::parse(input)?;

            packets.push(packet);
            length += bit_slice_diff(input, remaining);
            input = remaining;
        }

        Ok((input, packets))
    }

    fn parse_operator(input: BitSlice) -> IResult<BitSlice, Vec<Packet>> {
        alt((
            preceded(tag(0, 1u8), Packet::parse_bit_length_packets),
            preceded(
                tag(1, 1u8),
                length_count(take::<_, usize, _, _>(11u8), Packet::parse),
            ),
        ))(input)
    }

    fn parse(input: BitSlice) -> IResult<BitSlice, Packet> {
        alt((
            map(
                tuple((take(3u8), tag(4, 3u8), Packet::parse_literal)),
                |(version, _literal, value)| Packet::Literal { version, value },
            ),
            map_opt(
                tuple((take(3u8), take(3u8), Packet::parse_operator)),
                |(version, operator, data)| {
                    Some(Packet::Operator {
                        version,
                        operator: FromPrimitive::from_u8(operator)?,
                        data,
                    })
                },
            ),
        ))(input)
    }
}

pub fn solve() {
    let data = io::stdin().lock().lines().next().unwrap().unwrap();

    let decoded = decode(&data).unwrap();
    let (_, parsed) = Packet::parse((&decoded, 0)).unwrap();

    println!("[Part 1] {:?}", parsed.version_sum());
    println!("[Part 2] {:?}", parsed.value());
}
