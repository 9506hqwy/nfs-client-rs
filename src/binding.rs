use serde::ser::SerializeTuple;
use serde::{Deserialize, Serialize};
use serde_xdr::XdrIndexer;
use serde_xdr_derive::XdrIndexer;
pub const NFS4_FHSIZE: u32 = 128u32;
pub const NFS4_VERIFIER_SIZE: u32 = 8u32;
pub const NFS4_OTHER_SIZE: u32 = 12u32;
pub const NFS4_OPAQUE_LIMIT: u32 = 1024u32;
pub const NFS4_SESSIONID_SIZE: u32 = 16u32;
pub const NFS4_INT64_MAX: u64 = 9223372036854775807u64;
pub const NFS4_UINT64_MAX: u64 = 18446744073709551615u64;
pub const NFS4_INT32_MAX: u32 = 2147483647u32;
pub const NFS4_UINT32_MAX: u32 = 4294967295u32;
pub const NFS4_MAXFILELEN: u64 = 18446744073709551615u64;
pub const NFS4_MAXFILEOFF: u64 = 18446744073709551614u64;
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
pub const ACE4_INHERITED_ACE: u32 = 128u32;
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
pub const ACE4_WRITE_RETENTION: u32 = 512u32;
pub const ACE4_WRITE_RETENTION_HOLD: u32 = 1024u32;
pub const ACE4_DELETE: u32 = 65536u32;
pub const ACE4_READ_ACL: u32 = 131072u32;
pub const ACE4_WRITE_ACL: u32 = 262144u32;
pub const ACE4_WRITE_OWNER: u32 = 524288u32;
pub const ACE4_SYNCHRONIZE: u32 = 1048576u32;
pub const ACE4_GENERIC_READ: u32 = 1179777u32;
pub const ACE4_GENERIC_WRITE: u32 = 1442054u32;
pub const ACE4_GENERIC_EXECUTE: u32 = 1179808u32;
pub const ACL4_AUTO_INHERIT: u32 = 1u32;
pub const ACL4_PROTECTED: u32 = 2u32;
pub const ACL4_DEFAULTED: u32 = 4u32;
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
pub const NFS4_DEVICEID4_SIZE: u32 = 16u32;
pub const LAYOUT4_RET_REC_FILE: u32 = 1u32;
pub const LAYOUT4_RET_REC_FSID: u32 = 2u32;
pub const LAYOUT4_RET_REC_ALL: u32 = 3u32;
pub const TH4_READ_SIZE: u32 = 0u32;
pub const TH4_WRITE_SIZE: u32 = 1u32;
pub const TH4_READ_IOSIZE: u32 = 2u32;
pub const TH4_WRITE_IOSIZE: u32 = 3u32;
pub const RET4_DURATION_INFINITE: u64 = 18446744073709551615u64;
pub const FSCHARSET_CAP4_CONTAINS_NON_UTF8: u32 = 1u32;
pub const FSCHARSET_CAP4_ALLOWS_ONLY_UTF8: u32 = 2u32;
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
pub const FATTR4_SUPPATTR_EXCLCREAT: u32 = 75u32;
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
pub const FATTR4_DIR_NOTIF_DELAY: u32 = 56u32;
pub const FATTR4_DIRENT_NOTIF_DELAY: u32 = 57u32;
pub const FATTR4_DACL: u32 = 58u32;
pub const FATTR4_SACL: u32 = 59u32;
pub const FATTR4_CHANGE_POLICY: u32 = 60u32;
pub const FATTR4_FS_STATUS: u32 = 61u32;
pub const FATTR4_FS_LAYOUT_TYPES: u32 = 62u32;
pub const FATTR4_LAYOUT_HINT: u32 = 63u32;
pub const FATTR4_LAYOUT_TYPES: u32 = 64u32;
pub const FATTR4_LAYOUT_BLKSIZE: u32 = 65u32;
pub const FATTR4_LAYOUT_ALIGNMENT: u32 = 66u32;
pub const FATTR4_FS_LOCATIONS_INFO: u32 = 67u32;
pub const FATTR4_MDSTHRESHOLD: u32 = 68u32;
pub const FATTR4_RETENTION_GET: u32 = 69u32;
pub const FATTR4_RETENTION_SET: u32 = 70u32;
pub const FATTR4_RETENTEVT_GET: u32 = 71u32;
pub const FATTR4_RETENTEVT_SET: u32 = 72u32;
pub const FATTR4_RETENTION_HOLD: u32 = 73u32;
pub const FATTR4_MODE_SET_MASKED: u32 = 74u32;
pub const FATTR4_FS_CHARSET_CAP: u32 = 76u32;
pub const FATTR4_CLONE_BLKSIZE: u32 = 77u32;
pub const FATTR4_SPACE_FREED: u32 = 78u32;
pub const FATTR4_CHANGE_ATTR_TYPE: u32 = 79u32;
pub const FATTR4_SEC_LABEL: u32 = 80u32;
pub const FSLI4BX_GFLAGS: u32 = 0u32;
pub const FSLI4BX_TFLAGS: u32 = 1u32;
pub const FSLI4BX_CLSIMUL: u32 = 2u32;
pub const FSLI4BX_CLHANDLE: u32 = 3u32;
pub const FSLI4BX_CLFILEID: u32 = 4u32;
pub const FSLI4BX_CLWRITEVER: u32 = 5u32;
pub const FSLI4BX_CLCHANGE: u32 = 6u32;
pub const FSLI4BX_CLREADDIR: u32 = 7u32;
pub const FSLI4BX_READRANK: u32 = 8u32;
pub const FSLI4BX_WRITERANK: u32 = 9u32;
pub const FSLI4BX_READORDER: u32 = 10u32;
pub const FSLI4BX_WRITEORDER: u32 = 11u32;
pub const FSLI4GF_WRITABLE: u32 = 1u32;
pub const FSLI4GF_CUR_REQ: u32 = 2u32;
pub const FSLI4GF_ABSENT: u32 = 4u32;
pub const FSLI4GF_GOING: u32 = 8u32;
pub const FSLI4GF_SPLIT: u32 = 16u32;
pub const FSLI4TF_RDMA: u32 = 1u32;
pub const FSLI4IF_VAR_SUB: u32 = 1u32;
pub const NFL4_UFLG_MASK: u32 = 63u32;
pub const NFL4_UFLG_DENSE: u32 = 1u32;
pub const NFL4_UFLG_COMMIT_THRU_MDS: u32 = 2u32;
pub const NFL42_UFLG_IO_ADVISE_THRU_MDS: u32 = 4u32;
pub const NFL4_UFLG_STRIPE_UNIT_SIZE_MASK: u32 = 4294967232u32;
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
pub const OPEN4_SHARE_ACCESS_WANT_DELEG_MASK: u32 = 65280u32;
pub const OPEN4_SHARE_ACCESS_WANT_NO_PREFERENCE: u32 = 0u32;
pub const OPEN4_SHARE_ACCESS_WANT_READ_DELEG: u32 = 256u32;
pub const OPEN4_SHARE_ACCESS_WANT_WRITE_DELEG: u32 = 512u32;
pub const OPEN4_SHARE_ACCESS_WANT_ANY_DELEG: u32 = 768u32;
pub const OPEN4_SHARE_ACCESS_WANT_NO_DELEG: u32 = 1024u32;
pub const OPEN4_SHARE_ACCESS_WANT_CANCEL: u32 = 1280u32;
pub const OPEN4_SHARE_ACCESS_WANT_SIGNAL_DELEG_WHEN_RESRC_AVAIL: u32 = 65536u32;
pub const OPEN4_SHARE_ACCESS_WANT_PUSH_DELEG_WHEN_UNCONTENDED: u32 = 131072u32;
pub const OPEN4_RESULT_CONFIRM: u32 = 2u32;
pub const OPEN4_RESULT_LOCKTYPE_POSIX: u32 = 4u32;
pub const OPEN4_RESULT_PRESERVE_UNLINKED: u32 = 8u32;
pub const OPEN4_RESULT_MAY_NOTIFY_LOCK: u32 = 32u32;
pub const EXCHGID4_FLAG_SUPP_MOVED_REFER: u32 = 1u32;
pub const EXCHGID4_FLAG_SUPP_MOVED_MIGR: u32 = 2u32;
pub const EXCHGID4_FLAG_SUPP_FENCE_OPS: u32 = 4u32;
pub const EXCHGID4_FLAG_BIND_PRINC_STATEID: u32 = 256u32;
pub const EXCHGID4_FLAG_USE_NON_PNFS: u32 = 65536u32;
pub const EXCHGID4_FLAG_USE_PNFS_MDS: u32 = 131072u32;
pub const EXCHGID4_FLAG_USE_PNFS_DS: u32 = 262144u32;
pub const EXCHGID4_FLAG_MASK_PNFS: u32 = 458752u32;
pub const EXCHGID4_FLAG_UPD_CONFIRMED_REC_A: u32 = 1073741824u32;
pub const EXCHGID4_FLAG_CONFIRMED_R: u32 = 2147483648u32;
pub const CREATE_SESSION4_FLAG_PERSIST: u32 = 1u32;
pub const CREATE_SESSION4_FLAG_CONN_BACK_CHAN: u32 = 2u32;
pub const CREATE_SESSION4_FLAG_CONN_RDMA: u32 = 4u32;
pub const SEQ4_STATUS_CB_PATH_DOWN: u32 = 1u32;
pub const SEQ4_STATUS_CB_GSS_CONTEXTS_EXPIRING: u32 = 2u32;
pub const SEQ4_STATUS_CB_GSS_CONTEXTS_EXPIRED: u32 = 4u32;
pub const SEQ4_STATUS_EXPIRED_ALL_STATE_REVOKED: u32 = 8u32;
pub const SEQ4_STATUS_EXPIRED_SOME_STATE_REVOKED: u32 = 16u32;
pub const SEQ4_STATUS_ADMIN_STATE_REVOKED: u32 = 32u32;
pub const SEQ4_STATUS_RECALLABLE_STATE_REVOKED: u32 = 64u32;
pub const SEQ4_STATUS_LEASE_MOVED: u32 = 128u32;
pub const SEQ4_STATUS_RESTART_RECLAIM_NEEDED: u32 = 256u32;
pub const SEQ4_STATUS_CB_PATH_DOWN_SESSION: u32 = 512u32;
pub const SEQ4_STATUS_BACKCHANNEL_FAULT: u32 = 1024u32;
pub const SEQ4_STATUS_DEVID_CHANGED: u32 = 2048u32;
pub const SEQ4_STATUS_DEVID_DELETED: u32 = 4096u32;
pub const RCA4_TYPE_MASK_RDATA_DLG: u32 = 0u32;
pub const RCA4_TYPE_MASK_WDATA_DLG: u32 = 1u32;
pub const RCA4_TYPE_MASK_DIR_DLG: u32 = 2u32;
pub const RCA4_TYPE_MASK_FILE_LAYOUT: u32 = 3u32;
pub const RCA4_TYPE_MASK_BLK_LAYOUT: u32 = 4u32;
pub const RCA4_TYPE_MASK_OBJ_LAYOUT_MIN: u32 = 8u32;
pub const RCA4_TYPE_MASK_OBJ_LAYOUT_MAX: u32 = 9u32;
pub const RCA4_TYPE_MASK_OTHER_LAYOUT_MIN: u32 = 12u32;
pub const RCA4_TYPE_MASK_OTHER_LAYOUT_MAX: u32 = 15u32;
pub const AUTH_NONE: u32 = 0u32;
pub const AUTH_SYS: u32 = 1u32;
pub const RPCSEC_GSS: u32 = 6u32;
#[derive(Clone, Debug, PartialEq, XdrIndexer)]
#[repr(i32)]
#[derive(Default)]
pub enum NfsFtype4 {
    #[default]
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
impl XdrIndexer for NfsFtype4 {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            1i32 => Ok(stringify!(NF4REG)),
            2i32 => Ok(stringify!(NF4DIR)),
            3i32 => Ok(stringify!(NF4BLK)),
            4i32 => Ok(stringify!(NF4CHR)),
            5i32 => Ok(stringify!(NF4LNK)),
            6i32 => Ok(stringify!(NF4SOCK)),
            7i32 => Ok(stringify!(NF4FIFO)),
            8i32 => Ok(stringify!(NF4ATTRDIR)),
            9i32 => Ok(stringify!(NF4NAMEDATTR)),
            _ => Ok(stringify!(NF4REG)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            NfsFtype4::NF4REG => 1i32,
            NfsFtype4::NF4DIR => 2i32,
            NfsFtype4::NF4BLK => 3i32,
            NfsFtype4::NF4CHR => 4i32,
            NfsFtype4::NF4LNK => 5i32,
            NfsFtype4::NF4SOCK => 6i32,
            NfsFtype4::NF4FIFO => 7i32,
            NfsFtype4::NF4ATTRDIR => 8i32,
            NfsFtype4::NF4NAMEDATTR => 9i32,
        }
    }
}
#[derive(Clone, Debug, PartialEq, XdrIndexer)]
#[repr(i32)]
#[derive(Default)]
pub enum Nfsstat4 {
    #[default]
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
    Nfs4errBadiomode = 10049i32,
    Nfs4errBadlayout = 10050i32,
    Nfs4errBadSessionDigest = 10051i32,
    Nfs4errBadsession = 10052i32,
    Nfs4errBadslot = 10053i32,
    Nfs4errCompleteAlready = 10054i32,
    Nfs4errConnNotBoundToSession = 10055i32,
    Nfs4errDelegAlreadyWanted = 10056i32,
    Nfs4errBackChanBusy = 10057i32,
    Nfs4errLayouttrylater = 10058i32,
    Nfs4errLayoutunavailable = 10059i32,
    Nfs4errNomatchingLayout = 10060i32,
    Nfs4errRecallconflict = 10061i32,
    Nfs4errUnknownLayouttype = 10062i32,
    Nfs4errSeqMisordered = 10063i32,
    Nfs4errSequencePos = 10064i32,
    Nfs4errReqTooBig = 10065i32,
    Nfs4errRepTooBig = 10066i32,
    Nfs4errRepTooBigToCache = 10067i32,
    Nfs4errRetryUncachedRep = 10068i32,
    Nfs4errUnsafeCompound = 10069i32,
    Nfs4errTooManyOps = 10070i32,
    Nfs4errOpNotInSession = 10071i32,
    Nfs4errHashAlgUnsupp = 10072i32,
    Nfs4errClientidBusy = 10074i32,
    Nfs4errPnfsIoHole = 10075i32,
    Nfs4errSeqFalseRetry = 10076i32,
    Nfs4errBadHighSlot = 10077i32,
    Nfs4errDeadsession = 10078i32,
    Nfs4errEncrAlgUnsupp = 10079i32,
    Nfs4errPnfsNoLayout = 10080i32,
    Nfs4errNotOnlyOp = 10081i32,
    Nfs4errWrongCred = 10082i32,
    Nfs4errWrongType = 10083i32,
    Nfs4errDirdelegUnavail = 10084i32,
    Nfs4errRejectDeleg = 10085i32,
    Nfs4errReturnconflict = 10086i32,
    Nfs4errDelegRevoked = 10087i32,
    Nfs4errPartnerNotsupp = 10088i32,
    Nfs4errPartnerNoAuth = 10089i32,
    Nfs4errUnionNotsupp = 10090i32,
    Nfs4errOffloadDenied = 10091i32,
    Nfs4errWrongLfs = 10092i32,
    Nfs4errBadlabel = 10093i32,
    Nfs4errOffloadNoReqs = 10094i32,
}
impl XdrIndexer for Nfsstat4 {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Nfs4Ok)),
            1i32 => Ok(stringify!(Nfs4errPerm)),
            2i32 => Ok(stringify!(Nfs4errNoent)),
            5i32 => Ok(stringify!(Nfs4errIo)),
            6i32 => Ok(stringify!(Nfs4errNxio)),
            13i32 => Ok(stringify!(Nfs4errAccess)),
            17i32 => Ok(stringify!(Nfs4errExist)),
            18i32 => Ok(stringify!(Nfs4errXdev)),
            20i32 => Ok(stringify!(Nfs4errNotdir)),
            21i32 => Ok(stringify!(Nfs4errIsdir)),
            22i32 => Ok(stringify!(Nfs4errInval)),
            27i32 => Ok(stringify!(Nfs4errFbig)),
            28i32 => Ok(stringify!(Nfs4errNospc)),
            30i32 => Ok(stringify!(Nfs4errRofs)),
            31i32 => Ok(stringify!(Nfs4errMlink)),
            63i32 => Ok(stringify!(Nfs4errNametoolong)),
            66i32 => Ok(stringify!(Nfs4errNotempty)),
            69i32 => Ok(stringify!(Nfs4errDquot)),
            70i32 => Ok(stringify!(Nfs4errStale)),
            10001i32 => Ok(stringify!(Nfs4errBadhandle)),
            10003i32 => Ok(stringify!(Nfs4errBadCookie)),
            10004i32 => Ok(stringify!(Nfs4errNotsupp)),
            10005i32 => Ok(stringify!(Nfs4errToosmall)),
            10006i32 => Ok(stringify!(Nfs4errServerfault)),
            10007i32 => Ok(stringify!(Nfs4errBadtype)),
            10008i32 => Ok(stringify!(Nfs4errDelay)),
            10009i32 => Ok(stringify!(Nfs4errSame)),
            10010i32 => Ok(stringify!(Nfs4errDenied)),
            10011i32 => Ok(stringify!(Nfs4errExpired)),
            10012i32 => Ok(stringify!(Nfs4errLocked)),
            10013i32 => Ok(stringify!(Nfs4errGrace)),
            10014i32 => Ok(stringify!(Nfs4errFhexpired)),
            10015i32 => Ok(stringify!(Nfs4errShareDenied)),
            10016i32 => Ok(stringify!(Nfs4errWrongsec)),
            10017i32 => Ok(stringify!(Nfs4errClidInuse)),
            10018i32 => Ok(stringify!(Nfs4errResource)),
            10019i32 => Ok(stringify!(Nfs4errMoved)),
            10020i32 => Ok(stringify!(Nfs4errNofilehandle)),
            10021i32 => Ok(stringify!(Nfs4errMinorVersMismatch)),
            10022i32 => Ok(stringify!(Nfs4errStaleClientid)),
            10023i32 => Ok(stringify!(Nfs4errStaleStateid)),
            10024i32 => Ok(stringify!(Nfs4errOldStateid)),
            10025i32 => Ok(stringify!(Nfs4errBadStateid)),
            10026i32 => Ok(stringify!(Nfs4errBadSeqid)),
            10027i32 => Ok(stringify!(Nfs4errNotSame)),
            10028i32 => Ok(stringify!(Nfs4errLockRange)),
            10029i32 => Ok(stringify!(Nfs4errSymlink)),
            10030i32 => Ok(stringify!(Nfs4errRestorefh)),
            10031i32 => Ok(stringify!(Nfs4errLeaseMoved)),
            10032i32 => Ok(stringify!(Nfs4errAttrnotsupp)),
            10033i32 => Ok(stringify!(Nfs4errNoGrace)),
            10034i32 => Ok(stringify!(Nfs4errReclaimBad)),
            10035i32 => Ok(stringify!(Nfs4errReclaimConflict)),
            10036i32 => Ok(stringify!(Nfs4errBadxdr)),
            10037i32 => Ok(stringify!(Nfs4errLocksHeld)),
            10038i32 => Ok(stringify!(Nfs4errOpenmode)),
            10039i32 => Ok(stringify!(Nfs4errBadowner)),
            10040i32 => Ok(stringify!(Nfs4errBadchar)),
            10041i32 => Ok(stringify!(Nfs4errBadname)),
            10042i32 => Ok(stringify!(Nfs4errBadRange)),
            10043i32 => Ok(stringify!(Nfs4errLockNotsupp)),
            10044i32 => Ok(stringify!(Nfs4errOpIllegal)),
            10045i32 => Ok(stringify!(Nfs4errDeadlock)),
            10046i32 => Ok(stringify!(Nfs4errFileOpen)),
            10047i32 => Ok(stringify!(Nfs4errAdminRevoked)),
            10048i32 => Ok(stringify!(Nfs4errCbPathDown)),
            10049i32 => Ok(stringify!(Nfs4errBadiomode)),
            10050i32 => Ok(stringify!(Nfs4errBadlayout)),
            10051i32 => Ok(stringify!(Nfs4errBadSessionDigest)),
            10052i32 => Ok(stringify!(Nfs4errBadsession)),
            10053i32 => Ok(stringify!(Nfs4errBadslot)),
            10054i32 => Ok(stringify!(Nfs4errCompleteAlready)),
            10055i32 => Ok(stringify!(Nfs4errConnNotBoundToSession)),
            10056i32 => Ok(stringify!(Nfs4errDelegAlreadyWanted)),
            10057i32 => Ok(stringify!(Nfs4errBackChanBusy)),
            10058i32 => Ok(stringify!(Nfs4errLayouttrylater)),
            10059i32 => Ok(stringify!(Nfs4errLayoutunavailable)),
            10060i32 => Ok(stringify!(Nfs4errNomatchingLayout)),
            10061i32 => Ok(stringify!(Nfs4errRecallconflict)),
            10062i32 => Ok(stringify!(Nfs4errUnknownLayouttype)),
            10063i32 => Ok(stringify!(Nfs4errSeqMisordered)),
            10064i32 => Ok(stringify!(Nfs4errSequencePos)),
            10065i32 => Ok(stringify!(Nfs4errReqTooBig)),
            10066i32 => Ok(stringify!(Nfs4errRepTooBig)),
            10067i32 => Ok(stringify!(Nfs4errRepTooBigToCache)),
            10068i32 => Ok(stringify!(Nfs4errRetryUncachedRep)),
            10069i32 => Ok(stringify!(Nfs4errUnsafeCompound)),
            10070i32 => Ok(stringify!(Nfs4errTooManyOps)),
            10071i32 => Ok(stringify!(Nfs4errOpNotInSession)),
            10072i32 => Ok(stringify!(Nfs4errHashAlgUnsupp)),
            10074i32 => Ok(stringify!(Nfs4errClientidBusy)),
            10075i32 => Ok(stringify!(Nfs4errPnfsIoHole)),
            10076i32 => Ok(stringify!(Nfs4errSeqFalseRetry)),
            10077i32 => Ok(stringify!(Nfs4errBadHighSlot)),
            10078i32 => Ok(stringify!(Nfs4errDeadsession)),
            10079i32 => Ok(stringify!(Nfs4errEncrAlgUnsupp)),
            10080i32 => Ok(stringify!(Nfs4errPnfsNoLayout)),
            10081i32 => Ok(stringify!(Nfs4errNotOnlyOp)),
            10082i32 => Ok(stringify!(Nfs4errWrongCred)),
            10083i32 => Ok(stringify!(Nfs4errWrongType)),
            10084i32 => Ok(stringify!(Nfs4errDirdelegUnavail)),
            10085i32 => Ok(stringify!(Nfs4errRejectDeleg)),
            10086i32 => Ok(stringify!(Nfs4errReturnconflict)),
            10087i32 => Ok(stringify!(Nfs4errDelegRevoked)),
            10088i32 => Ok(stringify!(Nfs4errPartnerNotsupp)),
            10089i32 => Ok(stringify!(Nfs4errPartnerNoAuth)),
            10090i32 => Ok(stringify!(Nfs4errUnionNotsupp)),
            10091i32 => Ok(stringify!(Nfs4errOffloadDenied)),
            10092i32 => Ok(stringify!(Nfs4errWrongLfs)),
            10093i32 => Ok(stringify!(Nfs4errBadlabel)),
            10094i32 => Ok(stringify!(Nfs4errOffloadNoReqs)),
            _ => Ok(stringify!(Nfs4Ok)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            Nfsstat4::Nfs4Ok => 0i32,
            Nfsstat4::Nfs4errPerm => 1i32,
            Nfsstat4::Nfs4errNoent => 2i32,
            Nfsstat4::Nfs4errIo => 5i32,
            Nfsstat4::Nfs4errNxio => 6i32,
            Nfsstat4::Nfs4errAccess => 13i32,
            Nfsstat4::Nfs4errExist => 17i32,
            Nfsstat4::Nfs4errXdev => 18i32,
            Nfsstat4::Nfs4errNotdir => 20i32,
            Nfsstat4::Nfs4errIsdir => 21i32,
            Nfsstat4::Nfs4errInval => 22i32,
            Nfsstat4::Nfs4errFbig => 27i32,
            Nfsstat4::Nfs4errNospc => 28i32,
            Nfsstat4::Nfs4errRofs => 30i32,
            Nfsstat4::Nfs4errMlink => 31i32,
            Nfsstat4::Nfs4errNametoolong => 63i32,
            Nfsstat4::Nfs4errNotempty => 66i32,
            Nfsstat4::Nfs4errDquot => 69i32,
            Nfsstat4::Nfs4errStale => 70i32,
            Nfsstat4::Nfs4errBadhandle => 10001i32,
            Nfsstat4::Nfs4errBadCookie => 10003i32,
            Nfsstat4::Nfs4errNotsupp => 10004i32,
            Nfsstat4::Nfs4errToosmall => 10005i32,
            Nfsstat4::Nfs4errServerfault => 10006i32,
            Nfsstat4::Nfs4errBadtype => 10007i32,
            Nfsstat4::Nfs4errDelay => 10008i32,
            Nfsstat4::Nfs4errSame => 10009i32,
            Nfsstat4::Nfs4errDenied => 10010i32,
            Nfsstat4::Nfs4errExpired => 10011i32,
            Nfsstat4::Nfs4errLocked => 10012i32,
            Nfsstat4::Nfs4errGrace => 10013i32,
            Nfsstat4::Nfs4errFhexpired => 10014i32,
            Nfsstat4::Nfs4errShareDenied => 10015i32,
            Nfsstat4::Nfs4errWrongsec => 10016i32,
            Nfsstat4::Nfs4errClidInuse => 10017i32,
            Nfsstat4::Nfs4errResource => 10018i32,
            Nfsstat4::Nfs4errMoved => 10019i32,
            Nfsstat4::Nfs4errNofilehandle => 10020i32,
            Nfsstat4::Nfs4errMinorVersMismatch => 10021i32,
            Nfsstat4::Nfs4errStaleClientid => 10022i32,
            Nfsstat4::Nfs4errStaleStateid => 10023i32,
            Nfsstat4::Nfs4errOldStateid => 10024i32,
            Nfsstat4::Nfs4errBadStateid => 10025i32,
            Nfsstat4::Nfs4errBadSeqid => 10026i32,
            Nfsstat4::Nfs4errNotSame => 10027i32,
            Nfsstat4::Nfs4errLockRange => 10028i32,
            Nfsstat4::Nfs4errSymlink => 10029i32,
            Nfsstat4::Nfs4errRestorefh => 10030i32,
            Nfsstat4::Nfs4errLeaseMoved => 10031i32,
            Nfsstat4::Nfs4errAttrnotsupp => 10032i32,
            Nfsstat4::Nfs4errNoGrace => 10033i32,
            Nfsstat4::Nfs4errReclaimBad => 10034i32,
            Nfsstat4::Nfs4errReclaimConflict => 10035i32,
            Nfsstat4::Nfs4errBadxdr => 10036i32,
            Nfsstat4::Nfs4errLocksHeld => 10037i32,
            Nfsstat4::Nfs4errOpenmode => 10038i32,
            Nfsstat4::Nfs4errBadowner => 10039i32,
            Nfsstat4::Nfs4errBadchar => 10040i32,
            Nfsstat4::Nfs4errBadname => 10041i32,
            Nfsstat4::Nfs4errBadRange => 10042i32,
            Nfsstat4::Nfs4errLockNotsupp => 10043i32,
            Nfsstat4::Nfs4errOpIllegal => 10044i32,
            Nfsstat4::Nfs4errDeadlock => 10045i32,
            Nfsstat4::Nfs4errFileOpen => 10046i32,
            Nfsstat4::Nfs4errAdminRevoked => 10047i32,
            Nfsstat4::Nfs4errCbPathDown => 10048i32,
            Nfsstat4::Nfs4errBadiomode => 10049i32,
            Nfsstat4::Nfs4errBadlayout => 10050i32,
            Nfsstat4::Nfs4errBadSessionDigest => 10051i32,
            Nfsstat4::Nfs4errBadsession => 10052i32,
            Nfsstat4::Nfs4errBadslot => 10053i32,
            Nfsstat4::Nfs4errCompleteAlready => 10054i32,
            Nfsstat4::Nfs4errConnNotBoundToSession => 10055i32,
            Nfsstat4::Nfs4errDelegAlreadyWanted => 10056i32,
            Nfsstat4::Nfs4errBackChanBusy => 10057i32,
            Nfsstat4::Nfs4errLayouttrylater => 10058i32,
            Nfsstat4::Nfs4errLayoutunavailable => 10059i32,
            Nfsstat4::Nfs4errNomatchingLayout => 10060i32,
            Nfsstat4::Nfs4errRecallconflict => 10061i32,
            Nfsstat4::Nfs4errUnknownLayouttype => 10062i32,
            Nfsstat4::Nfs4errSeqMisordered => 10063i32,
            Nfsstat4::Nfs4errSequencePos => 10064i32,
            Nfsstat4::Nfs4errReqTooBig => 10065i32,
            Nfsstat4::Nfs4errRepTooBig => 10066i32,
            Nfsstat4::Nfs4errRepTooBigToCache => 10067i32,
            Nfsstat4::Nfs4errRetryUncachedRep => 10068i32,
            Nfsstat4::Nfs4errUnsafeCompound => 10069i32,
            Nfsstat4::Nfs4errTooManyOps => 10070i32,
            Nfsstat4::Nfs4errOpNotInSession => 10071i32,
            Nfsstat4::Nfs4errHashAlgUnsupp => 10072i32,
            Nfsstat4::Nfs4errClientidBusy => 10074i32,
            Nfsstat4::Nfs4errPnfsIoHole => 10075i32,
            Nfsstat4::Nfs4errSeqFalseRetry => 10076i32,
            Nfsstat4::Nfs4errBadHighSlot => 10077i32,
            Nfsstat4::Nfs4errDeadsession => 10078i32,
            Nfsstat4::Nfs4errEncrAlgUnsupp => 10079i32,
            Nfsstat4::Nfs4errPnfsNoLayout => 10080i32,
            Nfsstat4::Nfs4errNotOnlyOp => 10081i32,
            Nfsstat4::Nfs4errWrongCred => 10082i32,
            Nfsstat4::Nfs4errWrongType => 10083i32,
            Nfsstat4::Nfs4errDirdelegUnavail => 10084i32,
            Nfsstat4::Nfs4errRejectDeleg => 10085i32,
            Nfsstat4::Nfs4errReturnconflict => 10086i32,
            Nfsstat4::Nfs4errDelegRevoked => 10087i32,
            Nfsstat4::Nfs4errPartnerNotsupp => 10088i32,
            Nfsstat4::Nfs4errPartnerNoAuth => 10089i32,
            Nfsstat4::Nfs4errUnionNotsupp => 10090i32,
            Nfsstat4::Nfs4errOffloadDenied => 10091i32,
            Nfsstat4::Nfs4errWrongLfs => 10092i32,
            Nfsstat4::Nfs4errBadlabel => 10093i32,
            Nfsstat4::Nfs4errOffloadNoReqs => 10094i32,
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Nfstime4 {
    pub seconds: i64,
    pub nseconds: u32,
}
#[derive(Clone, Debug, PartialEq, XdrIndexer)]
#[repr(i32)]
#[derive(Default)]
pub enum TimeHow4 {
    #[default]
    SetToServerTime4 = 0i32,
    SetToClientTime4 = 1i32,
}
impl XdrIndexer for TimeHow4 {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(SetToServerTime4)),
            1i32 => Ok(stringify!(SetToClientTime4)),
            _ => Ok(stringify!(SetToServerTime4)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            TimeHow4::SetToServerTime4 => 0i32,
            TimeHow4::SetToClientTime4 => 1i32,
        }
    }
}
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum Settime4 {
    SetToClientTime4(Nfstime4),
    #[default]
    Default,
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
pub struct ChangePolicy4 {
    pub cp_major: u64,
    pub cp_minor: u64,
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
pub struct Nfsacl41 {
    pub na41_flag: u32,
    pub na41_aces: Vec<Nfsace4>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ModeMasked4 {
    pub mm_value_to_set: u32,
    pub mm_mask_bits: u32,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Specdata4 {
    pub specdata1: u32,
    pub specdata2: u32,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Netaddr4 {
    pub na_r_netid: String,
    pub na_r_addr: String,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct NfsImplId4 {
    pub nii_domain: String,
    pub nii_name: String,
    pub nii_date: Nfstime4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Stateid4 {
    pub seqid: u32,
    #[serde(with = "serde_xdr::opaque::fixed")]
    pub other: [u8; NFS4_OTHER_SIZE as usize],
}
#[derive(Clone, Debug, PartialEq, XdrIndexer)]
#[repr(i32)]
#[derive(Default)]
pub enum Layouttype4 {
    #[default]
    Layout4Nfsv41Files = 1i32,
    Layout4Osd2Objects = 2i32,
    Layout4BlockVolume = 3i32,
}
impl XdrIndexer for Layouttype4 {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            1i32 => Ok(stringify!(Layout4Nfsv41Files)),
            2i32 => Ok(stringify!(Layout4Osd2Objects)),
            3i32 => Ok(stringify!(Layout4BlockVolume)),
            _ => Ok(stringify!(Layout4Nfsv41Files)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            Layouttype4::Layout4Nfsv41Files => 1i32,
            Layouttype4::Layout4Osd2Objects => 2i32,
            Layouttype4::Layout4BlockVolume => 3i32,
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LayoutContent4 {
    pub loc_type: Layouttype4,
    #[serde(with = "serde_xdr::opaque::variable")]
    pub loc_body: Vec<u8>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Layouthint4 {
    pub loh_type: Layouttype4,
    #[serde(with = "serde_xdr::opaque::variable")]
    pub loh_body: Vec<u8>,
}
#[derive(Clone, Debug, PartialEq, XdrIndexer)]
#[repr(i32)]
#[derive(Default)]
pub enum Layoutiomode4 {
    #[default]
    Layoutiomode4Read = 1i32,
    Layoutiomode4Rw = 2i32,
    Layoutiomode4Any = 3i32,
}
impl XdrIndexer for Layoutiomode4 {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            1i32 => Ok(stringify!(Layoutiomode4Read)),
            2i32 => Ok(stringify!(Layoutiomode4Rw)),
            3i32 => Ok(stringify!(Layoutiomode4Any)),
            _ => Ok(stringify!(Layoutiomode4Read)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            Layoutiomode4::Layoutiomode4Read => 1i32,
            Layoutiomode4::Layoutiomode4Rw => 2i32,
            Layoutiomode4::Layoutiomode4Any => 3i32,
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Layout4 {
    pub lo_offset: u64,
    pub lo_length: u64,
    pub lo_iomode: Layoutiomode4,
    pub lo_content: LayoutContent4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DeviceAddr4 {
    pub da_layout_type: Layouttype4,
    #[serde(with = "serde_xdr::opaque::variable")]
    pub da_addr_body: Vec<u8>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Layoutupdate4 {
    pub lou_type: Layouttype4,
    #[serde(with = "serde_xdr::opaque::variable")]
    pub lou_body: Vec<u8>,
}
#[derive(Clone, Debug, PartialEq, XdrIndexer)]
#[repr(i32)]
#[derive(Default)]
pub enum LayoutreturnType4 {
    #[default]
    Layoutreturn4File = 1i32,
    Layoutreturn4Fsid = 2i32,
    Layoutreturn4All = 3i32,
}
impl XdrIndexer for LayoutreturnType4 {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            1i32 => Ok(stringify!(Layoutreturn4File)),
            2i32 => Ok(stringify!(Layoutreturn4Fsid)),
            3i32 => Ok(stringify!(Layoutreturn4All)),
            _ => Ok(stringify!(Layoutreturn4File)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            LayoutreturnType4::Layoutreturn4File => 1i32,
            LayoutreturnType4::Layoutreturn4Fsid => 2i32,
            LayoutreturnType4::Layoutreturn4All => 3i32,
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LayoutreturnFile4 {
    pub lrf_offset: u64,
    pub lrf_length: u64,
    pub lrf_stateid: Stateid4,
    #[serde(with = "serde_xdr::opaque::variable")]
    pub lrf_body: Vec<u8>,
}
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum Layoutreturn4 {
    Layoutreturn4File(LayoutreturnFile4),
    #[default]
    Default,
}
impl XdrIndexer for Layoutreturn4 {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            1i32 => Ok(stringify!(Layoutreturn4File)),
            _ => Ok(stringify!(Default)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            Layoutreturn4::Layoutreturn4File(_) => 1i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, XdrIndexer)]
#[repr(i32)]
#[derive(Default)]
pub enum Fs4StatusType {
    #[default]
    Status4Fixed = 1i32,
    Status4Updated = 2i32,
    Status4Versioned = 3i32,
    Status4Writable = 4i32,
    Status4Referral = 5i32,
}
impl XdrIndexer for Fs4StatusType {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            1i32 => Ok(stringify!(Status4Fixed)),
            2i32 => Ok(stringify!(Status4Updated)),
            3i32 => Ok(stringify!(Status4Versioned)),
            4i32 => Ok(stringify!(Status4Writable)),
            5i32 => Ok(stringify!(Status4Referral)),
            _ => Ok(stringify!(Status4Fixed)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            Fs4StatusType::Status4Fixed => 1i32,
            Fs4StatusType::Status4Updated => 2i32,
            Fs4StatusType::Status4Versioned => 3i32,
            Fs4StatusType::Status4Writable => 4i32,
            Fs4StatusType::Status4Referral => 5i32,
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Fs4Status {
    pub fss_absent: bool,
    pub fss_type: Fs4StatusType,
    pub fss_source: String,
    pub fss_current: String,
    pub fss_age: i32,
    pub fss_version: Nfstime4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ThresholdItem4 {
    pub thi_layout_type: Layouttype4,
    pub thi_hintset: Vec<u32>,
    #[serde(with = "serde_xdr::opaque::variable")]
    pub thi_hintlist: Vec<u8>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Mdsthreshold4 {
    pub mth_hints: Vec<ThresholdItem4>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RetentionGet4 {
    pub rg_duration: u64,
    pub rg_begin_time: Vec<Nfstime4>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RetentionSet4 {
    pub rs_enable: bool,
    pub rs_duration: Vec<u64>,
}
#[derive(Clone, Debug, PartialEq, XdrIndexer)]
#[repr(i32)]
#[derive(Default)]
pub enum NetlocType4 {
    #[default]
    Nl4Name = 1i32,
    Nl4Url = 2i32,
    Nl4Netaddr = 3i32,
}
impl XdrIndexer for NetlocType4 {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            1i32 => Ok(stringify!(Nl4Name)),
            2i32 => Ok(stringify!(Nl4Url)),
            3i32 => Ok(stringify!(Nl4Netaddr)),
            _ => Ok(stringify!(Nl4Name)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            NetlocType4::Nl4Name => 1i32,
            NetlocType4::Nl4Url => 2i32,
            NetlocType4::Nl4Netaddr => 3i32,
        }
    }
}
#[derive(Clone, Debug, XdrIndexer)]
pub enum Netloc4 {
    Nl4Name(String),
    Nl4Url(String),
    Nl4Netaddr(Netaddr4),
}
impl Default for Netloc4 {
    fn default() -> Self {
        Netloc4::Nl4Name(Default::default())
    }
}
impl XdrIndexer for Netloc4 {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            1i32 => Ok(stringify!(Nl4Name)),
            2i32 => Ok(stringify!(Nl4Url)),
            3i32 => Ok(stringify!(Nl4Netaddr)),
            _ => Err(::serde_xdr::error::Error::Convert),
        }
    }
    fn index(&self) -> i32 {
        match self {
            Netloc4::Nl4Name(_) => 1i32,
            Netloc4::Nl4Url(_) => 2i32,
            Netloc4::Nl4Netaddr(_) => 3i32,
        }
    }
}
#[derive(Clone, Debug, PartialEq, XdrIndexer)]
#[repr(i32)]
#[derive(Default)]
pub enum ChangeAttrType4 {
    #[default]
    Nfs4ChangeTypeIsMonotonicIncr = 0i32,
    Nfs4ChangeTypeIsVersionCounter = 1i32,
    Nfs4ChangeTypeIsVersionCounterNopnfs = 2i32,
    Nfs4ChangeTypeIsTimeMetadata = 3i32,
    Nfs4ChangeTypeIsUndefined = 4i32,
}
impl XdrIndexer for ChangeAttrType4 {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Nfs4ChangeTypeIsMonotonicIncr)),
            1i32 => Ok(stringify!(Nfs4ChangeTypeIsVersionCounter)),
            2i32 => Ok(stringify!(Nfs4ChangeTypeIsVersionCounterNopnfs)),
            3i32 => Ok(stringify!(Nfs4ChangeTypeIsTimeMetadata)),
            4i32 => Ok(stringify!(Nfs4ChangeTypeIsUndefined)),
            _ => Ok(stringify!(Nfs4ChangeTypeIsMonotonicIncr)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            ChangeAttrType4::Nfs4ChangeTypeIsMonotonicIncr => 0i32,
            ChangeAttrType4::Nfs4ChangeTypeIsVersionCounter => 1i32,
            ChangeAttrType4::Nfs4ChangeTypeIsVersionCounterNopnfs => 2i32,
            ChangeAttrType4::Nfs4ChangeTypeIsTimeMetadata => 3i32,
            ChangeAttrType4::Nfs4ChangeTypeIsUndefined => 4i32,
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LabelformatSpec4 {
    pub lfs_lfs: u32,
    pub lfs_pi: u32,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SecLabel4 {
    pub slai_lfs: LabelformatSpec4,
    #[serde(with = "serde_xdr::opaque::variable")]
    pub slai_data: Vec<u8>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CopyFromAuthPriv {
    pub cfap_shared_secret: String,
    pub cfap_destination: Netloc4,
    pub cfap_username: String,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CopyToAuthPriv {
    pub ctap_shared_secret: String,
    pub ctap_source: Vec<Netloc4>,
    pub ctap_username: String,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CopyConfirmAuthPriv {
    #[serde(with = "serde_xdr::opaque::variable")]
    pub ccap_shared_secret_mic: Vec<u8>,
    pub ccap_username: String,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AppDataBlock4 {
    pub adb_offset: u64,
    pub adb_block_size: u64,
    pub adb_block_count: u64,
    pub adb_reloff_blocknum: u64,
    pub adb_block_num: u32,
    pub adb_reloff_pattern: u64,
    #[serde(with = "serde_xdr::opaque::variable")]
    pub adb_pattern: Vec<u8>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Data4 {
    pub d_offset: u64,
    #[serde(with = "serde_xdr::opaque::variable")]
    pub d_data: Vec<u8>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DataInfo4 {
    pub di_offset: u64,
    pub di_length: u64,
}
#[derive(Clone, Debug, PartialEq, XdrIndexer)]
#[repr(i32)]
#[derive(Default)]
pub enum DataContent4 {
    #[default]
    Nfs4ContentData = 0i32,
    Nfs4ContentHole = 1i32,
}
impl XdrIndexer for DataContent4 {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Nfs4ContentData)),
            1i32 => Ok(stringify!(Nfs4ContentHole)),
            _ => Ok(stringify!(Nfs4ContentData)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            DataContent4::Nfs4ContentData => 0i32,
            DataContent4::Nfs4ContentHole => 1i32,
        }
    }
}
#[derive(Clone, Debug, PartialEq, XdrIndexer)]
#[repr(i32)]
#[derive(Default)]
pub enum StableHow4 {
    #[default]
    UNSTABLE4 = 0i32,
    DataSync4 = 1i32,
    FileSync4 = 2i32,
}
impl XdrIndexer for StableHow4 {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(UNSTABLE4)),
            1i32 => Ok(stringify!(DataSync4)),
            2i32 => Ok(stringify!(FileSync4)),
            _ => Ok(stringify!(UNSTABLE4)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            StableHow4::UNSTABLE4 => 0i32,
            StableHow4::DataSync4 => 1i32,
            StableHow4::FileSync4 => 2i32,
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct WriteResponse4 {
    pub wr_callback_id: Vec<Stateid4>,
    pub wr_count: u64,
    pub wr_committed: StableHow4,
    #[serde(with = "serde_xdr::opaque::fixed")]
    pub wr_writeverf: [u8; NFS4_VERIFIER_SIZE as usize],
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
pub struct CbClient4 {
    pub cb_program: u32,
    pub cb_location: Netaddr4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct NfsClientId4 {
    #[serde(with = "serde_xdr::opaque::fixed")]
    pub verifier: [u8; NFS4_VERIFIER_SIZE as usize],
    #[serde(with = "serde_xdr::opaque::variable")]
    pub id: Vec<u8>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ClientOwner4 {
    #[serde(with = "serde_xdr::opaque::fixed")]
    pub co_verifier: [u8; NFS4_VERIFIER_SIZE as usize],
    #[serde(with = "serde_xdr::opaque::variable")]
    pub co_ownerid: Vec<u8>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ServerOwner4 {
    pub so_minor_id: u64,
    #[serde(with = "serde_xdr::opaque::variable")]
    pub so_major_id: Vec<u8>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct StateOwner4 {
    pub clientid: u64,
    #[serde(with = "serde_xdr::opaque::variable")]
    pub owner: Vec<u8>,
}
#[derive(Clone, Debug, PartialEq, XdrIndexer)]
#[repr(i32)]
#[derive(Default)]
pub enum NfsLockType4 {
    #[default]
    ReadLt = 1i32,
    WriteLt = 2i32,
    ReadwLt = 3i32,
    WritewLt = 4i32,
}
impl XdrIndexer for NfsLockType4 {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            1i32 => Ok(stringify!(ReadLt)),
            2i32 => Ok(stringify!(WriteLt)),
            3i32 => Ok(stringify!(ReadwLt)),
            4i32 => Ok(stringify!(WritewLt)),
            _ => Ok(stringify!(ReadLt)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            NfsLockType4::ReadLt => 1i32,
            NfsLockType4::WriteLt => 2i32,
            NfsLockType4::ReadwLt => 3i32,
            NfsLockType4::WritewLt => 4i32,
        }
    }
}
#[derive(Clone, Debug, PartialEq, XdrIndexer)]
#[repr(i32)]
#[derive(Default)]
pub enum SsvSubkey4 {
    #[default]
    Ssv4SubkeyMicI2t = 1i32,
    Ssv4SubkeyMicT2i = 2i32,
    Ssv4SubkeySealI2t = 3i32,
    Ssv4SubkeySealT2i = 4i32,
}
impl XdrIndexer for SsvSubkey4 {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            1i32 => Ok(stringify!(Ssv4SubkeyMicI2t)),
            2i32 => Ok(stringify!(Ssv4SubkeyMicT2i)),
            3i32 => Ok(stringify!(Ssv4SubkeySealI2t)),
            4i32 => Ok(stringify!(Ssv4SubkeySealT2i)),
            _ => Ok(stringify!(Ssv4SubkeyMicI2t)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            SsvSubkey4::Ssv4SubkeyMicI2t => 1i32,
            SsvSubkey4::Ssv4SubkeyMicT2i => 2i32,
            SsvSubkey4::Ssv4SubkeySealI2t => 3i32,
            SsvSubkey4::Ssv4SubkeySealT2i => 4i32,
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SsvMicPlainTkn4 {
    pub smpt_ssv_seq: u32,
    #[serde(with = "serde_xdr::opaque::variable")]
    pub smpt_orig_plain: Vec<u8>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SsvMicTkn4 {
    pub smt_ssv_seq: u32,
    #[serde(with = "serde_xdr::opaque::variable")]
    pub smt_hmac: Vec<u8>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SsvSealPlainTkn4 {
    #[serde(with = "serde_xdr::opaque::variable")]
    pub sspt_confounder: Vec<u8>,
    pub sspt_ssv_seq: u32,
    #[serde(with = "serde_xdr::opaque::variable")]
    pub sspt_orig_plain: Vec<u8>,
    #[serde(with = "serde_xdr::opaque::variable")]
    pub sspt_pad: Vec<u8>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SsvSealCipherTkn4 {
    pub ssct_ssv_seq: u32,
    #[serde(with = "serde_xdr::opaque::variable")]
    pub ssct_iv: Vec<u8>,
    #[serde(with = "serde_xdr::opaque::variable")]
    pub ssct_encr_data: Vec<u8>,
    #[serde(with = "serde_xdr::opaque::variable")]
    pub ssct_hmac: Vec<u8>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct FsLocationsServer4 {
    pub fls_currency: i32,
    #[serde(with = "serde_xdr::opaque::variable")]
    pub fls_info: Vec<u8>,
    pub fls_server: String,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct FsLocationsItem4 {
    pub fli_entries: Vec<FsLocationsServer4>,
    pub fli_rootpath: Vec<String>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct FsLocationsInfo4 {
    pub fli_flags: u32,
    pub fli_valid_for: i32,
    pub fli_fs_root: Vec<String>,
    pub fli_items: Vec<FsLocationsItem4>,
}
#[derive(Clone, Debug, PartialEq, XdrIndexer)]
#[repr(i32)]
#[derive(Default)]
pub enum FilelayoutHintCare4 {
    #[default]
    Nflh4CareDense = 1i32,
    Nflh4CareCommitThruMds = 2i32,
    Nfl42CareIoAdviseThruMds = 4i32,
    Nflh4CareStripeUnitSize = 64i32,
    Nflh4CareStripeCount = 128i32,
}
impl XdrIndexer for FilelayoutHintCare4 {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            1i32 => Ok(stringify!(Nflh4CareDense)),
            2i32 => Ok(stringify!(Nflh4CareCommitThruMds)),
            4i32 => Ok(stringify!(Nfl42CareIoAdviseThruMds)),
            64i32 => Ok(stringify!(Nflh4CareStripeUnitSize)),
            128i32 => Ok(stringify!(Nflh4CareStripeCount)),
            _ => Ok(stringify!(Nflh4CareDense)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            FilelayoutHintCare4::Nflh4CareDense => 1i32,
            FilelayoutHintCare4::Nflh4CareCommitThruMds => 2i32,
            FilelayoutHintCare4::Nfl42CareIoAdviseThruMds => 4i32,
            FilelayoutHintCare4::Nflh4CareStripeUnitSize => 64i32,
            FilelayoutHintCare4::Nflh4CareStripeCount => 128i32,
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Nfsv41FileLayouthint4 {
    pub nflh_care: u32,
    pub nflh_util: u32,
    pub nflh_stripe_count: u32,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Nfsv41FileLayoutDsAddr4 {
    pub nflda_stripe_indices: Vec<u32>,
    pub nflda_multipath_ds_list: Vec<Vec<Netaddr4>>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Nfsv41FileLayout4 {
    #[serde(with = "serde_xdr::opaque::fixed")]
    pub nfl_deviceid: [u8; NFS4_DEVICEID4_SIZE as usize],
    pub nfl_util: u32,
    pub nfl_first_stripe_index: u32,
    pub nfl_pattern_offset: u64,
    pub nfl_fh_list: Vec<serde_xdr::opaque::VariableArray>,
}
#[derive(Clone, Debug, PartialEq, XdrIndexer)]
#[repr(i32)]
#[derive(Default)]
pub enum NfsOpnum4 {
    #[default]
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
    OpBackchannelCtl = 40i32,
    OpBindConnToSession = 41i32,
    OpExchangeId = 42i32,
    OpCreateSession = 43i32,
    OpDestroySession = 44i32,
    OpFreeStateid = 45i32,
    OpGetDirDelegation = 46i32,
    OpGetdeviceinfo = 47i32,
    OpGetdevicelist = 48i32,
    OpLayoutcommit = 49i32,
    OpLayoutget = 50i32,
    OpLayoutreturn = 51i32,
    OpSecinfoNoName = 52i32,
    OpSequence = 53i32,
    OpSetSsv = 54i32,
    OpTestStateid = 55i32,
    OpWantDelegation = 56i32,
    OpDestroyClientid = 57i32,
    OpReclaimComplete = 58i32,
    OpAllocate = 59i32,
    OpCopy = 60i32,
    OpCopyNotify = 61i32,
    OpDeallocate = 62i32,
    OpIoAdvise = 63i32,
    OpLayouterror = 64i32,
    OpLayoutstats = 65i32,
    OpOffloadCancel = 66i32,
    OpOffloadStatus = 67i32,
    OpReadPlus = 68i32,
    OpSeek = 69i32,
    OpWriteSame = 70i32,
    OpClone = 71i32,
    OpIllegal = 10044i32,
}
impl XdrIndexer for NfsOpnum4 {
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
            40i32 => Ok(stringify!(OpBackchannelCtl)),
            41i32 => Ok(stringify!(OpBindConnToSession)),
            42i32 => Ok(stringify!(OpExchangeId)),
            43i32 => Ok(stringify!(OpCreateSession)),
            44i32 => Ok(stringify!(OpDestroySession)),
            45i32 => Ok(stringify!(OpFreeStateid)),
            46i32 => Ok(stringify!(OpGetDirDelegation)),
            47i32 => Ok(stringify!(OpGetdeviceinfo)),
            48i32 => Ok(stringify!(OpGetdevicelist)),
            49i32 => Ok(stringify!(OpLayoutcommit)),
            50i32 => Ok(stringify!(OpLayoutget)),
            51i32 => Ok(stringify!(OpLayoutreturn)),
            52i32 => Ok(stringify!(OpSecinfoNoName)),
            53i32 => Ok(stringify!(OpSequence)),
            54i32 => Ok(stringify!(OpSetSsv)),
            55i32 => Ok(stringify!(OpTestStateid)),
            56i32 => Ok(stringify!(OpWantDelegation)),
            57i32 => Ok(stringify!(OpDestroyClientid)),
            58i32 => Ok(stringify!(OpReclaimComplete)),
            59i32 => Ok(stringify!(OpAllocate)),
            60i32 => Ok(stringify!(OpCopy)),
            61i32 => Ok(stringify!(OpCopyNotify)),
            62i32 => Ok(stringify!(OpDeallocate)),
            63i32 => Ok(stringify!(OpIoAdvise)),
            64i32 => Ok(stringify!(OpLayouterror)),
            65i32 => Ok(stringify!(OpLayoutstats)),
            66i32 => Ok(stringify!(OpOffloadCancel)),
            67i32 => Ok(stringify!(OpOffloadStatus)),
            68i32 => Ok(stringify!(OpReadPlus)),
            69i32 => Ok(stringify!(OpSeek)),
            70i32 => Ok(stringify!(OpWriteSame)),
            71i32 => Ok(stringify!(OpClone)),
            10044i32 => Ok(stringify!(OpIllegal)),
            _ => Ok(stringify!(OpAccess)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            NfsOpnum4::OpAccess => 3i32,
            NfsOpnum4::OpClose => 4i32,
            NfsOpnum4::OpCommit => 5i32,
            NfsOpnum4::OpCreate => 6i32,
            NfsOpnum4::OpDelegpurge => 7i32,
            NfsOpnum4::OpDelegreturn => 8i32,
            NfsOpnum4::OpGetattr => 9i32,
            NfsOpnum4::OpGetfh => 10i32,
            NfsOpnum4::OpLink => 11i32,
            NfsOpnum4::OpLock => 12i32,
            NfsOpnum4::OpLockt => 13i32,
            NfsOpnum4::OpLocku => 14i32,
            NfsOpnum4::OpLookup => 15i32,
            NfsOpnum4::OpLookupp => 16i32,
            NfsOpnum4::OpNverify => 17i32,
            NfsOpnum4::OpOpen => 18i32,
            NfsOpnum4::OpOpenattr => 19i32,
            NfsOpnum4::OpOpenConfirm => 20i32,
            NfsOpnum4::OpOpenDowngrade => 21i32,
            NfsOpnum4::OpPutfh => 22i32,
            NfsOpnum4::OpPutpubfh => 23i32,
            NfsOpnum4::OpPutrootfh => 24i32,
            NfsOpnum4::OpRead => 25i32,
            NfsOpnum4::OpReaddir => 26i32,
            NfsOpnum4::OpReadlink => 27i32,
            NfsOpnum4::OpRemove => 28i32,
            NfsOpnum4::OpRename => 29i32,
            NfsOpnum4::OpRenew => 30i32,
            NfsOpnum4::OpRestorefh => 31i32,
            NfsOpnum4::OpSavefh => 32i32,
            NfsOpnum4::OpSecinfo => 33i32,
            NfsOpnum4::OpSetattr => 34i32,
            NfsOpnum4::OpSetclientid => 35i32,
            NfsOpnum4::OpSetclientidConfirm => 36i32,
            NfsOpnum4::OpVerify => 37i32,
            NfsOpnum4::OpWrite => 38i32,
            NfsOpnum4::OpReleaseLockowner => 39i32,
            NfsOpnum4::OpBackchannelCtl => 40i32,
            NfsOpnum4::OpBindConnToSession => 41i32,
            NfsOpnum4::OpExchangeId => 42i32,
            NfsOpnum4::OpCreateSession => 43i32,
            NfsOpnum4::OpDestroySession => 44i32,
            NfsOpnum4::OpFreeStateid => 45i32,
            NfsOpnum4::OpGetDirDelegation => 46i32,
            NfsOpnum4::OpGetdeviceinfo => 47i32,
            NfsOpnum4::OpGetdevicelist => 48i32,
            NfsOpnum4::OpLayoutcommit => 49i32,
            NfsOpnum4::OpLayoutget => 50i32,
            NfsOpnum4::OpLayoutreturn => 51i32,
            NfsOpnum4::OpSecinfoNoName => 52i32,
            NfsOpnum4::OpSequence => 53i32,
            NfsOpnum4::OpSetSsv => 54i32,
            NfsOpnum4::OpTestStateid => 55i32,
            NfsOpnum4::OpWantDelegation => 56i32,
            NfsOpnum4::OpDestroyClientid => 57i32,
            NfsOpnum4::OpReclaimComplete => 58i32,
            NfsOpnum4::OpAllocate => 59i32,
            NfsOpnum4::OpCopy => 60i32,
            NfsOpnum4::OpCopyNotify => 61i32,
            NfsOpnum4::OpDeallocate => 62i32,
            NfsOpnum4::OpIoAdvise => 63i32,
            NfsOpnum4::OpLayouterror => 64i32,
            NfsOpnum4::OpLayoutstats => 65i32,
            NfsOpnum4::OpOffloadCancel => 66i32,
            NfsOpnum4::OpOffloadStatus => 67i32,
            NfsOpnum4::OpReadPlus => 68i32,
            NfsOpnum4::OpSeek => 69i32,
            NfsOpnum4::OpWriteSame => 70i32,
            NfsOpnum4::OpClone => 71i32,
            NfsOpnum4::OpIllegal => 10044i32,
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
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum ACCESS4res {
    Nfs4Ok(ACCESS4resok),
    #[default]
    Default,
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
pub struct CLONE4args {
    pub cl_src_stateid: Stateid4,
    pub cl_dst_stateid: Stateid4,
    pub cl_src_offset: u64,
    pub cl_dst_offset: u64,
    pub cl_count: u64,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CLONE4res {
    pub cl_status: Nfsstat4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CLOSE4args {
    pub seqid: u32,
    pub open_stateid: Stateid4,
}
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum CLOSE4res {
    Nfs4Ok(Stateid4),
    #[default]
    Default,
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
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum COMMIT4res {
    Nfs4Ok(COMMIT4resok),
    #[default]
    Default,
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
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum Createtype4 {
    NF4DIR,
    NF4BLK(Specdata4),
    NF4CHR(Specdata4),
    NF4LNK(serde_xdr::opaque::VariableArray),
    NF4SOCK,
    NF4FIFO,
    #[default]
    Default,
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
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum CREATE4res {
    Nfs4Ok(CREATE4resok),
    #[default]
    Default,
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
    pub status: Nfsstat4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DELEGRETURN4args {
    pub deleg_stateid: Stateid4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DELEGRETURN4res {
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
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum GETATTR4res {
    Nfs4Ok(GETATTR4resok),
    #[default]
    Default,
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
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum GETFH4res {
    Nfs4Ok(GETFH4resok),
    #[default]
    Default,
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
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum LINK4res {
    Nfs4Ok(LINK4resok),
    #[default]
    Default,
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
    pub lock_owner: StateOwner4,
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
    pub locktype: NfsLockType4,
    pub owner: StateOwner4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LOCK4resok {
    pub lock_stateid: Stateid4,
}
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum LOCK4res {
    Nfs4Ok(LOCK4resok),
    Nfs4errDenied(LOCK4denied),
    #[default]
    Default,
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
    pub locktype: NfsLockType4,
    pub offset: u64,
    pub length: u64,
    pub owner: StateOwner4,
}
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum LOCKT4res {
    Nfs4Ok,
    Nfs4errDenied(LOCK4denied),
    #[default]
    Default,
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
    pub locktype: NfsLockType4,
    pub seqid: u32,
    pub lock_stateid: Stateid4,
    pub offset: u64,
    pub length: u64,
}
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum LOCKU4res {
    Nfs4Ok(Stateid4),
    #[default]
    Default,
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
    pub status: Nfsstat4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LOOKUPP4res {
    pub status: Nfsstat4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct NVERIFY4args {
    pub obj_attributes: Fattr4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct NVERIFY4res {
    pub status: Nfsstat4,
}
#[derive(Clone, Debug, PartialEq, XdrIndexer)]
#[repr(i32)]
#[derive(Default)]
pub enum Createmode4 {
    #[default]
    UNCHECKED4 = 0i32,
    GUARDED4 = 1i32,
    EXCLUSIVE4 = 2i32,
    Exclusive41 = 3i32,
}
impl XdrIndexer for Createmode4 {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(UNCHECKED4)),
            1i32 => Ok(stringify!(GUARDED4)),
            2i32 => Ok(stringify!(EXCLUSIVE4)),
            3i32 => Ok(stringify!(Exclusive41)),
            _ => Ok(stringify!(UNCHECKED4)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            Createmode4::UNCHECKED4 => 0i32,
            Createmode4::GUARDED4 => 1i32,
            Createmode4::EXCLUSIVE4 => 2i32,
            Createmode4::Exclusive41 => 3i32,
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Creatverfattr {
    #[serde(with = "serde_xdr::opaque::fixed")]
    pub cva_verf: [u8; NFS4_VERIFIER_SIZE as usize],
    pub cva_attrs: Fattr4,
}
#[derive(Clone, Debug, XdrIndexer)]
pub enum Createhow4 {
    UNCHECKED4(Fattr4),
    GUARDED4(Fattr4),
    EXCLUSIVE4(serde_xdr::opaque::VariableArray),
    Exclusive41(Creatverfattr),
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
            3i32 => Ok(stringify!(Exclusive41)),
            _ => Err(::serde_xdr::error::Error::Convert),
        }
    }
    fn index(&self) -> i32 {
        match self {
            Createhow4::UNCHECKED4(_) => 0i32,
            Createhow4::GUARDED4(_) => 1i32,
            Createhow4::EXCLUSIVE4(_) => 2i32,
            Createhow4::Exclusive41(_) => 3i32,
        }
    }
}
#[derive(Clone, Debug, PartialEq, XdrIndexer)]
#[repr(i32)]
#[derive(Default)]
pub enum Opentype4 {
    #[default]
    Open4Nocreate = 0i32,
    Open4Create = 1i32,
}
impl XdrIndexer for Opentype4 {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Open4Nocreate)),
            1i32 => Ok(stringify!(Open4Create)),
            _ => Ok(stringify!(Open4Nocreate)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            Opentype4::Open4Nocreate => 0i32,
            Opentype4::Open4Create => 1i32,
        }
    }
}
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum Openflag4 {
    Open4Create(Createhow4),
    #[default]
    Default,
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
#[derive(Clone, Debug, PartialEq, XdrIndexer)]
#[repr(i32)]
#[derive(Default)]
pub enum LimitBy4 {
    #[default]
    NfsLimitSize = 1i32,
    NfsLimitBlocks = 2i32,
}
impl XdrIndexer for LimitBy4 {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            1i32 => Ok(stringify!(NfsLimitSize)),
            2i32 => Ok(stringify!(NfsLimitBlocks)),
            _ => Ok(stringify!(NfsLimitSize)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            LimitBy4::NfsLimitSize => 1i32,
            LimitBy4::NfsLimitBlocks => 2i32,
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
#[derive(Clone, Debug, PartialEq, XdrIndexer)]
#[repr(i32)]
#[derive(Default)]
pub enum OpenDelegationType4 {
    #[default]
    OpenDelegateNone = 0i32,
    OpenDelegateRead = 1i32,
    OpenDelegateWrite = 2i32,
    OpenDelegateNoneExt = 3i32,
}
impl XdrIndexer for OpenDelegationType4 {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(OpenDelegateNone)),
            1i32 => Ok(stringify!(OpenDelegateRead)),
            2i32 => Ok(stringify!(OpenDelegateWrite)),
            3i32 => Ok(stringify!(OpenDelegateNoneExt)),
            _ => Ok(stringify!(OpenDelegateNone)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            OpenDelegationType4::OpenDelegateNone => 0i32,
            OpenDelegationType4::OpenDelegateRead => 1i32,
            OpenDelegationType4::OpenDelegateWrite => 2i32,
            OpenDelegationType4::OpenDelegateNoneExt => 3i32,
        }
    }
}
#[derive(Clone, Debug, PartialEq, XdrIndexer)]
#[repr(i32)]
#[derive(Default)]
pub enum OpenClaimType4 {
    #[default]
    ClaimNull = 0i32,
    ClaimPrevious = 1i32,
    ClaimDelegateCur = 2i32,
    ClaimDelegatePrev = 3i32,
    ClaimFh = 4i32,
    ClaimDelegCurFh = 5i32,
    ClaimDelegPrevFh = 6i32,
}
impl XdrIndexer for OpenClaimType4 {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(ClaimNull)),
            1i32 => Ok(stringify!(ClaimPrevious)),
            2i32 => Ok(stringify!(ClaimDelegateCur)),
            3i32 => Ok(stringify!(ClaimDelegatePrev)),
            4i32 => Ok(stringify!(ClaimFh)),
            5i32 => Ok(stringify!(ClaimDelegCurFh)),
            6i32 => Ok(stringify!(ClaimDelegPrevFh)),
            _ => Ok(stringify!(ClaimNull)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            OpenClaimType4::ClaimNull => 0i32,
            OpenClaimType4::ClaimPrevious => 1i32,
            OpenClaimType4::ClaimDelegateCur => 2i32,
            OpenClaimType4::ClaimDelegatePrev => 3i32,
            OpenClaimType4::ClaimFh => 4i32,
            OpenClaimType4::ClaimDelegCurFh => 5i32,
            OpenClaimType4::ClaimDelegPrevFh => 6i32,
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
    ClaimFh,
    ClaimDelegCurFh(Stateid4),
    ClaimDelegPrevFh,
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
            4i32 => Ok(stringify!(ClaimFh)),
            5i32 => Ok(stringify!(ClaimDelegCurFh)),
            6i32 => Ok(stringify!(ClaimDelegPrevFh)),
            _ => Err(::serde_xdr::error::Error::Convert),
        }
    }
    fn index(&self) -> i32 {
        match self {
            OpenClaim4::ClaimNull(_) => 0i32,
            OpenClaim4::ClaimPrevious(_) => 1i32,
            OpenClaim4::ClaimDelegateCur(_) => 2i32,
            OpenClaim4::ClaimDelegatePrev(_) => 3i32,
            OpenClaim4::ClaimFh => 4i32,
            OpenClaim4::ClaimDelegCurFh(_) => 5i32,
            OpenClaim4::ClaimDelegPrevFh => 6i32,
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OPEN4args {
    pub seqid: u32,
    pub share_access: u32,
    pub share_deny: u32,
    pub owner: StateOwner4,
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
#[derive(Clone, Debug, PartialEq, XdrIndexer)]
#[repr(i32)]
#[derive(Default)]
pub enum WhyNoDelegation4 {
    #[default]
    Wnd4NotWanted = 0i32,
    Wnd4Contention = 1i32,
    Wnd4Resource = 2i32,
    Wnd4NotSuppFtype = 3i32,
    Wnd4WriteDelegNotSuppFtype = 4i32,
    Wnd4NotSuppUpgrade = 5i32,
    Wnd4NotSuppDowngrade = 6i32,
    Wnd4Cancelled = 7i32,
    Wnd4IsDir = 8i32,
}
impl XdrIndexer for WhyNoDelegation4 {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Wnd4NotWanted)),
            1i32 => Ok(stringify!(Wnd4Contention)),
            2i32 => Ok(stringify!(Wnd4Resource)),
            3i32 => Ok(stringify!(Wnd4NotSuppFtype)),
            4i32 => Ok(stringify!(Wnd4WriteDelegNotSuppFtype)),
            5i32 => Ok(stringify!(Wnd4NotSuppUpgrade)),
            6i32 => Ok(stringify!(Wnd4NotSuppDowngrade)),
            7i32 => Ok(stringify!(Wnd4Cancelled)),
            8i32 => Ok(stringify!(Wnd4IsDir)),
            _ => Ok(stringify!(Wnd4NotWanted)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            WhyNoDelegation4::Wnd4NotWanted => 0i32,
            WhyNoDelegation4::Wnd4Contention => 1i32,
            WhyNoDelegation4::Wnd4Resource => 2i32,
            WhyNoDelegation4::Wnd4NotSuppFtype => 3i32,
            WhyNoDelegation4::Wnd4WriteDelegNotSuppFtype => 4i32,
            WhyNoDelegation4::Wnd4NotSuppUpgrade => 5i32,
            WhyNoDelegation4::Wnd4NotSuppDowngrade => 6i32,
            WhyNoDelegation4::Wnd4Cancelled => 7i32,
            WhyNoDelegation4::Wnd4IsDir => 8i32,
        }
    }
}
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum OpenNoneDelegation4 {
    Wnd4Contention(bool),
    Wnd4Resource(bool),
    #[default]
    Default,
}
impl XdrIndexer for OpenNoneDelegation4 {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            1i32 => Ok(stringify!(Wnd4Contention)),
            2i32 => Ok(stringify!(Wnd4Resource)),
            _ => Ok(stringify!(Default)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            OpenNoneDelegation4::Wnd4Contention(_) => 1i32,
            OpenNoneDelegation4::Wnd4Resource(_) => 2i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum OpenDelegation4 {
    #[default]
    OpenDelegateNone,
    OpenDelegateRead(OpenReadDelegation4),
    OpenDelegateWrite(OpenWriteDelegation4),
    OpenDelegateNoneExt(OpenNoneDelegation4),
}
impl XdrIndexer for OpenDelegation4 {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(OpenDelegateNone)),
            1i32 => Ok(stringify!(OpenDelegateRead)),
            2i32 => Ok(stringify!(OpenDelegateWrite)),
            3i32 => Ok(stringify!(OpenDelegateNoneExt)),
            _ => Err(::serde_xdr::error::Error::Convert),
        }
    }
    fn index(&self) -> i32 {
        match self {
            OpenDelegation4::OpenDelegateNone => 0i32,
            OpenDelegation4::OpenDelegateRead(_) => 1i32,
            OpenDelegation4::OpenDelegateWrite(_) => 2i32,
            OpenDelegation4::OpenDelegateNoneExt(_) => 3i32,
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
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum OPEN4res {
    Nfs4Ok(OPEN4resok),
    #[default]
    Default,
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
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum OpenConfirm4res {
    Nfs4Ok(OpenConfirm4resok),
    #[default]
    Default,
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
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum OpenDowngrade4res {
    Nfs4Ok(OpenDowngrade4resok),
    #[default]
    Default,
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
    pub status: Nfsstat4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PUTPUBFH4res {
    pub status: Nfsstat4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PUTROOTFH4res {
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
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum READ4res {
    Nfs4Ok(READ4resok),
    #[default]
    Default,
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
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum READDIR4res {
    Nfs4Ok(READDIR4resok),
    #[default]
    Default,
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
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum READLINK4res {
    Nfs4Ok(READLINK4resok),
    #[default]
    Default,
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
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum REMOVE4res {
    Nfs4Ok(REMOVE4resok),
    #[default]
    Default,
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
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum RENAME4res {
    Nfs4Ok(RENAME4resok),
    #[default]
    Default,
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
    pub status: Nfsstat4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RESTOREFH4res {
    pub status: Nfsstat4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SAVEFH4res {
    pub status: Nfsstat4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SECINFO4args {
    pub name: String,
}
#[derive(Clone, Debug, PartialEq, XdrIndexer)]
#[repr(i32)]
#[derive(Default)]
pub enum RpcGssSvcT {
    #[default]
    RpcGssSvcNone = 1i32,
    RpcGssSvcIntegrity = 2i32,
    RpcGssSvcPrivacy = 3i32,
}
impl XdrIndexer for RpcGssSvcT {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            1i32 => Ok(stringify!(RpcGssSvcNone)),
            2i32 => Ok(stringify!(RpcGssSvcIntegrity)),
            3i32 => Ok(stringify!(RpcGssSvcPrivacy)),
            _ => Ok(stringify!(RpcGssSvcNone)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            RpcGssSvcT::RpcGssSvcNone => 1i32,
            RpcGssSvcT::RpcGssSvcIntegrity => 2i32,
            RpcGssSvcT::RpcGssSvcPrivacy => 3i32,
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RpcsecGssInfo {
    #[serde(with = "serde_xdr::opaque::variable")]
    pub oid: Vec<u8>,
    pub qop: u32,
    pub service: RpcGssSvcT,
}
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum Secinfo4 {
    RpcsecGss(RpcsecGssInfo),
    #[default]
    Default,
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
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum SECINFO4res {
    Nfs4Ok(Vec<Secinfo4>),
    #[default]
    Default,
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
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum SETCLIENTID4res {
    Nfs4Ok(SETCLIENTID4resok),
    Nfs4errClidInuse(Netaddr4),
    #[default]
    Default,
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
    pub status: Nfsstat4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct VERIFY4args {
    pub obj_attributes: Fattr4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct VERIFY4res {
    pub status: Nfsstat4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct WRITE4args {
    pub stateid: Stateid4,
    pub offset: u64,
    pub stable: StableHow4,
    #[serde(with = "serde_xdr::opaque::variable")]
    pub data: Vec<u8>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct WRITE4resok {
    pub count: u32,
    pub committed: StableHow4,
    #[serde(with = "serde_xdr::opaque::fixed")]
    pub writeverf: [u8; NFS4_VERIFIER_SIZE as usize],
}
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum WRITE4res {
    Nfs4Ok(WRITE4resok),
    #[default]
    Default,
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
    pub lock_owner: StateOwner4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ReleaseLockowner4res {
    pub status: Nfsstat4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ILLEGAL4res {
    pub status: Nfsstat4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct GssCbHandles4 {
    pub gcbp_service: RpcGssSvcT,
    #[serde(with = "serde_xdr::opaque::variable")]
    pub gcbp_handle_from_server: Vec<u8>,
    #[serde(with = "serde_xdr::opaque::variable")]
    pub gcbp_handle_from_client: Vec<u8>,
}
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum CallbackSecParms4 {
    #[default]
    AuthNone,
    AuthSys(AuthsysParms),
    RpcsecGss(GssCbHandles4),
}
impl XdrIndexer for CallbackSecParms4 {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(AuthNone)),
            1i32 => Ok(stringify!(AuthSys)),
            6i32 => Ok(stringify!(RpcsecGss)),
            _ => Err(::serde_xdr::error::Error::Convert),
        }
    }
    fn index(&self) -> i32 {
        match self {
            CallbackSecParms4::AuthNone => 0i32,
            CallbackSecParms4::AuthSys(_) => 1i32,
            CallbackSecParms4::RpcsecGss(_) => 6i32,
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BackchannelCtl4args {
    pub bca_cb_program: u32,
    pub bca_sec_parms: Vec<CallbackSecParms4>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BackchannelCtl4res {
    pub bcr_status: Nfsstat4,
}
#[derive(Clone, Debug, PartialEq, XdrIndexer)]
#[repr(i32)]
#[derive(Default)]
pub enum ChannelDirFromClient4 {
    #[default]
    Cdfc4Fore = 1i32,
    Cdfc4Back = 2i32,
    Cdfc4ForeOrBoth = 3i32,
    Cdfc4BackOrBoth = 7i32,
}
impl XdrIndexer for ChannelDirFromClient4 {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            1i32 => Ok(stringify!(Cdfc4Fore)),
            2i32 => Ok(stringify!(Cdfc4Back)),
            3i32 => Ok(stringify!(Cdfc4ForeOrBoth)),
            7i32 => Ok(stringify!(Cdfc4BackOrBoth)),
            _ => Ok(stringify!(Cdfc4Fore)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            ChannelDirFromClient4::Cdfc4Fore => 1i32,
            ChannelDirFromClient4::Cdfc4Back => 2i32,
            ChannelDirFromClient4::Cdfc4ForeOrBoth => 3i32,
            ChannelDirFromClient4::Cdfc4BackOrBoth => 7i32,
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BindConnToSession4args {
    #[serde(with = "serde_xdr::opaque::fixed")]
    pub bctsa_sessid: [u8; NFS4_SESSIONID_SIZE as usize],
    pub bctsa_dir: ChannelDirFromClient4,
    pub bctsa_use_conn_in_rdma_mode: bool,
}
#[derive(Clone, Debug, PartialEq, XdrIndexer)]
#[repr(i32)]
#[derive(Default)]
pub enum ChannelDirFromServer4 {
    #[default]
    Cdfs4Fore = 1i32,
    Cdfs4Back = 2i32,
    Cdfs4Both = 3i32,
}
impl XdrIndexer for ChannelDirFromServer4 {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            1i32 => Ok(stringify!(Cdfs4Fore)),
            2i32 => Ok(stringify!(Cdfs4Back)),
            3i32 => Ok(stringify!(Cdfs4Both)),
            _ => Ok(stringify!(Cdfs4Fore)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            ChannelDirFromServer4::Cdfs4Fore => 1i32,
            ChannelDirFromServer4::Cdfs4Back => 2i32,
            ChannelDirFromServer4::Cdfs4Both => 3i32,
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BindConnToSession4resok {
    #[serde(with = "serde_xdr::opaque::fixed")]
    pub bctsr_sessid: [u8; NFS4_SESSIONID_SIZE as usize],
    pub bctsr_dir: ChannelDirFromServer4,
    pub bctsr_use_conn_in_rdma_mode: bool,
}
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum BindConnToSession4res {
    Nfs4Ok(BindConnToSession4resok),
    #[default]
    Default,
}
impl XdrIndexer for BindConnToSession4res {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Nfs4Ok)),
            _ => Ok(stringify!(Default)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            BindConnToSession4res::Nfs4Ok(_) => 0i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct StateProtectOps4 {
    pub spo_must_enforce: Vec<u32>,
    pub spo_must_allow: Vec<u32>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SsvSpParms4 {
    pub ssp_ops: StateProtectOps4,
    pub ssp_hash_algs: Vec<serde_xdr::opaque::VariableArray>,
    pub ssp_encr_algs: Vec<serde_xdr::opaque::VariableArray>,
    pub ssp_window: u32,
    pub ssp_num_gss_handles: u32,
}
#[derive(Clone, Debug, PartialEq, XdrIndexer)]
#[repr(i32)]
#[derive(Default)]
pub enum StateProtectHow4 {
    #[default]
    Sp4None = 0i32,
    Sp4MachCred = 1i32,
    Sp4Ssv = 2i32,
}
impl XdrIndexer for StateProtectHow4 {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Sp4None)),
            1i32 => Ok(stringify!(Sp4MachCred)),
            2i32 => Ok(stringify!(Sp4Ssv)),
            _ => Ok(stringify!(Sp4None)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            StateProtectHow4::Sp4None => 0i32,
            StateProtectHow4::Sp4MachCred => 1i32,
            StateProtectHow4::Sp4Ssv => 2i32,
        }
    }
}
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum StateProtect4A {
    #[default]
    Sp4None,
    Sp4MachCred(StateProtectOps4),
    Sp4Ssv(SsvSpParms4),
}
impl XdrIndexer for StateProtect4A {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Sp4None)),
            1i32 => Ok(stringify!(Sp4MachCred)),
            2i32 => Ok(stringify!(Sp4Ssv)),
            _ => Err(::serde_xdr::error::Error::Convert),
        }
    }
    fn index(&self) -> i32 {
        match self {
            StateProtect4A::Sp4None => 0i32,
            StateProtect4A::Sp4MachCred(_) => 1i32,
            StateProtect4A::Sp4Ssv(_) => 2i32,
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ExchangeId4args {
    pub eia_clientowner: ClientOwner4,
    pub eia_flags: u32,
    pub eia_state_protect: StateProtect4A,
    pub eia_client_impl_id: Vec<NfsImplId4>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SsvProtInfo4 {
    pub spi_ops: StateProtectOps4,
    pub spi_hash_alg: u32,
    pub spi_encr_alg: u32,
    pub spi_ssv_len: u32,
    pub spi_window: u32,
    pub spi_handles: Vec<serde_xdr::opaque::VariableArray>,
}
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum StateProtect4R {
    #[default]
    Sp4None,
    Sp4MachCred(StateProtectOps4),
    Sp4Ssv(SsvProtInfo4),
}
impl XdrIndexer for StateProtect4R {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Sp4None)),
            1i32 => Ok(stringify!(Sp4MachCred)),
            2i32 => Ok(stringify!(Sp4Ssv)),
            _ => Err(::serde_xdr::error::Error::Convert),
        }
    }
    fn index(&self) -> i32 {
        match self {
            StateProtect4R::Sp4None => 0i32,
            StateProtect4R::Sp4MachCred(_) => 1i32,
            StateProtect4R::Sp4Ssv(_) => 2i32,
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ExchangeId4resok {
    pub eir_clientid: u64,
    pub eir_sequenceid: u32,
    pub eir_flags: u32,
    pub eir_state_protect: StateProtect4R,
    pub eir_server_owner: ServerOwner4,
    #[serde(with = "serde_xdr::opaque::variable")]
    pub eir_server_scope: Vec<u8>,
    pub eir_server_impl_id: Vec<NfsImplId4>,
}
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum ExchangeId4res {
    Nfs4Ok(ExchangeId4resok),
    #[default]
    Default,
}
impl XdrIndexer for ExchangeId4res {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Nfs4Ok)),
            _ => Ok(stringify!(Default)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            ExchangeId4res::Nfs4Ok(_) => 0i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ChannelAttrs4 {
    pub ca_headerpadsize: u32,
    pub ca_maxrequestsize: u32,
    pub ca_maxresponsesize: u32,
    pub ca_maxresponsesize_cached: u32,
    pub ca_maxoperations: u32,
    pub ca_maxrequests: u32,
    pub ca_rdma_ird: Vec<u32>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSession4args {
    pub csa_clientid: u64,
    pub csa_sequence: u32,
    pub csa_flags: u32,
    pub csa_fore_chan_attrs: ChannelAttrs4,
    pub csa_back_chan_attrs: ChannelAttrs4,
    pub csa_cb_program: u32,
    pub csa_sec_parms: Vec<CallbackSecParms4>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSession4resok {
    #[serde(with = "serde_xdr::opaque::fixed")]
    pub csr_sessionid: [u8; NFS4_SESSIONID_SIZE as usize],
    pub csr_sequence: u32,
    pub csr_flags: u32,
    pub csr_fore_chan_attrs: ChannelAttrs4,
    pub csr_back_chan_attrs: ChannelAttrs4,
}
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum CreateSession4res {
    Nfs4Ok(CreateSession4resok),
    #[default]
    Default,
}
impl XdrIndexer for CreateSession4res {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Nfs4Ok)),
            _ => Ok(stringify!(Default)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            CreateSession4res::Nfs4Ok(_) => 0i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DestroySession4args {
    #[serde(with = "serde_xdr::opaque::fixed")]
    pub dsa_sessionid: [u8; NFS4_SESSIONID_SIZE as usize],
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DestroySession4res {
    pub dsr_status: Nfsstat4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct FreeStateid4args {
    pub fsa_stateid: Stateid4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct FreeStateid4res {
    pub fsr_status: Nfsstat4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct GetDirDelegation4args {
    pub gdda_signal_deleg_avail: bool,
    pub gdda_notification_types: Vec<u32>,
    pub gdda_child_attr_delay: Nfstime4,
    pub gdda_dir_attr_delay: Nfstime4,
    pub gdda_child_attributes: Vec<u32>,
    pub gdda_dir_attributes: Vec<u32>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct GetDirDelegation4resok {
    #[serde(with = "serde_xdr::opaque::fixed")]
    pub gddr_cookieverf: [u8; NFS4_VERIFIER_SIZE as usize],
    pub gddr_stateid: Stateid4,
    pub gddr_notification: Vec<u32>,
    pub gddr_child_attributes: Vec<u32>,
    pub gddr_dir_attributes: Vec<u32>,
}
#[derive(Clone, Debug, PartialEq, XdrIndexer)]
#[repr(i32)]
#[derive(Default)]
pub enum Gddrnf4Status {
    #[default]
    Gdd4Ok = 0i32,
    Gdd4Unavail = 1i32,
}
impl XdrIndexer for Gddrnf4Status {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Gdd4Ok)),
            1i32 => Ok(stringify!(Gdd4Unavail)),
            _ => Ok(stringify!(Gdd4Ok)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            Gddrnf4Status::Gdd4Ok => 0i32,
            Gddrnf4Status::Gdd4Unavail => 1i32,
        }
    }
}
#[derive(Clone, Debug, XdrIndexer)]
pub enum GetDirDelegation4resNonFatal {
    Gdd4Ok(GetDirDelegation4resok),
    Gdd4Unavail(bool),
}
impl Default for GetDirDelegation4resNonFatal {
    fn default() -> Self {
        GetDirDelegation4resNonFatal::Gdd4Ok(Default::default())
    }
}
impl XdrIndexer for GetDirDelegation4resNonFatal {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Gdd4Ok)),
            1i32 => Ok(stringify!(Gdd4Unavail)),
            _ => Err(::serde_xdr::error::Error::Convert),
        }
    }
    fn index(&self) -> i32 {
        match self {
            GetDirDelegation4resNonFatal::Gdd4Ok(_) => 0i32,
            GetDirDelegation4resNonFatal::Gdd4Unavail(_) => 1i32,
        }
    }
}
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum GetDirDelegation4res {
    Nfs4Ok(GetDirDelegation4resNonFatal),
    #[default]
    Default,
}
impl XdrIndexer for GetDirDelegation4res {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Nfs4Ok)),
            _ => Ok(stringify!(Default)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            GetDirDelegation4res::Nfs4Ok(_) => 0i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct GETDEVICEINFO4args {
    #[serde(with = "serde_xdr::opaque::fixed")]
    pub gdia_device_id: [u8; NFS4_DEVICEID4_SIZE as usize],
    pub gdia_layout_type: Layouttype4,
    pub gdia_maxcount: u32,
    pub gdia_notify_types: Vec<u32>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct GETDEVICEINFO4resok {
    pub gdir_device_addr: DeviceAddr4,
    pub gdir_notification: Vec<u32>,
}
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum GETDEVICEINFO4res {
    Nfs4Ok(GETDEVICEINFO4resok),
    Nfs4errToosmall(u32),
    #[default]
    Default,
}
impl XdrIndexer for GETDEVICEINFO4res {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Nfs4Ok)),
            10005i32 => Ok(stringify!(Nfs4errToosmall)),
            _ => Ok(stringify!(Default)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            GETDEVICEINFO4res::Nfs4Ok(_) => 0i32,
            GETDEVICEINFO4res::Nfs4errToosmall(_) => 10005i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct GETDEVICELIST4args {
    pub gdla_layout_type: Layouttype4,
    pub gdla_maxdevices: u32,
    pub gdla_cookie: u64,
    #[serde(with = "serde_xdr::opaque::fixed")]
    pub gdla_cookieverf: [u8; NFS4_VERIFIER_SIZE as usize],
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DEVICEIDWRAP {
    #[serde(with = "serde_xdr::opaque::fixed")]
    pub gdlr_deviceid: [u8; NFS4_DEVICEID4_SIZE as usize],
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct GETDEVICELIST4resok {
    pub gdlr_cookie: u64,
    #[serde(with = "serde_xdr::opaque::fixed")]
    pub gdlr_cookieverf: [u8; NFS4_VERIFIER_SIZE as usize],
    pub gdlr_deviceid_list: Vec<DEVICEIDWRAP>,
    pub gdlr_eof: bool,
}
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum GETDEVICELIST4res {
    Nfs4Ok(GETDEVICELIST4resok),
    #[default]
    Default,
}
impl XdrIndexer for GETDEVICELIST4res {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Nfs4Ok)),
            _ => Ok(stringify!(Default)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            GETDEVICELIST4res::Nfs4Ok(_) => 0i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum Newtime4 {
    #[default]
    FALSE,
    TRUE(Nfstime4),
}
impl XdrIndexer for Newtime4 {
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
            Newtime4::FALSE => 0i32,
            Newtime4::TRUE(_) => 1i32,
        }
    }
}
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum Newoffset4 {
    #[default]
    FALSE,
    TRUE(u64),
}
impl XdrIndexer for Newoffset4 {
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
            Newoffset4::FALSE => 0i32,
            Newoffset4::TRUE(_) => 1i32,
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LAYOUTCOMMIT4args {
    pub loca_offset: u64,
    pub loca_length: u64,
    pub loca_reclaim: bool,
    pub loca_stateid: Stateid4,
    pub loca_last_write_offset: Newoffset4,
    pub loca_time_modify: Newtime4,
    pub loca_layoutupdate: Layoutupdate4,
}
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum Newsize4 {
    #[default]
    FALSE,
    TRUE(u64),
}
impl XdrIndexer for Newsize4 {
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
            Newsize4::FALSE => 0i32,
            Newsize4::TRUE(_) => 1i32,
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LAYOUTCOMMIT4resok {
    pub locr_newsize: Newsize4,
}
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum LAYOUTCOMMIT4res {
    Nfs4Ok(LAYOUTCOMMIT4resok),
    #[default]
    Default,
}
impl XdrIndexer for LAYOUTCOMMIT4res {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Nfs4Ok)),
            _ => Ok(stringify!(Default)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            LAYOUTCOMMIT4res::Nfs4Ok(_) => 0i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LAYOUTGET4args {
    pub loga_signal_layout_avail: bool,
    pub loga_layout_type: Layouttype4,
    pub loga_iomode: Layoutiomode4,
    pub loga_offset: u64,
    pub loga_length: u64,
    pub loga_minlength: u64,
    pub loga_stateid: Stateid4,
    pub loga_maxcount: u32,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LAYOUTGET4resok {
    pub logr_return_on_close: bool,
    pub logr_stateid: Stateid4,
    pub logr_layout: Vec<Layout4>,
}
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum LAYOUTGET4res {
    Nfs4Ok(LAYOUTGET4resok),
    Nfs4errLayouttrylater(bool),
    #[default]
    Default,
}
impl XdrIndexer for LAYOUTGET4res {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Nfs4Ok)),
            10058i32 => Ok(stringify!(Nfs4errLayouttrylater)),
            _ => Ok(stringify!(Default)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            LAYOUTGET4res::Nfs4Ok(_) => 0i32,
            LAYOUTGET4res::Nfs4errLayouttrylater(_) => 10058i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LAYOUTRETURN4args {
    pub lora_reclaim: bool,
    pub lora_layout_type: Layouttype4,
    pub lora_iomode: Layoutiomode4,
    pub lora_layoutreturn: Layoutreturn4,
}
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum LayoutreturnStateid {
    #[default]
    FALSE,
    TRUE(Stateid4),
}
impl XdrIndexer for LayoutreturnStateid {
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
            LayoutreturnStateid::FALSE => 0i32,
            LayoutreturnStateid::TRUE(_) => 1i32,
        }
    }
}
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum LAYOUTRETURN4res {
    Nfs4Ok(LayoutreturnStateid),
    #[default]
    Default,
}
impl XdrIndexer for LAYOUTRETURN4res {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Nfs4Ok)),
            _ => Ok(stringify!(Default)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            LAYOUTRETURN4res::Nfs4Ok(_) => 0i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, XdrIndexer)]
#[repr(i32)]
#[derive(Default)]
pub enum SecinfoStyle4 {
    #[default]
    SecinfoStyle4CurrentFh = 0i32,
    SecinfoStyle4Parent = 1i32,
}
impl XdrIndexer for SecinfoStyle4 {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(SecinfoStyle4CurrentFh)),
            1i32 => Ok(stringify!(SecinfoStyle4Parent)),
            _ => Ok(stringify!(SecinfoStyle4CurrentFh)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            SecinfoStyle4::SecinfoStyle4CurrentFh => 0i32,
            SecinfoStyle4::SecinfoStyle4Parent => 1i32,
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SEQUENCE4args {
    #[serde(with = "serde_xdr::opaque::fixed")]
    pub sa_sessionid: [u8; NFS4_SESSIONID_SIZE as usize],
    pub sa_sequenceid: u32,
    pub sa_slotid: u32,
    pub sa_highest_slotid: u32,
    pub sa_cachethis: bool,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SEQUENCE4resok {
    #[serde(with = "serde_xdr::opaque::fixed")]
    pub sr_sessionid: [u8; NFS4_SESSIONID_SIZE as usize],
    pub sr_sequenceid: u32,
    pub sr_slotid: u32,
    pub sr_highest_slotid: u32,
    pub sr_target_highest_slotid: u32,
    pub sr_status_flags: u32,
}
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum SEQUENCE4res {
    Nfs4Ok(SEQUENCE4resok),
    #[default]
    Default,
}
impl XdrIndexer for SEQUENCE4res {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Nfs4Ok)),
            _ => Ok(stringify!(Default)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            SEQUENCE4res::Nfs4Ok(_) => 0i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SsaDigestInput4 {
    pub sdi_seqargs: SEQUENCE4args,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SetSsv4args {
    #[serde(with = "serde_xdr::opaque::variable")]
    pub ssa_ssv: Vec<u8>,
    #[serde(with = "serde_xdr::opaque::variable")]
    pub ssa_digest: Vec<u8>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SsrDigestInput4 {
    pub sdi_seqres: SEQUENCE4res,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SetSsv4resok {
    #[serde(with = "serde_xdr::opaque::variable")]
    pub ssr_digest: Vec<u8>,
}
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum SetSsv4res {
    Nfs4Ok(SetSsv4resok),
    #[default]
    Default,
}
impl XdrIndexer for SetSsv4res {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Nfs4Ok)),
            _ => Ok(stringify!(Default)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            SetSsv4res::Nfs4Ok(_) => 0i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TestStateid4args {
    pub ts_stateids: Vec<Stateid4>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TestStateid4resok {
    pub tsr_status_codes: Vec<Nfsstat4>,
}
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum TestStateid4res {
    Nfs4Ok(TestStateid4resok),
    #[default]
    Default,
}
impl XdrIndexer for TestStateid4res {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Nfs4Ok)),
            _ => Ok(stringify!(Default)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            TestStateid4res::Nfs4Ok(_) => 0i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, XdrIndexer)]
pub enum DelegClaim4 {
    ClaimPrevious(OpenDelegationType4),
    ClaimFh,
    ClaimDelegPrevFh,
}
impl Default for DelegClaim4 {
    fn default() -> Self {
        DelegClaim4::ClaimPrevious(Default::default())
    }
}
impl XdrIndexer for DelegClaim4 {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            1i32 => Ok(stringify!(ClaimPrevious)),
            4i32 => Ok(stringify!(ClaimFh)),
            6i32 => Ok(stringify!(ClaimDelegPrevFh)),
            _ => Err(::serde_xdr::error::Error::Convert),
        }
    }
    fn index(&self) -> i32 {
        match self {
            DelegClaim4::ClaimPrevious(_) => 1i32,
            DelegClaim4::ClaimFh => 4i32,
            DelegClaim4::ClaimDelegPrevFh => 6i32,
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct WantDelegation4args {
    pub wda_want: u32,
    pub wda_claim: DelegClaim4,
}
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum WantDelegation4res {
    Nfs4Ok(OpenDelegation4),
    #[default]
    Default,
}
impl XdrIndexer for WantDelegation4res {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Nfs4Ok)),
            _ => Ok(stringify!(Default)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            WantDelegation4res::Nfs4Ok(_) => 0i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DestroyClientid4args {
    pub dca_clientid: u64,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DestroyClientid4res {
    pub dcr_status: Nfsstat4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ReclaimComplete4args {
    pub rca_one_fs: bool,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ReclaimComplete4res {
    pub rcr_status: Nfsstat4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct COPY4args {
    pub ca_src_stateid: Stateid4,
    pub ca_dst_stateid: Stateid4,
    pub ca_src_offset: u64,
    pub ca_dst_offset: u64,
    pub ca_count: u64,
    pub ca_consecutive: bool,
    pub ca_synchronous: bool,
    pub ca_source_server: Vec<Netloc4>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CopyRequirements4 {
    pub cr_consecutive: bool,
    pub cr_synchronous: bool,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct COPY4resok {
    pub cr_response: WriteResponse4,
    pub cr_requirements: CopyRequirements4,
}
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum COPY4res {
    Nfs4Ok(COPY4resok),
    Nfs4errOffloadNoReqs(CopyRequirements4),
    #[default]
    Default,
}
impl XdrIndexer for COPY4res {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Nfs4Ok)),
            10094i32 => Ok(stringify!(Nfs4errOffloadNoReqs)),
            _ => Ok(stringify!(Default)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            COPY4res::Nfs4Ok(_) => 0i32,
            COPY4res::Nfs4errOffloadNoReqs(_) => 10094i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CopyNotify4args {
    pub cna_src_stateid: Stateid4,
    pub cna_destination_server: Netloc4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CopyNotify4resok {
    pub cnr_lease_time: Nfstime4,
    pub cnr_stateid: Stateid4,
    pub cnr_source_server: Vec<Netloc4>,
}
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum CopyNotify4res {
    Nfs4Ok(CopyNotify4resok),
    #[default]
    Default,
}
impl XdrIndexer for CopyNotify4res {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Nfs4Ok)),
            _ => Ok(stringify!(Default)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            CopyNotify4res::Nfs4Ok(_) => 0i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OffloadCancel4args {
    pub oca_stateid: Stateid4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OffloadCancel4res {
    pub ocr_status: Nfsstat4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OffloadStatus4args {
    pub osa_stateid: Stateid4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OffloadStatus4resok {
    pub osr_count: u64,
    pub osr_complete: Vec<Nfsstat4>,
}
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum OffloadStatus4res {
    Nfs4Ok(OffloadStatus4resok),
    #[default]
    Default,
}
impl XdrIndexer for OffloadStatus4res {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Nfs4Ok)),
            _ => Ok(stringify!(Default)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            OffloadStatus4res::Nfs4Ok(_) => 0i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ALLOCATE4args {
    pub aa_stateid: Stateid4,
    pub aa_offset: u64,
    pub aa_length: u64,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ALLOCATE4res {
    pub ar_status: Nfsstat4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DEALLOCATE4args {
    pub da_stateid: Stateid4,
    pub da_offset: u64,
    pub da_length: u64,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DEALLOCATE4res {
    pub dr_status: Nfsstat4,
}
#[derive(Clone, Debug, PartialEq, XdrIndexer)]
#[repr(i32)]
#[derive(Default)]
pub enum IoAdviseType4 {
    #[default]
    IoAdvise4Normal = 0i32,
    IoAdvise4Sequential = 1i32,
    IoAdvise4SequentialBackwards = 2i32,
    IoAdvise4Random = 3i32,
    IoAdvise4Willneed = 4i32,
    IoAdvise4WillneedOpportunistic = 5i32,
    IoAdvise4Dontneed = 6i32,
    IoAdvise4Noreuse = 7i32,
    IoAdvise4Read = 8i32,
    IoAdvise4Write = 9i32,
    IoAdvise4InitProximity = 10i32,
}
impl XdrIndexer for IoAdviseType4 {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(IoAdvise4Normal)),
            1i32 => Ok(stringify!(IoAdvise4Sequential)),
            2i32 => Ok(stringify!(IoAdvise4SequentialBackwards)),
            3i32 => Ok(stringify!(IoAdvise4Random)),
            4i32 => Ok(stringify!(IoAdvise4Willneed)),
            5i32 => Ok(stringify!(IoAdvise4WillneedOpportunistic)),
            6i32 => Ok(stringify!(IoAdvise4Dontneed)),
            7i32 => Ok(stringify!(IoAdvise4Noreuse)),
            8i32 => Ok(stringify!(IoAdvise4Read)),
            9i32 => Ok(stringify!(IoAdvise4Write)),
            10i32 => Ok(stringify!(IoAdvise4InitProximity)),
            _ => Ok(stringify!(IoAdvise4Normal)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            IoAdviseType4::IoAdvise4Normal => 0i32,
            IoAdviseType4::IoAdvise4Sequential => 1i32,
            IoAdviseType4::IoAdvise4SequentialBackwards => 2i32,
            IoAdviseType4::IoAdvise4Random => 3i32,
            IoAdviseType4::IoAdvise4Willneed => 4i32,
            IoAdviseType4::IoAdvise4WillneedOpportunistic => 5i32,
            IoAdviseType4::IoAdvise4Dontneed => 6i32,
            IoAdviseType4::IoAdvise4Noreuse => 7i32,
            IoAdviseType4::IoAdvise4Read => 8i32,
            IoAdviseType4::IoAdvise4Write => 9i32,
            IoAdviseType4::IoAdvise4InitProximity => 10i32,
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IoAdvise4args {
    pub iaa_stateid: Stateid4,
    pub iaa_offset: u64,
    pub iaa_count: u64,
    pub iaa_hints: Vec<u32>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IoAdvise4resok {
    pub ior_hints: Vec<u32>,
}
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum IoAdvise4res {
    Nfs4Ok(IoAdvise4resok),
    #[default]
    Default,
}
impl XdrIndexer for IoAdvise4res {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Nfs4Ok)),
            _ => Ok(stringify!(Default)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            IoAdvise4res::Nfs4Ok(_) => 0i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DeviceError4 {
    #[serde(with = "serde_xdr::opaque::fixed")]
    pub de_deviceid: [u8; NFS4_DEVICEID4_SIZE as usize],
    pub de_status: Nfsstat4,
    pub de_opnum: NfsOpnum4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LAYOUTERROR4args {
    pub lea_offset: u64,
    pub lea_length: u64,
    pub lea_stateid: Stateid4,
    pub lea_errors: Vec<DeviceError4>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LAYOUTERROR4res {
    pub ler_status: Nfsstat4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IoInfo4 {
    pub ii_count: u64,
    pub ii_bytes: u64,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LAYOUTSTATS4args {
    pub lsa_offset: u64,
    pub lsa_length: u64,
    pub lsa_stateid: Stateid4,
    pub lsa_read: IoInfo4,
    pub lsa_write: IoInfo4,
    #[serde(with = "serde_xdr::opaque::fixed")]
    pub lsa_deviceid: [u8; NFS4_DEVICEID4_SIZE as usize],
    pub lsa_layoutupdate: Layoutupdate4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LAYOUTSTATS4res {
    pub lsr_status: Nfsstat4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ReadPlus4args {
    pub rpa_stateid: Stateid4,
    pub rpa_offset: u64,
    pub rpa_count: u32,
}
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum ReadPlusContent {
    Nfs4ContentData(Data4),
    Nfs4ContentHole(DataInfo4),
    #[default]
    Default,
}
impl XdrIndexer for ReadPlusContent {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Nfs4ContentData)),
            1i32 => Ok(stringify!(Nfs4ContentHole)),
            _ => Ok(stringify!(Default)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            ReadPlusContent::Nfs4ContentData(_) => 0i32,
            ReadPlusContent::Nfs4ContentHole(_) => 1i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ReadPlusRes4 {
    pub rpr_eof: bool,
    pub rpr_contents: Vec<ReadPlusContent>,
}
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum ReadPlus4res {
    Nfs4Ok(ReadPlusRes4),
    #[default]
    Default,
}
impl XdrIndexer for ReadPlus4res {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Nfs4Ok)),
            _ => Ok(stringify!(Default)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            ReadPlus4res::Nfs4Ok(_) => 0i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SEEK4args {
    pub sa_stateid: Stateid4,
    pub sa_offset: u64,
    pub sa_what: DataContent4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SeekRes4 {
    pub sr_eof: bool,
    pub sr_offset: u64,
}
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum SEEK4res {
    Nfs4Ok(SeekRes4),
    #[default]
    Default,
}
impl XdrIndexer for SEEK4res {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Nfs4Ok)),
            _ => Ok(stringify!(Default)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            SEEK4res::Nfs4Ok(_) => 0i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct WriteSame4args {
    pub wsa_stateid: Stateid4,
    pub wsa_stable: StableHow4,
    pub wsa_adb: AppDataBlock4,
}
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum WriteSame4res {
    Nfs4Ok(WriteResponse4),
    #[default]
    Default,
}
impl XdrIndexer for WriteSame4res {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Nfs4Ok)),
            _ => Ok(stringify!(Default)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            WriteSame4res::Nfs4Ok(_) => 0i32,
            _ => unimplemented!(),
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
    OpBackchannelCtl(BackchannelCtl4args),
    OpBindConnToSession(BindConnToSession4args),
    OpExchangeId(ExchangeId4args),
    OpCreateSession(CreateSession4args),
    OpDestroySession(DestroySession4args),
    OpFreeStateid(FreeStateid4args),
    OpGetDirDelegation(GetDirDelegation4args),
    OpGetdeviceinfo(GETDEVICEINFO4args),
    OpGetdevicelist(GETDEVICELIST4args),
    OpLayoutcommit(LAYOUTCOMMIT4args),
    OpLayoutget(LAYOUTGET4args),
    OpLayoutreturn(LAYOUTRETURN4args),
    OpSecinfoNoName(SecinfoStyle4),
    OpSequence(SEQUENCE4args),
    OpSetSsv(SetSsv4args),
    OpTestStateid(TestStateid4args),
    OpWantDelegation(WantDelegation4args),
    OpDestroyClientid(DestroyClientid4args),
    OpReclaimComplete(ReclaimComplete4args),
    OpAllocate(ALLOCATE4args),
    OpCopy(COPY4args),
    OpCopyNotify(CopyNotify4args),
    OpDeallocate(DEALLOCATE4args),
    OpIoAdvise(IoAdvise4args),
    OpLayouterror(LAYOUTERROR4args),
    OpLayoutstats(LAYOUTSTATS4args),
    OpOffloadCancel(OffloadCancel4args),
    OpOffloadStatus(OffloadStatus4args),
    OpReadPlus(ReadPlus4args),
    OpSeek(SEEK4args),
    OpWriteSame(WriteSame4args),
    OpClone(CLONE4args),
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
            40i32 => Ok(stringify!(OpBackchannelCtl)),
            41i32 => Ok(stringify!(OpBindConnToSession)),
            42i32 => Ok(stringify!(OpExchangeId)),
            43i32 => Ok(stringify!(OpCreateSession)),
            44i32 => Ok(stringify!(OpDestroySession)),
            45i32 => Ok(stringify!(OpFreeStateid)),
            46i32 => Ok(stringify!(OpGetDirDelegation)),
            47i32 => Ok(stringify!(OpGetdeviceinfo)),
            48i32 => Ok(stringify!(OpGetdevicelist)),
            49i32 => Ok(stringify!(OpLayoutcommit)),
            50i32 => Ok(stringify!(OpLayoutget)),
            51i32 => Ok(stringify!(OpLayoutreturn)),
            52i32 => Ok(stringify!(OpSecinfoNoName)),
            53i32 => Ok(stringify!(OpSequence)),
            54i32 => Ok(stringify!(OpSetSsv)),
            55i32 => Ok(stringify!(OpTestStateid)),
            56i32 => Ok(stringify!(OpWantDelegation)),
            57i32 => Ok(stringify!(OpDestroyClientid)),
            58i32 => Ok(stringify!(OpReclaimComplete)),
            59i32 => Ok(stringify!(OpAllocate)),
            60i32 => Ok(stringify!(OpCopy)),
            61i32 => Ok(stringify!(OpCopyNotify)),
            62i32 => Ok(stringify!(OpDeallocate)),
            63i32 => Ok(stringify!(OpIoAdvise)),
            64i32 => Ok(stringify!(OpLayouterror)),
            65i32 => Ok(stringify!(OpLayoutstats)),
            66i32 => Ok(stringify!(OpOffloadCancel)),
            67i32 => Ok(stringify!(OpOffloadStatus)),
            68i32 => Ok(stringify!(OpReadPlus)),
            69i32 => Ok(stringify!(OpSeek)),
            70i32 => Ok(stringify!(OpWriteSame)),
            71i32 => Ok(stringify!(OpClone)),
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
            NfsArgop4::OpBackchannelCtl(_) => 40i32,
            NfsArgop4::OpBindConnToSession(_) => 41i32,
            NfsArgop4::OpExchangeId(_) => 42i32,
            NfsArgop4::OpCreateSession(_) => 43i32,
            NfsArgop4::OpDestroySession(_) => 44i32,
            NfsArgop4::OpFreeStateid(_) => 45i32,
            NfsArgop4::OpGetDirDelegation(_) => 46i32,
            NfsArgop4::OpGetdeviceinfo(_) => 47i32,
            NfsArgop4::OpGetdevicelist(_) => 48i32,
            NfsArgop4::OpLayoutcommit(_) => 49i32,
            NfsArgop4::OpLayoutget(_) => 50i32,
            NfsArgop4::OpLayoutreturn(_) => 51i32,
            NfsArgop4::OpSecinfoNoName(_) => 52i32,
            NfsArgop4::OpSequence(_) => 53i32,
            NfsArgop4::OpSetSsv(_) => 54i32,
            NfsArgop4::OpTestStateid(_) => 55i32,
            NfsArgop4::OpWantDelegation(_) => 56i32,
            NfsArgop4::OpDestroyClientid(_) => 57i32,
            NfsArgop4::OpReclaimComplete(_) => 58i32,
            NfsArgop4::OpAllocate(_) => 59i32,
            NfsArgop4::OpCopy(_) => 60i32,
            NfsArgop4::OpCopyNotify(_) => 61i32,
            NfsArgop4::OpDeallocate(_) => 62i32,
            NfsArgop4::OpIoAdvise(_) => 63i32,
            NfsArgop4::OpLayouterror(_) => 64i32,
            NfsArgop4::OpLayoutstats(_) => 65i32,
            NfsArgop4::OpOffloadCancel(_) => 66i32,
            NfsArgop4::OpOffloadStatus(_) => 67i32,
            NfsArgop4::OpReadPlus(_) => 68i32,
            NfsArgop4::OpSeek(_) => 69i32,
            NfsArgop4::OpWriteSame(_) => 70i32,
            NfsArgop4::OpClone(_) => 71i32,
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
    OpBackchannelCtl(BackchannelCtl4res),
    OpBindConnToSession(BindConnToSession4res),
    OpExchangeId(ExchangeId4res),
    OpCreateSession(CreateSession4res),
    OpDestroySession(DestroySession4res),
    OpFreeStateid(FreeStateid4res),
    OpGetDirDelegation(GetDirDelegation4res),
    OpGetdeviceinfo(GETDEVICEINFO4res),
    OpGetdevicelist(GETDEVICELIST4res),
    OpLayoutcommit(LAYOUTCOMMIT4res),
    OpLayoutget(LAYOUTGET4res),
    OpLayoutreturn(LAYOUTRETURN4res),
    OpSecinfoNoName(SECINFO4res),
    OpSequence(SEQUENCE4res),
    OpSetSsv(SetSsv4res),
    OpTestStateid(TestStateid4res),
    OpWantDelegation(WantDelegation4res),
    OpDestroyClientid(DestroyClientid4res),
    OpReclaimComplete(ReclaimComplete4res),
    OpAllocate(ALLOCATE4res),
    OpCopy(COPY4res),
    OpCopyNotify(CopyNotify4res),
    OpDeallocate(DEALLOCATE4res),
    OpIoAdvise(IoAdvise4res),
    OpLayouterror(LAYOUTERROR4res),
    OpLayoutstats(LAYOUTSTATS4res),
    OpOffloadCancel(OffloadCancel4res),
    OpOffloadStatus(OffloadStatus4res),
    OpReadPlus(ReadPlus4res),
    OpSeek(SEEK4res),
    OpWriteSame(WriteSame4res),
    OpClone(CLONE4res),
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
            40i32 => Ok(stringify!(OpBackchannelCtl)),
            41i32 => Ok(stringify!(OpBindConnToSession)),
            42i32 => Ok(stringify!(OpExchangeId)),
            43i32 => Ok(stringify!(OpCreateSession)),
            44i32 => Ok(stringify!(OpDestroySession)),
            45i32 => Ok(stringify!(OpFreeStateid)),
            46i32 => Ok(stringify!(OpGetDirDelegation)),
            47i32 => Ok(stringify!(OpGetdeviceinfo)),
            48i32 => Ok(stringify!(OpGetdevicelist)),
            49i32 => Ok(stringify!(OpLayoutcommit)),
            50i32 => Ok(stringify!(OpLayoutget)),
            51i32 => Ok(stringify!(OpLayoutreturn)),
            52i32 => Ok(stringify!(OpSecinfoNoName)),
            53i32 => Ok(stringify!(OpSequence)),
            54i32 => Ok(stringify!(OpSetSsv)),
            55i32 => Ok(stringify!(OpTestStateid)),
            56i32 => Ok(stringify!(OpWantDelegation)),
            57i32 => Ok(stringify!(OpDestroyClientid)),
            58i32 => Ok(stringify!(OpReclaimComplete)),
            59i32 => Ok(stringify!(OpAllocate)),
            60i32 => Ok(stringify!(OpCopy)),
            61i32 => Ok(stringify!(OpCopyNotify)),
            62i32 => Ok(stringify!(OpDeallocate)),
            63i32 => Ok(stringify!(OpIoAdvise)),
            64i32 => Ok(stringify!(OpLayouterror)),
            65i32 => Ok(stringify!(OpLayoutstats)),
            66i32 => Ok(stringify!(OpOffloadCancel)),
            67i32 => Ok(stringify!(OpOffloadStatus)),
            68i32 => Ok(stringify!(OpReadPlus)),
            69i32 => Ok(stringify!(OpSeek)),
            70i32 => Ok(stringify!(OpWriteSame)),
            71i32 => Ok(stringify!(OpClone)),
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
            NfsResop4::OpBackchannelCtl(_) => 40i32,
            NfsResop4::OpBindConnToSession(_) => 41i32,
            NfsResop4::OpExchangeId(_) => 42i32,
            NfsResop4::OpCreateSession(_) => 43i32,
            NfsResop4::OpDestroySession(_) => 44i32,
            NfsResop4::OpFreeStateid(_) => 45i32,
            NfsResop4::OpGetDirDelegation(_) => 46i32,
            NfsResop4::OpGetdeviceinfo(_) => 47i32,
            NfsResop4::OpGetdevicelist(_) => 48i32,
            NfsResop4::OpLayoutcommit(_) => 49i32,
            NfsResop4::OpLayoutget(_) => 50i32,
            NfsResop4::OpLayoutreturn(_) => 51i32,
            NfsResop4::OpSecinfoNoName(_) => 52i32,
            NfsResop4::OpSequence(_) => 53i32,
            NfsResop4::OpSetSsv(_) => 54i32,
            NfsResop4::OpTestStateid(_) => 55i32,
            NfsResop4::OpWantDelegation(_) => 56i32,
            NfsResop4::OpDestroyClientid(_) => 57i32,
            NfsResop4::OpReclaimComplete(_) => 58i32,
            NfsResop4::OpAllocate(_) => 59i32,
            NfsResop4::OpCopy(_) => 60i32,
            NfsResop4::OpCopyNotify(_) => 61i32,
            NfsResop4::OpDeallocate(_) => 62i32,
            NfsResop4::OpIoAdvise(_) => 63i32,
            NfsResop4::OpLayouterror(_) => 64i32,
            NfsResop4::OpLayoutstats(_) => 65i32,
            NfsResop4::OpOffloadCancel(_) => 66i32,
            NfsResop4::OpOffloadStatus(_) => 67i32,
            NfsResop4::OpReadPlus(_) => 68i32,
            NfsResop4::OpSeek(_) => 69i32,
            NfsResop4::OpWriteSame(_) => 70i32,
            NfsResop4::OpClone(_) => 71i32,
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
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum CbGetattr4res {
    Nfs4Ok(CbGetattr4resok),
    #[default]
    Default,
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
    pub status: Nfsstat4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CbIllegal4res {
    pub status: Nfsstat4,
}
#[derive(Clone, Debug, PartialEq, XdrIndexer)]
#[repr(i32)]
#[derive(Default)]
pub enum LayoutrecallType4 {
    #[default]
    Layoutrecall4File = 1i32,
    Layoutrecall4Fsid = 2i32,
    Layoutrecall4All = 3i32,
}
impl XdrIndexer for LayoutrecallType4 {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            1i32 => Ok(stringify!(Layoutrecall4File)),
            2i32 => Ok(stringify!(Layoutrecall4Fsid)),
            3i32 => Ok(stringify!(Layoutrecall4All)),
            _ => Ok(stringify!(Layoutrecall4File)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            LayoutrecallType4::Layoutrecall4File => 1i32,
            LayoutrecallType4::Layoutrecall4Fsid => 2i32,
            LayoutrecallType4::Layoutrecall4All => 3i32,
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LayoutrecallFile4 {
    #[serde(with = "serde_xdr::opaque::variable")]
    pub lor_fh: Vec<u8>,
    pub lor_offset: u64,
    pub lor_length: u64,
    pub lor_stateid: Stateid4,
}
#[derive(Clone, Debug, XdrIndexer)]
pub enum Layoutrecall4 {
    Layoutrecall4File(LayoutrecallFile4),
    Layoutrecall4Fsid(Fsid4),
    Layoutrecall4All,
}
impl Default for Layoutrecall4 {
    fn default() -> Self {
        Layoutrecall4::Layoutrecall4File(Default::default())
    }
}
impl XdrIndexer for Layoutrecall4 {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            1i32 => Ok(stringify!(Layoutrecall4File)),
            2i32 => Ok(stringify!(Layoutrecall4Fsid)),
            3i32 => Ok(stringify!(Layoutrecall4All)),
            _ => Err(::serde_xdr::error::Error::Convert),
        }
    }
    fn index(&self) -> i32 {
        match self {
            Layoutrecall4::Layoutrecall4File(_) => 1i32,
            Layoutrecall4::Layoutrecall4Fsid(_) => 2i32,
            Layoutrecall4::Layoutrecall4All => 3i32,
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CbLayoutrecall4args {
    pub clora_type: Layouttype4,
    pub clora_iomode: Layoutiomode4,
    pub clora_changed: bool,
    pub clora_recall: Layoutrecall4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CbLayoutrecall4res {
    pub clorr_status: Nfsstat4,
}
#[derive(Clone, Debug, PartialEq, XdrIndexer)]
#[repr(i32)]
#[derive(Default)]
pub enum NotifyType4 {
    #[default]
    Notify4ChangeChildAttrs = 0i32,
    Notify4ChangeDirAttrs = 1i32,
    Notify4RemoveEntry = 2i32,
    Notify4AddEntry = 3i32,
    Notify4RenameEntry = 4i32,
    Notify4ChangeCookieVerifier = 5i32,
}
impl XdrIndexer for NotifyType4 {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Notify4ChangeChildAttrs)),
            1i32 => Ok(stringify!(Notify4ChangeDirAttrs)),
            2i32 => Ok(stringify!(Notify4RemoveEntry)),
            3i32 => Ok(stringify!(Notify4AddEntry)),
            4i32 => Ok(stringify!(Notify4RenameEntry)),
            5i32 => Ok(stringify!(Notify4ChangeCookieVerifier)),
            _ => Ok(stringify!(Notify4ChangeChildAttrs)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            NotifyType4::Notify4ChangeChildAttrs => 0i32,
            NotifyType4::Notify4ChangeDirAttrs => 1i32,
            NotifyType4::Notify4RemoveEntry => 2i32,
            NotifyType4::Notify4AddEntry => 3i32,
            NotifyType4::Notify4RenameEntry => 4i32,
            NotifyType4::Notify4ChangeCookieVerifier => 5i32,
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct NotifyEntry4 {
    pub ne_file: String,
    pub ne_attrs: Fattr4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PrevEntry4 {
    pub pe_prev_entry: NotifyEntry4,
    pub pe_prev_entry_cookie: u64,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct NotifyRemove4 {
    pub nrm_old_entry: NotifyEntry4,
    pub nrm_old_entry_cookie: u64,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct NotifyAdd4 {
    pub nad_old_entry: Vec<NotifyRemove4>,
    pub nad_new_entry: NotifyEntry4,
    pub nad_new_entry_cookie: Vec<u64>,
    pub nad_prev_entry: Vec<PrevEntry4>,
    pub nad_last_entry: bool,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct NotifyAttr4 {
    pub na_changed_entry: NotifyEntry4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct NotifyRename4 {
    pub nrn_old_entry: NotifyRemove4,
    pub nrn_new_entry: NotifyAdd4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct NotifyVerifier4 {
    #[serde(with = "serde_xdr::opaque::fixed")]
    pub nv_old_cookieverf: [u8; NFS4_VERIFIER_SIZE as usize],
    #[serde(with = "serde_xdr::opaque::fixed")]
    pub nv_new_cookieverf: [u8; NFS4_VERIFIER_SIZE as usize],
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Notify4 {
    pub notify_mask: Vec<u32>,
    #[serde(with = "serde_xdr::opaque::variable")]
    pub notify_vals: Vec<u8>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CbNotify4args {
    pub cna_stateid: Stateid4,
    #[serde(with = "serde_xdr::opaque::variable")]
    pub cna_fh: Vec<u8>,
    pub cna_changes: Vec<Notify4>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CbNotify4res {
    pub cnr_status: Nfsstat4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CbPushDeleg4args {
    #[serde(with = "serde_xdr::opaque::variable")]
    pub cpda_fh: Vec<u8>,
    pub cpda_delegation: OpenDelegation4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CbPushDeleg4res {
    pub cpdr_status: Nfsstat4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CbRecallAny4args {
    pub craa_objects_to_keep: u32,
    pub craa_type_mask: Vec<u32>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CbRecallAny4res {
    pub crar_status: Nfsstat4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CbRecallableObjAvail4res {
    pub croa_status: Nfsstat4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CbRecallSlot4args {
    pub rsa_target_highest_slotid: u32,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CbRecallSlot4res {
    pub rsr_status: Nfsstat4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ReferringCall4 {
    pub rc_sequenceid: u32,
    pub rc_slotid: u32,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ReferringCallList4 {
    #[serde(with = "serde_xdr::opaque::fixed")]
    pub rcl_sessionid: [u8; NFS4_SESSIONID_SIZE as usize],
    pub rcl_referring_calls: Vec<ReferringCall4>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CbSequence4args {
    #[serde(with = "serde_xdr::opaque::fixed")]
    pub csa_sessionid: [u8; NFS4_SESSIONID_SIZE as usize],
    pub csa_sequenceid: u32,
    pub csa_slotid: u32,
    pub csa_highest_slotid: u32,
    pub csa_cachethis: bool,
    pub csa_referring_call_lists: Vec<ReferringCallList4>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CbSequence4resok {
    #[serde(with = "serde_xdr::opaque::fixed")]
    pub csr_sessionid: [u8; NFS4_SESSIONID_SIZE as usize],
    pub csr_sequenceid: u32,
    pub csr_slotid: u32,
    pub csr_highest_slotid: u32,
    pub csr_target_highest_slotid: u32,
}
#[derive(Clone, Debug, XdrIndexer, Default)]
pub enum CbSequence4res {
    Nfs4Ok(CbSequence4resok),
    #[default]
    Default,
}
impl XdrIndexer for CbSequence4res {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Nfs4Ok)),
            _ => Ok(stringify!(Default)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            CbSequence4res::Nfs4Ok(_) => 0i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CbWantsCancelled4args {
    pub cwca_contended_wants_cancelled: bool,
    pub cwca_resourced_wants_cancelled: bool,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CbWantsCancelled4res {
    pub cwcr_status: Nfsstat4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CbNotifyLock4args {
    #[serde(with = "serde_xdr::opaque::variable")]
    pub cnla_fh: Vec<u8>,
    pub cnla_lock_owner: StateOwner4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CbNotifyLock4res {
    pub cnlr_status: Nfsstat4,
}
#[derive(Clone, Debug, PartialEq, XdrIndexer)]
#[repr(i32)]
#[derive(Default)]
pub enum NotifyDeviceidType4 {
    #[default]
    NotifyDeviceid4Change = 1i32,
    NotifyDeviceid4Delete = 2i32,
}
impl XdrIndexer for NotifyDeviceidType4 {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            1i32 => Ok(stringify!(NotifyDeviceid4Change)),
            2i32 => Ok(stringify!(NotifyDeviceid4Delete)),
            _ => Ok(stringify!(NotifyDeviceid4Change)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            NotifyDeviceidType4::NotifyDeviceid4Change => 1i32,
            NotifyDeviceidType4::NotifyDeviceid4Delete => 2i32,
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct NotifyDeviceidDelete4 {
    pub ndd_layouttype: Layouttype4,
    #[serde(with = "serde_xdr::opaque::fixed")]
    pub ndd_deviceid: [u8; NFS4_DEVICEID4_SIZE as usize],
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct NotifyDeviceidChange4 {
    pub ndc_layouttype: Layouttype4,
    #[serde(with = "serde_xdr::opaque::fixed")]
    pub ndc_deviceid: [u8; NFS4_DEVICEID4_SIZE as usize],
    pub ndc_immediate: bool,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CbNotifyDeviceid4args {
    pub cnda_changes: Vec<Notify4>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CbNotifyDeviceid4res {
    pub cndr_status: Nfsstat4,
}
#[derive(Clone, Debug, XdrIndexer)]
pub enum OffloadInfo4 {
    Nfs4Ok(WriteResponse4),
    Default(u64),
}
impl Default for OffloadInfo4 {
    fn default() -> Self {
        OffloadInfo4::Default(Default::default())
    }
}
impl XdrIndexer for OffloadInfo4 {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            0i32 => Ok(stringify!(Nfs4Ok)),
            _ => Ok(stringify!(Default)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            OffloadInfo4::Nfs4Ok(_) => 0i32,
            _ => unimplemented!(),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CbOffload4args {
    #[serde(with = "serde_xdr::opaque::variable")]
    pub coa_fh: Vec<u8>,
    pub coa_stateid: Stateid4,
    pub coa_offload_info: OffloadInfo4,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CbOffload4res {
    pub cor_status: Nfsstat4,
}
#[derive(Clone, Debug, PartialEq, XdrIndexer)]
#[repr(i32)]
#[derive(Default)]
pub enum NfsCbOpnum4 {
    #[default]
    OpCbGetattr = 3i32,
    OpCbRecall = 4i32,
    OpCbLayoutrecall = 5i32,
    OpCbNotify = 6i32,
    OpCbPushDeleg = 7i32,
    OpCbRecallAny = 8i32,
    OpCbRecallableObjAvail = 9i32,
    OpCbRecallSlot = 10i32,
    OpCbSequence = 11i32,
    OpCbWantsCancelled = 12i32,
    OpCbNotifyLock = 13i32,
    OpCbNotifyDeviceid = 14i32,
    OpCbOffload = 15i32,
    OpCbIllegal = 10044i32,
}
impl XdrIndexer for NfsCbOpnum4 {
    type Error = ::serde_xdr::error::Error;
    fn name_by_index(index: i32) -> Result<&'static str, Self::Error> {
        match index {
            3i32 => Ok(stringify!(OpCbGetattr)),
            4i32 => Ok(stringify!(OpCbRecall)),
            5i32 => Ok(stringify!(OpCbLayoutrecall)),
            6i32 => Ok(stringify!(OpCbNotify)),
            7i32 => Ok(stringify!(OpCbPushDeleg)),
            8i32 => Ok(stringify!(OpCbRecallAny)),
            9i32 => Ok(stringify!(OpCbRecallableObjAvail)),
            10i32 => Ok(stringify!(OpCbRecallSlot)),
            11i32 => Ok(stringify!(OpCbSequence)),
            12i32 => Ok(stringify!(OpCbWantsCancelled)),
            13i32 => Ok(stringify!(OpCbNotifyLock)),
            14i32 => Ok(stringify!(OpCbNotifyDeviceid)),
            15i32 => Ok(stringify!(OpCbOffload)),
            10044i32 => Ok(stringify!(OpCbIllegal)),
            _ => Ok(stringify!(OpCbGetattr)),
        }
    }
    fn index(&self) -> i32 {
        match self {
            NfsCbOpnum4::OpCbGetattr => 3i32,
            NfsCbOpnum4::OpCbRecall => 4i32,
            NfsCbOpnum4::OpCbLayoutrecall => 5i32,
            NfsCbOpnum4::OpCbNotify => 6i32,
            NfsCbOpnum4::OpCbPushDeleg => 7i32,
            NfsCbOpnum4::OpCbRecallAny => 8i32,
            NfsCbOpnum4::OpCbRecallableObjAvail => 9i32,
            NfsCbOpnum4::OpCbRecallSlot => 10i32,
            NfsCbOpnum4::OpCbSequence => 11i32,
            NfsCbOpnum4::OpCbWantsCancelled => 12i32,
            NfsCbOpnum4::OpCbNotifyLock => 13i32,
            NfsCbOpnum4::OpCbNotifyDeviceid => 14i32,
            NfsCbOpnum4::OpCbOffload => 15i32,
            NfsCbOpnum4::OpCbIllegal => 10044i32,
        }
    }
}
#[derive(Clone, Debug, XdrIndexer)]
pub enum NfsCbArgop4 {
    OpCbGetattr(CbGetattr4args),
    OpCbRecall(CbRecall4args),
    OpCbLayoutrecall(CbLayoutrecall4args),
    OpCbNotify(CbNotify4args),
    OpCbPushDeleg(CbPushDeleg4args),
    OpCbRecallAny(CbRecallAny4args),
    OpCbRecallableObjAvail(CbRecallAny4args),
    OpCbRecallSlot(CbRecallSlot4args),
    OpCbSequence(CbSequence4args),
    OpCbWantsCancelled(CbWantsCancelled4args),
    OpCbNotifyLock(CbNotifyLock4args),
    OpCbNotifyDeviceid(CbNotifyDeviceid4args),
    OpCbOffload(CbOffload4args),
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
            5i32 => Ok(stringify!(OpCbLayoutrecall)),
            6i32 => Ok(stringify!(OpCbNotify)),
            7i32 => Ok(stringify!(OpCbPushDeleg)),
            8i32 => Ok(stringify!(OpCbRecallAny)),
            9i32 => Ok(stringify!(OpCbRecallableObjAvail)),
            10i32 => Ok(stringify!(OpCbRecallSlot)),
            11i32 => Ok(stringify!(OpCbSequence)),
            12i32 => Ok(stringify!(OpCbWantsCancelled)),
            13i32 => Ok(stringify!(OpCbNotifyLock)),
            14i32 => Ok(stringify!(OpCbNotifyDeviceid)),
            15i32 => Ok(stringify!(OpCbOffload)),
            10044i32 => Ok(stringify!(OpCbIllegal)),
            _ => Err(::serde_xdr::error::Error::Convert),
        }
    }
    fn index(&self) -> i32 {
        match self {
            NfsCbArgop4::OpCbGetattr(_) => 3i32,
            NfsCbArgop4::OpCbRecall(_) => 4i32,
            NfsCbArgop4::OpCbLayoutrecall(_) => 5i32,
            NfsCbArgop4::OpCbNotify(_) => 6i32,
            NfsCbArgop4::OpCbPushDeleg(_) => 7i32,
            NfsCbArgop4::OpCbRecallAny(_) => 8i32,
            NfsCbArgop4::OpCbRecallableObjAvail(_) => 9i32,
            NfsCbArgop4::OpCbRecallSlot(_) => 10i32,
            NfsCbArgop4::OpCbSequence(_) => 11i32,
            NfsCbArgop4::OpCbWantsCancelled(_) => 12i32,
            NfsCbArgop4::OpCbNotifyLock(_) => 13i32,
            NfsCbArgop4::OpCbNotifyDeviceid(_) => 14i32,
            NfsCbArgop4::OpCbOffload(_) => 15i32,
            NfsCbArgop4::OpCbIllegal => 10044i32,
        }
    }
}
#[derive(Clone, Debug, XdrIndexer)]
pub enum NfsCbResop4 {
    OpCbGetattr(CbGetattr4res),
    OpCbRecall(CbRecall4res),
    OpCbLayoutrecall(CbLayoutrecall4res),
    OpCbNotify(CbNotify4res),
    OpCbPushDeleg(CbPushDeleg4res),
    OpCbRecallAny(CbRecallAny4res),
    OpCbRecallableObjAvail(CbRecallableObjAvail4res),
    OpCbRecallSlot(CbRecallSlot4res),
    OpCbSequence(CbSequence4res),
    OpCbWantsCancelled(CbWantsCancelled4res),
    OpCbNotifyLock(CbNotifyLock4res),
    OpCbNotifyDeviceid(CbNotifyDeviceid4res),
    OpCbOffload(CbOffload4res),
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
            5i32 => Ok(stringify!(OpCbLayoutrecall)),
            6i32 => Ok(stringify!(OpCbNotify)),
            7i32 => Ok(stringify!(OpCbPushDeleg)),
            8i32 => Ok(stringify!(OpCbRecallAny)),
            9i32 => Ok(stringify!(OpCbRecallableObjAvail)),
            10i32 => Ok(stringify!(OpCbRecallSlot)),
            11i32 => Ok(stringify!(OpCbSequence)),
            12i32 => Ok(stringify!(OpCbWantsCancelled)),
            13i32 => Ok(stringify!(OpCbNotifyLock)),
            14i32 => Ok(stringify!(OpCbNotifyDeviceid)),
            15i32 => Ok(stringify!(OpCbOffload)),
            10044i32 => Ok(stringify!(OpCbIllegal)),
            _ => Err(::serde_xdr::error::Error::Convert),
        }
    }
    fn index(&self) -> i32 {
        match self {
            NfsCbResop4::OpCbGetattr(_) => 3i32,
            NfsCbResop4::OpCbRecall(_) => 4i32,
            NfsCbResop4::OpCbLayoutrecall(_) => 5i32,
            NfsCbResop4::OpCbNotify(_) => 6i32,
            NfsCbResop4::OpCbPushDeleg(_) => 7i32,
            NfsCbResop4::OpCbRecallAny(_) => 8i32,
            NfsCbResop4::OpCbRecallableObjAvail(_) => 9i32,
            NfsCbResop4::OpCbRecallSlot(_) => 10i32,
            NfsCbResop4::OpCbSequence(_) => 11i32,
            NfsCbResop4::OpCbWantsCancelled(_) => 12i32,
            NfsCbResop4::OpCbNotifyLock(_) => 13i32,
            NfsCbResop4::OpCbNotifyDeviceid(_) => 14i32,
            NfsCbResop4::OpCbOffload(_) => 15i32,
            NfsCbResop4::OpCbIllegal(_) => 10044i32,
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
    pub status: Nfsstat4,
    pub tag: String,
    pub resarray: Vec<NfsCbResop4>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AuthsysParms {
    pub stamp: u32,
    pub machinename: String,
    pub uid: u32,
    pub gid: u32,
    pub gids: Vec<u32>,
}
