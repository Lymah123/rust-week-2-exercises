// Data Encoding and Conversion
use hex::{decode, encode};

pub fn decode_hex(hex_str: &str) -> Result<Vec<u8>, String> {
    decode(hex_str).map_err(|e| format!("Failed to decode hex: {}", e))
}

pub fn to_big_endian(bytes: &[u8]) -> Vec<u8> {
    let mut result = bytes.to_vec();
    result.reverse();
    result
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    encode(bytes)
}

pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, hex::FromHexError> {
    decode(hex)
}

pub fn swap_endian_u32(num: u32) -> [u8; 4] {
    num.to_le_bytes()
}

pub fn parse_satoshis(input: &str) -> Result<u64, String> {
    input
        .parse::<u64>()
        .map_err(|_| "Invalid satoshi amount".to_string())
}

// Script and Transaction Components
pub enum ScriptType {
    P2PKH,
    P2WPKH,
    Unknown,
}

pub fn classify_script(script: &[u8]) -> ScriptType {
    match script {
        [0x76, 0xa9, 0x14, ..] => ScriptType::P2PKH,
        [0x00, 0x14, ..] => ScriptType::P2WPKH,
        _ => ScriptType::Unknown,
    }
}

// Outpoint tuple struct
pub struct Outpoint(pub String, pub u32);

impl Outpoint {
    pub fn new(txid_str: String, vout: u32) -> Self {
        Outpoint(txid_str, vout)
    }
    pub fn txid(&self) -> &str {
        &self.0
    }

    pub fn txid_bytes(&self) -> Result<Vec<u8>, String> {
        decode(&self.0).map_err(|e| format!("Invalid hex string: {}", e))
    }
}

impl From<(String, u32)> for Outpoint {
    fn from((txid_str, vout): (String, u32)) -> Self {
        Outpoint(txid_str, vout)
    }
}

pub fn read_pushdata(script: &[u8]) -> &[u8] {
    if script.len() >= 3 { &script[2..] } else { &[] }
}

// Wallet and UTXO Management
pub trait Wallet {
    fn balance(&self) -> u64;
}

pub struct TestWallet {
    pub confirmed: u64,
}

impl Wallet for TestWallet {
    fn balance(&self) -> u64 {
        self.confirmed
    }
}

pub fn apply_fee(balance: &mut u64, fee: u64) {
    *balance = balance.saturating_sub(fee);
}

pub fn move_txid(txid: String) -> String {
    format!("txid: {}", txid)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Opcode {
    OpChecksig,
    OpDup,
    OpInvalid,
}

impl Opcode {
    pub fn from_byte(byte: u8) -> Result<Self, String> {
        match byte {
            0xac => Ok(Opcode::OpChecksig),
            0x76 => Ok(Opcode::OpDup),
            _ => Err(format!("Invalid opcode: 0x{:02x}", byte)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UTXO {
    pub txid: Vec<u8>,
    pub vout: u32,
    pub value: u64,
}

pub fn consume_utxo(utxo: UTXO) -> UTXO {
    utxo
}
