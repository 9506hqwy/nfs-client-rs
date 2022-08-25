use serde::ser::SerializeTuple;
use serde::{Deserialize, Serialize};
use serde_xdr::XdrIndexer;
use serde_xdr_derive::XdrIndexer;
pub const NFS4_FHSIZE: u32 = 128u32;
pub const NFS4_VERIFIER_SIZE: u32 = 8u32;
pub const NFS4_OTHER_SIZE: u32 = 12u32;
pub const NFS4_OPAQUE_LIMIT: u32 = 1024u32;
pub const NFS4_INT64_MAX: u64 = 9223372036854775807u64;
pub const NFS4_UINT64_MAX: u64 = 18446744073709551615u64;
pub const NFS4_INT32_MAX: u32 = 2147483647u32;
pub const NFS4_UINT32_MAX: u32 = 4294967295u32;
pub const ACL4_SUPPORT_ALLOW_ACL: u32 = 1u32;
pub const ACL4_SUPPORT_DENY_ACL: u32 = 2u32;
pub const ACL4_SUPPORT_AUDIT_ACL: u32 = 4u32;
pub const ACL4_SUPPORT_ALARM_ACL: u32 = 8u32;
pub const ACE4_ACCESS_ALLOWED_ACE_TYPE: u32 = 0u32;
pub const ACE4_ACCESS_DENIED_ACE_TYPE: u32 = 1u32;
pub const ACE4_SYSTEM_AUDIT_ACE_TYPE: u32 = 2u32;
pub const ACE4_SYSTEM_ALARM_ACE_TYPE: u32 = 3u32;
pub const ACE4_FILE_INHERIT_ACE: u32 = 1u32;
pub const ACE4_DIRECTORY_INHERIT_ACE: u32 = 2u32;
pub const ACE4_NO_PROPAGATE_INHERIT_ACE: u32 = 4u32;
pub const ACE4_INHERIT_ONLY_ACE: u32 = 8u32;
pub const ACE4_SUCCESSFUL_ACCESS_ACE_FLAG: u32 = 16u32;
pub const ACE4_FAILED_ACCESS_ACE_FLAG: u32 = 32u32;
pub const ACE4_IDENTIFIER_GROUP: u32 = 64u32;
pub const ACE4_READ_DATA: u32 = 1u32;
pub const ACE4_LIST_DIRECTORY: u32 = 1u32;
pub const ACE4_WRITE_DATA: u32 = 2u32;
pub const ACE4_ADD_FILE: u32 = 2u32;
pub const ACE4_APPEND_DATA: u32 = 4u32;
pub const ACE4_ADD_SUBDIRECTORY: u32 = 4u32;
pub const ACE4_READ_NAMED_ATTRS: u32 = 8u32;
pub const ACE4_WRITE_NAMED_ATTRS: u32 = 16u32;
pub const ACE4_EXECUTE: u32 = 32u32;
pub const ACE4_DELETE_CHILD: u32 = 64u32;
pub const ACE4_READ_ATTRIBUTES: u32 = 128u32;
pub const ACE4_WRITE_ATTRIBUTES: u32 = 256u32;
pub const ACE4_DELETE: u32 = 65536u32;
pub const ACE4_READ_ACL: u32 = 131072u32;
pub const ACE4_WRITE_ACL: u32 = 262144u32;
pub const ACE4_WRITE_OWNER: u32 = 524288u32;
pub const ACE4_SYNCHRONIZE: u32 = 1048576u32;
pub const ACE4_GENERIC_READ: u32 = 1179777u32;
pub const ACE4_GENERIC_WRITE: u32 = 1442054u32;
pub const ACE4_GENERIC_EXECUTE: u32 = 1179808u32;
pub const MODE4_SUID: u32 = 2048u32;
pub const MODE4_SGID: u32 = 1024u32;
pub const MODE4_SVTX: u32 = 512u32;
pub const MODE4_RUSR: u32 = 256u32;
pub const MODE4_WUSR: u32 = 128u32;
pub const MODE4_XUSR: u32 = 64u32;
pub const MODE4_RGRP: u32 = 32u32;
pub const MODE4_WGRP: u32 = 16u32;
pub const MODE4_XGRP: u32 = 8u32;
pub const MODE4_ROTH: u32 = 4u32;
pub const MODE4_WOTH: u32 = 2u32;
pub const MODE4_XOTH: u32 = 1u32;
pub const FH4_PERSISTENT: u32 = 0u32;
pub const FH4_NOEXPIRE_WITH_OPEN: u32 = 1u32;
pub const FH4_VOLATILE_ANY: u32 = 2u32;
pub const FH4_VOL_MIGRATION: u32 = 4u32;
pub const FH4_VOL_RENAME: u32 = 8u32;
pub const FATTR4_SUPPORTED_ATTRS: u32 = 0u32;
pub const FATTR4_TYPE: u32 = 1u32;
pub const FATTR4_FH_EXPIRE_TYPE: u32 = 2u32;
pub const FATTR4_CHANGE: u32 = 3u32;
pub const FATTR4_SIZE: u32 = 4u32;
pub const FATTR4_LINK_SUPPORT: u32 = 5u32;
pub const FATTR4_SYMLINK_SUPPORT: u32 = 6u32;
pub const FATTR4_NAMED_ATTR: u32 = 7u32;
pub const FATTR4_FSID: u32 = 8u32;
pub const FATTR4_UNIQUE_HANDLES: u32 = 9u32;
pub const FATTR4_LEASE_TIME: u32 = 10u32;
pub const FATTR4_RDATTR_ERROR: u32 = 11u32;
pub const FATTR4_FILEHANDLE: u32 = 19u32;
pub const FATTR4_ACL: u32 = 12u32;
pub const FATTR4_ACLSUPPORT: u32 = 13u32;
pub const FATTR4_ARCHIVE: u32 = 14u32;
pub const FATTR4_CANSETTIME: u32 = 15u32;
pub const FATTR4_CASE_INSENSITIVE: u32 = 16u32;
pub const FATTR4_CASE_PRESERVING: u32 = 17u32;
pub const FATTR4_CHOWN_RESTRICTED: u32 = 18u32;
pub const FATTR4_FILEID: u32 = 20u32;
pub const FATTR4_FILES_AVAIL: u32 = 21u32;
pub const FATTR4_FILES_FREE: u32 = 22u32;
pub const FATTR4_FILES_TOTAL: u32 = 23u32;
pub const FATTR4_FS_LOCATIONS: u32 = 24u32;
pub const FATTR4_HIDDEN: u32 = 25u32;
pub const FATTR4_HOMOGENEOUS: u32 = 26u32;
pub const FATTR4_MAXFILESIZE: u32 = 27u32;
pub const FATTR4_MAXLINK: u32 = 28u32;
pub const FATTR4_MAXNAME: u32 = 29u32;
pub const FATTR4_MAXREAD: u32 = 30u32;
pub const FATTR4_MAXWRITE: u32 = 31u32;
pub const FATTR4_MIMETYPE: u32 = 32u32;
pub const FATTR4_MODE: u32 = 33u32;
pub const FATTR4_NO_TRUNC: u32 = 34u32;
pub const FATTR4_NUMLINKS: u32 = 35u32;
pub const FATTR4_OWNER: u32 = 36u32;
pub const FATTR4_OWNER_GROUP: u32 = 37u32;
pub const FATTR4_QUOTA_AVAIL_HARD: u32 = 38u32;
pub const FATTR4_QUOTA_AVAIL_SOFT: u32 = 39u32;
pub const FATTR4_QUOTA_USED: u32 = 40u32;
pub const FATTR4_RAWDEV: u32 = 41u32;
pub const FATTR4_SPACE_AVAIL: u32 = 42u32;
pub const FATTR4_SPACE_FREE: u32 = 43u32;
pub const FATTR4_SPACE_TOTAL: u32 = 44u32;
pub const FATTR4_SPACE_USED: u32 = 45u32;
pub const FATTR4_SYSTEM: u32 = 46u32;
pub const FATTR4_TIME_ACCESS: u32 = 47u32;
pub const FATTR4_TIME_ACCESS_SET: u32 = 48u32;
pub const FATTR4_TIME_BACKUP: u32 = 49u32;
pub const FATTR4_TIME_CREATE: u32 = 50u32;
pub const FATTR4_TIME_DELTA: u32 = 51u32;
pub const FATTR4_TIME_METADATA: u32 = 52u32;
pub const FATTR4_TIME_MODIFY: u32 = 53u32;
pub const FATTR4_TIME_MODIFY_SET: u32 = 54u32;
pub const FATTR4_MOUNTED_ON_FILEID: u32 = 55u32;
pub const ACCESS4_READ: u32 = 1u32;
pub const ACCESS4_LOOKUP: u32 = 2u32;
pub const ACCESS4_MODIFY: u32 = 4u32;
pub const ACCESS4_EXTEND: u32 = 8u32;
pub const ACCESS4_DELETE: u32 = 16u32;
pub const ACCESS4_EXECUTE: u32 = 32u32;
pub const OPEN4_SHARE_ACCESS_READ: u32 = 1u32;
pub const OPEN4_SHARE_ACCESS_WRITE: u32 = 2u32;
pub const OPEN4_SHARE_ACCESS_BOTH: u32 = 3u32;
pub const OPEN4_SHARE_DENY_NONE: u32 = 0u32;
pub const OPEN4_SHARE_DENY_READ: u32 = 1u32;
pub const OPEN4_SHARE_DENY_WRITE: u32 = 2u32;
pub const OPEN4_SHARE_DENY_BOTH: u32 = 3u32;
pub const OPEN4_RESULT_CONFIRM: u32 = 2u32;
pub const OPEN4_RESULT_LOCKTYPE_POSIX: u32 = 4u32;
pub const RPCSEC_GSS: u32 = 6u32;
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[repr(i32)]
pub enum NfsFtype4 {
    NF4REG = 1i32,
    NF4DIR = 2i32,
    NF4BLK = 3i32,
    NF4CHR = 4i32,
    NF4LNK = 5i32,
    NF4SOCK = 6i32,
    NF4FIFO = 7i32,
    NF4ATTRDIR = 8i32,
    NF4NAMEDATTR = 9i32,
}
impl Default for NfsFtype4 {
    fn default() -> Self {
        NfsFtype4::NF4REG
    }
}
impl AsRef<i32> for NfsFtype4 {
    fn as_ref(&self) -> &'static i32 {
        match self {
            NfsFtype4::NF4REG => &1i32,
            NfsFtype4::NF4DIR => &2i32,
            NfsFtype4::NF4BLK => &3i32,
            NfsFtype4::NF4CHR => &4i32,
            NfsFtype4::NF4LNK => &5i32,
            NfsFtype4::NF4SOCK => &6i32,
            NfsFtype4::NF4FIFO => &7i32,
            NfsFtype4::NF4ATTRDIR => &8i32,
            NfsFtype4::NF4NAMEDATTR => &9i32,
        }
    }
}
impl TryFrom<i32> for NfsFtype4 {
    type Error = serde_xdr::error::Error;
    fn try_from(v: i32) -> Result<Self, serde_xdr::error::Error> {
        match v {
            1i32 => Ok(NfsFtype4::NF4REG),
            2i32 => Ok(NfsFtype4::NF4DIR),
            3i32 => Ok(NfsFtype4::NF4BLK),
            4i32 => Ok(NfsFtype4::NF4CHR),
            5i32 => Ok(NfsFtype4::NF4LNK),
            6i32 => Ok(NfsFtype4::NF4SOCK),
            7i32 => Ok(NfsFtype4::NF4FIFO),
            8i32 => Ok(NfsFtype4::NF4ATTRDIR),
            9i32 => Ok(NfsFtype4::NF4NAMEDATTR),
            _ => Err(serde_xdr::error::Error::Convert),
        }
    }
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[repr(i32)]
pub enum Nfsstat4 {
    Nfs4Ok = 0i32,
    Nfs4errPerm = 1i32,
    Nfs4errNoent = 2i32,
    Nfs4errIo = 5i32,
    Nfs4errNxio = 6i32,
    Nfs4errAccess = 13i32,
    Nfs4errExist = 17i32,
    Nfs4errXdev = 18i32,
    Nfs4errNotdir = 20i32,
    Nfs4errIsdir = 21i32,
    Nfs4errInval = 22i32,
    Nfs4errFbig = 27i32,
    Nfs4errNospc = 28i32,
    Nfs4errRofs = 30i32,
    Nfs4errMlink = 31i32,
    Nfs4errNametoolong = 63i32,
    Nfs4errNotempty = 66i32,
    Nfs4errDquot = 69i32,
    Nfs4errStale = 70i32,
    Nfs4errBadhandle = 10001i32,
    Nfs4errBadCookie = 10003i32,
    Nfs4errNotsupp = 10004i32,
    Nfs4errToosmall = 10005i32,
    Nfs4errServerfault = 10006i32,
    Nfs4errBadtype = 10007i32,
    Nfs4errDelay = 10008i32,
    Nfs4errSame = 10009i32,
    Nfs4errDenied = 10010i32,
    Nfs4errExpired = 10011i32,
    Nfs4errLocked = 10012i32,
    Nfs4errGrace = 10013i32,
    Nfs4errFhexpired = 10014i32,
    Nfs4errShareDenied = 10015i32,
    Nfs4errWrongsec = 10016i32,
    Nfs4errClidInuse = 10017i32,
    Nfs4errResource = 10018i32,
    Nfs4errMoved = 10019i32,
    Nfs4errNofilehandle = 10020i32,
    Nfs4errMinorVersMismatch = 10021i32,
    Nfs4errStaleClientid = 10022i32,
    Nfs4errStaleStateid = 10023i32,
    Nfs4errOldStateid = 10024i32,
    Nfs4errBadStateid = 10025i32,
    Nfs4errBadSeqid = 10026i32,
    Nfs4errNotSame = 10027i32,
    Nfs4errLockRange = 10028i32,
    Nfs4errSymlink = 10029i32,
    Nfs4errRestorefh = 10030i32,
    Nfs4errLeaseMoved = 10031i32,
    Nfs4errAttrnotsupp = 10032i32,
    Nfs4errNoGrace = 10033i32,
    Nfs4errReclaimBad = 10034i32,
    Nfs4errReclaimConflict = 10035i32,
    Nfs4errBadxdr = 10036i32,
    Nfs4errLocksHeld = 10037i32,
    Nfs4errOpenmode = 10038i32,
    Nfs4errBadowner = 10039i32,
    Nfs4errBadchar = 10040i32,
    Nfs4errBadname = 10041i32,
    Nfs4errBadRange = 10042i32,
    Nfs4errLockNotsupp = 10043i32,
    Nfs4errOpIllegal = 10044i32,
    Nfs4errDeadlock = 10045i32,
    Nfs4errFileOpen = 10046i32,
    Nfs4errAdminRevoked = 10047i32,
    Nfs4errCbPathDown = 10048i32,
}
impl Default for Nfsstat4 {
    fn default() -> Self {
        Nfsstat4::Nfs4Ok
    }
}
impl AsRef<i32> for Nfsstat4 {
    fn as_ref(&self) -> &'static i32 {
        match self {
            Nfsstat4::Nfs4Ok => &0i32,
            Nfsstat4::Nfs4errPerm => &1i32,
            Nfsstat4::Nfs4errNoent => &2i32,
            Nfsstat4::Nfs4errIo => &5i32,
            Nfsstat4::Nfs4errNxio => &6i32,
            Nfsstat4::Nfs4errAccess => &13i32,
            Nfsstat4::Nfs4errExist => &17i32,
            Nfsstat4::Nfs4errXdev => &18i32,
            Nfsstat4::Nfs4errNotdir => &20i32,
            Nfsstat4::Nfs4errIsdir => &21i32,
            Nfsstat4::Nfs4errInval => &22i32,
            Nfsstat4::Nfs4errFbig => &27i32,
            Nfsstat4::Nfs4errNospc => &28i32,
            Nfsstat4::Nfs4errRofs => &30i32,
            Nfsstat4::Nfs4errMlink => &31i32,
            Nfsstat4::Nfs4errNametoolong => &63i32,
            Nfsstat4::Nfs4errNotempty => &66i32,
            Nfsstat4::Nfs4errDquot => &69i32,
            Nfsstat4::Nfs4errStale => &70i32,
            Nfsstat4::Nfs4errBadhandle => &10001i32,
            Nfsstat4::Nfs4errBadCookie => &10003i32,
            Nfsstat4::Nfs4errNotsupp => &10004i32,
            Nfsstat4::Nfs4errToosmall => &10005i32,
            Nfsstat4::Nfs4errServerfault => &10006i32,
            Nfsstat4::Nfs4errBadtype => &10007i32,
            Nfsstat4::Nfs4errDelay => &10008i32,
            Nfsstat4::Nfs4errSame => &10009i32,
            Nfsstat4::Nfs4errDenied => &10010i32,
            Nfsstat4::Nfs4errExpired => &10011i32,
            Nfsstat4::Nfs4errLocked => &10012i32,
            Nfsstat4::Nfs4errGrace => &10013i32,
            Nfsstat4::Nfs4errFhexpired => &10014i32,
            Nfsstat4::Nfs4errShareDenied => &10015i32,
            Nfsstat4::Nfs4errWrongsec => &10016i32,
            Nfsstat4::Nfs4errClidInuse => &10017i32,
            Nfsstat4::Nfs4errResource => &10018i32,
            Nfsstat4::Nfs4errMoved => &10019i32,
            Nfsstat4::Nfs4errNofilehandle => &10020i32,
            Nfsstat4::Nfs4errMinorVersMismatch => &10021i32,
            Nfsstat4::Nfs4errStaleClientid => &10022i32,
            Nfsstat4::Nfs4errStaleStateid => &10023i32,
            Nfsstat4::Nfs4errOldStateid => &10024i32,
            Nfsstat4::Nfs4errBadStateid => &10025i32,
            Nfsstat4::Nfs4errBadSeqid => &10026i32,
            Nfsstat4::Nfs4errNotSame => &10027i32,
            Nfsstat4::Nfs4errLockRange => &10028i32,
            Nfsstat4::Nfs4errSymlink => &10029i32,
            Nfsstat4::Nfs4errRestorefh => &10030i32,
            Nfsstat4::Nfs4errLeaseMoved => &10031i32,
            Nfsstat4::Nfs4errAttrnotsupp => &10032i32,
            Nfsstat4::Nfs4errNoGrace => &10033i32,
            Nfsstat4::Nfs4errReclaimBad => &10034i32,
            Nfsstat4::Nfs4errReclaimConflict => &10035i32,
            Nfsstat4::Nfs4errBadxdr => &10036i32,
            Nfsstat4::Nfs4errLocksHeld => &10037i32,
            Nfsstat4::Nfs4errOpenmode => &10038i32,
            Nfsstat4::Nfs4errBadowner => &10039i32,
            Nfsstat4::Nfs4errBadchar => &10040i32,
            Nfsstat4::Nfs4errBadname => &10041i32,
            Nfsstat4::Nfs4errBadRange => &10042i32,
            Nfsstat4::Nfs4errLockNotsupp => &10043i32,
            Nfsstat4::Nfs4errOpIllegal => &10044i32,
            Nfsstat4::Nfs4errDeadlock => &10045i32,
            Nfsstat4::Nfs4errFileOpen => &10046i32,
            Nfsstat4::Nfs4errAdminRevoked => &10047i32,
            Nfsstat4::Nfs4errCbPathDown => &10048i32,
        }
    }
}
impl TryFrom<i32> for Nfsstat4 {
    type Error = serde_xdr::error::Error;
    fn try_from(v: i32) -> Result<Self, serde_xdr::error::Error> {
        match v {
            0i32 => Ok(Nfsstat4::Nfs4Ok),
            1i32 => Ok(Nfsstat4::Nfs4errPerm),
            2i32 => Ok(Nfsstat4::Nfs4errNoent),
            5i32 => Ok(Nfsstat4::Nfs4errIo),
            6i32 => Ok(Nfsstat4::Nfs4errNxio),
            13i32 => Ok(Nfsstat4::Nfs4errAccess),
            17i32 => Ok(Nfsstat4::Nfs4errExist),
            18i32 => Ok(Nfsstat4::Nfs4errXdev),
            20i32 => Ok(Nfsstat4::Nfs4errNotdir),
            21i32 => Ok(Nfsstat4::Nfs4errIsdir),
            22i32 => Ok(Nfsstat4::Nfs4errInval),
            27i32 => Ok(Nfsstat4::Nfs4errFbig),
            28i32 => Ok(Nfsstat4::Nfs4errNospc),
            30i32 => Ok(Nfsstat4::Nfs4errRofs),
            31i32 => Ok(Nfsstat4::Nfs4errMlink),
            63i32 => Ok(Nfsstat4::Nfs4errNametoolong),
            66i32 => Ok(Nfsstat4::Nfs4errNotempty),
            69i32 => Ok(Nfsstat4::Nfs4errDquot),
            70i32 => Ok(Nfsstat4::Nfs4errStale),
            10001i32 => Ok(Nfsstat4::Nfs4errBadhandle),
            10003i32 => Ok(Nfsstat4::Nfs4errBadCookie),
            10004i32 => Ok(Nfsstat4::Nfs4errNotsupp),
            10005i32 => Ok(Nfsstat4::Nfs4errToosmall),
            10006i32 => Ok(Nfsstat4::Nfs4errServerfault),
            10007i32 => Ok(Nfsstat4::Nfs4errBadtype),
            10008i32 => Ok(Nfsstat4::Nfs4errDelay),
            10009i32 => Ok(Nfsstat4::Nfs4errSame),
            10010i32 => Ok(Nfsstat4::Nfs4errDenied),
            10011i32 => Ok(Nfsstat4::Nfs4errExpired),
            10012i32 => Ok(Nfsstat4::Nfs4errLocked),
            10013i32 => Ok(Nfsstat4::Nfs4errGrace),
            10014i32 => Ok(Nfsstat4::Nfs4errFhexpired),
            10015i32 => Ok(Nfsstat4::Nfs4errShareDenied),
            10016i32 => Ok(Nfsstat4::Nfs4errWrongsec),
            10017i32 => Ok(Nfsstat4::Nfs4errClidInuse),
            10018i32 => Ok(Nfsstat4::Nfs4errResource),
            10019i32 => Ok(Nfsstat4::Nfs4errMoved),
            10020i32 => Ok(Nfsstat4::Nfs4errNofilehandle),
            10021i32 => Ok(Nfsstat4::Nfs4errMinorVersMismatch),
            10022i32 => Ok(Nfsstat4::Nfs4errStaleClientid),
            10023i32 => Ok(Nfsstat4::Nfs4errStaleStateid),
            10024i32 => Ok(Nfsstat4::Nfs4errOldStateid),
            10025i32 => Ok(Nfsstat4::Nfs4errBadStateid),
            10026i32 => Ok(Nfsstat4::Nfs4errBadSeqid),
            10027i32 => Ok(Nfsstat4::Nfs4errNotSame),
            10028i32 => Ok(Nfsstat4::Nfs4errLockRange),
            10029i32 => Ok(Nfsstat4::Nfs4errSymlink),
            10030i32 => Ok(Nfsstat4::Nfs4errRestorefh),
            10031i32 => Ok(Nfsstat4::Nfs4errLeaseMoved),
            10032i32 => Ok(Nfsstat4::Nfs4errAttrnotsupp),
            10033i32 => Ok(Nfsstat4::Nfs4errNoGrace),
            10034i32 => Ok(Nfsstat4::Nfs4errReclaimBad),
            10035i32 => Ok(Nfsstat4::Nfs4errReclaimConflict),
            10036i32 => Ok(Nfsstat4::Nfs4errBadxdr),
            10037i32 => Ok(Nfsstat4::Nfs4errLocksHeld),
            10038i32 => Ok(Nfsstat4::Nfs4errOpenmode),
            10039i32 => Ok(Nfsstat4::Nfs4errBadowner),
            10040i32 => Ok(Nfsstat4::Nfs4errBadchar),
            10041i32 => Ok(Nfsstat4::Nfs4errBadname),
            10042i32 => Ok(Nfsstat4::Nfs4errBadRange),
            10043i32 => Ok(Nfsstat4::Nfs4errLockNotsupp),
            10044i32 => Ok(Nfsstat4::Nfs4errOpIllegal),
            10045i32 => Ok(Nfsstat4::Nfs4errDeadlock),
            10046i32 => Ok(Nfsstat4::Nfs4errFileOpen),
            10047i32 => Ok(Nfsstat4::Nfs4errAdminRevoked),
            10048i32 => Ok(Nfsstat4::Nfs4errCbPathDown),
            _ => Err(serde_xdr::error::Error::Convert),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Nfstime4 {
    pub seconds: i64,
    pub nseconds: u32,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[repr(i32)]
pub enum TimeHow4 {
    SetToServerTime4 = 0i32,
    SetToClientTime4 = 1i32,
}
impl Default for TimeHow4 {
    fn default() -> Self {
        TimeHow4::SetToServerTime4
    }
}
impl AsRef<i32> for TimeHow4 {
    fn as_ref(&self) -> &'static i32 {
        match self {
            TimeHow4::SetToServerTime4 => &0i32,
            TimeHow4::SetToClientTime4 => &1i32,
        }
    }
}
impl TryFrom<i32> for TimeHow4 {
    type Error = serde_xdr::error::Error;
    fn try_from(v: i32) -> Result<Self, serde_xdr::error::Error> {
        match v {
            0i32 => Ok(TimeHow4::SetToServerTime4),
            1i32 => Ok(TimeHow4::SetToClientTime4),
            _ => Err(serde_xdr::error::Error::Convert),
        }
    }
}
#[derive(Clone, Debug, XdrIndexer)]
pub enum Settime4 {
    SetToClientTime4(Nfstime4),
    Default,
}
impl Default for Settime4 {
    fn default() -> Self {
        Settime4::Default
    }
}
impl XdrIndexer for Settime4 {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            1i32 => Ok(stringify!(SetToClientTime4)),
            _ => Ok(stringify!(Default)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            Settime4::SetToClientTime4(_) => 1i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Fsid4 {
    pub major: u64,
    pub minor: u64,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct FsLocation4 {
    pub server: Vec<String>,
    pub rootpath: Vec<String>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct FsLocations4 {
    pub fs_root: Vec<String>,
    pub locations: Vec<FsLocation4>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Nfsace4 {
    pub r#type: u32,
    pub flag: u32,
    pub access_mask: u32,
    pub who: String,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Specdata4 {
    pub specdata1: u32,
    pub specdata2: u32,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Fattr4 {
    pub attrmask: Vec<u32>,
    #[serde(with = "serde_xdr::opaque::variable")]
    pub attr_vals: Vec<u8>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ChangeInfo4 {
    pub atomic: bool,
    pub before: u64,
    pub after: u64,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Clientaddr4 {
    pub r_netid: String,
    pub r_addr: String,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CbClient4 {
    pub cb_program: u32,
    pub cb_location: Clientaddr4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Stateid4 {
    pub seqid: u32,
    #[serde(with = "serde_xdr::opaque::fixed")]
    pub other: [u8; NFS4_OTHER_SIZE as usize],
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct NfsClientId4 {
    #[serde(with = "serde_xdr::opaque::fixed")]
    pub verifier: [u8; NFS4_VERIFIER_SIZE as usize],
    #[serde(with = "serde_xdr::opaque::variable")]
    pub id: Vec<u8>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OpenOwner4 {
    pub clientid: u64,
    #[serde(with = "serde_xdr::opaque::variable")]
    pub owner: Vec<u8>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LockOwner4 {
    pub clientid: u64,
    #[serde(with = "serde_xdr::opaque::variable")]
    pub owner: Vec<u8>,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[repr(i32)]
pub enum NfsLockType4 {
    ReadLt = 1i32,
    WriteLt = 2i32,
    ReadwLt = 3i32,
    WritewLt = 4i32,
}
impl Default for NfsLockType4 {
    fn default() -> Self {
        NfsLockType4::ReadLt
    }
}
impl AsRef<i32> for NfsLockType4 {
    fn as_ref(&self) -> &'static i32 {
        match self {
            NfsLockType4::ReadLt => &1i32,
            NfsLockType4::WriteLt => &2i32,
            NfsLockType4::ReadwLt => &3i32,
            NfsLockType4::WritewLt => &4i32,
        }
    }
}
impl TryFrom<i32> for NfsLockType4 {
    type Error = serde_xdr::error::Error;
    fn try_from(v: i32) -> Result<Self, serde_xdr::error::Error> {
        match v {
            1i32 => Ok(NfsLockType4::ReadLt),
            2i32 => Ok(NfsLockType4::WriteLt),
            3i32 => Ok(NfsLockType4::ReadwLt),
            4i32 => Ok(NfsLockType4::WritewLt),
            _ => Err(serde_xdr::error::Error::Convert),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ACCESS4args {
    pub access: u32,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ACCESS4resok {
    pub supported: u32,
    pub access: u32,
}
#[derive(Clone, Debug, XdrIndexer)]
pub enum ACCESS4res {
    Nfs4Ok(ACCESS4resok),
    Default,
}
impl Default for ACCESS4res {
    fn default() -> Self {
        ACCESS4res::Default
    }
}
impl XdrIndexer for ACCESS4res {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Nfs4Ok)),
            _ => Ok(stringify!(Default)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            ACCESS4res::Nfs4Ok(_) => 0i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CLOSE4args {
    pub seqid: u32,
    pub open_stateid: Stateid4,
}
#[derive(Clone, Debug, XdrIndexer)]
pub enum CLOSE4res {
    Nfs4Ok(Stateid4),
    Default,
}
impl Default for CLOSE4res {
    fn default() -> Self {
        CLOSE4res::Default
    }
}
impl XdrIndexer for CLOSE4res {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Nfs4Ok)),
            _ => Ok(stringify!(Default)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            CLOSE4res::Nfs4Ok(_) => 0i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct COMMIT4args {
    pub offset: u64,
    pub count: u32,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct COMMIT4resok {
    #[serde(with = "serde_xdr::opaque::fixed")]
    pub writeverf: [u8; NFS4_VERIFIER_SIZE as usize],
}
#[derive(Clone, Debug, XdrIndexer)]
pub enum COMMIT4res {
    Nfs4Ok(COMMIT4resok),
    Default,
}
impl Default for COMMIT4res {
    fn default() -> Self {
        COMMIT4res::Default
    }
}
impl XdrIndexer for COMMIT4res {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Nfs4Ok)),
            _ => Ok(stringify!(Default)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            COMMIT4res::Nfs4Ok(_) => 0i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, XdrIndexer)]
pub enum Createtype4 {
    NF4DIR,
    NF4BLK(Specdata4),
    NF4CHR(Specdata4),
    NF4LNK(serde_xdr::opaque::VariableArray),
    NF4SOCK,
    NF4FIFO,
    Default,
}
impl Default for Createtype4 {
    fn default() -> Self {
        Createtype4::Default
    }
}
impl XdrIndexer for Createtype4 {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            2i32 => Ok(stringify!(NF4DIR)),
            3i32 => Ok(stringify!(NF4BLK)),
            4i32 => Ok(stringify!(NF4CHR)),
            5i32 => Ok(stringify!(NF4LNK)),
            6i32 => Ok(stringify!(NF4SOCK)),
            7i32 => Ok(stringify!(NF4FIFO)),
            _ => Ok(stringify!(Default)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            Createtype4::NF4DIR => 2i32,
            Createtype4::NF4BLK(_) => 3i32,
            Createtype4::NF4CHR(_) => 4i32,
            Createtype4::NF4LNK(_) => 5i32,
            Createtype4::NF4SOCK => 6i32,
            Createtype4::NF4FIFO => 7i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CREATE4args {
    pub objtype: Createtype4,
    pub objname: String,
    pub createattrs: Fattr4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CREATE4resok {
    pub cinfo: ChangeInfo4,
    pub attrset: Vec<u32>,
}
#[derive(Clone, Debug, XdrIndexer)]
pub enum CREATE4res {
    Nfs4Ok(CREATE4resok),
    Default,
}
impl Default for CREATE4res {
    fn default() -> Self {
        CREATE4res::Default
    }
}
impl XdrIndexer for CREATE4res {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Nfs4Ok)),
            _ => Ok(stringify!(Default)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            CREATE4res::Nfs4Ok(_) => 0i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DELEGPURGE4args {
    pub clientid: u64,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DELEGPURGE4res {
    #[serde(with = "serde_xdr::primitive::signed32")]
    pub status: Nfsstat4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DELEGRETURN4args {
    pub deleg_stateid: Stateid4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DELEGRETURN4res {
    #[serde(with = "serde_xdr::primitive::signed32")]
    pub status: Nfsstat4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct GETATTR4args {
    pub attr_request: Vec<u32>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct GETATTR4resok {
    pub obj_attributes: Fattr4,
}
#[derive(Clone, Debug, XdrIndexer)]
pub enum GETATTR4res {
    Nfs4Ok(GETATTR4resok),
    Default,
}
impl Default for GETATTR4res {
    fn default() -> Self {
        GETATTR4res::Default
    }
}
impl XdrIndexer for GETATTR4res {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Nfs4Ok)),
            _ => Ok(stringify!(Default)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            GETATTR4res::Nfs4Ok(_) => 0i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct GETFH4resok {
    #[serde(with = "serde_xdr::opaque::variable")]
    pub object: Vec<u8>,
}
#[derive(Clone, Debug, XdrIndexer)]
pub enum GETFH4res {
    Nfs4Ok(GETFH4resok),
    Default,
}
impl Default for GETFH4res {
    fn default() -> Self {
        GETFH4res::Default
    }
}
impl XdrIndexer for GETFH4res {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Nfs4Ok)),
            _ => Ok(stringify!(Default)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            GETFH4res::Nfs4Ok(_) => 0i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LINK4args {
    pub newname: String,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LINK4resok {
    pub cinfo: ChangeInfo4,
}
#[derive(Clone, Debug, XdrIndexer)]
pub enum LINK4res {
    Nfs4Ok(LINK4resok),
    Default,
}
impl Default for LINK4res {
    fn default() -> Self {
        LINK4res::Default
    }
}
impl XdrIndexer for LINK4res {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Nfs4Ok)),
            _ => Ok(stringify!(Default)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            LINK4res::Nfs4Ok(_) => 0i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OpenToLockOwner4 {
    pub open_seqid: u32,
    pub open_stateid: Stateid4,
    pub lock_seqid: u32,
    pub lock_owner: LockOwner4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ExistLockOwner4 {
    pub lock_stateid: Stateid4,
    pub lock_seqid: u32,
}
#[derive(Clone, Debug, XdrIndexer)]
pub enum Locker4 {
    FALSE(ExistLockOwner4),
    TRUE(OpenToLockOwner4),
}
impl Default for Locker4 {
    fn default() -> Self {
        Locker4::FALSE(Default::default())
    }
}
impl XdrIndexer for Locker4 {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(FALSE)),
            1i32 => Ok(stringify!(TRUE)),
            _ => Err(::serde_xdr::error::Error::Convert),
        }
    }
    fn index(&self) -> i32 {
        match self {
            Locker4::FALSE(_) => 0i32,
            Locker4::TRUE(_) => 1i32,
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LOCK4args {
    #[serde(with = "serde_xdr::primitive::signed32")]
    pub locktype: NfsLockType4,
    pub reclaim: bool,
    pub offset: u64,
    pub length: u64,
    pub locker: Locker4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LOCK4denied {
    pub offset: u64,
    pub length: u64,
    #[serde(with = "serde_xdr::primitive::signed32")]
    pub locktype: NfsLockType4,
    pub owner: LockOwner4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LOCK4resok {
    pub lock_stateid: Stateid4,
}
#[derive(Clone, Debug, XdrIndexer)]
pub enum LOCK4res {
    Nfs4Ok(LOCK4resok),
    Nfs4errDenied(LOCK4denied),
    Default,
}
impl Default for LOCK4res {
    fn default() -> Self {
        LOCK4res::Default
    }
}
impl XdrIndexer for LOCK4res {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Nfs4Ok)),
            10010i32 => Ok(stringify!(Nfs4errDenied)),
            _ => Ok(stringify!(Default)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            LOCK4res::Nfs4Ok(_) => 0i32,
            LOCK4res::Nfs4errDenied(_) => 10010i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LOCKT4args {
    #[serde(with = "serde_xdr::primitive::signed32")]
    pub locktype: NfsLockType4,
    pub offset: u64,
    pub length: u64,
    pub owner: LockOwner4,
}
#[derive(Clone, Debug, XdrIndexer)]
pub enum LOCKT4res {
    Nfs4Ok,
    Nfs4errDenied(LOCK4denied),
    Default,
}
impl Default for LOCKT4res {
    fn default() -> Self {
        LOCKT4res::Default
    }
}
impl XdrIndexer for LOCKT4res {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Nfs4Ok)),
            10010i32 => Ok(stringify!(Nfs4errDenied)),
            _ => Ok(stringify!(Default)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            LOCKT4res::Nfs4Ok => 0i32,
            LOCKT4res::Nfs4errDenied(_) => 10010i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LOCKU4args {
    #[serde(with = "serde_xdr::primitive::signed32")]
    pub locktype: NfsLockType4,
    pub seqid: u32,
    pub lock_stateid: Stateid4,
    pub offset: u64,
    pub length: u64,
}
#[derive(Clone, Debug, XdrIndexer)]
pub enum LOCKU4res {
    Nfs4Ok(Stateid4),
    Default,
}
impl Default for LOCKU4res {
    fn default() -> Self {
        LOCKU4res::Default
    }
}
impl XdrIndexer for LOCKU4res {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Nfs4Ok)),
            _ => Ok(stringify!(Default)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            LOCKU4res::Nfs4Ok(_) => 0i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LOOKUP4args {
    pub objname: String,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LOOKUP4res {
    #[serde(with = "serde_xdr::primitive::signed32")]
    pub status: Nfsstat4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LOOKUPP4res {
    #[serde(with = "serde_xdr::primitive::signed32")]
    pub status: Nfsstat4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct NVERIFY4args {
    pub obj_attributes: Fattr4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct NVERIFY4res {
    #[serde(with = "serde_xdr::primitive::signed32")]
    pub status: Nfsstat4,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[repr(i32)]
pub enum Createmode4 {
    UNCHECKED4 = 0i32,
    GUARDED4 = 1i32,
    EXCLUSIVE4 = 2i32,
}
impl Default for Createmode4 {
    fn default() -> Self {
        Createmode4::UNCHECKED4
    }
}
impl AsRef<i32> for Createmode4 {
    fn as_ref(&self) -> &'static i32 {
        match self {
            Createmode4::UNCHECKED4 => &0i32,
            Createmode4::GUARDED4 => &1i32,
            Createmode4::EXCLUSIVE4 => &2i32,
        }
    }
}
impl TryFrom<i32> for Createmode4 {
    type Error = serde_xdr::error::Error;
    fn try_from(v: i32) -> Result<Self, serde_xdr::error::Error> {
        match v {
            0i32 => Ok(Createmode4::UNCHECKED4),
            1i32 => Ok(Createmode4::GUARDED4),
            2i32 => Ok(Createmode4::EXCLUSIVE4),
            _ => Err(serde_xdr::error::Error::Convert),
        }
    }
}
#[derive(Clone, Debug, XdrIndexer)]
pub enum Createhow4 {
    UNCHECKED4(Fattr4),
    GUARDED4(Fattr4),
    EXCLUSIVE4(serde_xdr::opaque::VariableArray),
}
impl Default for Createhow4 {
    fn default() -> Self {
        Createhow4::UNCHECKED4(Default::default())
    }
}
impl XdrIndexer for Createhow4 {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(UNCHECKED4)),
            1i32 => Ok(stringify!(GUARDED4)),
            2i32 => Ok(stringify!(EXCLUSIVE4)),
            _ => Err(::serde_xdr::error::Error::Convert),
        }
    }
    fn index(&self) -> i32 {
        match self {
            Createhow4::UNCHECKED4(_) => 0i32,
            Createhow4::GUARDED4(_) => 1i32,
            Createhow4::EXCLUSIVE4(_) => 2i32,
        }
    }
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[repr(i32)]
pub enum Opentype4 {
    Open4Nocreate = 0i32,
    Open4Create = 1i32,
}
impl Default for Opentype4 {
    fn default() -> Self {
        Opentype4::Open4Nocreate
    }
}
impl AsRef<i32> for Opentype4 {
    fn as_ref(&self) -> &'static i32 {
        match self {
            Opentype4::Open4Nocreate => &0i32,
            Opentype4::Open4Create => &1i32,
        }
    }
}
impl TryFrom<i32> for Opentype4 {
    type Error = serde_xdr::error::Error;
    fn try_from(v: i32) -> Result<Self, serde_xdr::error::Error> {
        match v {
            0i32 => Ok(Opentype4::Open4Nocreate),
            1i32 => Ok(Opentype4::Open4Create),
            _ => Err(serde_xdr::error::Error::Convert),
        }
    }
}
#[derive(Clone, Debug, XdrIndexer)]
pub enum Openflag4 {
    Open4Create(Createhow4),
    Default,
}
impl Default for Openflag4 {
    fn default() -> Self {
        Openflag4::Default
    }
}
impl XdrIndexer for Openflag4 {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            1i32 => Ok(stringify!(Open4Create)),
            _ => Ok(stringify!(Default)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            Openflag4::Open4Create(_) => 1i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[repr(i32)]
pub enum LimitBy4 {
    NfsLimitSize = 1i32,
    NfsLimitBlocks = 2i32,
}
impl Default for LimitBy4 {
    fn default() -> Self {
        LimitBy4::NfsLimitSize
    }
}
impl AsRef<i32> for LimitBy4 {
    fn as_ref(&self) -> &'static i32 {
        match self {
            LimitBy4::NfsLimitSize => &1i32,
            LimitBy4::NfsLimitBlocks => &2i32,
        }
    }
}
impl TryFrom<i32> for LimitBy4 {
    type Error = serde_xdr::error::Error;
    fn try_from(v: i32) -> Result<Self, serde_xdr::error::Error> {
        match v {
            1i32 => Ok(LimitBy4::NfsLimitSize),
            2i32 => Ok(LimitBy4::NfsLimitBlocks),
            _ => Err(serde_xdr::error::Error::Convert),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct NfsModifiedLimit4 {
    pub num_blocks: u32,
    pub bytes_per_block: u32,
}
#[derive(Clone, Debug, XdrIndexer)]
pub enum NfsSpaceLimit4 {
    NfsLimitSize(u64),
    NfsLimitBlocks(NfsModifiedLimit4),
}
impl Default for NfsSpaceLimit4 {
    fn default() -> Self {
        NfsSpaceLimit4::NfsLimitSize(Default::default())
    }
}
impl XdrIndexer for NfsSpaceLimit4 {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            1i32 => Ok(stringify!(NfsLimitSize)),
            2i32 => Ok(stringify!(NfsLimitBlocks)),
            _ => Err(::serde_xdr::error::Error::Convert),
        }
    }
    fn index(&self) -> i32 {
        match self {
            NfsSpaceLimit4::NfsLimitSize(_) => 1i32,
            NfsSpaceLimit4::NfsLimitBlocks(_) => 2i32,
        }
    }
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[repr(i32)]
pub enum OpenDelegationType4 {
    OpenDelegateNone = 0i32,
    OpenDelegateRead = 1i32,
    OpenDelegateWrite = 2i32,
}
impl Default for OpenDelegationType4 {
    fn default() -> Self {
        OpenDelegationType4::OpenDelegateNone
    }
}
impl AsRef<i32> for OpenDelegationType4 {
    fn as_ref(&self) -> &'static i32 {
        match self {
            OpenDelegationType4::OpenDelegateNone => &0i32,
            OpenDelegationType4::OpenDelegateRead => &1i32,
            OpenDelegationType4::OpenDelegateWrite => &2i32,
        }
    }
}
impl TryFrom<i32> for OpenDelegationType4 {
    type Error = serde_xdr::error::Error;
    fn try_from(v: i32) -> Result<Self, serde_xdr::error::Error> {
        match v {
            0i32 => Ok(OpenDelegationType4::OpenDelegateNone),
            1i32 => Ok(OpenDelegationType4::OpenDelegateRead),
            2i32 => Ok(OpenDelegationType4::OpenDelegateWrite),
            _ => Err(serde_xdr::error::Error::Convert),
        }
    }
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[repr(i32)]
pub enum OpenClaimType4 {
    ClaimNull = 0i32,
    ClaimPrevious = 1i32,
    ClaimDelegateCur = 2i32,
    ClaimDelegatePrev = 3i32,
}
impl Default for OpenClaimType4 {
    fn default() -> Self {
        OpenClaimType4::ClaimNull
    }
}
impl AsRef<i32> for OpenClaimType4 {
    fn as_ref(&self) -> &'static i32 {
        match self {
            OpenClaimType4::ClaimNull => &0i32,
            OpenClaimType4::ClaimPrevious => &1i32,
            OpenClaimType4::ClaimDelegateCur => &2i32,
            OpenClaimType4::ClaimDelegatePrev => &3i32,
        }
    }
}
impl TryFrom<i32> for OpenClaimType4 {
    type Error = serde_xdr::error::Error;
    fn try_from(v: i32) -> Result<Self, serde_xdr::error::Error> {
        match v {
            0i32 => Ok(OpenClaimType4::ClaimNull),
            1i32 => Ok(OpenClaimType4::ClaimPrevious),
            2i32 => Ok(OpenClaimType4::ClaimDelegateCur),
            3i32 => Ok(OpenClaimType4::ClaimDelegatePrev),
            _ => Err(serde_xdr::error::Error::Convert),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OpenClaimDelegateCur4 {
    pub delegate_stateid: Stateid4,
    pub file: String,
}
#[derive(Clone, Debug, XdrIndexer)]
pub enum OpenClaim4 {
    ClaimNull(String),
    ClaimPrevious(OpenDelegationType4),
    ClaimDelegateCur(OpenClaimDelegateCur4),
    ClaimDelegatePrev(String),
}
impl Default for OpenClaim4 {
    fn default() -> Self {
        OpenClaim4::ClaimNull(Default::default())
    }
}
impl XdrIndexer for OpenClaim4 {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(ClaimNull)),
            1i32 => Ok(stringify!(ClaimPrevious)),
            2i32 => Ok(stringify!(ClaimDelegateCur)),
            3i32 => Ok(stringify!(ClaimDelegatePrev)),
            _ => Err(::serde_xdr::error::Error::Convert),
        }
    }
    fn index(&self) -> i32 {
        match self {
            OpenClaim4::ClaimNull(_) => 0i32,
            OpenClaim4::ClaimPrevious(_) => 1i32,
            OpenClaim4::ClaimDelegateCur(_) => 2i32,
            OpenClaim4::ClaimDelegatePrev(_) => 3i32,
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OPEN4args {
    pub seqid: u32,
    pub share_access: u32,
    pub share_deny: u32,
    pub owner: OpenOwner4,
    pub openhow: Openflag4,
    pub claim: OpenClaim4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OpenReadDelegation4 {
    pub stateid: Stateid4,
    pub recall: bool,
    pub permissions: Nfsace4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OpenWriteDelegation4 {
    pub stateid: Stateid4,
    pub recall: bool,
    pub space_limit: NfsSpaceLimit4,
    pub permissions: Nfsace4,
}
#[derive(Clone, Debug, XdrIndexer)]
pub enum OpenDelegation4 {
    OpenDelegateNone,
    OpenDelegateRead(OpenReadDelegation4),
    OpenDelegateWrite(OpenWriteDelegation4),
}
impl Default for OpenDelegation4 {
    fn default() -> Self {
        OpenDelegation4::OpenDelegateNone
    }
}
impl XdrIndexer for OpenDelegation4 {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(OpenDelegateNone)),
            1i32 => Ok(stringify!(OpenDelegateRead)),
            2i32 => Ok(stringify!(OpenDelegateWrite)),
            _ => Err(::serde_xdr::error::Error::Convert),
        }
    }
    fn index(&self) -> i32 {
        match self {
            OpenDelegation4::OpenDelegateNone => 0i32,
            OpenDelegation4::OpenDelegateRead(_) => 1i32,
            OpenDelegation4::OpenDelegateWrite(_) => 2i32,
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OPEN4resok {
    pub stateid: Stateid4,
    pub cinfo: ChangeInfo4,
    pub rflags: u32,
    pub attrset: Vec<u32>,
    pub delegation: OpenDelegation4,
}
#[derive(Clone, Debug, XdrIndexer)]
pub enum OPEN4res {
    Nfs4Ok(OPEN4resok),
    Default,
}
impl Default for OPEN4res {
    fn default() -> Self {
        OPEN4res::Default
    }
}
impl XdrIndexer for OPEN4res {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Nfs4Ok)),
            _ => Ok(stringify!(Default)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            OPEN4res::Nfs4Ok(_) => 0i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OPENATTR4args {
    pub createdir: bool,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OPENATTR4res {
    #[serde(with = "serde_xdr::primitive::signed32")]
    pub status: Nfsstat4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OpenConfirm4args {
    pub open_stateid: Stateid4,
    pub seqid: u32,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OpenConfirm4resok {
    pub open_stateid: Stateid4,
}
#[derive(Clone, Debug, XdrIndexer)]
pub enum OpenConfirm4res {
    Nfs4Ok(OpenConfirm4resok),
    Default,
}
impl Default for OpenConfirm4res {
    fn default() -> Self {
        OpenConfirm4res::Default
    }
}
impl XdrIndexer for OpenConfirm4res {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Nfs4Ok)),
            _ => Ok(stringify!(Default)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            OpenConfirm4res::Nfs4Ok(_) => 0i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OpenDowngrade4args {
    pub open_stateid: Stateid4,
    pub seqid: u32,
    pub share_access: u32,
    pub share_deny: u32,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OpenDowngrade4resok {
    pub open_stateid: Stateid4,
}
#[derive(Clone, Debug, XdrIndexer)]
pub enum OpenDowngrade4res {
    Nfs4Ok(OpenDowngrade4resok),
    Default,
}
impl Default for OpenDowngrade4res {
    fn default() -> Self {
        OpenDowngrade4res::Default
    }
}
impl XdrIndexer for OpenDowngrade4res {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Nfs4Ok)),
            _ => Ok(stringify!(Default)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            OpenDowngrade4res::Nfs4Ok(_) => 0i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PUTFH4args {
    #[serde(with = "serde_xdr::opaque::variable")]
    pub object: Vec<u8>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PUTFH4res {
    #[serde(with = "serde_xdr::primitive::signed32")]
    pub status: Nfsstat4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PUTPUBFH4res {
    #[serde(with = "serde_xdr::primitive::signed32")]
    pub status: Nfsstat4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PUTROOTFH4res {
    #[serde(with = "serde_xdr::primitive::signed32")]
    pub status: Nfsstat4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct READ4args {
    pub stateid: Stateid4,
    pub offset: u64,
    pub count: u32,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct READ4resok {
    pub eof: bool,
    #[serde(with = "serde_xdr::opaque::variable")]
    pub data: Vec<u8>,
}
#[derive(Clone, Debug, XdrIndexer)]
pub enum READ4res {
    Nfs4Ok(READ4resok),
    Default,
}
impl Default for READ4res {
    fn default() -> Self {
        READ4res::Default
    }
}
impl XdrIndexer for READ4res {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Nfs4Ok)),
            _ => Ok(stringify!(Default)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            READ4res::Nfs4Ok(_) => 0i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct READDIR4args {
    pub cookie: u64,
    #[serde(with = "serde_xdr::opaque::fixed")]
    pub cookieverf: [u8; NFS4_VERIFIER_SIZE as usize],
    pub dircount: u32,
    pub maxcount: u32,
    pub attr_request: Vec<u32>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Entry4 {
    pub cookie: u64,
    pub name: String,
    pub attrs: Fattr4,
    pub nextentry: Box<Option<Entry4>>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Dirlist4 {
    pub entries: Option<Entry4>,
    pub eof: bool,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct READDIR4resok {
    #[serde(with = "serde_xdr::opaque::fixed")]
    pub cookieverf: [u8; NFS4_VERIFIER_SIZE as usize],
    pub reply: Dirlist4,
}
#[derive(Clone, Debug, XdrIndexer)]
pub enum READDIR4res {
    Nfs4Ok(READDIR4resok),
    Default,
}
impl Default for READDIR4res {
    fn default() -> Self {
        READDIR4res::Default
    }
}
impl XdrIndexer for READDIR4res {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Nfs4Ok)),
            _ => Ok(stringify!(Default)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            READDIR4res::Nfs4Ok(_) => 0i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct READLINK4resok {
    #[serde(with = "serde_xdr::opaque::variable")]
    pub link: Vec<u8>,
}
#[derive(Clone, Debug, XdrIndexer)]
pub enum READLINK4res {
    Nfs4Ok(READLINK4resok),
    Default,
}
impl Default for READLINK4res {
    fn default() -> Self {
        READLINK4res::Default
    }
}
impl XdrIndexer for READLINK4res {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Nfs4Ok)),
            _ => Ok(stringify!(Default)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            READLINK4res::Nfs4Ok(_) => 0i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct REMOVE4args {
    pub target: String,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct REMOVE4resok {
    pub cinfo: ChangeInfo4,
}
#[derive(Clone, Debug, XdrIndexer)]
pub enum REMOVE4res {
    Nfs4Ok(REMOVE4resok),
    Default,
}
impl Default for REMOVE4res {
    fn default() -> Self {
        REMOVE4res::Default
    }
}
impl XdrIndexer for REMOVE4res {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Nfs4Ok)),
            _ => Ok(stringify!(Default)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            REMOVE4res::Nfs4Ok(_) => 0i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RENAME4args {
    pub oldname: String,
    pub newname: String,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RENAME4resok {
    pub source_cinfo: ChangeInfo4,
    pub target_cinfo: ChangeInfo4,
}
#[derive(Clone, Debug, XdrIndexer)]
pub enum RENAME4res {
    Nfs4Ok(RENAME4resok),
    Default,
}
impl Default for RENAME4res {
    fn default() -> Self {
        RENAME4res::Default
    }
}
impl XdrIndexer for RENAME4res {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Nfs4Ok)),
            _ => Ok(stringify!(Default)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            RENAME4res::Nfs4Ok(_) => 0i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RENEW4args {
    pub clientid: u64,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RENEW4res {
    #[serde(with = "serde_xdr::primitive::signed32")]
    pub status: Nfsstat4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RESTOREFH4res {
    #[serde(with = "serde_xdr::primitive::signed32")]
    pub status: Nfsstat4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SAVEFH4res {
    #[serde(with = "serde_xdr::primitive::signed32")]
    pub status: Nfsstat4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SECINFO4args {
    pub name: String,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[repr(i32)]
pub enum RpcGssSvcT {
    RpcGssSvcNone = 1i32,
    RpcGssSvcIntegrity = 2i32,
    RpcGssSvcPrivacy = 3i32,
}
impl Default for RpcGssSvcT {
    fn default() -> Self {
        RpcGssSvcT::RpcGssSvcNone
    }
}
impl AsRef<i32> for RpcGssSvcT {
    fn as_ref(&self) -> &'static i32 {
        match self {
            RpcGssSvcT::RpcGssSvcNone => &1i32,
            RpcGssSvcT::RpcGssSvcIntegrity => &2i32,
            RpcGssSvcT::RpcGssSvcPrivacy => &3i32,
        }
    }
}
impl TryFrom<i32> for RpcGssSvcT {
    type Error = serde_xdr::error::Error;
    fn try_from(v: i32) -> Result<Self, serde_xdr::error::Error> {
        match v {
            1i32 => Ok(RpcGssSvcT::RpcGssSvcNone),
            2i32 => Ok(RpcGssSvcT::RpcGssSvcIntegrity),
            3i32 => Ok(RpcGssSvcT::RpcGssSvcPrivacy),
            _ => Err(serde_xdr::error::Error::Convert),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RpcsecGssInfo {
    #[serde(with = "serde_xdr::opaque::variable")]
    pub oid: Vec<u8>,
    pub qop: u32,
    #[serde(with = "serde_xdr::primitive::signed32")]
    pub service: RpcGssSvcT,
}
#[derive(Clone, Debug, XdrIndexer)]
pub enum Secinfo4 {
    RpcsecGss(RpcsecGssInfo),
    Default,
}
impl Default for Secinfo4 {
    fn default() -> Self {
        Secinfo4::Default
    }
}
impl XdrIndexer for Secinfo4 {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            6i32 => Ok(stringify!(RpcsecGss)),
            _ => Ok(stringify!(Default)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            Secinfo4::RpcsecGss(_) => 6i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, XdrIndexer)]
pub enum SECINFO4res {
    Nfs4Ok(Vec<Secinfo4>),
    Default,
}
impl Default for SECINFO4res {
    fn default() -> Self {
        SECINFO4res::Default
    }
}
impl XdrIndexer for SECINFO4res {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Nfs4Ok)),
            _ => Ok(stringify!(Default)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            SECINFO4res::Nfs4Ok(_) => 0i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SETATTR4args {
    pub stateid: Stateid4,
    pub obj_attributes: Fattr4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SETATTR4res {
    #[serde(with = "serde_xdr::primitive::signed32")]
    pub status: Nfsstat4,
    pub attrsset: Vec<u32>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SETCLIENTID4args {
    pub client: NfsClientId4,
    pub callback: CbClient4,
    pub callback_ident: u32,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SETCLIENTID4resok {
    pub clientid: u64,
    #[serde(with = "serde_xdr::opaque::fixed")]
    pub setclientid_confirm: [u8; NFS4_VERIFIER_SIZE as usize],
}
#[derive(Clone, Debug, XdrIndexer)]
pub enum SETCLIENTID4res {
    Nfs4Ok(SETCLIENTID4resok),
    Nfs4errClidInuse(Clientaddr4),
    Default,
}
impl Default for SETCLIENTID4res {
    fn default() -> Self {
        SETCLIENTID4res::Default
    }
}
impl XdrIndexer for SETCLIENTID4res {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Nfs4Ok)),
            10017i32 => Ok(stringify!(Nfs4errClidInuse)),
            _ => Ok(stringify!(Default)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            SETCLIENTID4res::Nfs4Ok(_) => 0i32,
            SETCLIENTID4res::Nfs4errClidInuse(_) => 10017i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SetclientidConfirm4args {
    pub clientid: u64,
    #[serde(with = "serde_xdr::opaque::fixed")]
    pub setclientid_confirm: [u8; NFS4_VERIFIER_SIZE as usize],
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SetclientidConfirm4res {
    #[serde(with = "serde_xdr::primitive::signed32")]
    pub status: Nfsstat4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct VERIFY4args {
    pub obj_attributes: Fattr4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct VERIFY4res {
    #[serde(with = "serde_xdr::primitive::signed32")]
    pub status: Nfsstat4,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[repr(i32)]
pub enum StableHow4 {
    UNSTABLE4 = 0i32,
    DataSync4 = 1i32,
    FileSync4 = 2i32,
}
impl Default for StableHow4 {
    fn default() -> Self {
        StableHow4::UNSTABLE4
    }
}
impl AsRef<i32> for StableHow4 {
    fn as_ref(&self) -> &'static i32 {
        match self {
            StableHow4::UNSTABLE4 => &0i32,
            StableHow4::DataSync4 => &1i32,
            StableHow4::FileSync4 => &2i32,
        }
    }
}
impl TryFrom<i32> for StableHow4 {
    type Error = serde_xdr::error::Error;
    fn try_from(v: i32) -> Result<Self, serde_xdr::error::Error> {
        match v {
            0i32 => Ok(StableHow4::UNSTABLE4),
            1i32 => Ok(StableHow4::DataSync4),
            2i32 => Ok(StableHow4::FileSync4),
            _ => Err(serde_xdr::error::Error::Convert),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct WRITE4args {
    pub stateid: Stateid4,
    pub offset: u64,
    #[serde(with = "serde_xdr::primitive::signed32")]
    pub stable: StableHow4,
    #[serde(with = "serde_xdr::opaque::variable")]
    pub data: Vec<u8>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct WRITE4resok {
    pub count: u32,
    #[serde(with = "serde_xdr::primitive::signed32")]
    pub committed: StableHow4,
    #[serde(with = "serde_xdr::opaque::fixed")]
    pub writeverf: [u8; NFS4_VERIFIER_SIZE as usize],
}
#[derive(Clone, Debug, XdrIndexer)]
pub enum WRITE4res {
    Nfs4Ok(WRITE4resok),
    Default,
}
impl Default for WRITE4res {
    fn default() -> Self {
        WRITE4res::Default
    }
}
impl XdrIndexer for WRITE4res {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Nfs4Ok)),
            _ => Ok(stringify!(Default)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            WRITE4res::Nfs4Ok(_) => 0i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ReleaseLockowner4args {
    pub lock_owner: LockOwner4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ReleaseLockowner4res {
    #[serde(with = "serde_xdr::primitive::signed32")]
    pub status: Nfsstat4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ILLEGAL4res {
    #[serde(with = "serde_xdr::primitive::signed32")]
    pub status: Nfsstat4,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[repr(i32)]
pub enum NfsOpnum4 {
    OpAccess = 3i32,
    OpClose = 4i32,
    OpCommit = 5i32,
    OpCreate = 6i32,
    OpDelegpurge = 7i32,
    OpDelegreturn = 8i32,
    OpGetattr = 9i32,
    OpGetfh = 10i32,
    OpLink = 11i32,
    OpLock = 12i32,
    OpLockt = 13i32,
    OpLocku = 14i32,
    OpLookup = 15i32,
    OpLookupp = 16i32,
    OpNverify = 17i32,
    OpOpen = 18i32,
    OpOpenattr = 19i32,
    OpOpenConfirm = 20i32,
    OpOpenDowngrade = 21i32,
    OpPutfh = 22i32,
    OpPutpubfh = 23i32,
    OpPutrootfh = 24i32,
    OpRead = 25i32,
    OpReaddir = 26i32,
    OpReadlink = 27i32,
    OpRemove = 28i32,
    OpRename = 29i32,
    OpRenew = 30i32,
    OpRestorefh = 31i32,
    OpSavefh = 32i32,
    OpSecinfo = 33i32,
    OpSetattr = 34i32,
    OpSetclientid = 35i32,
    OpSetclientidConfirm = 36i32,
    OpVerify = 37i32,
    OpWrite = 38i32,
    OpReleaseLockowner = 39i32,
    OpIllegal = 10044i32,
}
impl Default for NfsOpnum4 {
    fn default() -> Self {
        NfsOpnum4::OpAccess
    }
}
impl AsRef<i32> for NfsOpnum4 {
    fn as_ref(&self) -> &'static i32 {
        match self {
            NfsOpnum4::OpAccess => &3i32,
            NfsOpnum4::OpClose => &4i32,
            NfsOpnum4::OpCommit => &5i32,
            NfsOpnum4::OpCreate => &6i32,
            NfsOpnum4::OpDelegpurge => &7i32,
            NfsOpnum4::OpDelegreturn => &8i32,
            NfsOpnum4::OpGetattr => &9i32,
            NfsOpnum4::OpGetfh => &10i32,
            NfsOpnum4::OpLink => &11i32,
            NfsOpnum4::OpLock => &12i32,
            NfsOpnum4::OpLockt => &13i32,
            NfsOpnum4::OpLocku => &14i32,
            NfsOpnum4::OpLookup => &15i32,
            NfsOpnum4::OpLookupp => &16i32,
            NfsOpnum4::OpNverify => &17i32,
            NfsOpnum4::OpOpen => &18i32,
            NfsOpnum4::OpOpenattr => &19i32,
            NfsOpnum4::OpOpenConfirm => &20i32,
            NfsOpnum4::OpOpenDowngrade => &21i32,
            NfsOpnum4::OpPutfh => &22i32,
            NfsOpnum4::OpPutpubfh => &23i32,
            NfsOpnum4::OpPutrootfh => &24i32,
            NfsOpnum4::OpRead => &25i32,
            NfsOpnum4::OpReaddir => &26i32,
            NfsOpnum4::OpReadlink => &27i32,
            NfsOpnum4::OpRemove => &28i32,
            NfsOpnum4::OpRename => &29i32,
            NfsOpnum4::OpRenew => &30i32,
            NfsOpnum4::OpRestorefh => &31i32,
            NfsOpnum4::OpSavefh => &32i32,
            NfsOpnum4::OpSecinfo => &33i32,
            NfsOpnum4::OpSetattr => &34i32,
            NfsOpnum4::OpSetclientid => &35i32,
            NfsOpnum4::OpSetclientidConfirm => &36i32,
            NfsOpnum4::OpVerify => &37i32,
            NfsOpnum4::OpWrite => &38i32,
            NfsOpnum4::OpReleaseLockowner => &39i32,
            NfsOpnum4::OpIllegal => &10044i32,
        }
    }
}
impl TryFrom<i32> for NfsOpnum4 {
    type Error = serde_xdr::error::Error;
    fn try_from(v: i32) -> Result<Self, serde_xdr::error::Error> {
        match v {
            3i32 => Ok(NfsOpnum4::OpAccess),
            4i32 => Ok(NfsOpnum4::OpClose),
            5i32 => Ok(NfsOpnum4::OpCommit),
            6i32 => Ok(NfsOpnum4::OpCreate),
            7i32 => Ok(NfsOpnum4::OpDelegpurge),
            8i32 => Ok(NfsOpnum4::OpDelegreturn),
            9i32 => Ok(NfsOpnum4::OpGetattr),
            10i32 => Ok(NfsOpnum4::OpGetfh),
            11i32 => Ok(NfsOpnum4::OpLink),
            12i32 => Ok(NfsOpnum4::OpLock),
            13i32 => Ok(NfsOpnum4::OpLockt),
            14i32 => Ok(NfsOpnum4::OpLocku),
            15i32 => Ok(NfsOpnum4::OpLookup),
            16i32 => Ok(NfsOpnum4::OpLookupp),
            17i32 => Ok(NfsOpnum4::OpNverify),
            18i32 => Ok(NfsOpnum4::OpOpen),
            19i32 => Ok(NfsOpnum4::OpOpenattr),
            20i32 => Ok(NfsOpnum4::OpOpenConfirm),
            21i32 => Ok(NfsOpnum4::OpOpenDowngrade),
            22i32 => Ok(NfsOpnum4::OpPutfh),
            23i32 => Ok(NfsOpnum4::OpPutpubfh),
            24i32 => Ok(NfsOpnum4::OpPutrootfh),
            25i32 => Ok(NfsOpnum4::OpRead),
            26i32 => Ok(NfsOpnum4::OpReaddir),
            27i32 => Ok(NfsOpnum4::OpReadlink),
            28i32 => Ok(NfsOpnum4::OpRemove),
            29i32 => Ok(NfsOpnum4::OpRename),
            30i32 => Ok(NfsOpnum4::OpRenew),
            31i32 => Ok(NfsOpnum4::OpRestorefh),
            32i32 => Ok(NfsOpnum4::OpSavefh),
            33i32 => Ok(NfsOpnum4::OpSecinfo),
            34i32 => Ok(NfsOpnum4::OpSetattr),
            35i32 => Ok(NfsOpnum4::OpSetclientid),
            36i32 => Ok(NfsOpnum4::OpSetclientidConfirm),
            37i32 => Ok(NfsOpnum4::OpVerify),
            38i32 => Ok(NfsOpnum4::OpWrite),
            39i32 => Ok(NfsOpnum4::OpReleaseLockowner),
            10044i32 => Ok(NfsOpnum4::OpIllegal),
            _ => Err(serde_xdr::error::Error::Convert),
        }
    }
}
#[derive(Clone, Debug, XdrIndexer)]
pub enum NfsArgop4 {
    OpAccess(ACCESS4args),
    OpClose(CLOSE4args),
    OpCommit(COMMIT4args),
    OpCreate(CREATE4args),
    OpDelegpurge(DELEGPURGE4args),
    OpDelegreturn(DELEGRETURN4args),
    OpGetattr(GETATTR4args),
    OpGetfh,
    OpLink(LINK4args),
    OpLock(LOCK4args),
    OpLockt(LOCKT4args),
    OpLocku(LOCKU4args),
    OpLookup(LOOKUP4args),
    OpLookupp,
    OpNverify(NVERIFY4args),
    OpOpen(OPEN4args),
    OpOpenattr(OPENATTR4args),
    OpOpenConfirm(OpenConfirm4args),
    OpOpenDowngrade(OpenDowngrade4args),
    OpPutfh(PUTFH4args),
    OpPutpubfh,
    OpPutrootfh,
    OpRead(READ4args),
    OpReaddir(READDIR4args),
    OpReadlink,
    OpRemove(REMOVE4args),
    OpRename(RENAME4args),
    OpRenew(RENEW4args),
    OpRestorefh,
    OpSavefh,
    OpSecinfo(SECINFO4args),
    OpSetattr(SETATTR4args),
    OpSetclientid(SETCLIENTID4args),
    OpSetclientidConfirm(SetclientidConfirm4args),
    OpVerify(VERIFY4args),
    OpWrite(WRITE4args),
    OpReleaseLockowner(ReleaseLockowner4args),
    OpIllegal,
}
impl Default for NfsArgop4 {
    fn default() -> Self {
        NfsArgop4::OpAccess(Default::default())
    }
}
impl XdrIndexer for NfsArgop4 {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            3i32 => Ok(stringify!(OpAccess)),
            4i32 => Ok(stringify!(OpClose)),
            5i32 => Ok(stringify!(OpCommit)),
            6i32 => Ok(stringify!(OpCreate)),
            7i32 => Ok(stringify!(OpDelegpurge)),
            8i32 => Ok(stringify!(OpDelegreturn)),
            9i32 => Ok(stringify!(OpGetattr)),
            10i32 => Ok(stringify!(OpGetfh)),
            11i32 => Ok(stringify!(OpLink)),
            12i32 => Ok(stringify!(OpLock)),
            13i32 => Ok(stringify!(OpLockt)),
            14i32 => Ok(stringify!(OpLocku)),
            15i32 => Ok(stringify!(OpLookup)),
            16i32 => Ok(stringify!(OpLookupp)),
            17i32 => Ok(stringify!(OpNverify)),
            18i32 => Ok(stringify!(OpOpen)),
            19i32 => Ok(stringify!(OpOpenattr)),
            20i32 => Ok(stringify!(OpOpenConfirm)),
            21i32 => Ok(stringify!(OpOpenDowngrade)),
            22i32 => Ok(stringify!(OpPutfh)),
            23i32 => Ok(stringify!(OpPutpubfh)),
            24i32 => Ok(stringify!(OpPutrootfh)),
            25i32 => Ok(stringify!(OpRead)),
            26i32 => Ok(stringify!(OpReaddir)),
            27i32 => Ok(stringify!(OpReadlink)),
            28i32 => Ok(stringify!(OpRemove)),
            29i32 => Ok(stringify!(OpRename)),
            30i32 => Ok(stringify!(OpRenew)),
            31i32 => Ok(stringify!(OpRestorefh)),
            32i32 => Ok(stringify!(OpSavefh)),
            33i32 => Ok(stringify!(OpSecinfo)),
            34i32 => Ok(stringify!(OpSetattr)),
            35i32 => Ok(stringify!(OpSetclientid)),
            36i32 => Ok(stringify!(OpSetclientidConfirm)),
            37i32 => Ok(stringify!(OpVerify)),
            38i32 => Ok(stringify!(OpWrite)),
            39i32 => Ok(stringify!(OpReleaseLockowner)),
            10044i32 => Ok(stringify!(OpIllegal)),
            _ => Err(::serde_xdr::error::Error::Convert),
        }
    }
    fn index(&self) -> i32 {
        match self {
            NfsArgop4::OpAccess(_) => 3i32,
            NfsArgop4::OpClose(_) => 4i32,
            NfsArgop4::OpCommit(_) => 5i32,
            NfsArgop4::OpCreate(_) => 6i32,
            NfsArgop4::OpDelegpurge(_) => 7i32,
            NfsArgop4::OpDelegreturn(_) => 8i32,
            NfsArgop4::OpGetattr(_) => 9i32,
            NfsArgop4::OpGetfh => 10i32,
            NfsArgop4::OpLink(_) => 11i32,
            NfsArgop4::OpLock(_) => 12i32,
            NfsArgop4::OpLockt(_) => 13i32,
            NfsArgop4::OpLocku(_) => 14i32,
            NfsArgop4::OpLookup(_) => 15i32,
            NfsArgop4::OpLookupp => 16i32,
            NfsArgop4::OpNverify(_) => 17i32,
            NfsArgop4::OpOpen(_) => 18i32,
            NfsArgop4::OpOpenattr(_) => 19i32,
            NfsArgop4::OpOpenConfirm(_) => 20i32,
            NfsArgop4::OpOpenDowngrade(_) => 21i32,
            NfsArgop4::OpPutfh(_) => 22i32,
            NfsArgop4::OpPutpubfh => 23i32,
            NfsArgop4::OpPutrootfh => 24i32,
            NfsArgop4::OpRead(_) => 25i32,
            NfsArgop4::OpReaddir(_) => 26i32,
            NfsArgop4::OpReadlink => 27i32,
            NfsArgop4::OpRemove(_) => 28i32,
            NfsArgop4::OpRename(_) => 29i32,
            NfsArgop4::OpRenew(_) => 30i32,
            NfsArgop4::OpRestorefh => 31i32,
            NfsArgop4::OpSavefh => 32i32,
            NfsArgop4::OpSecinfo(_) => 33i32,
            NfsArgop4::OpSetattr(_) => 34i32,
            NfsArgop4::OpSetclientid(_) => 35i32,
            NfsArgop4::OpSetclientidConfirm(_) => 36i32,
            NfsArgop4::OpVerify(_) => 37i32,
            NfsArgop4::OpWrite(_) => 38i32,
            NfsArgop4::OpReleaseLockowner(_) => 39i32,
            NfsArgop4::OpIllegal => 10044i32,
        }
    }
}
#[derive(Clone, Debug, XdrIndexer)]
pub enum NfsResop4 {
    OpAccess(ACCESS4res),
    OpClose(CLOSE4res),
    OpCommit(COMMIT4res),
    OpCreate(CREATE4res),
    OpDelegpurge(DELEGPURGE4res),
    OpDelegreturn(DELEGRETURN4res),
    OpGetattr(GETATTR4res),
    OpGetfh(GETFH4res),
    OpLink(LINK4res),
    OpLock(LOCK4res),
    OpLockt(LOCKT4res),
    OpLocku(LOCKU4res),
    OpLookup(LOOKUP4res),
    OpLookupp(LOOKUPP4res),
    OpNverify(NVERIFY4res),
    OpOpen(OPEN4res),
    OpOpenattr(OPENATTR4res),
    OpOpenConfirm(OpenConfirm4res),
    OpOpenDowngrade(OpenDowngrade4res),
    OpPutfh(PUTFH4res),
    OpPutpubfh(PUTPUBFH4res),
    OpPutrootfh(PUTROOTFH4res),
    OpRead(READ4res),
    OpReaddir(READDIR4res),
    OpReadlink(READLINK4res),
    OpRemove(REMOVE4res),
    OpRename(RENAME4res),
    OpRenew(RENEW4res),
    OpRestorefh(RESTOREFH4res),
    OpSavefh(SAVEFH4res),
    OpSecinfo(SECINFO4res),
    OpSetattr(SETATTR4res),
    OpSetclientid(SETCLIENTID4res),
    OpSetclientidConfirm(SetclientidConfirm4res),
    OpVerify(VERIFY4res),
    OpWrite(WRITE4res),
    OpReleaseLockowner(ReleaseLockowner4res),
    OpIllegal(ILLEGAL4res),
}
impl Default for NfsResop4 {
    fn default() -> Self {
        NfsResop4::OpAccess(Default::default())
    }
}
impl XdrIndexer for NfsResop4 {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            3i32 => Ok(stringify!(OpAccess)),
            4i32 => Ok(stringify!(OpClose)),
            5i32 => Ok(stringify!(OpCommit)),
            6i32 => Ok(stringify!(OpCreate)),
            7i32 => Ok(stringify!(OpDelegpurge)),
            8i32 => Ok(stringify!(OpDelegreturn)),
            9i32 => Ok(stringify!(OpGetattr)),
            10i32 => Ok(stringify!(OpGetfh)),
            11i32 => Ok(stringify!(OpLink)),
            12i32 => Ok(stringify!(OpLock)),
            13i32 => Ok(stringify!(OpLockt)),
            14i32 => Ok(stringify!(OpLocku)),
            15i32 => Ok(stringify!(OpLookup)),
            16i32 => Ok(stringify!(OpLookupp)),
            17i32 => Ok(stringify!(OpNverify)),
            18i32 => Ok(stringify!(OpOpen)),
            19i32 => Ok(stringify!(OpOpenattr)),
            20i32 => Ok(stringify!(OpOpenConfirm)),
            21i32 => Ok(stringify!(OpOpenDowngrade)),
            22i32 => Ok(stringify!(OpPutfh)),
            23i32 => Ok(stringify!(OpPutpubfh)),
            24i32 => Ok(stringify!(OpPutrootfh)),
            25i32 => Ok(stringify!(OpRead)),
            26i32 => Ok(stringify!(OpReaddir)),
            27i32 => Ok(stringify!(OpReadlink)),
            28i32 => Ok(stringify!(OpRemove)),
            29i32 => Ok(stringify!(OpRename)),
            30i32 => Ok(stringify!(OpRenew)),
            31i32 => Ok(stringify!(OpRestorefh)),
            32i32 => Ok(stringify!(OpSavefh)),
            33i32 => Ok(stringify!(OpSecinfo)),
            34i32 => Ok(stringify!(OpSetattr)),
            35i32 => Ok(stringify!(OpSetclientid)),
            36i32 => Ok(stringify!(OpSetclientidConfirm)),
            37i32 => Ok(stringify!(OpVerify)),
            38i32 => Ok(stringify!(OpWrite)),
            39i32 => Ok(stringify!(OpReleaseLockowner)),
            10044i32 => Ok(stringify!(OpIllegal)),
            _ => Err(::serde_xdr::error::Error::Convert),
        }
    }
    fn index(&self) -> i32 {
        match self {
            NfsResop4::OpAccess(_) => 3i32,
            NfsResop4::OpClose(_) => 4i32,
            NfsResop4::OpCommit(_) => 5i32,
            NfsResop4::OpCreate(_) => 6i32,
            NfsResop4::OpDelegpurge(_) => 7i32,
            NfsResop4::OpDelegreturn(_) => 8i32,
            NfsResop4::OpGetattr(_) => 9i32,
            NfsResop4::OpGetfh(_) => 10i32,
            NfsResop4::OpLink(_) => 11i32,
            NfsResop4::OpLock(_) => 12i32,
            NfsResop4::OpLockt(_) => 13i32,
            NfsResop4::OpLocku(_) => 14i32,
            NfsResop4::OpLookup(_) => 15i32,
            NfsResop4::OpLookupp(_) => 16i32,
            NfsResop4::OpNverify(_) => 17i32,
            NfsResop4::OpOpen(_) => 18i32,
            NfsResop4::OpOpenattr(_) => 19i32,
            NfsResop4::OpOpenConfirm(_) => 20i32,
            NfsResop4::OpOpenDowngrade(_) => 21i32,
            NfsResop4::OpPutfh(_) => 22i32,
            NfsResop4::OpPutpubfh(_) => 23i32,
            NfsResop4::OpPutrootfh(_) => 24i32,
            NfsResop4::OpRead(_) => 25i32,
            NfsResop4::OpReaddir(_) => 26i32,
            NfsResop4::OpReadlink(_) => 27i32,
            NfsResop4::OpRemove(_) => 28i32,
            NfsResop4::OpRename(_) => 29i32,
            NfsResop4::OpRenew(_) => 30i32,
            NfsResop4::OpRestorefh(_) => 31i32,
            NfsResop4::OpSavefh(_) => 32i32,
            NfsResop4::OpSecinfo(_) => 33i32,
            NfsResop4::OpSetattr(_) => 34i32,
            NfsResop4::OpSetclientid(_) => 35i32,
            NfsResop4::OpSetclientidConfirm(_) => 36i32,
            NfsResop4::OpVerify(_) => 37i32,
            NfsResop4::OpWrite(_) => 38i32,
            NfsResop4::OpReleaseLockowner(_) => 39i32,
            NfsResop4::OpIllegal(_) => 10044i32,
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct COMPOUND4args {
    pub tag: String,
    pub minorversion: u32,
    pub argarray: Vec<NfsArgop4>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct COMPOUND4res {
    #[serde(with = "serde_xdr::primitive::signed32")]
    pub status: Nfsstat4,
    pub tag: String,
    pub resarray: Vec<NfsResop4>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CbGetattr4args {
    #[serde(with = "serde_xdr::opaque::variable")]
    pub fh: Vec<u8>,
    pub attr_request: Vec<u32>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CbGetattr4resok {
    pub obj_attributes: Fattr4,
}
#[derive(Clone, Debug, XdrIndexer)]
pub enum CbGetattr4res {
    Nfs4Ok(CbGetattr4resok),
    Default,
}
impl Default for CbGetattr4res {
    fn default() -> Self {
        CbGetattr4res::Default
    }
}
impl XdrIndexer for CbGetattr4res {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Nfs4Ok)),
            _ => Ok(stringify!(Default)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            CbGetattr4res::Nfs4Ok(_) => 0i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CbRecall4args {
    pub stateid: Stateid4,
    pub truncate: bool,
    #[serde(with = "serde_xdr::opaque::variable")]
    pub fh: Vec<u8>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CbRecall4res {
    #[serde(with = "serde_xdr::primitive::signed32")]
    pub status: Nfsstat4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CbIllegal4res {
    #[serde(with = "serde_xdr::primitive::signed32")]
    pub status: Nfsstat4,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[repr(i32)]
pub enum NfsCbOpnum4 {
    OpCbGetattr = 3i32,
    OpCbRecall = 4i32,
    OpCbIllegal = 10044i32,
}
impl Default for NfsCbOpnum4 {
    fn default() -> Self {
        NfsCbOpnum4::OpCbGetattr
    }
}
impl AsRef<i32> for NfsCbOpnum4 {
    fn as_ref(&self) -> &'static i32 {
        match self {
            NfsCbOpnum4::OpCbGetattr => &3i32,
            NfsCbOpnum4::OpCbRecall => &4i32,
            NfsCbOpnum4::OpCbIllegal => &10044i32,
        }
    }
}
impl TryFrom<i32> for NfsCbOpnum4 {
    type Error = serde_xdr::error::Error;
    fn try_from(v: i32) -> Result<Self, serde_xdr::error::Error> {
        match v {
            3i32 => Ok(NfsCbOpnum4::OpCbGetattr),
            4i32 => Ok(NfsCbOpnum4::OpCbRecall),
            10044i32 => Ok(NfsCbOpnum4::OpCbIllegal),
            _ => Err(serde_xdr::error::Error::Convert),
        }
    }
}
#[derive(Clone, Debug, XdrIndexer)]
pub enum NfsCbArgop4 {
    OpCbGetattr(CbGetattr4args),
    OpCbRecall(CbRecall4args),
    OpCbIllegal,
}
impl Default for NfsCbArgop4 {
    fn default() -> Self {
        NfsCbArgop4::OpCbGetattr(Default::default())
    }
}
impl XdrIndexer for NfsCbArgop4 {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            3i32 => Ok(stringify!(OpCbGetattr)),
            4i32 => Ok(stringify!(OpCbRecall)),
            10044i32 => Ok(stringify!(OpCbIllegal)),
            _ => Err(::serde_xdr::error::Error::Convert),
        }
    }
    fn index(&self) -> i32 {
        match self {
            NfsCbArgop4::OpCbGetattr(_) => 3i32,
            NfsCbArgop4::OpCbRecall(_) => 4i32,
            NfsCbArgop4::OpCbIllegal => 10044i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, XdrIndexer)]
pub enum NfsCbResop4 {
    OpCbGetattr(CbGetattr4res),
    OpCbRecall(CbRecall4res),
    OpCbIllegal(CbIllegal4res),
}
impl Default for NfsCbResop4 {
    fn default() -> Self {
        NfsCbResop4::OpCbGetattr(Default::default())
    }
}
impl XdrIndexer for NfsCbResop4 {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            3i32 => Ok(stringify!(OpCbGetattr)),
            4i32 => Ok(stringify!(OpCbRecall)),
            10044i32 => Ok(stringify!(OpCbIllegal)),
            _ => Err(::serde_xdr::error::Error::Convert),
        }
    }
    fn index(&self) -> i32 {
        match self {
            NfsCbResop4::OpCbGetattr(_) => 3i32,
            NfsCbResop4::OpCbRecall(_) => 4i32,
            NfsCbResop4::OpCbIllegal(_) => 10044i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CbCompound4args {
    pub tag: String,
    pub minorversion: u32,
    pub callback_ident: u32,
    pub argarray: Vec<NfsCbArgop4>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CbCompound4res {
    #[serde(with = "serde_xdr::primitive::signed32")]
    pub status: Nfsstat4,
    pub tag: String,
    pub resarray: Vec<NfsCbResop4>,
}
