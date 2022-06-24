use super::binding;
use super::protocol;
use std::io;

#[derive(Debug)]
pub enum Error {
    Accept(protocol::AcceptStat),
    Denied(protocol::RejectedReply),
    DeserializeError,
    InvalidAcceptStat(i32),
    InvalidAuthFlavor(i32),
    InvalidAuthStat(i32),
    InvalidMsgType(i32),
    InvalidRejectStat(i32),
    InvalidReplyStat(i32),
    NfsError(binding::Nfsstat4),
    NotSupported,
    ProgMismatch(protocol::AcceptedReplyMismatchInfo),
    RecvError(io::Error),
    SendError(io::Error),
    SerializeError,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
