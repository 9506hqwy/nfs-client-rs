pub mod binding;
pub mod error;
pub mod protocol;

use bytes::{Buf, Bytes};
use error::Error;
use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpStream;
use std::path::{Component, Path, PathBuf};
use url::Url;

const RPC_VERS: u32 = 2;
const NFS_PROG: u32 = 100003;
const NFS_V4: u32 = 4;

pub struct Client {
    serial: u32,
    stream: TcpStream,
    path: PathBuf,
}

impl Client {
    pub fn new(url: Url) -> Result<Self, std::io::Error> {
        let host = url.host_str().unwrap_or("127.0.0.1");
        let port = url.port().unwrap_or(2049);
        let path = PathBuf::from(url.path());

        let addr = format!("{}:{}", host, port);
        let stream = TcpStream::connect(addr)?;

        Ok(Client {
            serial: 0,
            stream,
            path,
        })
    }

    // op = 1
    pub fn compound(
        &mut self,
        argarray: Vec<binding::NfsArgop4>,
    ) -> Result<Vec<binding::NfsResop4>, Error> {
        self.serial += 1;

        let header = call_msg(self.serial, 1);
        let header_bytes = serde_xdr::to_bytes(&header).map_err(|_| Error::SerializeError)?;

        let args = call_compound_args(argarray);
        let args_bytes = serde_xdr::to_bytes(&args).map_err(|_| Error::SerializeError)?;

        let mut record_marking: u32 = (header_bytes.len() + args_bytes.len()).try_into().unwrap();
        record_marking |= 0x8000_0000;

        self.stream
            .write_all(&record_marking.to_be_bytes())
            .map_err(Error::SendError)?;
        self.stream
            .write_all(&header_bytes)
            .map_err(Error::SendError)?;
        self.stream
            .write_all(&args_bytes)
            .map_err(Error::SendError)?;

        let mut buf = [0; 4];
        self.stream.read_exact(&mut buf).map_err(Error::RecvError)?;

        let size = u32::from_be_bytes(buf);
        let complated = (size & 0x8000_0000) != 0;
        let size = size & 0x7FFF_FFFF;
        if !complated {
            return Err(Error::NotSupported);
        }

        let mut buf = vec![0; size as usize];
        self.stream.read_exact(&mut buf).map_err(Error::RecvError)?;

        let mut buf = Bytes::from(buf);
        parse_response(&mut buf)?;

        let res: binding::COMPOUND4res =
            serde_xdr::from_bytes(&buf).map_err(|_| Error::DeserializeError)?;

        if res.status != binding::Nfsstat4::Nfs4Ok {
            return Err(Error::NfsError(res.status));
        }

        Ok(res.resarray)
    }

    // op = 15
    pub fn lookup(&mut self, objname: Option<&Path>) -> Result<Vec<u8>, Error> {
        let mut path = self.path.clone();
        if let Some(name) = objname {
            path = path.join(name);
        }

        let mut args = path_to_arg(&path)?;

        let arg = binding::NfsArgop4::OpGetfh;
        args.push(arg);

        let mut ret = self.compound(args)?;

        match ret.pop() {
            Some(binding::NfsResop4::OpGetfh(binding::GETFH4res::Nfs4Ok(res))) => Ok(res.object),
            _ => unreachable!(),
        }
    }

    // op = 25
    pub fn read(&mut self, objname: Option<&Path>) -> Result<binding::READ4resok, Error> {
        let mut path = self.path.clone();
        if let Some(name) = objname {
            path = path.join(name);
        }

        let mut args = path_to_arg(&path)?;

        let arg = binding::READ4args {
            count: u32::MAX,
            ..Default::default()
        };
        let arg = binding::NfsArgop4::OpRead(arg);
        args.push(arg);

        let mut ret = self.compound(args)?;

        match ret.pop() {
            Some(binding::NfsResop4::OpRead(binding::READ4res::Nfs4Ok(res))) => Ok(res),
            _ => unreachable!(),
        }
    }

    // op = 26
    pub fn read_dir(&mut self, object: Vec<u8>, attrs: Vec<u32>) -> Result<Vec<Entry>, Error> {
        let mut args = vec![];

        let arg = binding::PUTFH4args { object };
        let arg = binding::NfsArgop4::OpPutfh(arg);
        args.push(arg);

        let arg = binding::READDIR4args {
            dircount: 8192,
            maxcount: 8192,
            attr_request: fattr4_bitmap(attrs),
            ..binding::READDIR4args::default()
        };
        let arg = binding::NfsArgop4::OpReaddir(arg);
        args.push(arg);

        let mut ret = self.compound(args)?;

        match ret.pop() {
            Some(binding::NfsResop4::OpReaddir(binding::READDIR4res::Nfs4Ok(res))) => {
                Ok(parse_readdir_res(res)?)
            }
            _ => unreachable!(),
        }
    }

    // op = 33
    pub fn secinfo(&mut self, objname: Option<&Path>) -> Result<Vec<binding::Secinfo4>, Error> {
        let mut path = self.path.clone();
        if let Some(name) = objname {
            path = path.join(name);
        }

        let mut args = vec![];

        match path.parent() {
            Some(parent) => {
                let mut arg = path_to_arg(parent)?;
                args.append(&mut arg);

                let arg = binding::SECINFO4args {
                    name: path.file_name().unwrap().to_str().unwrap().to_string(),
                };
                let arg = binding::NfsArgop4::OpSecinfo(arg);
                args.push(arg);
            }
            _ => {
                return Ok(vec![]);
            }
        }

        let mut ret = self.compound(args)?;

        match ret.pop() {
            Some(binding::NfsResop4::OpSecinfo(binding::SECINFO4res::Nfs4Ok(res))) => Ok(res),
            _ => unreachable!(),
        }
    }

    // op = 35, op = 36
    pub fn set_clientid(&mut self, id: Vec<u8>) -> Result<u64, Error> {
        let mut args = vec![];

        let mut arg = binding::SETCLIENTID4args::default();
        arg.client.id = id;
        let arg = binding::NfsArgop4::OpSetclientid(arg);
        args.push(arg);

        let mut ret = self.compound(args)?;

        match ret.pop() {
            Some(binding::NfsResop4::OpSetclientid(binding::SETCLIENTID4res::Nfs4Ok(res))) => {
                let mut args = vec![];

                let arg = binding::SetclientidConfirm4args {
                    clientid: res.clientid,
                    setclientid_confirm: res.setclientid_confirm,
                };
                let arg = binding::NfsArgop4::OpSetclientidConfirm(arg);
                args.push(arg);

                let mut ret = self.compound(args)?;

                match ret.pop() {
                    Some(binding::NfsResop4::OpSetclientidConfirm(r))
                        if r.status == binding::Nfsstat4::Nfs4Ok =>
                    {
                        Ok(res.clientid)
                    }
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Entry {
    pub name: String,
    pub attrs: HashMap<u32, Vec<u8>>,
}

fn call_msg(serial: u32, proc: u32) -> protocol::RpcMsg {
    let body = protocol::CallBody {
        rpcvers: RPC_VERS,
        prog: NFS_PROG,
        vers: NFS_V4,
        proc,
        cred: protocol::OpaqueAuth {
            flavor: protocol::AuthFlavor::AuthSys,
            body: serde_xdr::to_bytes(&protocol::AuthsysParms::default()).unwrap(),
        },
        ..protocol::CallBody::default()
    };

    let body = protocol::RpcMsgBody::CALL(body);

    protocol::RpcMsg { xid: serial, body }
}

fn call_compound_args(argarray: Vec<binding::NfsArgop4>) -> binding::COMPOUND4args {
    binding::COMPOUND4args {
        tag: "nfs-client-rs".to_string(),
        minorversion: 0,
        argarray,
    }
}

fn fattr4_bitmap(attrs: Vec<u32>) -> Vec<u32> {
    let max_index = attrs.iter().map(|&i| i / 32).max().unwrap() as usize;
    let mut bitmap = vec![0u32; max_index + 1];

    for attr in attrs {
        let index = (attr / 32) as usize;
        let shift = attr % 32;
        bitmap[index] |= 1 << shift;
    }

    bitmap
}

fn padded4(value: u32) -> u32 {
    ((value + 3) / 4) * 4
}

fn parse_fattr4(attr: &binding::Fattr4) -> HashMap<u32, Vec<u8>> {
    let mut vals = Bytes::from(attr.attr_vals.clone());

    let mut attrs = HashMap::new();

    let max_num = 32 * attr.attrmask.len();
    for num in 0..max_num {
        let index = (num / 32) as usize;
        let pos = num % 32;
        if (attr.attrmask[index] & (1 << pos)) > 0 {
            let i = num as u32;
            attrs.insert(i, drain_fattr4(i, &mut vals));
        }
    }

    attrs
}

fn drain_fattr4(attr: u32, vals: &mut Bytes) -> Vec<u8> {
    let mut r = vec![];

    // sec 5.6 and 5.7
    let size = match attr {
        binding::FATTR4_TYPE => 4,
        binding::FATTR4_SIZE => 8,
        binding::FATTR4_MODE => 4,
        binding::FATTR4_NUMLINKS => 4,
        binding::FATTR4_OWNER => padded4(vals.get_u32()),
        binding::FATTR4_OWNER_GROUP => padded4(vals.get_u32()),
        binding::FATTR4_TIME_MODIFY => 12,
        _ => unreachable!(),
    };

    for _ in 0..size {
        r.push(vals.get_u8());
    }

    r
}

fn parse_response(data: &mut Bytes) -> Result<protocol::RpcMsg, Error> {
    let xid = data.get_u32();
    let mtype = data.get_i32();
    let mtype = protocol::MsgType::try_from(mtype).map_err(|_| Error::InvalidMsgType(mtype))?;
    match mtype {
        protocol::MsgType::CALL => Err(Error::InvalidMsgType(*mtype.as_ref())),
        protocol::MsgType::REPLY => {
            let stat = data.get_i32();
            let stat =
                protocol::ReplyStat::try_from(stat).map_err(|_| Error::InvalidReplyStat(stat))?;
            let reply = match stat {
                protocol::ReplyStat::MsgAccepted => {
                    Ok(protocol::ReplyBody::MsgAccepted(parse_accepted(data)?))
                }
                protocol::ReplyStat::MsgDenied => Err(Error::Denied(parse_denied(data)?)),
            }?;
            let body = protocol::RpcMsgBody::REPLY(reply);
            Ok(protocol::RpcMsg { xid, body })
        }
    }
}

fn parse_accepted(data: &mut Bytes) -> Result<protocol::AcceptedReply, Error> {
    let verf = parse_opaque_auth(data)?;

    let stat = data.get_i32();
    let stat = protocol::AcceptStat::try_from(stat).map_err(|_| Error::InvalidAcceptStat(stat))?;
    let reply_data = match stat {
        protocol::AcceptStat::SUCCESS => Ok(protocol::AcceptedReplyData::SUCCESS([])),
        protocol::AcceptStat::ProgMismatch => {
            let low = data.get_u32();
            let high = data.get_u32();
            let info = protocol::AcceptedReplyMismatchInfo { low, high };
            Err(Error::ProgMismatch(info))
        }
        _ => Err(Error::InvalidAcceptStat(*stat.as_ref())),
    }?;

    Ok(protocol::AcceptedReply { verf, reply_data })
}

fn parse_denied(data: &mut Bytes) -> Result<protocol::RejectedReply, Error> {
    let stat = data.get_i32();
    let stat = protocol::RejectStat::try_from(stat).map_err(|_| Error::InvalidRejectStat(stat))?;
    match stat {
        protocol::RejectStat::RpcMismatch => {
            let low = data.get_u32();
            let high = data.get_u32();
            let info = protocol::RejectedReplyMismatchInfo { low, high };
            Ok(protocol::RejectedReply::RpcMismatch(info))
        }
        protocol::RejectStat::AuthError => {
            let auth_stat = data.get_i32();
            let auth_stat = protocol::AuthStat::try_from(auth_stat)
                .map_err(|_| Error::InvalidAuthStat(auth_stat))?;
            Ok(protocol::RejectedReply::AuthError(auth_stat))
        }
    }
}

fn parse_opaque_auth(data: &mut Bytes) -> Result<protocol::OpaqueAuth, Error> {
    let flavor = data.get_i32();
    let flavor =
        protocol::AuthFlavor::try_from(flavor).map_err(|_| Error::InvalidAuthFlavor(flavor))?;

    let mut body = vec![];
    let size = data.get_u32() as usize;
    if size > 0 {
        for _ in 0..size {
            body.push(data.get_u8());
        }
    }

    Ok(protocol::OpaqueAuth { flavor, body })
}

fn parse_readdir_res(res: binding::READDIR4resok) -> Result<Vec<Entry>, Error> {
    let mut entries = vec![];

    loop {
        let mut entry = &res.reply.entries;
        while let Some(item) = entry {
            let name = item.name.clone();
            let attrs = parse_fattr4(&item.attrs);

            let e = Entry { name, attrs };
            entries.push(e);

            // Next
            entry = item.nextentry.as_ref();
        }

        if res.reply.eof {
            break;
        }
    }

    Ok(entries)
}

fn path_to_arg(path: &Path) -> Result<Vec<binding::NfsArgop4>, Error> {
    let mut args = vec![];

    for comp in path.components() {
        match comp {
            Component::Prefix(_) => {
                return Err(Error::NotSupported);
            }
            Component::RootDir => {
                let arg = binding::NfsArgop4::OpPutrootfh;
                args.push(arg);
            }
            Component::CurDir => {
                // PASS
            }
            Component::ParentDir => {
                args.pop();
            }
            Component::Normal(name) => {
                let arg = binding::LOOKUP4args {
                    objname: name.to_str().unwrap().to_string(),
                };
                let arg = binding::NfsArgop4::OpLookup(arg);
                args.push(arg);
            }
        }
    }

    Ok(args)
}
