enum auth_flavor {
    AUTH_NONE  = 0,
    AUTH_SYS   = 1,
    AUTH_SHORT = 2,
    AUTH_DH    = 3,
    RPCSEC_GSS = 6
};

struct opaque_auth {
    auth_flavor flavor;
    opaque body<400>;
};

enum msg_type {
    CALL  = 0,
    REPLY = 1
};

enum reply_stat {
    MSG_ACCEPTED = 0,
    MSG_DENIED   = 1
};

enum accept_stat {
    SUCCESS       = 0,
    PROG_UNAVAIL  = 1,
    PROG_MISMATCH = 2,
    PROC_UNAVAIL  = 3,
    GARBAGE_ARGS  = 4,
    SYSTEM_ERR    = 5
};

enum reject_stat {
    RPC_MISMATCH = 0,
    AUTH_ERROR   = 1
};

enum auth_stat {
    AUTH_OK                = 0,
    AUTH_BADCRED           = 1,
    AUTH_REJECTEDCRED      = 2,
    AUTH_BADVERF           = 3,
    AUTH_REJECTEDVERF      = 4,
    AUTH_TOOWEAK           = 5,
    AUTH_INVALIDRESP       = 6,
    AUTH_FAILED            = 7,
    AUTH_KERB_GENERIC      = 8,
    AUTH_TIMEEXPIRE        = 9,
    AUTH_TKT_FILE          = 10,
    AUTH_DECODE            = 11,
    AUTH_NET_ADDR          = 12,
    RPCSEC_GSS_CREDPROBLEM = 13,
    RPCSEC_GSS_CTXPROBLEM  = 14
};

union rpc_msg_body switch (msg_type mtype) {
case CALL:
    call_body cbody;
case REPLY:
    reply_body rbody;
};

struct rpc_msg {
    unsigned int xid;
    rpc_msg_body body;
};

struct call_body {
   unsigned int rpcvers;
   unsigned int prog;
   unsigned int vers;
   unsigned int proc;
   opaque_auth cred;
   opaque_auth verf;
};

union reply_body switch (reply_stat stat) {
case MSG_ACCEPTED:
    accepted_reply areply;
case MSG_DENIED:
    rejected_reply rreply;
};

struct accepted_reply_mismatch_info {
    unsigned int low;
    unsigned int high;
};

union accepted_reply_data switch (accept_stat stat) {
case SUCCESS:
    opaque results[0];
case PROG_MISMATCH:
    accepted_reply_mismatch_info mismatch_info;
default:
    void;
};

struct accepted_reply {
    opaque_auth verf;
    accepted_reply_data reply_data;
};

struct rejected_reply_mismatch_info {
    unsigned int low;
    unsigned int high;
};

union rejected_reply switch (reject_stat stat) {
case RPC_MISMATCH:
    rejected_reply_mismatch_info mismatch_info;
case AUTH_ERROR:
    auth_stat stat;
};
