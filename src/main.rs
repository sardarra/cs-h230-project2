// Ticket-booking session packed into a single 16-bit unsigned integer (u16).
// Think of this number as a row of 16 light switches. Each switch (bit) is
// either off (0) or on (1). We assign groups of switches a meaning:
//
//   bit  0      : activity        0 = inactive, 1 = active
//   bits 1–2    : role            00 = viewer, 01 = buyer, 10 = seller, 11 = admin
//   bits 3–7    : event / session id   (which event this user is looking at)
//   bits 8–14   : seats remaining (how many tickets are still left)
//   bit  15     : endianness      0 = little-endian, 1 = big-endian
//
// Bit 0 is the "ones" place (least significant). Bit 15 is the highest place.

use std::io;

const ACTIVITY_SHIFT: u16 = 0;
const ROLE_SHIFT: u16 = 1;
const SESSION_ID_SHIFT: u16 = 3;
const SEATS_SHIFT: u16 = 8;
const ENDIAN_SHIFT: u16 = 15;

// Masks keep only the bits we care about after we slide the field down to 0.
const ACTIVITY_MASK: u16 = 0b1; // 1 bit
const ROLE_MASK: u16 = 0b11; // 2 bits
const SESSION_ID_MASK: u16 = 0b1_1111; // 5 bits
const SEATS_MASK: u16 = 0b111_1111; // 7 bits
const ENDIAN_MASK: u16 = 0b1; // 1 bit

fn pack_session(
    activity: u16,
    role: u16,
    session_id: u16,
    seats_remaining: u16,
    endianness: u16,
) -> u16 {
    // "<< N" slides the number N places to the left, which is how we park
    // each field in its assigned bit slots. "|" (bitwise OR) then layers
    // those fields into one number without overlapping.
    (activity & ACTIVITY_MASK) << ACTIVITY_SHIFT
        | (role & ROLE_MASK) << ROLE_SHIFT
        | (session_id & SESSION_ID_MASK) << SESSION_ID_SHIFT
        | (seats_remaining & SEATS_MASK) << SEATS_SHIFT
        | (endianness & ENDIAN_MASK) << ENDIAN_SHIFT
}

fn get_activity(state: u16) -> u16 {
    // Slide nothing; the activity flag already lives in bit 0.
    (state >> ACTIVITY_SHIFT) & ACTIVITY_MASK
}

fn get_role(state: u16) -> u16 {
    // Slide bits 1–2 down to positions 0–1, then keep those two bits.
    (state >> ROLE_SHIFT) & ROLE_MASK
}

fn get_session_id(state: u16) -> u16 {
    (state >> SESSION_ID_SHIFT) & SESSION_ID_MASK
}

fn get_endianness(state: u16) -> u16 {
    // Bit 15 is our "how should numbers be stored?" switch.
    (state >> ENDIAN_SHIFT) & ENDIAN_MASK
}

fn get_seats_remaining(state: u16) -> u16 {
    // First, pull seats out of bits 8–14 as a normal number (0–127).
    let seats = (state >> SEATS_SHIFT) & SEATS_MASK;

    // Then look at the endianness flag packed in the same u16.
    // 0 = little-endian: return the number as-is.
    // 1 = big-endian: convert that 16-bit value to big-endian byte order
    //     before we hand it back. On a Mac (little-endian CPU), .to_be()
    //     swaps the two bytes. Example: 42 (0x002A) becomes 10752 (0x2A00).
    if get_endianness(state) == 1 {
        seats.to_be()
    } else {
        seats
    }
}

fn role_name(role: u16) -> &'static str {
    match role {
        0 => "viewer",
        1 => "buyer",
        2 => "seller",
        3 => "admin",
        _ => "unknown",
    }
}

fn endian_name(endianness: u16) -> &'static str {
    if endianness == 1 {
        "big-endian"
    } else {
        "little-endian"
    }
}

fn print_session(label: &str, state: u16) {
    println!("{label}");
    println!("  packed u16          = {state} (0b{state:016b})");
    println!("  activity            = {}", get_activity(state));
    println!("  role                = {} ({})", get_role(state), role_name(get_role(state)));
    println!("  session / event id  = {}", get_session_id(state));
    println!("  endianness          = {} ({})", get_endianness(state), endian_name(get_endianness(state)));
    println!("  seats remaining     = {}  (may be byte-swapped if big-endian)", get_seats_remaining(state));
    println!();
}

fn main() {

    // ask the user for input with prompts, and then pack the session
    println!("Enter activity (0 or 1, 1 is active):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let activity = input.trim().parse::<u16>().unwrap();

    println!("Enter role (0-3, 0 is viewer, 1 is buyer, 2 is seller, 3 is admin):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let role = input.trim().parse::<u16>().unwrap();

    println!("Enter session id (1-127):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let session_id = input.trim().parse::<u16>().unwrap();

    println!("Enter seats remaining (1-127):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let seats_remaining = input.trim().parse::<u16>().unwrap();
    
    println!("Enter endianness (0 or 1, 0 is little-endian, 1 is big-endian):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let endianness = input.trim().parse::<u16>().unwrap();
    

    let session = pack_session(activity, role, session_id, seats_remaining, endianness);
    print_session("Session:", session);
}
