use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[repr(i32)]
pub enum AuthFlavor {
    AuthNone = 0i32,
    AuthSys = 1i32,
    AuthShort = 2i32,
    AuthDh = 3i32,
    RpcsecGss = 6i32,
}
impl Default for AuthFlavor {
    fn default() -> Self {
        AuthFlavor::AuthNone
    }
}
impl AsRef<i32> for AuthFlavor {
    fn as_ref(&self) -> &'static i32 {
        match self {
            AuthFlavor::AuthNone => &0i32,
            AuthFlavor::AuthSys => &1i32,
            AuthFlavor::AuthShort => &2i32,
            AuthFlavor::AuthDh => &3i32,
            AuthFlavor::RpcsecGss => &6i32,
        }
    }
}
impl TryFrom<i32> for AuthFlavor {
    type Error = serde_xdr::error::Error;
    fn try_from(v: i32) -> Result<Self, serde_xdr::error::Error> {
        match v {
            0i32 => Ok(AuthFlavor::AuthNone),
            1i32 => Ok(AuthFlavor::AuthSys),
            2i32 => Ok(AuthFlavor::AuthShort),
            3i32 => Ok(AuthFlavor::AuthDh),
            6i32 => Ok(AuthFlavor::RpcsecGss),
            _ => Err(serde_xdr::error::Error::Convert),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OpaqueAuth {
    #[serde(with = "serde_xdr::primitive::signed32")]
    pub flavor: AuthFlavor,
    #[serde(with = "serde_xdr::opaque::variable")]
    pub body: Vec<u8>,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[repr(i32)]
pub enum MsgType {
    CALL = 0i32,
    REPLY = 1i32,
}
impl Default for MsgType {
    fn default() -> Self {
        MsgType::CALL
    }
}
impl AsRef<i32> for MsgType {
    fn as_ref(&self) -> &'static i32 {
        match self {
            MsgType::CALL => &0i32,
            MsgType::REPLY => &1i32,
        }
    }
}
impl TryFrom<i32> for MsgType {
    type Error = serde_xdr::error::Error;
    fn try_from(v: i32) -> Result<Self, serde_xdr::error::Error> {
        match v {
            0i32 => Ok(MsgType::CALL),
            1i32 => Ok(MsgType::REPLY),
            _ => Err(serde_xdr::error::Error::Convert),
        }
    }
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[repr(i32)]
pub enum ReplyStat {
    MsgAccepted = 0i32,
    MsgDenied = 1i32,
}
impl Default for ReplyStat {
    fn default() -> Self {
        ReplyStat::MsgAccepted
    }
}
impl AsRef<i32> for ReplyStat {
    fn as_ref(&self) -> &'static i32 {
        match self {
            ReplyStat::MsgAccepted => &0i32,
            ReplyStat::MsgDenied => &1i32,
        }
    }
}
impl TryFrom<i32> for ReplyStat {
    type Error = serde_xdr::error::Error;
    fn try_from(v: i32) -> Result<Self, serde_xdr::error::Error> {
        match v {
            0i32 => Ok(ReplyStat::MsgAccepted),
            1i32 => Ok(ReplyStat::MsgDenied),
            _ => Err(serde_xdr::error::Error::Convert),
        }
    }
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[repr(i32)]
pub enum AcceptStat {
    SUCCESS = 0i32,
    ProgUnavail = 1i32,
    ProgMismatch = 2i32,
    ProcUnavail = 3i32,
    GarbageArgs = 4i32,
    SystemErr = 5i32,
}
impl Default for AcceptStat {
    fn default() -> Self {
        AcceptStat::SUCCESS
    }
}
impl AsRef<i32> for AcceptStat {
    fn as_ref(&self) -> &'static i32 {
        match self {
            AcceptStat::SUCCESS => &0i32,
            AcceptStat::ProgUnavail => &1i32,
            AcceptStat::ProgMismatch => &2i32,
            AcceptStat::ProcUnavail => &3i32,
            AcceptStat::GarbageArgs => &4i32,
            AcceptStat::SystemErr => &5i32,
        }
    }
}
impl TryFrom<i32> for AcceptStat {
    type Error = serde_xdr::error::Error;
    fn try_from(v: i32) -> Result<Self, serde_xdr::error::Error> {
        match v {
            0i32 => Ok(AcceptStat::SUCCESS),
            1i32 => Ok(AcceptStat::ProgUnavail),
            2i32 => Ok(AcceptStat::ProgMismatch),
            3i32 => Ok(AcceptStat::ProcUnavail),
            4i32 => Ok(AcceptStat::GarbageArgs),
            5i32 => Ok(AcceptStat::SystemErr),
            _ => Err(serde_xdr::error::Error::Convert),
        }
    }
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[repr(i32)]
pub enum RejectStat {
    RpcMismatch = 0i32,
    AuthError = 1i32,
}
impl Default for RejectStat {
    fn default() -> Self {
        RejectStat::RpcMismatch
    }
}
impl AsRef<i32> for RejectStat {
    fn as_ref(&self) -> &'static i32 {
        match self {
            RejectStat::RpcMismatch => &0i32,
            RejectStat::AuthError => &1i32,
        }
    }
}
impl TryFrom<i32> for RejectStat {
    type Error = serde_xdr::error::Error;
    fn try_from(v: i32) -> Result<Self, serde_xdr::error::Error> {
        match v {
            0i32 => Ok(RejectStat::RpcMismatch),
            1i32 => Ok(RejectStat::AuthError),
            _ => Err(serde_xdr::error::Error::Convert),
        }
    }
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[repr(i32)]
pub enum AuthStat {
    AuthOk = 0i32,
    AuthBadcred = 1i32,
    AuthRejectedcred = 2i32,
    AuthBadverf = 3i32,
    AuthRejectedverf = 4i32,
    AuthTooweak = 5i32,
    AuthInvalidresp = 6i32,
    AuthFailed = 7i32,
    AuthKerbGeneric = 8i32,
    AuthTimeexpire = 9i32,
    AuthTktFile = 10i32,
    AuthDecode = 11i32,
    AuthNetAddr = 12i32,
    RpcsecGssCredproblem = 13i32,
    RpcsecGssCtxproblem = 14i32,
}
impl Default for AuthStat {
    fn default() -> Self {
        AuthStat::AuthOk
    }
}
impl AsRef<i32> for AuthStat {
    fn as_ref(&self) -> &'static i32 {
        match self {
            AuthStat::AuthOk => &0i32,
            AuthStat::AuthBadcred => &1i32,
            AuthStat::AuthRejectedcred => &2i32,
            AuthStat::AuthBadverf => &3i32,
            AuthStat::AuthRejectedverf => &4i32,
            AuthStat::AuthTooweak => &5i32,
            AuthStat::AuthInvalidresp => &6i32,
            AuthStat::AuthFailed => &7i32,
            AuthStat::AuthKerbGeneric => &8i32,
            AuthStat::AuthTimeexpire => &9i32,
            AuthStat::AuthTktFile => &10i32,
            AuthStat::AuthDecode => &11i32,
            AuthStat::AuthNetAddr => &12i32,
            AuthStat::RpcsecGssCredproblem => &13i32,
            AuthStat::RpcsecGssCtxproblem => &14i32,
        }
    }
}
impl TryFrom<i32> for AuthStat {
    type Error = serde_xdr::error::Error;
    fn try_from(v: i32) -> Result<Self, serde_xdr::error::Error> {
        match v {
            0i32 => Ok(AuthStat::AuthOk),
            1i32 => Ok(AuthStat::AuthBadcred),
            2i32 => Ok(AuthStat::AuthRejectedcred),
            3i32 => Ok(AuthStat::AuthBadverf),
            4i32 => Ok(AuthStat::AuthRejectedverf),
            5i32 => Ok(AuthStat::AuthTooweak),
            6i32 => Ok(AuthStat::AuthInvalidresp),
            7i32 => Ok(AuthStat::AuthFailed),
            8i32 => Ok(AuthStat::AuthKerbGeneric),
            9i32 => Ok(AuthStat::AuthTimeexpire),
            10i32 => Ok(AuthStat::AuthTktFile),
            11i32 => Ok(AuthStat::AuthDecode),
            12i32 => Ok(AuthStat::AuthNetAddr),
            13i32 => Ok(AuthStat::RpcsecGssCredproblem),
            14i32 => Ok(AuthStat::RpcsecGssCtxproblem),
            _ => Err(serde_xdr::error::Error::Convert),
        }
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum RpcMsgBody {
    CALL(CallBody),
    REPLY(ReplyBody),
}
impl Default for RpcMsgBody {
    fn default() -> Self {
        RpcMsgBody::CALL(Default::default())
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RpcMsg {
    pub xid: u32,
    pub body: RpcMsgBody,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CallBody {
    pub rpcvers: u32,
    pub prog: u32,
    pub vers: u32,
    pub proc: u32,
    pub cred: OpaqueAuth,
    pub verf: OpaqueAuth,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ReplyBody {
    MsgAccepted(AcceptedReply),
    MsgDenied(RejectedReply),
}
impl Default for ReplyBody {
    fn default() -> Self {
        ReplyBody::MsgAccepted(Default::default())
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AcceptedReplyMismatchInfo {
    pub low: u32,
    pub high: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum AcceptedReplyData {
    #[serde(with = "serde_xdr::opaque::fixed")]
    SUCCESS([u8; 0usize as usize]),
    _Reserved1,
    ProgMismatch(AcceptedReplyMismatchInfo),
    Default,
}
impl Default for AcceptedReplyData {
    fn default() -> Self {
        AcceptedReplyData::Default
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AcceptedReply {
    pub verf: OpaqueAuth,
    pub reply_data: AcceptedReplyData,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RejectedReplyMismatchInfo {
    pub low: u32,
    pub high: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum RejectedReply {
    RpcMismatch(RejectedReplyMismatchInfo),
    AuthError(AuthStat),
}
impl Default for RejectedReply {
    fn default() -> Self {
        RejectedReply::RpcMismatch(Default::default())
    }
}
