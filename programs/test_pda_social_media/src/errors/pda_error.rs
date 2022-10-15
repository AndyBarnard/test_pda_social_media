use anchor_lang::prelude::*;

#[error_code]
enum PdaError {
    NameTooLong
}