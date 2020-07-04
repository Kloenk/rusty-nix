use std::convert::From;
use std::io;

use custom_error::custom_error;

custom_error! {
    pub StoreError
        Io{source: io::Error} = "IoError: {source}",
        StringToLong{len: usize} = "string is to long",
        ConnectionError{source: ConnectionError} = "ConnectionError: {source}",
        InvalidStoreUri{uri: String} = "InvalidStoreUri: {uri}",
        NotInStore{path: String} = "path \"{path}\" is not in the Nix store",
        UtilError{source: libutil::error::UtilError} = "UtilError: {source}",
        SqlError{source: rusqlite::Error} = "SQLError: {source}",
        MissingHash{path: String} = "{} lacks valid signature",
        OsError{ call: String, ret: i32 } = "Os Error: {call}: {ret}",
        SysError{ msg: String } = "SysError: {msg}",
        InvalidKey{ key: String } = "The key {key} is invalid",
        RingError{ source: ring::error::Unspecified } = "error in crypto: {source}",
        NoFingerprint{ path: String } = "cannot calculate fingerprint of path {path} because its size/hash is not known",
        HashDecodeError { source: data_encoding::DecodeError } = "cannot decode: {source}",
        //HashDecodePartialError { source: data_encoding::DecodePartial } = "cannot decode {source}", // TODO: to_string missing on DecodePartial
        HashDecodePartialError { error: String } = "cannont decode {error}",
        InvalidFileIngestionMethode { methode: u8 } = "invalid FileIngestionMethode: {methode}",
        BadArchive{ msg: String } = "BadArchive: {msg}",
        //BadArchive{ source: NarError } = "BadArchive: {source}",

        Unimplemented{ msg: String } = "Unimplemented: {msg}",
}

custom_error! {
    pub ConnectionError
        Io{source: io::Error} = "IoError: {source}",
}

custom_error! {
    pub NarError
        NotAArchive{} = "input does not look like a Nix Archive",
        MissingOpenTag{} = "expected an open tag",
        MultipleTypeFieleds{} = "there are multiple type fieleds",
        UnknownFileType{ file: String } = "Unknown file type: {file}",
        InvalidExecutableMarker{} = "executable marker has non-empty value",
        ReadError{source: std::io::Error} = "could not read nar: {source}",
        StoreError{source: StoreError} = "StoreError: {source}",
        InvalidFileName{name: String} = "NAR contains invalid file name: '{name}'",
        NotSorted{} = "NAR archive is not sorted",
        MissingName{} = "etry name is missing",
        InvalidSymlinkMarker{ marker: String } = "invalid target marker for symlink: '{marker}'",
        InvalidState{state: crate::archive::State } = "InvalidState: {state}", // TODO: add case info
}
