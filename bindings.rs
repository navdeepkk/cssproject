# [repr ( C )] # [derive ( Default )] pub struct __IncompleteArrayField < T > (:: std :: marker :: PhantomData < T > , [ T ; 0 ]) ;
 impl < T > __IncompleteArrayField < T > {
# [inline] pub const fn new () -> Self {
__IncompleteArrayField (:: std :: marker :: PhantomData , [ ])
}
 # [inline] pub unsafe fn as_ptr (& self) -> * const T {
:: std :: mem :: transmute (self)
}
 # [inline] pub unsafe fn as_mut_ptr (& mut self) -> * mut T {
:: std :: mem :: transmute (self)
}
 # [inline] pub unsafe fn as_slice (& self , len : usize) -> & [T] {
:: std :: slice :: from_raw_parts (self . as_ptr ( ) , len)
}
 # [inline] pub unsafe fn as_mut_slice (& mut self , len : usize) -> & mut [T] {
:: std :: slice :: from_raw_parts_mut (self . as_mut_ptr ( ) , len)
}

}
 impl < T > :: std :: fmt :: Debug for __IncompleteArrayField < T > {
fn fmt (& self , fmt : & mut :: std :: fmt :: Formatter < '_ >) -> :: std :: fmt :: Result {
fmt . write_str ("__IncompleteArrayField")
}

}
 impl < T > :: std :: clone :: Clone for __IncompleteArrayField < T > {
# [inline] fn clone (& self) -> Self {
Self :: new ()
}

}
 pub const VSFTP_DEFAULT_CONFIG : & 'static [u8 ; 17usize] = b"/etc/vsftpd.conf\0" ;
 pub const VSFTP_COMMAND_FD : u32 = 0 ;
 pub const VSFTP_PASSWORD_MAX : u32 = 128 ;
 pub const VSFTP_USERNAME_MAX : u32 = 128 ;
 pub const VSFTP_MAX_COMMAND_LINE : u32 = 4096 ;
 pub const VSFTP_DATA_BUFSIZE : u32 = 65536 ;
 pub const VSFTP_DIR_BUFSIZE : u32 = 16384 ;
 pub const VSFTP_MATCHITERS_MAX : u32 = 1000 ;
 pub const VSFTP_PATH_MAX : u32 = 4096 ;
 pub const VSFTP_CONF_FILE_MAX : u32 = 100000 ;
 pub const VSFTP_LISTEN_BACKLOG : u32 = 32 ;
 pub const VSFTP_SECURE_UMASK : u32 = 63 ;
 pub const VSFTP_ROOT_UID : u32 = 0 ;
 pub const VSFTP_PRIVSOCK_MAXSTR : u32 = 131072 ;
 pub const VSFTP_AS_LIMIT : u32 = 209715200 ;
 pub const FTP_DATACONN : u32 = 150 ;
 pub const FTP_NOOPOK : u32 = 200 ;
 pub const FTP_TYPEOK : u32 = 200 ;
 pub const FTP_PORTOK : u32 = 200 ;
 pub const FTP_EPRTOK : u32 = 200 ;
 pub const FTP_UMASKOK : u32 = 200 ;
 pub const FTP_CHMODOK : u32 = 200 ;
 pub const FTP_EPSVALLOK : u32 = 200 ;
 pub const FTP_STRUOK : u32 = 200 ;
 pub const FTP_MODEOK : u32 = 200 ;
 pub const FTP_PBSZOK : u32 = 200 ;
 pub const FTP_PROTOK : u32 = 200 ;
 pub const FTP_OPTSOK : u32 = 200 ;
 pub const FTP_ALLOOK : u32 = 202 ;
 pub const FTP_FEAT : u32 = 211 ;
 pub const FTP_STATOK : u32 = 211 ;
 pub const FTP_SIZEOK : u32 = 213 ;
 pub const FTP_MDTMOK : u32 = 213 ;
 pub const FTP_STATFILE_OK : u32 = 213 ;
 pub const FTP_SITEHELP : u32 = 214 ;
 pub const FTP_HELP : u32 = 214 ;
 pub const FTP_SYSTOK : u32 = 215 ;
 pub const FTP_GREET : u32 = 220 ;
 pub const FTP_GOODBYE : u32 = 221 ;
 pub const FTP_ABOR_NOCONN : u32 = 225 ;
 pub const FTP_TRANSFEROK : u32 = 226 ;
 pub const FTP_ABOROK : u32 = 226 ;
 pub const FTP_PASVOK : u32 = 227 ;
 pub const FTP_EPSVOK : u32 = 229 ;
 pub const FTP_LOGINOK : u32 = 230 ;
 pub const FTP_AUTHOK : u32 = 234 ;
 pub const FTP_CWDOK : u32 = 250 ;
 pub const FTP_RMDIROK : u32 = 250 ;
 pub const FTP_DELEOK : u32 = 250 ;
 pub const FTP_RENAMEOK : u32 = 250 ;
 pub const FTP_PWDOK : u32 = 257 ;
 pub const FTP_MKDIROK : u32 = 257 ;
 pub const FTP_GIVEPWORD : u32 = 331 ;
 pub const FTP_RESTOK : u32 = 350 ;
 pub const FTP_RNFROK : u32 = 350 ;
 pub const FTP_IDLE_TIMEOUT : u32 = 421 ;
 pub const FTP_DATA_TIMEOUT : u32 = 421 ;
 pub const FTP_TOO_MANY_USERS : u32 = 421 ;
 pub const FTP_IP_LIMIT : u32 = 421 ;
 pub const FTP_IP_DENY : u32 = 421 ;
 pub const FTP_TLS_FAIL : u32 = 421 ;
 pub const FTP_BADSENDCONN : u32 = 425 ;
 pub const FTP_BADSENDNET : u32 = 426 ;
 pub const FTP_BADSENDFILE : u32 = 451 ;
 pub const FTP_BADCMD : u32 = 500 ;
 pub const FTP_BADOPTS : u32 = 501 ;
 pub const FTP_COMMANDNOTIMPL : u32 = 502 ;
 pub const FTP_NEEDUSER : u32 = 503 ;
 pub const FTP_NEEDRNFR : u32 = 503 ;
 pub const FTP_BADPBSZ : u32 = 503 ;
 pub const FTP_BADPROT : u32 = 503 ;
 pub const FTP_BADSTRU : u32 = 504 ;
 pub const FTP_BADMODE : u32 = 504 ;
 pub const FTP_BADAUTH : u32 = 504 ;
 pub const FTP_NOSUCHPROT : u32 = 504 ;
 pub const FTP_NEEDENCRYPT : u32 = 522 ;
 pub const FTP_EPSVBAD : u32 = 522 ;
 pub const FTP_DATATLSBAD : u32 = 522 ;
 pub const FTP_LOGINERR : u32 = 530 ;
 pub const FTP_NOHANDLEPROT : u32 = 536 ;
 pub const FTP_FILEFAIL : u32 = 550 ;
 pub const FTP_NOPERM : u32 = 550 ;
 pub const FTP_UPLOADFAIL : u32 = 553 ;
 pub const _SYS_TYPES_H : u32 = 1 ;
 pub const _FEATURES_H : u32 = 1 ;
 pub const _ISOC95_SOURCE : u32 = 1 ;
 pub const _ISOC99_SOURCE : u32 = 1 ;
 pub const _ISOC11_SOURCE : u32 = 1 ;
 pub const _POSIX_SOURCE : u32 = 1 ;
 pub const _POSIX_C_SOURCE : u32 = 200809 ;
 pub const _XOPEN_SOURCE : u32 = 700 ;
 pub const _XOPEN_SOURCE_EXTENDED : u32 = 1 ;
 pub const _LARGEFILE64_SOURCE : u32 = 1 ;
 pub const _DEFAULT_SOURCE : u32 = 1 ;
 pub const _ATFILE_SOURCE : u32 = 1 ;
 pub const __USE_ISOC11 : u32 = 1 ;
 pub const __USE_ISOC99 : u32 = 1 ;
 pub const __USE_ISOC95 : u32 = 1 ;
 pub const __USE_ISOCXX11 : u32 = 1 ;
 pub const __USE_POSIX : u32 = 1 ;
 pub const __USE_POSIX2 : u32 = 1 ;
 pub const __USE_POSIX199309 : u32 = 1 ;
 pub const __USE_POSIX199506 : u32 = 1 ;
 pub const __USE_XOPEN2K : u32 = 1 ;
 pub const __USE_XOPEN2K8 : u32 = 1 ;
 pub const __USE_XOPEN : u32 = 1 ;
 pub const __USE_XOPEN_EXTENDED : u32 = 1 ;
 pub const __USE_UNIX98 : u32 = 1 ;
 pub const _LARGEFILE_SOURCE : u32 = 1 ;
 pub const __USE_XOPEN2K8XSI : u32 = 1 ;
 pub const __USE_XOPEN2KXSI : u32 = 1 ;
 pub const __USE_LARGEFILE : u32 = 1 ;
 pub const __USE_LARGEFILE64 : u32 = 1 ;
 pub const __USE_MISC : u32 = 1 ;
 pub const __USE_ATFILE : u32 = 1 ;
 pub const __USE_GNU : u32 = 1 ;
 pub const __USE_FORTIFY_LEVEL : u32 = 0 ;
 pub const __GLIBC_USE_DEPRECATED_GETS : u32 = 1 ;
 pub const _STDC_PREDEF_H : u32 = 1 ;
 pub const __STDC_IEC_559__ : u32 = 1 ;
 pub const __STDC_IEC_559_COMPLEX__ : u32 = 1 ;
 pub const __STDC_ISO_10646__ : u32 = 201706 ;
 pub const __STDC_NO_THREADS__ : u32 = 1 ;
 pub const __GNU_LIBRARY__ : u32 = 6 ;
 pub const __GLIBC__ : u32 = 2 ;
 pub const __GLIBC_MINOR__ : u32 = 27 ;
 pub const _SYS_CDEFS_H : u32 = 1 ;
 pub const __glibc_c99_flexarr_available : u32 = 1 ;
 pub const __WORDSIZE : u32 = 64 ;
 pub const __WORDSIZE_TIME64_COMPAT32 : u32 = 1 ;
 pub const __SYSCALL_WORDSIZE : u32 = 64 ;
 pub const __HAVE_GENERIC_SELECTION : u32 = 0 ;
 pub const _BITS_TYPES_H : u32 = 1 ;
 pub const _BITS_TYPESIZES_H : u32 = 1 ;
 pub const __OFF_T_MATCHES_OFF64_T : u32 = 1 ;
 pub const __INO_T_MATCHES_INO64_T : u32 = 1 ;
 pub const __RLIM_T_MATCHES_RLIM64_T : u32 = 1 ;
 pub const __FD_SETSIZE : u32 = 1024 ;
 pub const __clock_t_defined : u32 = 1 ;
 pub const __clockid_t_defined : u32 = 1 ;
 pub const __time_t_defined : u32 = 1 ;
 pub const __timer_t_defined : u32 = 1 ;
 pub const _BITS_STDINT_INTN_H : u32 = 1 ;
 pub const __BIT_TYPES_DEFINED__ : u32 = 1 ;
 pub const _ENDIAN_H : u32 = 1 ;
 pub const __LITTLE_ENDIAN : u32 = 1234 ;
 pub const __BIG_ENDIAN : u32 = 4321 ;
 pub const __PDP_ENDIAN : u32 = 3412 ;
 pub const __BYTE_ORDER : u32 = 1234 ;
 pub const __FLOAT_WORD_ORDER : u32 = 1234 ;
 pub const LITTLE_ENDIAN : u32 = 1234 ;
 pub const BIG_ENDIAN : u32 = 4321 ;
 pub const PDP_ENDIAN : u32 = 3412 ;
 pub const BYTE_ORDER : u32 = 1234 ;
 pub const _BITS_BYTESWAP_H : u32 = 1 ;
 pub const _BITS_UINTN_IDENTITY_H : u32 = 1 ;
 pub const _SYS_SELECT_H : u32 = 1 ;
 pub const __FD_ZERO_STOS : & 'static [u8 ; 6usize] = b"stosq\0" ;
 pub const __sigset_t_defined : u32 = 1 ;
 pub const __timeval_defined : u32 = 1 ;
 pub const __timespec_defined : u32 = 1 ;
 pub const FD_SETSIZE : u32 = 1024 ;
 pub const _SYS_SYSMACROS_H : u32 = 1 ;
 pub const _BITS_SYSMACROS_H : u32 = 1 ;
 pub const _BITS_PTHREADTYPES_COMMON_H : u32 = 1 ;
 pub const _THREAD_SHARED_TYPES_H : u32 = 1 ;
 pub const _BITS_PTHREADTYPES_ARCH_H : u32 = 1 ;
 pub const __SIZEOF_PTHREAD_MUTEX_T : u32 = 40 ;
 pub const __SIZEOF_PTHREAD_ATTR_T : u32 = 56 ;
 pub const __SIZEOF_PTHREAD_RWLOCK_T : u32 = 56 ;
 pub const __SIZEOF_PTHREAD_BARRIER_T : u32 = 32 ;
 pub const __SIZEOF_PTHREAD_MUTEXATTR_T : u32 = 4 ;
 pub const __SIZEOF_PTHREAD_COND_T : u32 = 48 ;
 pub const __SIZEOF_PTHREAD_CONDATTR_T : u32 = 4 ;
 pub const __SIZEOF_PTHREAD_RWLOCKATTR_T : u32 = 8 ;
 pub const __SIZEOF_PTHREAD_BARRIERATTR_T : u32 = 4 ;
 pub const __PTHREAD_MUTEX_LOCK_ELISION : u32 = 1 ;
 pub const __PTHREAD_MUTEX_NUSERS_AFTER_KIND : u32 = 0 ;
 pub const __PTHREAD_MUTEX_USE_UNION : u32 = 0 ;
 pub const __PTHREAD_RWLOCK_INT_FLAGS_SHARED : u32 = 1 ;
 pub const __PTHREAD_MUTEX_HAVE_PREV : u32 = 1 ;
 pub const __have_pthread_attr_t : u32 = 1 ;
 pub const _SYS_SOCKET_H : u32 = 1 ;
 pub const __iovec_defined : u32 = 1 ;
 pub const PF_UNSPEC : u32 = 0 ;
 pub const PF_LOCAL : u32 = 1 ;
 pub const PF_UNIX : u32 = 1 ;
 pub const PF_FILE : u32 = 1 ;
 pub const PF_INET : u32 = 2 ;
 pub const PF_AX25 : u32 = 3 ;
 pub const PF_IPX : u32 = 4 ;
 pub const PF_APPLETALK : u32 = 5 ;
 pub const PF_NETROM : u32 = 6 ;
 pub const PF_BRIDGE : u32 = 7 ;
 pub const PF_ATMPVC : u32 = 8 ;
 pub const PF_X25 : u32 = 9 ;
 pub const PF_INET6 : u32 = 10 ;
 pub const PF_ROSE : u32 = 11 ;
 pub const PF_DECnet : u32 = 12 ;
 pub const PF_NETBEUI : u32 = 13 ;
 pub const PF_SECURITY : u32 = 14 ;
 pub const PF_KEY : u32 = 15 ;
 pub const PF_NETLINK : u32 = 16 ;
 pub const PF_ROUTE : u32 = 16 ;
 pub const PF_PACKET : u32 = 17 ;
 pub const PF_ASH : u32 = 18 ;
 pub const PF_ECONET : u32 = 19 ;
 pub const PF_ATMSVC : u32 = 20 ;
 pub const PF_RDS : u32 = 21 ;
 pub const PF_SNA : u32 = 22 ;
 pub const PF_IRDA : u32 = 23 ;
 pub const PF_PPPOX : u32 = 24 ;
 pub const PF_WANPIPE : u32 = 25 ;
 pub const PF_LLC : u32 = 26 ;
 pub const PF_IB : u32 = 27 ;
 pub const PF_MPLS : u32 = 28 ;
 pub const PF_CAN : u32 = 29 ;
 pub const PF_TIPC : u32 = 30 ;
 pub const PF_BLUETOOTH : u32 = 31 ;
 pub const PF_IUCV : u32 = 32 ;
 pub const PF_RXRPC : u32 = 33 ;
 pub const PF_ISDN : u32 = 34 ;
 pub const PF_PHONET : u32 = 35 ;
 pub const PF_IEEE802154 : u32 = 36 ;
 pub const PF_CAIF : u32 = 37 ;
 pub const PF_ALG : u32 = 38 ;
 pub const PF_NFC : u32 = 39 ;
 pub const PF_VSOCK : u32 = 40 ;
 pub const PF_KCM : u32 = 41 ;
 pub const PF_QIPCRTR : u32 = 42 ;
 pub const PF_SMC : u32 = 43 ;
 pub const PF_MAX : u32 = 44 ;
 pub const AF_UNSPEC : u32 = 0 ;
 pub const AF_LOCAL : u32 = 1 ;
 pub const AF_UNIX : u32 = 1 ;
 pub const AF_FILE : u32 = 1 ;
 pub const AF_INET : u32 = 2 ;
 pub const AF_AX25 : u32 = 3 ;
 pub const AF_IPX : u32 = 4 ;
 pub const AF_APPLETALK : u32 = 5 ;
 pub const AF_NETROM : u32 = 6 ;
 pub const AF_BRIDGE : u32 = 7 ;
 pub const AF_ATMPVC : u32 = 8 ;
 pub const AF_X25 : u32 = 9 ;
 pub const AF_INET6 : u32 = 10 ;
 pub const AF_ROSE : u32 = 11 ;
 pub const AF_DECnet : u32 = 12 ;
 pub const AF_NETBEUI : u32 = 13 ;
 pub const AF_SECURITY : u32 = 14 ;
 pub const AF_KEY : u32 = 15 ;
 pub const AF_NETLINK : u32 = 16 ;
 pub const AF_ROUTE : u32 = 16 ;
 pub const AF_PACKET : u32 = 17 ;
 pub const AF_ASH : u32 = 18 ;
 pub const AF_ECONET : u32 = 19 ;
 pub const AF_ATMSVC : u32 = 20 ;
 pub const AF_RDS : u32 = 21 ;
 pub const AF_SNA : u32 = 22 ;
 pub const AF_IRDA : u32 = 23 ;
 pub const AF_PPPOX : u32 = 24 ;
 pub const AF_WANPIPE : u32 = 25 ;
 pub const AF_LLC : u32 = 26 ;
 pub const AF_IB : u32 = 27 ;
 pub const AF_MPLS : u32 = 28 ;
 pub const AF_CAN : u32 = 29 ;
 pub const AF_TIPC : u32 = 30 ;
 pub const AF_BLUETOOTH : u32 = 31 ;
 pub const AF_IUCV : u32 = 32 ;
 pub const AF_RXRPC : u32 = 33 ;
 pub const AF_ISDN : u32 = 34 ;
 pub const AF_PHONET : u32 = 35 ;
 pub const AF_IEEE802154 : u32 = 36 ;
 pub const AF_CAIF : u32 = 37 ;
 pub const AF_ALG : u32 = 38 ;
 pub const AF_NFC : u32 = 39 ;
 pub const AF_VSOCK : u32 = 40 ;
 pub const AF_KCM : u32 = 41 ;
 pub const AF_QIPCRTR : u32 = 42 ;
 pub const AF_SMC : u32 = 43 ;
 pub const AF_MAX : u32 = 44 ;
 pub const SOL_RAW : u32 = 255 ;
 pub const SOL_DECNET : u32 = 261 ;
 pub const SOL_X25 : u32 = 262 ;
 pub const SOL_PACKET : u32 = 263 ;
 pub const SOL_ATM : u32 = 264 ;
 pub const SOL_AAL : u32 = 265 ;
 pub const SOL_IRDA : u32 = 266 ;
 pub const SOL_NETBEUI : u32 = 267 ;
 pub const SOL_LLC : u32 = 268 ;
 pub const SOL_DCCP : u32 = 269 ;
 pub const SOL_NETLINK : u32 = 270 ;
 pub const SOL_TIPC : u32 = 271 ;
 pub const SOL_RXRPC : u32 = 272 ;
 pub const SOL_PPPOL2TP : u32 = 273 ;
 pub const SOL_BLUETOOTH : u32 = 274 ;
 pub const SOL_PNPIPE : u32 = 275 ;
 pub const SOL_RDS : u32 = 276 ;
 pub const SOL_IUCV : u32 = 277 ;
 pub const SOL_CAIF : u32 = 278 ;
 pub const SOL_ALG : u32 = 279 ;
 pub const SOL_NFC : u32 = 280 ;
 pub const SOL_KCM : u32 = 281 ;
 pub const SOL_TLS : u32 = 282 ;
 pub const SOMAXCONN : u32 = 128 ;
 pub const _BITS_SOCKADDR_H : u32 = 1 ;
 pub const _SS_SIZE : u32 = 128 ;
 pub const FIOSETOWN : u32 = 35073 ;
 pub const SIOCSPGRP : u32 = 35074 ;
 pub const FIOGETOWN : u32 = 35075 ;
 pub const SIOCGPGRP : u32 = 35076 ;
 pub const SIOCATMARK : u32 = 35077 ;
 pub const SIOCGSTAMP : u32 = 35078 ;
 pub const SIOCGSTAMPNS : u32 = 35079 ;
 pub const SOL_SOCKET : u32 = 1 ;
 pub const SO_DEBUG : u32 = 1 ;
 pub const SO_REUSEADDR : u32 = 2 ;
 pub const SO_TYPE : u32 = 3 ;
 pub const SO_ERROR : u32 = 4 ;
 pub const SO_DONTROUTE : u32 = 5 ;
 pub const SO_BROADCAST : u32 = 6 ;
 pub const SO_SNDBUF : u32 = 7 ;
 pub const SO_RCVBUF : u32 = 8 ;
 pub const SO_SNDBUFFORCE : u32 = 32 ;
 pub const SO_RCVBUFFORCE : u32 = 33 ;
 pub const SO_KEEPALIVE : u32 = 9 ;
 pub const SO_OOBINLINE : u32 = 10 ;
 pub const SO_NO_CHECK : u32 = 11 ;
 pub const SO_PRIORITY : u32 = 12 ;
 pub const SO_LINGER : u32 = 13 ;
 pub const SO_BSDCOMPAT : u32 = 14 ;
 pub const SO_REUSEPORT : u32 = 15 ;
 pub const SO_PASSCRED : u32 = 16 ;
 pub const SO_PEERCRED : u32 = 17 ;
 pub const SO_RCVLOWAT : u32 = 18 ;
 pub const SO_SNDLOWAT : u32 = 19 ;
 pub const SO_RCVTIMEO : u32 = 20 ;
 pub const SO_SNDTIMEO : u32 = 21 ;
 pub const SO_SECURITY_AUTHENTICATION : u32 = 22 ;
 pub const SO_SECURITY_ENCRYPTION_TRANSPORT : u32 = 23 ;
 pub const SO_SECURITY_ENCRYPTION_NETWORK : u32 = 24 ;
 pub const SO_BINDTODEVICE : u32 = 25 ;
 pub const SO_ATTACH_FILTER : u32 = 26 ;
 pub const SO_DETACH_FILTER : u32 = 27 ;
 pub const SO_GET_FILTER : u32 = 26 ;
 pub const SO_PEERNAME : u32 = 28 ;
 pub const SO_TIMESTAMP : u32 = 29 ;
 pub const SCM_TIMESTAMP : u32 = 29 ;
 pub const SO_ACCEPTCONN : u32 = 30 ;
 pub const SO_PEERSEC : u32 = 31 ;
 pub const SO_PASSSEC : u32 = 34 ;
 pub const SO_TIMESTAMPNS : u32 = 35 ;
 pub const SCM_TIMESTAMPNS : u32 = 35 ;
 pub const SO_MARK : u32 = 36 ;
 pub const SO_TIMESTAMPING : u32 = 37 ;
 pub const SCM_TIMESTAMPING : u32 = 37 ;
 pub const SO_PROTOCOL : u32 = 38 ;
 pub const SO_DOMAIN : u32 = 39 ;
 pub const SO_RXQ_OVFL : u32 = 40 ;
 pub const SO_WIFI_STATUS : u32 = 41 ;
 pub const SCM_WIFI_STATUS : u32 = 41 ;
 pub const SO_PEEK_OFF : u32 = 42 ;
 pub const SO_NOFCS : u32 = 43 ;
 pub const SO_LOCK_FILTER : u32 = 44 ;
 pub const SO_SELECT_ERR_QUEUE : u32 = 45 ;
 pub const SO_BUSY_POLL : u32 = 46 ;
 pub const SO_MAX_PACING_RATE : u32 = 47 ;
 pub const SO_BPF_EXTENSIONS : u32 = 48 ;
 pub const SO_INCOMING_CPU : u32 = 49 ;
 pub const SO_ATTACH_BPF : u32 = 50 ;
 pub const SO_DETACH_BPF : u32 = 27 ;
 pub const SO_ATTACH_REUSEPORT_CBPF : u32 = 51 ;
 pub const SO_ATTACH_REUSEPORT_EBPF : u32 = 52 ;
 pub const SO_CNX_ADVICE : u32 = 53 ;
 pub const SCM_TIMESTAMPING_OPT_STATS : u32 = 54 ;
 pub const SO_MEMINFO : u32 = 55 ;
 pub const SO_INCOMING_NAPI_ID : u32 = 56 ;
 pub const SO_COOKIE : u32 = 57 ;
 pub const SCM_TIMESTAMPING_PKTINFO : u32 = 58 ;
 pub const SO_PEERGROUPS : u32 = 59 ;
 pub const SO_ZEROCOPY : u32 = 60 ;
 pub const __osockaddr_defined : u32 = 1 ;
 pub const _SYS_MMAN_H : u32 = 1 ;
 pub const MAP_32BIT : u32 = 64 ;
 pub const MAP_GROWSDOWN : u32 = 256 ;
 pub const MAP_DENYWRITE : u32 = 2048 ;
 pub const MAP_EXECUTABLE : u32 = 4096 ;
 pub const MAP_LOCKED : u32 = 8192 ;
 pub const MAP_NORESERVE : u32 = 16384 ;
 pub const MAP_POPULATE : u32 = 32768 ;
 pub const MAP_NONBLOCK : u32 = 65536 ;
 pub const MAP_STACK : u32 = 131072 ;
 pub const MAP_HUGETLB : u32 = 262144 ;
 pub const PROT_READ : u32 = 1 ;
 pub const PROT_WRITE : u32 = 2 ;
 pub const PROT_EXEC : u32 = 4 ;
 pub const PROT_NONE : u32 = 0 ;
 pub const PROT_GROWSDOWN : u32 = 16777216 ;
 pub const PROT_GROWSUP : u32 = 33554432 ;
 pub const MAP_SHARED : u32 = 1 ;
 pub const MAP_PRIVATE : u32 = 2 ;
 pub const MAP_TYPE : u32 = 15 ;
 pub const MAP_FIXED : u32 = 16 ;
 pub const MAP_FILE : u32 = 0 ;
 pub const MAP_ANONYMOUS : u32 = 32 ;
 pub const MAP_ANON : u32 = 32 ;
 pub const MAP_HUGE_SHIFT : u32 = 26 ;
 pub const MAP_HUGE_MASK : u32 = 63 ;
 pub const MS_ASYNC : u32 = 1 ;
 pub const MS_SYNC : u32 = 4 ;
 pub const MS_INVALIDATE : u32 = 2 ;
 pub const MREMAP_MAYMOVE : u32 = 1 ;
 pub const MREMAP_FIXED : u32 = 2 ;
 pub const MADV_NORMAL : u32 = 0 ;
 pub const MADV_RANDOM : u32 = 1 ;
 pub const MADV_SEQUENTIAL : u32 = 2 ;
 pub const MADV_WILLNEED : u32 = 3 ;
 pub const MADV_DONTNEED : u32 = 4 ;
 pub const MADV_FREE : u32 = 8 ;
 pub const MADV_REMOVE : u32 = 9 ;
 pub const MADV_DONTFORK : u32 = 10 ;
 pub const MADV_DOFORK : u32 = 11 ;
 pub const MADV_MERGEABLE : u32 = 12 ;
 pub const MADV_UNMERGEABLE : u32 = 13 ;
 pub const MADV_HUGEPAGE : u32 = 14 ;
 pub const MADV_NOHUGEPAGE : u32 = 15 ;
 pub const MADV_DONTDUMP : u32 = 16 ;
 pub const MADV_DODUMP : u32 = 17 ;
 pub const MADV_WIPEONFORK : u32 = 18 ;
 pub const MADV_KEEPONFORK : u32 = 19 ;
 pub const MADV_HWPOISON : u32 = 100 ;
 pub const POSIX_MADV_NORMAL : u32 = 0 ;
 pub const POSIX_MADV_RANDOM : u32 = 1 ;
 pub const POSIX_MADV_SEQUENTIAL : u32 = 2 ;
 pub const POSIX_MADV_WILLNEED : u32 = 3 ;
 pub const POSIX_MADV_DONTNEED : u32 = 4 ;
 pub const MCL_CURRENT : u32 = 1 ;
 pub const MCL_FUTURE : u32 = 2 ;
 pub const MCL_ONFAULT : u32 = 4 ;
 pub const MFD_CLOEXEC : u32 = 1 ;
 pub const MFD_ALLOW_SEALING : u32 = 2 ;
 pub const MFD_HUGETLB : u32 = 4 ;
 pub const MLOCK_ONFAULT : u32 = 1 ;
 pub const PKEY_DISABLE_ACCESS : u32 = 1 ;
 pub const PKEY_DISABLE_WRITE : u32 = 2 ;
 pub const _FILE_OFFSET_BITS : u32 = 64 ;
 pub const PRIV_SOCK_LOGIN : u32 = 1 ;
 pub const PRIV_SOCK_CHOWN : u32 = 2 ;
 pub const PRIV_SOCK_GET_DATA_SOCK : u32 = 3 ;
 pub const PRIV_SOCK_GET_USER_CMD : u32 = 4 ;
 pub const PRIV_SOCK_WRITE_USER_RESP : u32 = 5 ;
 pub const PRIV_SOCK_DO_SSL_HANDSHAKE : u32 = 6 ;
 pub const PRIV_SOCK_DO_SSL_CLOSE : u32 = 7 ;
 pub const PRIV_SOCK_DO_SSL_READ : u32 = 8 ;
 pub const PRIV_SOCK_DO_SSL_WRITE : u32 = 9 ;
 pub const PRIV_SOCK_PASV_CLEANUP : u32 = 10 ;
 pub const PRIV_SOCK_PASV_ACTIVE : u32 = 11 ;
 pub const PRIV_SOCK_PASV_LISTEN : u32 = 12 ;
 pub const PRIV_SOCK_PASV_ACCEPT : u32 = 13 ;
 pub const PRIV_SOCK_RESULT_OK : u32 = 1 ;
 pub const PRIV_SOCK_RESULT_BAD : u32 = 2 ;
 pub const PTRACE_SANDBOX_ERR_DEAD : i32 = - 1 ;
 pub const PTRACE_SANDBOX_ERR_PTRACE : i32 = - 2 ;
 pub const PTRACE_SANDBOX_ERR_WAITPID : i32 = - 3 ;
 pub const PTRACE_SANDBOX_ERR_WAIT_STATUS : i32 = - 4 ;
 pub const PTRACE_SANDBOX_ERR_POLICY_SYSCALL : i32 = - 5 ;
 pub const PTRACE_SANDBOX_ERR_BAD_SYSCALL : i32 = - 6 ;
 pub const PTRACE_SANDBOX_ERR_POLICY_ARGS : i32 = - 7 ;
 pub const PTRACE_SANDBOX_ERR_API_ABUSE_STOPIT : i32 = - 8 ;
 pub const VSF_SECUTIL_OPTION_CHROOT : u32 = 1 ;
 pub const VSF_SECUTIL_OPTION_USE_GROUPS : u32 = 2 ;
 pub const VSF_SECUTIL_OPTION_CHANGE_EUID : u32 = 4 ;
 pub const VSF_SECUTIL_OPTION_NO_FDS : u32 = 8 ;
 pub const VSF_SECUTIL_OPTION_NO_PROCS : u32 = 16 ;
 pub const VSF_SECUTIL_OPTION_ALLOW_WRITEABLE_ROOT : u32 = 32 ;
 pub const VSF_VERSION : & 'static [u8 ; 6usize] = b"3.0.3\0" ;
 extern "C" {
# [link_name = "\u{1}_Z21vsf_access_check_filePK5mystr"] pub fn vsf_access_check_file (p_filename_str : * const mystr) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z29vsf_access_check_file_visiblePK5mystr"] pub fn vsf_access_check_file_visible (p_filename_str : * const mystr) -> :: std :: os :: raw :: c_int ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct ascii_to_bin_ret {
pub stored : :: std :: os :: raw :: c_uint , pub last_was_cr : :: std :: os :: raw :: c_int , pub p_buf : * mut :: std :: os :: raw :: c_char ,
}
 # [test] fn bindgen_test_layout_ascii_to_bin_ret () {
assert_eq ! (:: std :: mem :: size_of :: < ascii_to_bin_ret > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( ascii_to_bin_ret ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < ascii_to_bin_ret > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( ascii_to_bin_ret ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ascii_to_bin_ret > ( ) ) ) . stored as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( ascii_to_bin_ret ) , "::" , stringify ! ( stored ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ascii_to_bin_ret > ( ) ) ) . last_was_cr as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( ascii_to_bin_ret ) , "::" , stringify ! ( last_was_cr ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ascii_to_bin_ret > ( ) ) ) . p_buf as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( ascii_to_bin_ret ) , "::" , stringify ! ( p_buf ) )) ;

}
 extern "C" {
# [link_name = "\u{1}_Z22vsf_ascii_ascii_to_binPcji"] pub fn vsf_ascii_ascii_to_bin (p_in : * mut :: std :: os :: raw :: c_char , in_len : :: std :: os :: raw :: c_uint , prev_cr : :: std :: os :: raw :: c_int) -> ascii_to_bin_ret ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct bin_to_ascii_ret {
pub stored : :: std :: os :: raw :: c_uint , pub last_was_cr : :: std :: os :: raw :: c_int ,
}
 # [test] fn bindgen_test_layout_bin_to_ascii_ret () {
assert_eq ! (:: std :: mem :: size_of :: < bin_to_ascii_ret > ( ) , 8usize , concat ! ( "Size of: " , stringify ! ( bin_to_ascii_ret ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < bin_to_ascii_ret > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( bin_to_ascii_ret ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < bin_to_ascii_ret > ( ) ) ) . stored as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( bin_to_ascii_ret ) , "::" , stringify ! ( stored ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < bin_to_ascii_ret > ( ) ) ) . last_was_cr as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( bin_to_ascii_ret ) , "::" , stringify ! ( last_was_cr ) )) ;

}
 extern "C" {
# [link_name = "\u{1}_Z22vsf_ascii_bin_to_asciiPKcPcji"] pub fn vsf_ascii_bin_to_ascii (p_in : * const :: std :: os :: raw :: c_char , p_out : * mut :: std :: os :: raw :: c_char , in_len : :: std :: os :: raw :: c_uint , prev_cr : :: std :: os :: raw :: c_int) -> bin_to_ascii_ret ;

}
 extern "C" {
# [link_name = "\u{1}_Z22vsf_banner_dir_changedP11vsf_sessioni"] pub fn vsf_banner_dir_changed (p_sess : * mut vsf_session , ftpcode : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
# [link_name = "\u{1}_Z16vsf_banner_writeP11vsf_sessionP5mystri"] pub fn vsf_banner_write (p_sess : * mut vsf_session , p_str : * mut mystr , ftpcode : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
# [link_name = "\u{1}_Z5cryptPKcS0_"] pub fn crypt (arg1 : * const :: std :: os :: raw :: c_char , arg2 : * const :: std :: os :: raw :: c_char) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
# [link_name = "\u{1}_Z11handle_featP11vsf_session"] pub fn handle_feat (p_sess : * mut vsf_session) ;

}
 pub type filesize_t = :: std :: os :: raw :: c_longlong ;
 extern "C" {
# [link_name = "\u{1}_Z12str_filereadP5mystrPKcj"] pub fn str_fileread (p_str : * mut mystr , p_filename : * const :: std :: os :: raw :: c_char , maxsize : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z20vsf_cmdio_sock_setupv"] pub fn vsf_cmdio_sock_setup () ;

}
 extern "C" {
# [link_name = "\u{1}_Z15vsf_cmdio_writeP11vsf_sessioniPKc"] pub fn vsf_cmdio_write (p_sess : * mut vsf_session , status : :: std :: os :: raw :: c_int , p_text : * const :: std :: os :: raw :: c_char) ;

}
 extern "C" {
# [link_name = "\u{1}_Z22vsf_cmdio_write_hyphenP11vsf_sessioniPKc"] pub fn vsf_cmdio_write_hyphen (p_sess : * mut vsf_session , status : :: std :: os :: raw :: c_int , p_text : * const :: std :: os :: raw :: c_char) ;

}
 extern "C" {
# [link_name = "\u{1}_Z19vsf_cmdio_write_rawP11vsf_sessionPKc"] pub fn vsf_cmdio_write_raw (p_sess : * mut vsf_session , p_text : * const :: std :: os :: raw :: c_char) ;

}
 extern "C" {
# [link_name = "\u{1}_Z20vsf_cmdio_write_exitP11vsf_sessioniPKci"] pub fn vsf_cmdio_write_exit (p_sess : * mut vsf_session , status : :: std :: os :: raw :: c_int , p_text : * const :: std :: os :: raw :: c_char , exit_val : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
# [link_name = "\u{1}_Z19vsf_cmdio_write_strP11vsf_sessioniPK5mystr"] pub fn vsf_cmdio_write_str (p_sess : * mut vsf_session , status : :: std :: os :: raw :: c_int , p_str : * const mystr) ;

}
 extern "C" {
# [link_name = "\u{1}_Z26vsf_cmdio_write_str_hyphenP11vsf_sessioniPK5mystr"] pub fn vsf_cmdio_write_str_hyphen (p_sess : * mut vsf_session , status : :: std :: os :: raw :: c_int , p_str : * const mystr) ;

}
 extern "C" {
# [link_name = "\u{1}_Z19vsf_cmdio_set_alarmP11vsf_session"] pub fn vsf_cmdio_set_alarm (p_sess : * mut vsf_session) ;

}
 extern "C" {
# [link_name = "\u{1}_Z25vsf_cmdio_get_cmd_and_argP11vsf_sessionP5mystrS2_i"] pub fn vsf_cmdio_get_cmd_and_arg (p_sess : * mut vsf_session , p_cmd_str : * mut mystr , p_arg_str : * mut mystr , set_alarm : :: std :: os :: raw :: c_int) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct vsf_sysutil_sockaddr {
_unused : [u8 ; 0] ,
}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct vsf_sysutil_dir {
_unused : [u8 ; 0] ,
}
 extern "C" {
# [link_name = "\u{1}_Z33vsf_ftpdataio_dispose_transfer_fdP11vsf_session"] pub fn vsf_ftpdataio_dispose_transfer_fd (p_sess : * mut vsf_session) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z25vsf_ftpdataio_get_pasv_fdP11vsf_session"] pub fn vsf_ftpdataio_get_pasv_fd (p_sess : * mut vsf_session) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z25vsf_ftpdataio_get_port_fdP11vsf_session"] pub fn vsf_ftpdataio_get_port_fd (p_sess : * mut vsf_session) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z31vsf_ftpdataio_post_mark_connectP11vsf_session"] pub fn vsf_ftpdataio_post_mark_connect (p_sess : * mut vsf_session) -> :: std :: os :: raw :: c_int ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct vsf_transfer_ret {
pub retval : :: std :: os :: raw :: c_int , pub transferred : filesize_t ,
}
 # [test] fn bindgen_test_layout_vsf_transfer_ret () {
assert_eq ! (:: std :: mem :: size_of :: < vsf_transfer_ret > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( vsf_transfer_ret ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < vsf_transfer_ret > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( vsf_transfer_ret ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_transfer_ret > ( ) ) ) . retval as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( vsf_transfer_ret ) , "::" , stringify ! ( retval ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_transfer_ret > ( ) ) ) . transferred as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( vsf_transfer_ret ) , "::" , stringify ! ( transferred ) )) ;

}
 extern "C" {
# [link_name = "\u{1}_Z27vsf_ftpdataio_transfer_fileP11vsf_sessioniiii"] pub fn vsf_ftpdataio_transfer_file (p_sess : * mut vsf_session , remote_fd : :: std :: os :: raw :: c_int , file_fd : :: std :: os :: raw :: c_int , is_recv : :: std :: os :: raw :: c_int , is_ascii : :: std :: os :: raw :: c_int) -> vsf_transfer_ret ;

}
 extern "C" {
# [link_name = "\u{1}_Z26vsf_ftpdataio_transfer_dirP11vsf_sessioniP15vsf_sysutil_dirPK5mystrS5_S5_i"] pub fn vsf_ftpdataio_transfer_dir (p_sess : * mut vsf_session , is_control : :: std :: os :: raw :: c_int , p_dir : * mut vsf_sysutil_dir , p_base_dir_str : * const mystr , p_option_str : * const mystr , p_filter_str : * const mystr , is_verbose : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct pt_sandbox {
_unused : [u8 ; 0] ,
}
 extern "C" {
# [link_name = "\u{1}_Z12policy_setupP10pt_sandboxPK11vsf_session"] pub fn policy_setup (p_sandbox : * mut pt_sandbox , p_sess : * const vsf_session) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct hash {
_unused : [u8 ; 0] ,
}
 pub type hashfunc_t = :: std :: option :: Option < unsafe extern "C" fn (arg1 : :: std :: os :: raw :: c_uint , arg2 : * mut :: std :: os :: raw :: c_void) -> :: std :: os :: raw :: c_uint > ;
 extern "C" {
# [link_name = "\u{1}_Z10hash_allocjjjPFjjPvE"] pub fn hash_alloc (buckets : :: std :: os :: raw :: c_uint , key_size : :: std :: os :: raw :: c_uint , value_size : :: std :: os :: raw :: c_uint , hash_func : hashfunc_t) -> * mut hash ;

}
 extern "C" {
# [link_name = "\u{1}_Z17hash_lookup_entryP4hashPv"] pub fn hash_lookup_entry (p_hash : * mut hash , p_key : * mut :: std :: os :: raw :: c_void) -> * mut :: std :: os :: raw :: c_void ;

}
 extern "C" {
# [link_name = "\u{1}_Z14hash_add_entryP4hashPvS1_"] pub fn hash_add_entry (p_hash : * mut hash , p_key : * mut :: std :: os :: raw :: c_void , p_value : * mut :: std :: os :: raw :: c_void) ;

}
 extern "C" {
# [link_name = "\u{1}_Z15hash_free_entryP4hashPv"] pub fn hash_free_entry (p_hash : * mut hash , p_key : * mut :: std :: os :: raw :: c_void) ;

}
 extern "C" {
# [link_name = "\u{1}_Z22vsf_sysutil_parse_ipv6PK5mystr"] pub fn vsf_sysutil_parse_ipv6 (p_str : * const mystr) -> * const :: std :: os :: raw :: c_uchar ;

}
 extern "C" {
# [link_name = "\u{1}_Z22vsf_sysutil_parse_ipv4PK5mystr"] pub fn vsf_sysutil_parse_ipv4 (p_str : * const mystr) -> * const :: std :: os :: raw :: c_uchar ;

}
 extern "C" {
# [link_name = "\u{1}_Z34vsf_sysutil_parse_uchar_string_sepPK5mystrcPhj"] pub fn vsf_sysutil_parse_uchar_string_sep (p_str : * const mystr , sep : :: std :: os :: raw :: c_char , p_items : * mut :: std :: os :: raw :: c_uchar , items : :: std :: os :: raw :: c_uint) -> * const :: std :: os :: raw :: c_uchar ;

}
 pub const EVSFLogEntryType_kVSFLogEntryNull : EVSFLogEntryType = 1 ;
 pub const EVSFLogEntryType_kVSFLogEntryDownload : EVSFLogEntryType = 2 ;
 pub const EVSFLogEntryType_kVSFLogEntryUpload : EVSFLogEntryType = 3 ;
 pub const EVSFLogEntryType_kVSFLogEntryMkdir : EVSFLogEntryType = 4 ;
 pub const EVSFLogEntryType_kVSFLogEntryLogin : EVSFLogEntryType = 5 ;
 pub const EVSFLogEntryType_kVSFLogEntryFTPInput : EVSFLogEntryType = 6 ;
 pub const EVSFLogEntryType_kVSFLogEntryFTPOutput : EVSFLogEntryType = 7 ;
 pub const EVSFLogEntryType_kVSFLogEntryConnection : EVSFLogEntryType = 8 ;
 pub const EVSFLogEntryType_kVSFLogEntryDelete : EVSFLogEntryType = 9 ;
 pub const EVSFLogEntryType_kVSFLogEntryRename : EVSFLogEntryType = 10 ;
 pub const EVSFLogEntryType_kVSFLogEntryRmdir : EVSFLogEntryType = 11 ;
 pub const EVSFLogEntryType_kVSFLogEntryChmod : EVSFLogEntryType = 12 ;
 pub const EVSFLogEntryType_kVSFLogEntryDebug : EVSFLogEntryType = 13 ;
 pub type EVSFLogEntryType = u32 ;
 extern "C" {
# [link_name = "\u{1}_Z12vsf_log_initP11vsf_session"] pub fn vsf_log_init (p_sess : * mut vsf_session) ;

}
 extern "C" {
# [link_name = "\u{1}_Z19vsf_log_start_entryP11vsf_session16EVSFLogEntryType"] pub fn vsf_log_start_entry (p_sess : * mut vsf_session , what : EVSFLogEntryType) ;

}
 extern "C" {
# [link_name = "\u{1}_Z21vsf_log_entry_pendingP11vsf_session"] pub fn vsf_log_entry_pending (p_sess : * mut vsf_session) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z19vsf_log_clear_entryP11vsf_session"] pub fn vsf_log_clear_entry (p_sess : * mut vsf_session) ;

}
 extern "C" {
# [link_name = "\u{1}_Z14vsf_log_do_logP11vsf_sessioni"] pub fn vsf_log_do_log (p_sess : * mut vsf_session , succeeded : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
# [link_name = "\u{1}_Z12vsf_log_lineP11vsf_session16EVSFLogEntryTypeP5mystr"] pub fn vsf_log_line (p_sess : * mut vsf_session , what : EVSFLogEntryType , p_str : * mut mystr) ;

}
 extern "C" {
# [link_name = "\u{1}_Z24vsf_ls_populate_dir_listP10mystr_listS0_P15vsf_sysutil_dirPK5mystrS5_S5_i"] pub fn vsf_ls_populate_dir_list (p_list : * mut mystr_list , p_subdir_list : * mut mystr_list , p_dir : * mut vsf_sysutil_dir , p_base_dir_str : * const mystr , p_option_str : * const mystr , p_filter_str : * const mystr , is_verbose : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
# [link_name = "\u{1}_Z26vsf_filename_passes_filterPK5mystrS1_Pj"] pub fn vsf_filename_passes_filter (p_filename_str : * const mystr , p_filter_str : * const mystr , iters : * mut :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 pub type str_netfd_read_t = :: std :: option :: Option < unsafe extern "C" fn (p_sess : * mut vsf_session , arg1 : * mut :: std :: os :: raw :: c_char , arg2 : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int > ;
 extern "C" {
# [link_name = "\u{1}_Z15str_netfd_allocP11vsf_sessionP5mystrcPcjPFiS0_S3_jES5_"] pub fn str_netfd_alloc (p_sess : * mut vsf_session , p_str : * mut mystr , term : :: std :: os :: raw :: c_char , p_readbuf : * mut :: std :: os :: raw :: c_char , maxlen : :: std :: os :: raw :: c_uint , p_peekfunc : str_netfd_read_t , p_readfunc : str_netfd_read_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z14str_netfd_readP5mystrij"] pub fn str_netfd_read (p_str : * mut mystr , fd : :: std :: os :: raw :: c_int , len : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z15str_netfd_writePK5mystri"] pub fn str_netfd_write (p_str : * const mystr , fd : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z21vsf_one_process_startP11vsf_session"] pub fn vsf_one_process_start (p_sess : * mut vsf_session) ;

}
 extern "C" {
# [link_name = "\u{1}_Z21vsf_one_process_loginP11vsf_sessionPK5mystr"] pub fn vsf_one_process_login (p_sess : * mut vsf_session , p_pass_str : * const mystr) ;

}
 extern "C" {
# [link_name = "\u{1}_Z34vsf_one_process_get_priv_data_sockP11vsf_session"] pub fn vsf_one_process_get_priv_data_sock (p_sess : * mut vsf_session) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z28vsf_one_process_pasv_cleanupP11vsf_session"] pub fn vsf_one_process_pasv_cleanup (p_sess : * mut vsf_session) ;

}
 extern "C" {
# [link_name = "\u{1}_Z27vsf_one_process_pasv_activeP11vsf_session"] pub fn vsf_one_process_pasv_active (p_sess : * mut vsf_session) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z22vsf_one_process_listenP11vsf_session"] pub fn vsf_one_process_listen (p_sess : * mut vsf_session) -> :: std :: os :: raw :: c_ushort ;

}
 extern "C" {
# [link_name = "\u{1}_Z27vsf_one_process_get_pasv_fdP11vsf_session"] pub fn vsf_one_process_get_pasv_fd (p_sess : * mut vsf_session) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z28vsf_one_process_chown_uploadP11vsf_sessioni"] pub fn vsf_one_process_chown_upload (p_sess : * mut vsf_session , fd : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
# [link_name = "\u{1}_Z11handle_optsP11vsf_session"] pub fn handle_opts (p_sess : * mut vsf_session) ;

}
 extern "C" {
# [link_name = "\u{1}_Z23vsf_parseconf_load_filePKci"] pub fn vsf_parseconf_load_file (p_filename : * const :: std :: os :: raw :: c_char , errs_fatal : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
# [link_name = "\u{1}_Z26vsf_parseconf_load_settingPKci"] pub fn vsf_parseconf_load_setting (p_setting : * const :: std :: os :: raw :: c_char , errs_fatal : :: std :: os :: raw :: c_int) ;

}
 pub type __u_char = :: std :: os :: raw :: c_uchar ;
 pub type __u_short = :: std :: os :: raw :: c_ushort ;
 pub type __u_int = :: std :: os :: raw :: c_uint ;
 pub type __u_long = :: std :: os :: raw :: c_ulong ;
 pub type __int8_t = :: std :: os :: raw :: c_schar ;
 pub type __uint8_t = :: std :: os :: raw :: c_uchar ;
 pub type __int16_t = :: std :: os :: raw :: c_short ;
 pub type __uint16_t = :: std :: os :: raw :: c_ushort ;
 pub type __int32_t = :: std :: os :: raw :: c_int ;
 pub type __uint32_t = :: std :: os :: raw :: c_uint ;
 pub type __int64_t = :: std :: os :: raw :: c_long ;
 pub type __uint64_t = :: std :: os :: raw :: c_ulong ;
 pub type __quad_t = :: std :: os :: raw :: c_long ;
 pub type __u_quad_t = :: std :: os :: raw :: c_ulong ;
 pub type __intmax_t = :: std :: os :: raw :: c_long ;
 pub type __uintmax_t = :: std :: os :: raw :: c_ulong ;
 pub type __dev_t = :: std :: os :: raw :: c_ulong ;
 pub type __uid_t = :: std :: os :: raw :: c_uint ;
 pub type __gid_t = :: std :: os :: raw :: c_uint ;
 pub type __ino_t = :: std :: os :: raw :: c_ulong ;
 pub type __ino64_t = :: std :: os :: raw :: c_ulong ;
 pub type __mode_t = :: std :: os :: raw :: c_uint ;
 pub type __nlink_t = :: std :: os :: raw :: c_ulong ;
 pub type __off_t = :: std :: os :: raw :: c_long ;
 pub type __off64_t = :: std :: os :: raw :: c_long ;
 pub type __pid_t = :: std :: os :: raw :: c_int ;
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct __fsid_t {
pub __val : [:: std :: os :: raw :: c_int ; 2usize] ,
}
 # [test] fn bindgen_test_layout___fsid_t () {
assert_eq ! (:: std :: mem :: size_of :: < __fsid_t > ( ) , 8usize , concat ! ( "Size of: " , stringify ! ( __fsid_t ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < __fsid_t > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( __fsid_t ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __fsid_t > ( ) ) ) . __val as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( __fsid_t ) , "::" , stringify ! ( __val ) )) ;

}
 pub type __clock_t = :: std :: os :: raw :: c_long ;
 pub type __rlim_t = :: std :: os :: raw :: c_ulong ;
 pub type __rlim64_t = :: std :: os :: raw :: c_ulong ;
 pub type __id_t = :: std :: os :: raw :: c_uint ;
 pub type __time_t = :: std :: os :: raw :: c_long ;
 pub type __useconds_t = :: std :: os :: raw :: c_uint ;
 pub type __suseconds_t = :: std :: os :: raw :: c_long ;
 pub type __daddr_t = :: std :: os :: raw :: c_int ;
 pub type __key_t = :: std :: os :: raw :: c_int ;
 pub type __clockid_t = :: std :: os :: raw :: c_int ;
 pub type __timer_t = * mut :: std :: os :: raw :: c_void ;
 pub type __blksize_t = :: std :: os :: raw :: c_long ;
 pub type __blkcnt_t = :: std :: os :: raw :: c_long ;
 pub type __blkcnt64_t = :: std :: os :: raw :: c_long ;
 pub type __fsblkcnt_t = :: std :: os :: raw :: c_ulong ;
 pub type __fsblkcnt64_t = :: std :: os :: raw :: c_ulong ;
 pub type __fsfilcnt_t = :: std :: os :: raw :: c_ulong ;
 pub type __fsfilcnt64_t = :: std :: os :: raw :: c_ulong ;
 pub type __fsword_t = :: std :: os :: raw :: c_long ;
 pub type __ssize_t = :: std :: os :: raw :: c_long ;
 pub type __syscall_slong_t = :: std :: os :: raw :: c_long ;
 pub type __syscall_ulong_t = :: std :: os :: raw :: c_ulong ;
 pub type __loff_t = __off64_t ;
 pub type __caddr_t = * mut :: std :: os :: raw :: c_char ;
 pub type __intptr_t = :: std :: os :: raw :: c_long ;
 pub type __socklen_t = :: std :: os :: raw :: c_uint ;
 pub type __sig_atomic_t = :: std :: os :: raw :: c_int ;
 pub type u_char = __u_char ;
 pub type u_short = __u_short ;
 pub type u_int = __u_int ;
 pub type u_long = __u_long ;
 pub type quad_t = __quad_t ;
 pub type u_quad_t = __u_quad_t ;
 pub type fsid_t = __fsid_t ;
 pub type loff_t = __loff_t ;
 pub type ino_t = __ino_t ;
 pub type ino64_t = __ino64_t ;
 pub type dev_t = __dev_t ;
 pub type gid_t = __gid_t ;
 pub type mode_t = __mode_t ;
 pub type nlink_t = __nlink_t ;
 pub type uid_t = __uid_t ;
 pub type off_t = __off_t ;
 pub type off64_t = __off64_t ;
 pub type pid_t = __pid_t ;
 pub type id_t = __id_t ;
 pub type daddr_t = __daddr_t ;
 pub type caddr_t = __caddr_t ;
 pub type key_t = __key_t ;
 pub type clock_t = __clock_t ;
 pub type clockid_t = __clockid_t ;
 pub type time_t = __time_t ;
 pub type timer_t = __timer_t ;
 pub type useconds_t = __useconds_t ;
 pub type suseconds_t = __suseconds_t ;
 pub type ulong = :: std :: os :: raw :: c_ulong ;
 pub type ushort = :: std :: os :: raw :: c_ushort ;
 pub type uint = :: std :: os :: raw :: c_uint ;
 pub type u_int8_t = :: std :: os :: raw :: c_uchar ;
 pub type u_int16_t = :: std :: os :: raw :: c_ushort ;
 pub type u_int32_t = :: std :: os :: raw :: c_uint ;
 pub type u_int64_t = :: std :: os :: raw :: c_ulong ;
 pub type register_t = :: std :: os :: raw :: c_long ;
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct __sigset_t {
pub __val : [:: std :: os :: raw :: c_ulong ; 16usize] ,
}
 # [test] fn bindgen_test_layout___sigset_t () {
assert_eq ! (:: std :: mem :: size_of :: < __sigset_t > ( ) , 128usize , concat ! ( "Size of: " , stringify ! ( __sigset_t ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < __sigset_t > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( __sigset_t ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __sigset_t > ( ) ) ) . __val as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( __sigset_t ) , "::" , stringify ! ( __val ) )) ;

}
 pub type sigset_t = __sigset_t ;
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct timeval {
pub tv_sec : __time_t , pub tv_usec : __suseconds_t ,
}
 # [test] fn bindgen_test_layout_timeval () {
assert_eq ! (:: std :: mem :: size_of :: < timeval > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( timeval ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < timeval > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( timeval ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < timeval > ( ) ) ) . tv_sec as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( timeval ) , "::" , stringify ! ( tv_sec ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < timeval > ( ) ) ) . tv_usec as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( timeval ) , "::" , stringify ! ( tv_usec ) )) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct timespec {
pub tv_sec : __time_t , pub tv_nsec : __syscall_slong_t ,
}
 # [test] fn bindgen_test_layout_timespec () {
assert_eq ! (:: std :: mem :: size_of :: < timespec > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( timespec ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < timespec > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( timespec ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < timespec > ( ) ) ) . tv_sec as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( timespec ) , "::" , stringify ! ( tv_sec ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < timespec > ( ) ) ) . tv_nsec as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( timespec ) , "::" , stringify ! ( tv_nsec ) )) ;

}
 pub type __fd_mask = :: std :: os :: raw :: c_long ;
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct fd_set {
pub fds_bits : [__fd_mask ; 16usize] ,
}
 # [test] fn bindgen_test_layout_fd_set () {
assert_eq ! (:: std :: mem :: size_of :: < fd_set > ( ) , 128usize , concat ! ( "Size of: " , stringify ! ( fd_set ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < fd_set > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( fd_set ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < fd_set > ( ) ) ) . fds_bits as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( fd_set ) , "::" , stringify ! ( fds_bits ) )) ;

}
 pub type fd_mask = __fd_mask ;
 extern "C" {
pub fn select (__nfds : :: std :: os :: raw :: c_int , __readfds : * mut fd_set , __writefds : * mut fd_set , __exceptfds : * mut fd_set , __timeout : * mut timeval) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn pselect (__nfds : :: std :: os :: raw :: c_int , __readfds : * mut fd_set , __writefds : * mut fd_set , __exceptfds : * mut fd_set , __timeout : * const timespec , __sigmask : * const __sigset_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn gnu_dev_major (__dev : __dev_t) -> :: std :: os :: raw :: c_uint ;

}
 extern "C" {
pub fn gnu_dev_minor (__dev : __dev_t) -> :: std :: os :: raw :: c_uint ;

}
 extern "C" {
pub fn gnu_dev_makedev (__major : :: std :: os :: raw :: c_uint , __minor : :: std :: os :: raw :: c_uint) -> __dev_t ;

}
 pub type blksize_t = __blksize_t ;
 pub type blkcnt_t = __blkcnt_t ;
 pub type fsblkcnt_t = __fsblkcnt_t ;
 pub type fsfilcnt_t = __fsfilcnt_t ;
 pub type blkcnt64_t = __blkcnt64_t ;
 pub type fsblkcnt64_t = __fsblkcnt64_t ;
 pub type fsfilcnt64_t = __fsfilcnt64_t ;
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct __pthread_rwlock_arch_t {
pub __readers : :: std :: os :: raw :: c_uint , pub __writers : :: std :: os :: raw :: c_uint , pub __wrphase_futex : :: std :: os :: raw :: c_uint , pub __writers_futex : :: std :: os :: raw :: c_uint , pub __pad3 : :: std :: os :: raw :: c_uint , pub __pad4 : :: std :: os :: raw :: c_uint , pub __cur_writer : :: std :: os :: raw :: c_int , pub __shared : :: std :: os :: raw :: c_int , pub __rwelision : :: std :: os :: raw :: c_schar , pub __pad1 : [:: std :: os :: raw :: c_uchar ; 7usize] , pub __pad2 : :: std :: os :: raw :: c_ulong , pub __flags : :: std :: os :: raw :: c_uint ,
}
 # [test] fn bindgen_test_layout___pthread_rwlock_arch_t () {
assert_eq ! (:: std :: mem :: size_of :: < __pthread_rwlock_arch_t > ( ) , 56usize , concat ! ( "Size of: " , stringify ! ( __pthread_rwlock_arch_t ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < __pthread_rwlock_arch_t > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( __pthread_rwlock_arch_t ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_rwlock_arch_t > ( ) ) ) . __readers as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_rwlock_arch_t ) , "::" , stringify ! ( __readers ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_rwlock_arch_t > ( ) ) ) . __writers as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_rwlock_arch_t ) , "::" , stringify ! ( __writers ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_rwlock_arch_t > ( ) ) ) . __wrphase_futex as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_rwlock_arch_t ) , "::" , stringify ! ( __wrphase_futex ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_rwlock_arch_t > ( ) ) ) . __writers_futex as * const _ as usize } , 12usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_rwlock_arch_t ) , "::" , stringify ! ( __writers_futex ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_rwlock_arch_t > ( ) ) ) . __pad3 as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_rwlock_arch_t ) , "::" , stringify ! ( __pad3 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_rwlock_arch_t > ( ) ) ) . __pad4 as * const _ as usize } , 20usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_rwlock_arch_t ) , "::" , stringify ! ( __pad4 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_rwlock_arch_t > ( ) ) ) . __cur_writer as * const _ as usize } , 24usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_rwlock_arch_t ) , "::" , stringify ! ( __cur_writer ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_rwlock_arch_t > ( ) ) ) . __shared as * const _ as usize } , 28usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_rwlock_arch_t ) , "::" , stringify ! ( __shared ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_rwlock_arch_t > ( ) ) ) . __rwelision as * const _ as usize } , 32usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_rwlock_arch_t ) , "::" , stringify ! ( __rwelision ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_rwlock_arch_t > ( ) ) ) . __pad1 as * const _ as usize } , 33usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_rwlock_arch_t ) , "::" , stringify ! ( __pad1 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_rwlock_arch_t > ( ) ) ) . __pad2 as * const _ as usize } , 40usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_rwlock_arch_t ) , "::" , stringify ! ( __pad2 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_rwlock_arch_t > ( ) ) ) . __flags as * const _ as usize } , 48usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_rwlock_arch_t ) , "::" , stringify ! ( __flags ) )) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct __pthread_internal_list {
pub __prev : * mut __pthread_internal_list , pub __next : * mut __pthread_internal_list ,
}
 # [test] fn bindgen_test_layout___pthread_internal_list () {
assert_eq ! (:: std :: mem :: size_of :: < __pthread_internal_list > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( __pthread_internal_list ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < __pthread_internal_list > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( __pthread_internal_list ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_internal_list > ( ) ) ) . __prev as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_internal_list ) , "::" , stringify ! ( __prev ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_internal_list > ( ) ) ) . __next as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_internal_list ) , "::" , stringify ! ( __next ) )) ;

}
 pub type __pthread_list_t = __pthread_internal_list ;
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct __pthread_mutex_s {
pub __lock : :: std :: os :: raw :: c_int , pub __count : :: std :: os :: raw :: c_uint , pub __owner : :: std :: os :: raw :: c_int , pub __nusers : :: std :: os :: raw :: c_uint , pub __kind : :: std :: os :: raw :: c_int , pub __spins : :: std :: os :: raw :: c_short , pub __elision : :: std :: os :: raw :: c_short , pub __list : __pthread_list_t ,
}
 # [test] fn bindgen_test_layout___pthread_mutex_s () {
assert_eq ! (:: std :: mem :: size_of :: < __pthread_mutex_s > ( ) , 40usize , concat ! ( "Size of: " , stringify ! ( __pthread_mutex_s ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < __pthread_mutex_s > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( __pthread_mutex_s ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_mutex_s > ( ) ) ) . __lock as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_mutex_s ) , "::" , stringify ! ( __lock ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_mutex_s > ( ) ) ) . __count as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_mutex_s ) , "::" , stringify ! ( __count ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_mutex_s > ( ) ) ) . __owner as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_mutex_s ) , "::" , stringify ! ( __owner ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_mutex_s > ( ) ) ) . __nusers as * const _ as usize } , 12usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_mutex_s ) , "::" , stringify ! ( __nusers ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_mutex_s > ( ) ) ) . __kind as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_mutex_s ) , "::" , stringify ! ( __kind ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_mutex_s > ( ) ) ) . __spins as * const _ as usize } , 20usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_mutex_s ) , "::" , stringify ! ( __spins ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_mutex_s > ( ) ) ) . __elision as * const _ as usize } , 22usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_mutex_s ) , "::" , stringify ! ( __elision ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_mutex_s > ( ) ) ) . __list as * const _ as usize } , 24usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_mutex_s ) , "::" , stringify ! ( __list ) )) ;

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub struct __pthread_cond_s {
pub __bindgen_anon_1 : __pthread_cond_s__bindgen_ty_1 , pub __bindgen_anon_2 : __pthread_cond_s__bindgen_ty_2 , pub __g_refs : [:: std :: os :: raw :: c_uint ; 2usize] , pub __g_size : [:: std :: os :: raw :: c_uint ; 2usize] , pub __g1_orig_size : :: std :: os :: raw :: c_uint , pub __wrefs : :: std :: os :: raw :: c_uint , pub __g_signals : [:: std :: os :: raw :: c_uint ; 2usize] ,
}
 # [repr ( C )] # [derive ( Copy , Clone )] pub union __pthread_cond_s__bindgen_ty_1 {
pub __wseq : :: std :: os :: raw :: c_ulonglong , pub __wseq32 : __pthread_cond_s__bindgen_ty_1__bindgen_ty_1 , _bindgen_union_align : u64 ,
}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct __pthread_cond_s__bindgen_ty_1__bindgen_ty_1 {
pub __low : :: std :: os :: raw :: c_uint , pub __high : :: std :: os :: raw :: c_uint ,
}
 # [test] fn bindgen_test_layout___pthread_cond_s__bindgen_ty_1__bindgen_ty_1 () {
assert_eq ! (:: std :: mem :: size_of :: < __pthread_cond_s__bindgen_ty_1__bindgen_ty_1 > ( ) , 8usize , concat ! ( "Size of: " , stringify ! ( __pthread_cond_s__bindgen_ty_1__bindgen_ty_1 ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < __pthread_cond_s__bindgen_ty_1__bindgen_ty_1 > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( __pthread_cond_s__bindgen_ty_1__bindgen_ty_1 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_cond_s__bindgen_ty_1__bindgen_ty_1 > ( ) ) ) . __low as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_cond_s__bindgen_ty_1__bindgen_ty_1 ) , "::" , stringify ! ( __low ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_cond_s__bindgen_ty_1__bindgen_ty_1 > ( ) ) ) . __high as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_cond_s__bindgen_ty_1__bindgen_ty_1 ) , "::" , stringify ! ( __high ) )) ;

}
 # [test] fn bindgen_test_layout___pthread_cond_s__bindgen_ty_1 () {
assert_eq ! (:: std :: mem :: size_of :: < __pthread_cond_s__bindgen_ty_1 > ( ) , 8usize , concat ! ( "Size of: " , stringify ! ( __pthread_cond_s__bindgen_ty_1 ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < __pthread_cond_s__bindgen_ty_1 > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( __pthread_cond_s__bindgen_ty_1 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_cond_s__bindgen_ty_1 > ( ) ) ) . __wseq as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_cond_s__bindgen_ty_1 ) , "::" , stringify ! ( __wseq ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_cond_s__bindgen_ty_1 > ( ) ) ) . __wseq32 as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_cond_s__bindgen_ty_1 ) , "::" , stringify ! ( __wseq32 ) )) ;

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub union __pthread_cond_s__bindgen_ty_2 {
pub __g1_start : :: std :: os :: raw :: c_ulonglong , pub __g1_start32 : __pthread_cond_s__bindgen_ty_2__bindgen_ty_1 , _bindgen_union_align : u64 ,
}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct __pthread_cond_s__bindgen_ty_2__bindgen_ty_1 {
pub __low : :: std :: os :: raw :: c_uint , pub __high : :: std :: os :: raw :: c_uint ,
}
 # [test] fn bindgen_test_layout___pthread_cond_s__bindgen_ty_2__bindgen_ty_1 () {
assert_eq ! (:: std :: mem :: size_of :: < __pthread_cond_s__bindgen_ty_2__bindgen_ty_1 > ( ) , 8usize , concat ! ( "Size of: " , stringify ! ( __pthread_cond_s__bindgen_ty_2__bindgen_ty_1 ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < __pthread_cond_s__bindgen_ty_2__bindgen_ty_1 > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( __pthread_cond_s__bindgen_ty_2__bindgen_ty_1 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_cond_s__bindgen_ty_2__bindgen_ty_1 > ( ) ) ) . __low as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_cond_s__bindgen_ty_2__bindgen_ty_1 ) , "::" , stringify ! ( __low ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_cond_s__bindgen_ty_2__bindgen_ty_1 > ( ) ) ) . __high as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_cond_s__bindgen_ty_2__bindgen_ty_1 ) , "::" , stringify ! ( __high ) )) ;

}
 # [test] fn bindgen_test_layout___pthread_cond_s__bindgen_ty_2 () {
assert_eq ! (:: std :: mem :: size_of :: < __pthread_cond_s__bindgen_ty_2 > ( ) , 8usize , concat ! ( "Size of: " , stringify ! ( __pthread_cond_s__bindgen_ty_2 ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < __pthread_cond_s__bindgen_ty_2 > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( __pthread_cond_s__bindgen_ty_2 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_cond_s__bindgen_ty_2 > ( ) ) ) . __g1_start as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_cond_s__bindgen_ty_2 ) , "::" , stringify ! ( __g1_start ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_cond_s__bindgen_ty_2 > ( ) ) ) . __g1_start32 as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_cond_s__bindgen_ty_2 ) , "::" , stringify ! ( __g1_start32 ) )) ;

}
 # [test] fn bindgen_test_layout___pthread_cond_s () {
assert_eq ! (:: std :: mem :: size_of :: < __pthread_cond_s > ( ) , 48usize , concat ! ( "Size of: " , stringify ! ( __pthread_cond_s ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < __pthread_cond_s > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( __pthread_cond_s ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_cond_s > ( ) ) ) . __g_refs as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_cond_s ) , "::" , stringify ! ( __g_refs ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_cond_s > ( ) ) ) . __g_size as * const _ as usize } , 24usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_cond_s ) , "::" , stringify ! ( __g_size ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_cond_s > ( ) ) ) . __g1_orig_size as * const _ as usize } , 32usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_cond_s ) , "::" , stringify ! ( __g1_orig_size ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_cond_s > ( ) ) ) . __wrefs as * const _ as usize } , 36usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_cond_s ) , "::" , stringify ! ( __wrefs ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_cond_s > ( ) ) ) . __g_signals as * const _ as usize } , 40usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_cond_s ) , "::" , stringify ! ( __g_signals ) )) ;

}
 pub type pthread_t = :: std :: os :: raw :: c_ulong ;
 # [repr ( C )] # [derive ( Copy , Clone )] pub union pthread_mutexattr_t {
pub __size : [:: std :: os :: raw :: c_char ; 4usize] , pub __align : :: std :: os :: raw :: c_int , _bindgen_union_align : u32 ,
}
 # [test] fn bindgen_test_layout_pthread_mutexattr_t () {
assert_eq ! (:: std :: mem :: size_of :: < pthread_mutexattr_t > ( ) , 4usize , concat ! ( "Size of: " , stringify ! ( pthread_mutexattr_t ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < pthread_mutexattr_t > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( pthread_mutexattr_t ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < pthread_mutexattr_t > ( ) ) ) . __size as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( pthread_mutexattr_t ) , "::" , stringify ! ( __size ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < pthread_mutexattr_t > ( ) ) ) . __align as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( pthread_mutexattr_t ) , "::" , stringify ! ( __align ) )) ;

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub union pthread_condattr_t {
pub __size : [:: std :: os :: raw :: c_char ; 4usize] , pub __align : :: std :: os :: raw :: c_int , _bindgen_union_align : u32 ,
}
 # [test] fn bindgen_test_layout_pthread_condattr_t () {
assert_eq ! (:: std :: mem :: size_of :: < pthread_condattr_t > ( ) , 4usize , concat ! ( "Size of: " , stringify ! ( pthread_condattr_t ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < pthread_condattr_t > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( pthread_condattr_t ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < pthread_condattr_t > ( ) ) ) . __size as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( pthread_condattr_t ) , "::" , stringify ! ( __size ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < pthread_condattr_t > ( ) ) ) . __align as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( pthread_condattr_t ) , "::" , stringify ! ( __align ) )) ;

}
 pub type pthread_key_t = :: std :: os :: raw :: c_uint ;
 pub type pthread_once_t = :: std :: os :: raw :: c_int ;
 # [repr ( C )] # [derive ( Copy , Clone )] pub union pthread_attr_t {
pub __size : [:: std :: os :: raw :: c_char ; 56usize] , pub __align : :: std :: os :: raw :: c_long , _bindgen_union_align : [u64 ; 7usize] ,
}
 # [test] fn bindgen_test_layout_pthread_attr_t () {
assert_eq ! (:: std :: mem :: size_of :: < pthread_attr_t > ( ) , 56usize , concat ! ( "Size of: " , stringify ! ( pthread_attr_t ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < pthread_attr_t > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( pthread_attr_t ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < pthread_attr_t > ( ) ) ) . __size as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( pthread_attr_t ) , "::" , stringify ! ( __size ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < pthread_attr_t > ( ) ) ) . __align as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( pthread_attr_t ) , "::" , stringify ! ( __align ) )) ;

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub union pthread_mutex_t {
pub __data : __pthread_mutex_s , pub __size : [:: std :: os :: raw :: c_char ; 40usize] , pub __align : :: std :: os :: raw :: c_long , _bindgen_union_align : [u64 ; 5usize] ,
}
 # [test] fn bindgen_test_layout_pthread_mutex_t () {
assert_eq ! (:: std :: mem :: size_of :: < pthread_mutex_t > ( ) , 40usize , concat ! ( "Size of: " , stringify ! ( pthread_mutex_t ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < pthread_mutex_t > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( pthread_mutex_t ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < pthread_mutex_t > ( ) ) ) . __data as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( pthread_mutex_t ) , "::" , stringify ! ( __data ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < pthread_mutex_t > ( ) ) ) . __size as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( pthread_mutex_t ) , "::" , stringify ! ( __size ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < pthread_mutex_t > ( ) ) ) . __align as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( pthread_mutex_t ) , "::" , stringify ! ( __align ) )) ;

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub union pthread_cond_t {
pub __data : __pthread_cond_s , pub __size : [:: std :: os :: raw :: c_char ; 48usize] , pub __align : :: std :: os :: raw :: c_longlong , _bindgen_union_align : [u64 ; 6usize] ,
}
 # [test] fn bindgen_test_layout_pthread_cond_t () {
assert_eq ! (:: std :: mem :: size_of :: < pthread_cond_t > ( ) , 48usize , concat ! ( "Size of: " , stringify ! ( pthread_cond_t ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < pthread_cond_t > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( pthread_cond_t ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < pthread_cond_t > ( ) ) ) . __data as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( pthread_cond_t ) , "::" , stringify ! ( __data ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < pthread_cond_t > ( ) ) ) . __size as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( pthread_cond_t ) , "::" , stringify ! ( __size ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < pthread_cond_t > ( ) ) ) . __align as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( pthread_cond_t ) , "::" , stringify ! ( __align ) )) ;

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub union pthread_rwlock_t {
pub __data : __pthread_rwlock_arch_t , pub __size : [:: std :: os :: raw :: c_char ; 56usize] , pub __align : :: std :: os :: raw :: c_long , _bindgen_union_align : [u64 ; 7usize] ,
}
 # [test] fn bindgen_test_layout_pthread_rwlock_t () {
assert_eq ! (:: std :: mem :: size_of :: < pthread_rwlock_t > ( ) , 56usize , concat ! ( "Size of: " , stringify ! ( pthread_rwlock_t ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < pthread_rwlock_t > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( pthread_rwlock_t ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < pthread_rwlock_t > ( ) ) ) . __data as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( pthread_rwlock_t ) , "::" , stringify ! ( __data ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < pthread_rwlock_t > ( ) ) ) . __size as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( pthread_rwlock_t ) , "::" , stringify ! ( __size ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < pthread_rwlock_t > ( ) ) ) . __align as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( pthread_rwlock_t ) , "::" , stringify ! ( __align ) )) ;

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub union pthread_rwlockattr_t {
pub __size : [:: std :: os :: raw :: c_char ; 8usize] , pub __align : :: std :: os :: raw :: c_long , _bindgen_union_align : u64 ,
}
 # [test] fn bindgen_test_layout_pthread_rwlockattr_t () {
assert_eq ! (:: std :: mem :: size_of :: < pthread_rwlockattr_t > ( ) , 8usize , concat ! ( "Size of: " , stringify ! ( pthread_rwlockattr_t ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < pthread_rwlockattr_t > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( pthread_rwlockattr_t ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < pthread_rwlockattr_t > ( ) ) ) . __size as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( pthread_rwlockattr_t ) , "::" , stringify ! ( __size ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < pthread_rwlockattr_t > ( ) ) ) . __align as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( pthread_rwlockattr_t ) , "::" , stringify ! ( __align ) )) ;

}
 pub type pthread_spinlock_t = :: std :: os :: raw :: c_int ;
 # [repr ( C )] # [derive ( Copy , Clone )] pub union pthread_barrier_t {
pub __size : [:: std :: os :: raw :: c_char ; 32usize] , pub __align : :: std :: os :: raw :: c_long , _bindgen_union_align : [u64 ; 4usize] ,
}
 # [test] fn bindgen_test_layout_pthread_barrier_t () {
assert_eq ! (:: std :: mem :: size_of :: < pthread_barrier_t > ( ) , 32usize , concat ! ( "Size of: " , stringify ! ( pthread_barrier_t ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < pthread_barrier_t > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( pthread_barrier_t ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < pthread_barrier_t > ( ) ) ) . __size as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( pthread_barrier_t ) , "::" , stringify ! ( __size ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < pthread_barrier_t > ( ) ) ) . __align as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( pthread_barrier_t ) , "::" , stringify ! ( __align ) )) ;

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub union pthread_barrierattr_t {
pub __size : [:: std :: os :: raw :: c_char ; 4usize] , pub __align : :: std :: os :: raw :: c_int , _bindgen_union_align : u32 ,
}
 # [test] fn bindgen_test_layout_pthread_barrierattr_t () {
assert_eq ! (:: std :: mem :: size_of :: < pthread_barrierattr_t > ( ) , 4usize , concat ! ( "Size of: " , stringify ! ( pthread_barrierattr_t ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < pthread_barrierattr_t > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( pthread_barrierattr_t ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < pthread_barrierattr_t > ( ) ) ) . __size as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( pthread_barrierattr_t ) , "::" , stringify ! ( __size ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < pthread_barrierattr_t > ( ) ) ) . __align as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( pthread_barrierattr_t ) , "::" , stringify ! ( __align ) )) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct iovec {
pub iov_base : * mut :: std :: os :: raw :: c_void , pub iov_len : usize ,
}
 # [test] fn bindgen_test_layout_iovec () {
assert_eq ! (:: std :: mem :: size_of :: < iovec > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( iovec ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < iovec > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( iovec ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < iovec > ( ) ) ) . iov_base as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( iovec ) , "::" , stringify ! ( iov_base ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < iovec > ( ) ) ) . iov_len as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( iovec ) , "::" , stringify ! ( iov_len ) )) ;

}
 pub type socklen_t = __socklen_t ;
 pub const __socket_type_SOCK_STREAM : __socket_type = 1 ;
 pub const __socket_type_SOCK_DGRAM : __socket_type = 2 ;
 pub const __socket_type_SOCK_RAW : __socket_type = 3 ;
 pub const __socket_type_SOCK_RDM : __socket_type = 4 ;
 pub const __socket_type_SOCK_SEQPACKET : __socket_type = 5 ;
 pub const __socket_type_SOCK_DCCP : __socket_type = 6 ;
 pub const __socket_type_SOCK_PACKET : __socket_type = 10 ;
 pub const __socket_type_SOCK_CLOEXEC : __socket_type = 524288 ;
 pub const __socket_type_SOCK_NONBLOCK : __socket_type = 2048 ;
 pub type __socket_type = u32 ;
 pub type sa_family_t = :: std :: os :: raw :: c_ushort ;
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct sockaddr {
pub sa_family : sa_family_t , pub sa_data : [:: std :: os :: raw :: c_char ; 14usize] ,
}
 # [test] fn bindgen_test_layout_sockaddr () {
assert_eq ! (:: std :: mem :: size_of :: < sockaddr > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( sockaddr ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < sockaddr > ( ) , 2usize , concat ! ( "Alignment of " , stringify ! ( sockaddr ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sockaddr > ( ) ) ) . sa_family as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( sockaddr ) , "::" , stringify ! ( sa_family ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sockaddr > ( ) ) ) . sa_data as * const _ as usize } , 2usize , concat ! ( "Offset of field: " , stringify ! ( sockaddr ) , "::" , stringify ! ( sa_data ) )) ;

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub struct sockaddr_storage {
pub ss_family : sa_family_t , pub __ss_padding : [:: std :: os :: raw :: c_char ; 118usize] , pub __ss_align : :: std :: os :: raw :: c_ulong ,
}
 # [test] fn bindgen_test_layout_sockaddr_storage () {
assert_eq ! (:: std :: mem :: size_of :: < sockaddr_storage > ( ) , 128usize , concat ! ( "Size of: " , stringify ! ( sockaddr_storage ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < sockaddr_storage > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( sockaddr_storage ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sockaddr_storage > ( ) ) ) . ss_family as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( sockaddr_storage ) , "::" , stringify ! ( ss_family ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sockaddr_storage > ( ) ) ) . __ss_padding as * const _ as usize } , 2usize , concat ! ( "Offset of field: " , stringify ! ( sockaddr_storage ) , "::" , stringify ! ( __ss_padding ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sockaddr_storage > ( ) ) ) . __ss_align as * const _ as usize } , 120usize , concat ! ( "Offset of field: " , stringify ! ( sockaddr_storage ) , "::" , stringify ! ( __ss_align ) )) ;

}
 pub const MSG_OOB : _bindgen_ty_1 = 1 ;
 pub const MSG_PEEK : _bindgen_ty_1 = 2 ;
 pub const MSG_DONTROUTE : _bindgen_ty_1 = 4 ;
 pub const MSG_TRYHARD : _bindgen_ty_1 = 4 ;
 pub const MSG_CTRUNC : _bindgen_ty_1 = 8 ;
 pub const MSG_PROXY : _bindgen_ty_1 = 16 ;
 pub const MSG_TRUNC : _bindgen_ty_1 = 32 ;
 pub const MSG_DONTWAIT : _bindgen_ty_1 = 64 ;
 pub const MSG_EOR : _bindgen_ty_1 = 128 ;
 pub const MSG_WAITALL : _bindgen_ty_1 = 256 ;
 pub const MSG_FIN : _bindgen_ty_1 = 512 ;
 pub const MSG_SYN : _bindgen_ty_1 = 1024 ;
 pub const MSG_CONFIRM : _bindgen_ty_1 = 2048 ;
 pub const MSG_RST : _bindgen_ty_1 = 4096 ;
 pub const MSG_ERRQUEUE : _bindgen_ty_1 = 8192 ;
 pub const MSG_NOSIGNAL : _bindgen_ty_1 = 16384 ;
 pub const MSG_MORE : _bindgen_ty_1 = 32768 ;
 pub const MSG_WAITFORONE : _bindgen_ty_1 = 65536 ;
 pub const MSG_BATCH : _bindgen_ty_1 = 262144 ;
 pub const MSG_ZEROCOPY : _bindgen_ty_1 = 67108864 ;
 pub const MSG_FASTOPEN : _bindgen_ty_1 = 536870912 ;
 pub const MSG_CMSG_CLOEXEC : _bindgen_ty_1 = 1073741824 ;
 pub type _bindgen_ty_1 = u32 ;
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct msghdr {
pub msg_name : * mut :: std :: os :: raw :: c_void , pub msg_namelen : socklen_t , pub msg_iov : * mut iovec , pub msg_iovlen : usize , pub msg_control : * mut :: std :: os :: raw :: c_void , pub msg_controllen : usize , pub msg_flags : :: std :: os :: raw :: c_int ,
}
 # [test] fn bindgen_test_layout_msghdr () {
assert_eq ! (:: std :: mem :: size_of :: < msghdr > ( ) , 56usize , concat ! ( "Size of: " , stringify ! ( msghdr ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < msghdr > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( msghdr ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < msghdr > ( ) ) ) . msg_name as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( msghdr ) , "::" , stringify ! ( msg_name ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < msghdr > ( ) ) ) . msg_namelen as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( msghdr ) , "::" , stringify ! ( msg_namelen ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < msghdr > ( ) ) ) . msg_iov as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( msghdr ) , "::" , stringify ! ( msg_iov ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < msghdr > ( ) ) ) . msg_iovlen as * const _ as usize } , 24usize , concat ! ( "Offset of field: " , stringify ! ( msghdr ) , "::" , stringify ! ( msg_iovlen ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < msghdr > ( ) ) ) . msg_control as * const _ as usize } , 32usize , concat ! ( "Offset of field: " , stringify ! ( msghdr ) , "::" , stringify ! ( msg_control ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < msghdr > ( ) ) ) . msg_controllen as * const _ as usize } , 40usize , concat ! ( "Offset of field: " , stringify ! ( msghdr ) , "::" , stringify ! ( msg_controllen ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < msghdr > ( ) ) ) . msg_flags as * const _ as usize } , 48usize , concat ! ( "Offset of field: " , stringify ! ( msghdr ) , "::" , stringify ! ( msg_flags ) )) ;

}
 # [repr ( C )] # [derive ( Debug )] pub struct cmsghdr {
pub cmsg_len : usize , pub cmsg_level : :: std :: os :: raw :: c_int , pub cmsg_type : :: std :: os :: raw :: c_int , pub __cmsg_data : __IncompleteArrayField < :: std :: os :: raw :: c_uchar > ,
}
 # [test] fn bindgen_test_layout_cmsghdr () {
assert_eq ! (:: std :: mem :: size_of :: < cmsghdr > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( cmsghdr ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < cmsghdr > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( cmsghdr ) )) ;

}
 extern "C" {
pub fn __cmsg_nxthdr (__mhdr : * mut msghdr , __cmsg : * mut cmsghdr) -> * mut cmsghdr ;

}
 pub const SCM_RIGHTS : _bindgen_ty_2 = 1 ;
 pub const SCM_CREDENTIALS : _bindgen_ty_2 = 2 ;
 pub type _bindgen_ty_2 = u32 ;
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct ucred {
pub pid : pid_t , pub uid : uid_t , pub gid : gid_t ,
}
 # [test] fn bindgen_test_layout_ucred () {
assert_eq ! (:: std :: mem :: size_of :: < ucred > ( ) , 12usize , concat ! ( "Size of: " , stringify ! ( ucred ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < ucred > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( ucred ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ucred > ( ) ) ) . pid as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( ucred ) , "::" , stringify ! ( pid ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ucred > ( ) ) ) . uid as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( ucred ) , "::" , stringify ! ( uid ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ucred > ( ) ) ) . gid as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( ucred ) , "::" , stringify ! ( gid ) )) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct linger {
pub l_onoff : :: std :: os :: raw :: c_int , pub l_linger : :: std :: os :: raw :: c_int ,
}
 # [test] fn bindgen_test_layout_linger () {
assert_eq ! (:: std :: mem :: size_of :: < linger > ( ) , 8usize , concat ! ( "Size of: " , stringify ! ( linger ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < linger > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( linger ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < linger > ( ) ) ) . l_onoff as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( linger ) , "::" , stringify ! ( l_onoff ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < linger > ( ) ) ) . l_linger as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( linger ) , "::" , stringify ! ( l_linger ) )) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct osockaddr {
pub sa_family : :: std :: os :: raw :: c_ushort , pub sa_data : [:: std :: os :: raw :: c_uchar ; 14usize] ,
}
 # [test] fn bindgen_test_layout_osockaddr () {
assert_eq ! (:: std :: mem :: size_of :: < osockaddr > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( osockaddr ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < osockaddr > ( ) , 2usize , concat ! ( "Alignment of " , stringify ! ( osockaddr ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < osockaddr > ( ) ) ) . sa_family as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( osockaddr ) , "::" , stringify ! ( sa_family ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < osockaddr > ( ) ) ) . sa_data as * const _ as usize } , 2usize , concat ! ( "Offset of field: " , stringify ! ( osockaddr ) , "::" , stringify ! ( sa_data ) )) ;

}
 pub const SHUT_RD : _bindgen_ty_3 = 0 ;
 pub const SHUT_WR : _bindgen_ty_3 = 1 ;
 pub const SHUT_RDWR : _bindgen_ty_3 = 2 ;
 pub type _bindgen_ty_3 = u32 ;
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct mmsghdr {
pub msg_hdr : msghdr , pub msg_len : :: std :: os :: raw :: c_uint ,
}
 # [test] fn bindgen_test_layout_mmsghdr () {
assert_eq ! (:: std :: mem :: size_of :: < mmsghdr > ( ) , 64usize , concat ! ( "Size of: " , stringify ! ( mmsghdr ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < mmsghdr > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( mmsghdr ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < mmsghdr > ( ) ) ) . msg_hdr as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( mmsghdr ) , "::" , stringify ! ( msg_hdr ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < mmsghdr > ( ) ) ) . msg_len as * const _ as usize } , 56usize , concat ! ( "Offset of field: " , stringify ! ( mmsghdr ) , "::" , stringify ! ( msg_len ) )) ;

}
 extern "C" {
pub fn socket (__domain : :: std :: os :: raw :: c_int , __type : :: std :: os :: raw :: c_int , __protocol : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn socketpair (__domain : :: std :: os :: raw :: c_int , __type : :: std :: os :: raw :: c_int , __protocol : :: std :: os :: raw :: c_int , __fds : * mut :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn bind (__fd : :: std :: os :: raw :: c_int , __addr : * const sockaddr , __len : socklen_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn getsockname (__fd : :: std :: os :: raw :: c_int , __addr : * mut sockaddr , __len : * mut socklen_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn connect (__fd : :: std :: os :: raw :: c_int , __addr : * const sockaddr , __len : socklen_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn getpeername (__fd : :: std :: os :: raw :: c_int , __addr : * mut sockaddr , __len : * mut socklen_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn send (__fd : :: std :: os :: raw :: c_int , __buf : * const :: std :: os :: raw :: c_void , __n : usize , __flags : :: std :: os :: raw :: c_int) -> isize ;

}
 extern "C" {
pub fn recv (__fd : :: std :: os :: raw :: c_int , __buf : * mut :: std :: os :: raw :: c_void , __n : usize , __flags : :: std :: os :: raw :: c_int) -> isize ;

}
 extern "C" {
pub fn sendto (__fd : :: std :: os :: raw :: c_int , __buf : * const :: std :: os :: raw :: c_void , __n : usize , __flags : :: std :: os :: raw :: c_int , __addr : * const sockaddr , __addr_len : socklen_t) -> isize ;

}
 extern "C" {
pub fn recvfrom (__fd : :: std :: os :: raw :: c_int , __buf : * mut :: std :: os :: raw :: c_void , __n : usize , __flags : :: std :: os :: raw :: c_int , __addr : * mut sockaddr , __addr_len : * mut socklen_t) -> isize ;

}
 extern "C" {
pub fn sendmsg (__fd : :: std :: os :: raw :: c_int , __message : * const msghdr , __flags : :: std :: os :: raw :: c_int) -> isize ;

}
 extern "C" {
pub fn sendmmsg (__fd : :: std :: os :: raw :: c_int , __vmessages : * mut mmsghdr , __vlen : :: std :: os :: raw :: c_uint , __flags : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn recvmsg (__fd : :: std :: os :: raw :: c_int , __message : * mut msghdr , __flags : :: std :: os :: raw :: c_int) -> isize ;

}
 extern "C" {
pub fn recvmmsg (__fd : :: std :: os :: raw :: c_int , __vmessages : * mut mmsghdr , __vlen : :: std :: os :: raw :: c_uint , __flags : :: std :: os :: raw :: c_int , __tmo : * mut timespec) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn getsockopt (__fd : :: std :: os :: raw :: c_int , __level : :: std :: os :: raw :: c_int , __optname : :: std :: os :: raw :: c_int , __optval : * mut :: std :: os :: raw :: c_void , __optlen : * mut socklen_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn setsockopt (__fd : :: std :: os :: raw :: c_int , __level : :: std :: os :: raw :: c_int , __optname : :: std :: os :: raw :: c_int , __optval : * const :: std :: os :: raw :: c_void , __optlen : socklen_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn listen (__fd : :: std :: os :: raw :: c_int , __n : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn accept (__fd : :: std :: os :: raw :: c_int , __addr : * mut sockaddr , __addr_len : * mut socklen_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn accept4 (__fd : :: std :: os :: raw :: c_int , __addr : * mut sockaddr , __addr_len : * mut socklen_t , __flags : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn shutdown (__fd : :: std :: os :: raw :: c_int , __how : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn sockatmark (__fd : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn isfdtype (__fd : :: std :: os :: raw :: c_int , __fdtype : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn memfd_create (__name : * const :: std :: os :: raw :: c_char , __flags : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn mlock2 (__addr : * const :: std :: os :: raw :: c_void , __length : usize , __flags : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn pkey_alloc (__flags : :: std :: os :: raw :: c_uint , __access_rights : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn pkey_set (__key : :: std :: os :: raw :: c_int , __access_rights : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn pkey_get (__key : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn pkey_free (__key : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn pkey_mprotect (__addr : * mut :: std :: os :: raw :: c_void , __len : usize , __prot : :: std :: os :: raw :: c_int , __pkey : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn mmap (__addr : * mut :: std :: os :: raw :: c_void , __len : usize , __prot : :: std :: os :: raw :: c_int , __flags : :: std :: os :: raw :: c_int , __fd : :: std :: os :: raw :: c_int , __offset : __off_t) -> * mut :: std :: os :: raw :: c_void ;

}
 extern "C" {
pub fn mmap64 (__addr : * mut :: std :: os :: raw :: c_void , __len : usize , __prot : :: std :: os :: raw :: c_int , __flags : :: std :: os :: raw :: c_int , __fd : :: std :: os :: raw :: c_int , __offset : __off64_t) -> * mut :: std :: os :: raw :: c_void ;

}
 extern "C" {
pub fn munmap (__addr : * mut :: std :: os :: raw :: c_void , __len : usize) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn mprotect (__addr : * mut :: std :: os :: raw :: c_void , __len : usize , __prot : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn msync (__addr : * mut :: std :: os :: raw :: c_void , __len : usize , __flags : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn madvise (__addr : * mut :: std :: os :: raw :: c_void , __len : usize , __advice : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn posix_madvise (__addr : * mut :: std :: os :: raw :: c_void , __len : usize , __advice : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn mlock (__addr : * const :: std :: os :: raw :: c_void , __len : usize) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn munlock (__addr : * const :: std :: os :: raw :: c_void , __len : usize) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn mlockall (__flags : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn munlockall () -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn mincore (__start : * mut :: std :: os :: raw :: c_void , __len : usize , __vec : * mut :: std :: os :: raw :: c_uchar) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn mremap (__addr : * mut :: std :: os :: raw :: c_void , __old_len : usize , __new_len : usize , __flags : :: std :: os :: raw :: c_int , ...) -> * mut :: std :: os :: raw :: c_void ;

}
 extern "C" {
pub fn remap_file_pages (__start : * mut :: std :: os :: raw :: c_void , __size : usize , __prot : :: std :: os :: raw :: c_int , __pgoff : usize , __flags : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn shm_open (__name : * const :: std :: os :: raw :: c_char , __oflag : :: std :: os :: raw :: c_int , __mode : mode_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn shm_unlink (__name : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z18process_post_loginP11vsf_session"] pub fn process_post_login (p_sess : * mut vsf_session) ;

}
 extern "C" {
# [link_name = "\u{1}_Z25vsf_priv_parent_postloginP11vsf_session"] pub fn vsf_priv_parent_postlogin (p_sess : * mut vsf_session) ;

}
 extern "C" {
# [link_name = "\u{1}_Z15init_connectionP11vsf_session"] pub fn init_connection (p_sess : * mut vsf_session) ;

}
 extern "C" {
# [link_name = "\u{1}_Z28vsf_privop_get_ftp_port_sockP11vsf_sessionti"] pub fn vsf_privop_get_ftp_port_sock (p_sess : * mut vsf_session , remote_port : :: std :: os :: raw :: c_ushort , use_port_sockaddr : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z23vsf_privop_pasv_cleanupP11vsf_session"] pub fn vsf_privop_pasv_cleanup (p_sess : * mut vsf_session) ;

}
 extern "C" {
# [link_name = "\u{1}_Z22vsf_privop_pasv_listenP11vsf_session"] pub fn vsf_privop_pasv_listen (p_sess : * mut vsf_session) -> :: std :: os :: raw :: c_ushort ;

}
 extern "C" {
# [link_name = "\u{1}_Z22vsf_privop_pasv_activeP11vsf_session"] pub fn vsf_privop_pasv_active (p_sess : * mut vsf_session) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z22vsf_privop_accept_pasvP11vsf_session"] pub fn vsf_privop_accept_pasv (p_sess : * mut vsf_session) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z24vsf_privop_do_file_chownP11vsf_sessioni"] pub fn vsf_privop_do_file_chown (p_sess : * mut vsf_session , fd : :: std :: os :: raw :: c_int) ;

}
 pub const EVSFPrivopLoginResult_kVSFLoginNull : EVSFPrivopLoginResult = 0 ;
 pub const EVSFPrivopLoginResult_kVSFLoginFail : EVSFPrivopLoginResult = 1 ;
 pub const EVSFPrivopLoginResult_kVSFLoginAnon : EVSFPrivopLoginResult = 2 ;
 pub const EVSFPrivopLoginResult_kVSFLoginReal : EVSFPrivopLoginResult = 3 ;
 pub type EVSFPrivopLoginResult = u32 ;
 extern "C" {
# [link_name = "\u{1}_Z19vsf_privop_do_loginP11vsf_sessionPK5mystr"] pub fn vsf_privop_do_login (p_sess : * mut vsf_session , p_pass_str : * const mystr) -> EVSFPrivopLoginResult ;

}
 extern "C" {
# [link_name = "\u{1}_Z14priv_sock_initP11vsf_session"] pub fn priv_sock_init (p_sess : * mut vsf_session) ;

}
 extern "C" {
# [link_name = "\u{1}_Z15priv_sock_closeP11vsf_session"] pub fn priv_sock_close (p_sess : * mut vsf_session) ;

}
 extern "C" {
# [link_name = "\u{1}_Z28priv_sock_set_parent_contextP11vsf_session"] pub fn priv_sock_set_parent_context (p_sess : * mut vsf_session) ;

}
 extern "C" {
# [link_name = "\u{1}_Z27priv_sock_set_child_contextP11vsf_session"] pub fn priv_sock_set_child_context (p_sess : * mut vsf_session) ;

}
 extern "C" {
# [link_name = "\u{1}_Z18priv_sock_send_cmdic"] pub fn priv_sock_send_cmd (fd : :: std :: os :: raw :: c_int , cmd : :: std :: os :: raw :: c_char) ;

}
 extern "C" {
# [link_name = "\u{1}_Z18priv_sock_send_striPK5mystr"] pub fn priv_sock_send_str (fd : :: std :: os :: raw :: c_int , p_str : * const mystr) ;

}
 extern "C" {
# [link_name = "\u{1}_Z18priv_sock_send_bufiPKcj"] pub fn priv_sock_send_buf (fd : :: std :: os :: raw :: c_int , p_buf : * const :: std :: os :: raw :: c_char , len : :: std :: os :: raw :: c_uint) ;

}
 extern "C" {
# [link_name = "\u{1}_Z18priv_sock_recv_bufiPcj"] pub fn priv_sock_recv_buf (fd : :: std :: os :: raw :: c_int , p_buf : * mut :: std :: os :: raw :: c_char , len : :: std :: os :: raw :: c_uint) ;

}
 extern "C" {
# [link_name = "\u{1}_Z20priv_sock_get_resulti"] pub fn priv_sock_get_result (fd : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_char ;

}
 extern "C" {
# [link_name = "\u{1}_Z17priv_sock_get_cmdi"] pub fn priv_sock_get_cmd (fd : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_char ;

}
 extern "C" {
# [link_name = "\u{1}_Z17priv_sock_get_striP5mystr"] pub fn priv_sock_get_str (fd : :: std :: os :: raw :: c_int , p_dest : * mut mystr) ;

}
 extern "C" {
# [link_name = "\u{1}_Z21priv_sock_send_resultic"] pub fn priv_sock_send_result (fd : :: std :: os :: raw :: c_int , res : :: std :: os :: raw :: c_char) ;

}
 extern "C" {
# [link_name = "\u{1}_Z17priv_sock_send_fdii"] pub fn priv_sock_send_fd (fd : :: std :: os :: raw :: c_int , send_fd : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
# [link_name = "\u{1}_Z17priv_sock_recv_fdi"] pub fn priv_sock_recv_fd (fd : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z18priv_sock_send_intii"] pub fn priv_sock_send_int (fd : :: std :: os :: raw :: c_int , the_int : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
# [link_name = "\u{1}_Z17priv_sock_get_inti"] pub fn priv_sock_get_int (fd : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 pub type ptrace_sandbox_validator_t = :: std :: option :: Option < unsafe extern "C" fn (arg1 : * mut pt_sandbox , arg2 : * mut :: std :: os :: raw :: c_void) -> :: std :: os :: raw :: c_int > ;
 extern "C" {
# [link_name = "\u{1}_Z20ptrace_sandbox_allocv"] pub fn ptrace_sandbox_alloc () -> * mut pt_sandbox ;

}
 extern "C" {
# [link_name = "\u{1}_Z19ptrace_sandbox_freeP10pt_sandbox"] pub fn ptrace_sandbox_free (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
# [link_name = "\u{1}_Z29ptrace_sandbox_launch_processP10pt_sandboxPFvPvES1_"] pub fn ptrace_sandbox_launch_process (p_sandbox : * mut pt_sandbox , p_func : :: std :: option :: Option < unsafe extern "C" fn ( arg1 : * mut :: std :: os :: raw :: c_void ) > , p_arg : * mut :: std :: os :: raw :: c_void) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z28ptrace_sandbox_run_processesP10pt_sandbox"] pub fn ptrace_sandbox_run_processes (p_sandbox : * mut pt_sandbox) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z29ptrace_sandbox_kill_processesP10pt_sandbox"] pub fn ptrace_sandbox_kill_processes (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
# [link_name = "\u{1}_Z22ptrace_sandbox_get_argP10pt_sandboxiPm"] pub fn ptrace_sandbox_get_arg (p_sandbox : * mut pt_sandbox , arg : :: std :: os :: raw :: c_int , p_out : * mut :: std :: os :: raw :: c_ulong) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z33ptrace_sandbox_get_socketcall_argP10pt_sandboxiPm"] pub fn ptrace_sandbox_get_socketcall_arg (p_sandbox : * mut pt_sandbox , arg : :: std :: os :: raw :: c_int , p_out : * mut :: std :: os :: raw :: c_ulong) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z23ptrace_sandbox_get_longP10pt_sandboxmPm"] pub fn ptrace_sandbox_get_long (p_sandbox : * mut pt_sandbox , ptr : :: std :: os :: raw :: c_ulong , p_out : * mut :: std :: os :: raw :: c_ulong) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z22ptrace_sandbox_get_bufP10pt_sandboxmmPv"] pub fn ptrace_sandbox_get_buf (p_sandbox : * mut pt_sandbox , ptr : :: std :: os :: raw :: c_ulong , len : :: std :: os :: raw :: c_ulong , p_buf : * mut :: std :: os :: raw :: c_void) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z27ptrace_sandbox_attach_pointv"] pub fn ptrace_sandbox_attach_point () ;

}
 extern "C" {
# [link_name = "\u{1}_Z26ptrace_sandbox_permit_exitP10pt_sandbox"] pub fn ptrace_sandbox_permit_exit (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
# [link_name = "\u{1}_Z26ptrace_sandbox_permit_readP10pt_sandbox"] pub fn ptrace_sandbox_permit_read (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
# [link_name = "\u{1}_Z27ptrace_sandbox_permit_writeP10pt_sandbox"] pub fn ptrace_sandbox_permit_write (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
# [link_name = "\u{1}_Z31ptrace_sandbox_permit_sigactionP10pt_sandbox"] pub fn ptrace_sandbox_permit_sigaction (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
# [link_name = "\u{1}_Z27ptrace_sandbox_permit_alarmP10pt_sandbox"] pub fn ptrace_sandbox_permit_alarm (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
# [link_name = "\u{1}_Z32ptrace_sandbox_permit_query_timeP10pt_sandbox"] pub fn ptrace_sandbox_permit_query_time (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
# [link_name = "\u{1}_Z26ptrace_sandbox_permit_mmapP10pt_sandbox"] pub fn ptrace_sandbox_permit_mmap (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
# [link_name = "\u{1}_Z30ptrace_sandbox_permit_mprotectP10pt_sandbox"] pub fn ptrace_sandbox_permit_mprotect (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
# [link_name = "\u{1}_Z32ptrace_sandbox_permit_file_statsP10pt_sandbox"] pub fn ptrace_sandbox_permit_file_stats (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
# [link_name = "\u{1}_Z30ptrace_sandbox_permit_fd_statsP10pt_sandbox"] pub fn ptrace_sandbox_permit_fd_stats (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
# [link_name = "\u{1}_Z28ptrace_sandbox_permit_getcwdP10pt_sandbox"] pub fn ptrace_sandbox_permit_getcwd (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
# [link_name = "\u{1}_Z27ptrace_sandbox_permit_chdirP10pt_sandbox"] pub fn ptrace_sandbox_permit_chdir (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
# [link_name = "\u{1}_Z27ptrace_sandbox_permit_umaskP10pt_sandbox"] pub fn ptrace_sandbox_permit_umask (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
# [link_name = "\u{1}_Z26ptrace_sandbox_permit_openP10pt_sandboxi"] pub fn ptrace_sandbox_permit_open (p_sandbox : * mut pt_sandbox , writeable : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
# [link_name = "\u{1}_Z27ptrace_sandbox_permit_closeP10pt_sandbox"] pub fn ptrace_sandbox_permit_close (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
# [link_name = "\u{1}_Z30ptrace_sandbox_permit_getdentsP10pt_sandbox"] pub fn ptrace_sandbox_permit_getdents (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
# [link_name = "\u{1}_Z27ptrace_sandbox_permit_fcntlP10pt_sandbox"] pub fn ptrace_sandbox_permit_fcntl (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
# [link_name = "\u{1}_Z30ptrace_sandbox_permit_sendfileP10pt_sandbox"] pub fn ptrace_sandbox_permit_sendfile (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
# [link_name = "\u{1}_Z26ptrace_sandbox_permit_seekP10pt_sandbox"] pub fn ptrace_sandbox_permit_seek (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
# [link_name = "\u{1}_Z28ptrace_sandbox_permit_selectP10pt_sandbox"] pub fn ptrace_sandbox_permit_select (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
# [link_name = "\u{1}_Z28ptrace_sandbox_permit_unlinkP10pt_sandbox"] pub fn ptrace_sandbox_permit_unlink (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
# [link_name = "\u{1}_Z27ptrace_sandbox_permit_mkdirP10pt_sandbox"] pub fn ptrace_sandbox_permit_mkdir (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
# [link_name = "\u{1}_Z27ptrace_sandbox_permit_rmdirP10pt_sandbox"] pub fn ptrace_sandbox_permit_rmdir (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
# [link_name = "\u{1}_Z28ptrace_sandbox_permit_renameP10pt_sandbox"] pub fn ptrace_sandbox_permit_rename (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
# [link_name = "\u{1}_Z27ptrace_sandbox_permit_utimeP10pt_sandbox"] pub fn ptrace_sandbox_permit_utime (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
# [link_name = "\u{1}_Z31ptrace_sandbox_permit_sigreturnP10pt_sandbox"] pub fn ptrace_sandbox_permit_sigreturn (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
# [link_name = "\u{1}_Z26ptrace_sandbox_permit_recvP10pt_sandbox"] pub fn ptrace_sandbox_permit_recv (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
# [link_name = "\u{1}_Z30ptrace_sandbox_permit_readlinkP10pt_sandbox"] pub fn ptrace_sandbox_permit_readlink (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
# [link_name = "\u{1}_Z25ptrace_sandbox_permit_brkP10pt_sandbox"] pub fn ptrace_sandbox_permit_brk (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
# [link_name = "\u{1}_Z27ptrace_sandbox_permit_sleepP10pt_sandbox"] pub fn ptrace_sandbox_permit_sleep (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
# [link_name = "\u{1}_Z28ptrace_sandbox_permit_fchmodP10pt_sandbox"] pub fn ptrace_sandbox_permit_fchmod (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
# [link_name = "\u{1}_Z27ptrace_sandbox_permit_chmodP10pt_sandbox"] pub fn ptrace_sandbox_permit_chmod (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
# [link_name = "\u{1}_Z28ptrace_sandbox_permit_fchownP10pt_sandbox"] pub fn ptrace_sandbox_permit_fchown (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
# [link_name = "\u{1}_Z28ptrace_sandbox_permit_mremapP10pt_sandbox"] pub fn ptrace_sandbox_permit_mremap (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
# [link_name = "\u{1}_Z31ptrace_sandbox_permit_ftruncateP10pt_sandbox"] pub fn ptrace_sandbox_permit_ftruncate (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
# [link_name = "\u{1}_Z28ptrace_sandbox_permit_socketP10pt_sandbox"] pub fn ptrace_sandbox_permit_socket (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
# [link_name = "\u{1}_Z35ptrace_sandbox_set_socket_validatorP10pt_sandboxPFiS0_PvES1_"] pub fn ptrace_sandbox_set_socket_validator (p_sandbox : * mut pt_sandbox , val : ptrace_sandbox_validator_t , p_arg : * mut :: std :: os :: raw :: c_void) ;

}
 extern "C" {
# [link_name = "\u{1}_Z26ptrace_sandbox_permit_bindP10pt_sandbox"] pub fn ptrace_sandbox_permit_bind (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
# [link_name = "\u{1}_Z33ptrace_sandbox_set_bind_validatorP10pt_sandboxPFiS0_PvES1_"] pub fn ptrace_sandbox_set_bind_validator (p_sandbox : * mut pt_sandbox , val : ptrace_sandbox_validator_t , p_arg : * mut :: std :: os :: raw :: c_void) ;

}
 extern "C" {
# [link_name = "\u{1}_Z29ptrace_sandbox_permit_connectP10pt_sandbox"] pub fn ptrace_sandbox_permit_connect (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
# [link_name = "\u{1}_Z36ptrace_sandbox_set_connect_validatorP10pt_sandboxPFiS0_PvES1_"] pub fn ptrace_sandbox_set_connect_validator (p_sandbox : * mut pt_sandbox , val : ptrace_sandbox_validator_t , p_arg : * mut :: std :: os :: raw :: c_void) ;

}
 extern "C" {
# [link_name = "\u{1}_Z28ptrace_sandbox_permit_listenP10pt_sandbox"] pub fn ptrace_sandbox_permit_listen (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
# [link_name = "\u{1}_Z28ptrace_sandbox_permit_acceptP10pt_sandbox"] pub fn ptrace_sandbox_permit_accept (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
# [link_name = "\u{1}_Z32ptrace_sandbox_permit_setsockoptP10pt_sandbox"] pub fn ptrace_sandbox_permit_setsockopt (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
# [link_name = "\u{1}_Z39ptrace_sandbox_set_setsockopt_validatorP10pt_sandboxPFiS0_PvES1_"] pub fn ptrace_sandbox_set_setsockopt_validator (p_sandbox : * mut pt_sandbox , val : ptrace_sandbox_validator_t , p_arg : * mut :: std :: os :: raw :: c_void) ;

}
 extern "C" {
# [link_name = "\u{1}_Z32ptrace_sandbox_permit_getsockoptP10pt_sandbox"] pub fn ptrace_sandbox_permit_getsockopt (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
# [link_name = "\u{1}_Z39ptrace_sandbox_set_getsockopt_validatorP10pt_sandboxPFiS0_PvES1_"] pub fn ptrace_sandbox_set_getsockopt_validator (p_sandbox : * mut pt_sandbox , val : ptrace_sandbox_validator_t , p_arg : * mut :: std :: os :: raw :: c_void) ;

}
 extern "C" {
# [link_name = "\u{1}_Z30ptrace_sandbox_permit_shutdownP10pt_sandbox"] pub fn ptrace_sandbox_permit_shutdown (p_sandbox : * mut pt_sandbox) ;

}
 pub const EVSFRWTarget_kVSFRWControl : EVSFRWTarget = 1 ;
 pub const EVSFRWTarget_kVSFRWData : EVSFRWTarget = 2 ;
 pub type EVSFRWTarget = u32 ;
 extern "C" {
# [link_name = "\u{1}_Z13ftp_write_strPK11vsf_sessionPK5mystr12EVSFRWTarget"] pub fn ftp_write_str (p_sess : * const vsf_session , p_str : * const mystr , target : EVSFRWTarget) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z13ftp_read_dataP11vsf_sessionPcj"] pub fn ftp_read_data (p_sess : * mut vsf_session , p_buf : * mut :: std :: os :: raw :: c_char , len : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z14ftp_write_dataPK11vsf_sessionPKcj"] pub fn ftp_write_data (p_sess : * const vsf_session , p_buf : * const :: std :: os :: raw :: c_char , len : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z11ftp_getlineP11vsf_sessionP5mystrPc"] pub fn ftp_getline (p_sess : * mut vsf_session , p_str : * mut mystr , p_buf : * mut :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z16vsf_secbuf_allocPPcj"] pub fn vsf_secbuf_alloc (p_ptr : * mut * mut :: std :: os :: raw :: c_char , size : :: std :: os :: raw :: c_uint) ;

}
 extern "C" {
# [link_name = "\u{1}_Z15vsf_secbuf_freePPc"] pub fn vsf_secbuf_free (p_ptr : * mut * mut :: std :: os :: raw :: c_char) ;

}
 extern "C" {
# [link_name = "\u{1}_Z20seccomp_sandbox_initv"] pub fn seccomp_sandbox_init () ;

}
 extern "C" {
# [link_name = "\u{1}_Z30seccomp_sandbox_setup_preloginPK11vsf_session"] pub fn seccomp_sandbox_setup_prelogin (p_sess : * const vsf_session) ;

}
 extern "C" {
# [link_name = "\u{1}_Z31seccomp_sandbox_setup_postloginPK11vsf_session"] pub fn seccomp_sandbox_setup_postlogin (p_sess : * const vsf_session) ;

}
 extern "C" {
# [link_name = "\u{1}_Z38seccomp_sandbox_setup_postlogin_brokerv"] pub fn seccomp_sandbox_setup_postlogin_broker () ;

}
 extern "C" {
# [link_name = "\u{1}_Z24seccomp_sandbox_lockdownv"] pub fn seccomp_sandbox_lockdown () ;

}
 extern "C" {
# [link_name = "\u{1}_Z30vsf_secutil_change_credentialsPK5mystrS1_S1_jj"] pub fn vsf_secutil_change_credentials (p_user_str : * const mystr , p_dir_str : * const mystr , p_ext_dir_str : * const mystr , caps : :: std :: os :: raw :: c_uint , options : :: std :: os :: raw :: c_uint) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct mystr {
pub PRIVATE_HANDS_OFF_p_buf : * mut :: std :: os :: raw :: c_char , pub PRIVATE_HANDS_OFF_len : :: std :: os :: raw :: c_uint , pub PRIVATE_HANDS_OFF_alloc_bytes : :: std :: os :: raw :: c_uint ,
}
 # [test] fn bindgen_test_layout_mystr () {
assert_eq ! (:: std :: mem :: size_of :: < mystr > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( mystr ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < mystr > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( mystr ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < mystr > ( ) ) ) . PRIVATE_HANDS_OFF_p_buf as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( mystr ) , "::" , stringify ! ( PRIVATE_HANDS_OFF_p_buf ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < mystr > ( ) ) ) . PRIVATE_HANDS_OFF_len as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( mystr ) , "::" , stringify ! ( PRIVATE_HANDS_OFF_len ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < mystr > ( ) ) ) . PRIVATE_HANDS_OFF_alloc_bytes as * const _ as usize } , 12usize , concat ! ( "Offset of field: " , stringify ! ( mystr ) , "::" , stringify ! ( PRIVATE_HANDS_OFF_alloc_bytes ) )) ;

}
 extern "C" {
# [link_name = "\u{1}_Z26private_str_alloc_memchunkP5mystrPKcj"] pub fn private_str_alloc_memchunk (p_str : * mut mystr , p_src : * const :: std :: os :: raw :: c_char , len : :: std :: os :: raw :: c_uint) ;

}
 extern "C" {
# [link_name = "\u{1}_Z14str_alloc_textP5mystrPKc"] pub fn str_alloc_text (p_str : * mut mystr , p_src : * const :: std :: os :: raw :: c_char) ;

}
 extern "C" {
# [link_name = "\u{1}_Z18str_alloc_alt_termP5mystrPKcc"] pub fn str_alloc_alt_term (p_str : * mut mystr , p_src : * const :: std :: os :: raw :: c_char , term : :: std :: os :: raw :: c_char) ;

}
 extern "C" {
# [link_name = "\u{1}_Z15str_alloc_ulongP5mystrm"] pub fn str_alloc_ulong (p_str : * mut mystr , the_ulong : :: std :: os :: raw :: c_ulong) ;

}
 extern "C" {
# [link_name = "\u{1}_Z20str_alloc_filesize_tP5mystrx"] pub fn str_alloc_filesize_t (p_str : * mut mystr , the_filesize : filesize_t) ;

}
 extern "C" {
# [link_name = "\u{1}_Z8str_copyP5mystrPKS_"] pub fn str_copy (p_dest : * mut mystr , p_src : * const mystr) ;

}
 extern "C" {
# [link_name = "\u{1}_Z10str_strdupPK5mystr"] pub fn str_strdup (p_str : * const mystr) -> * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
# [link_name = "\u{1}_Z9str_emptyP5mystr"] pub fn str_empty (p_str : * mut mystr) ;

}
 extern "C" {
# [link_name = "\u{1}_Z8str_freeP5mystr"] pub fn str_free (p_str : * mut mystr) ;

}
 extern "C" {
# [link_name = "\u{1}_Z9str_truncP5mystrj"] pub fn str_trunc (p_str : * mut mystr , trunc_len : :: std :: os :: raw :: c_uint) ;

}
 extern "C" {
# [link_name = "\u{1}_Z11str_reserveP5mystrj"] pub fn str_reserve (p_str : * mut mystr , res_len : :: std :: os :: raw :: c_uint) ;

}
 extern "C" {
# [link_name = "\u{1}_Z11str_isemptyPK5mystr"] pub fn str_isempty (p_str : * const mystr) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z10str_getlenPK5mystr"] pub fn str_getlen (p_str : * const mystr) -> :: std :: os :: raw :: c_uint ;

}
 extern "C" {
# [link_name = "\u{1}_Z10str_getbufPK5mystr"] pub fn str_getbuf (p_str : * const mystr) -> * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
# [link_name = "\u{1}_Z10str_strcmpPK5mystrS1_"] pub fn str_strcmp (p_str1 : * const mystr , p_str2 : * const mystr) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z9str_equalPK5mystrS1_"] pub fn str_equal (p_str1 : * const mystr , p_str2 : * const mystr) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z14str_equal_textPK5mystrPKc"] pub fn str_equal_text (p_str : * const mystr , p_text : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z14str_append_strP5mystrPKS_"] pub fn str_append_str (p_str : * mut mystr , p_other : * const mystr) ;

}
 extern "C" {
# [link_name = "\u{1}_Z15str_append_textP5mystrPKc"] pub fn str_append_text (p_str : * mut mystr , p_src : * const :: std :: os :: raw :: c_char) ;

}
 extern "C" {
# [link_name = "\u{1}_Z16str_append_ulongP5mystrm"] pub fn str_append_ulong (p_str : * mut mystr , the_long : :: std :: os :: raw :: c_ulong) ;

}
 extern "C" {
# [link_name = "\u{1}_Z21str_append_filesize_tP5mystrx"] pub fn str_append_filesize_t (p_str : * mut mystr , the_filesize : filesize_t) ;

}
 extern "C" {
# [link_name = "\u{1}_Z15str_append_charP5mystrc"] pub fn str_append_char (p_str : * mut mystr , the_char : :: std :: os :: raw :: c_char) ;

}
 extern "C" {
# [link_name = "\u{1}_Z17str_append_doubleP5mystrd"] pub fn str_append_double (p_str : * mut mystr , the_double : f64) ;

}
 extern "C" {
# [link_name = "\u{1}_Z9str_upperP5mystr"] pub fn str_upper (p_str : * mut mystr) ;

}
 extern "C" {
# [link_name = "\u{1}_Z8str_rpadP5mystrj"] pub fn str_rpad (p_str : * mut mystr , min_width : :: std :: os :: raw :: c_uint) ;

}
 extern "C" {
# [link_name = "\u{1}_Z8str_lpadP5mystrj"] pub fn str_lpad (p_str : * mut mystr , min_width : :: std :: os :: raw :: c_uint) ;

}
 extern "C" {
# [link_name = "\u{1}_Z16str_replace_charP5mystrcc"] pub fn str_replace_char (p_str : * mut mystr , from : :: std :: os :: raw :: c_char , to : :: std :: os :: raw :: c_char) ;

}
 extern "C" {
# [link_name = "\u{1}_Z16str_replace_textP5mystrPKcS2_"] pub fn str_replace_text (p_str : * mut mystr , p_from : * const :: std :: os :: raw :: c_char , p_to : * const :: std :: os :: raw :: c_char) ;

}
 extern "C" {
# [link_name = "\u{1}_Z14str_split_charP5mystrS0_c"] pub fn str_split_char (p_src : * mut mystr , p_rhs : * mut mystr , c : :: std :: os :: raw :: c_char) ;

}
 extern "C" {
# [link_name = "\u{1}_Z22str_split_char_reverseP5mystrS0_c"] pub fn str_split_char_reverse (p_src : * mut mystr , p_rhs : * mut mystr , c : :: std :: os :: raw :: c_char) ;

}
 extern "C" {
# [link_name = "\u{1}_Z14str_split_textP5mystrS0_PKc"] pub fn str_split_text (p_src : * mut mystr , p_rhs : * mut mystr , p_text : * const :: std :: os :: raw :: c_char) ;

}
 extern "C" {
# [link_name = "\u{1}_Z22str_split_text_reverseP5mystrS0_PKc"] pub fn str_split_text_reverse (p_src : * mut mystr , p_rhs : * mut mystr , p_text : * const :: std :: os :: raw :: c_char) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct str_locate_result {
pub found : :: std :: os :: raw :: c_int , pub index : :: std :: os :: raw :: c_uint , pub char_found : :: std :: os :: raw :: c_char ,
}
 # [test] fn bindgen_test_layout_str_locate_result () {
assert_eq ! (:: std :: mem :: size_of :: < str_locate_result > ( ) , 12usize , concat ! ( "Size of: " , stringify ! ( str_locate_result ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < str_locate_result > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( str_locate_result ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < str_locate_result > ( ) ) ) . found as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( str_locate_result ) , "::" , stringify ! ( found ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < str_locate_result > ( ) ) ) . index as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( str_locate_result ) , "::" , stringify ! ( index ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < str_locate_result > ( ) ) ) . char_found as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( str_locate_result ) , "::" , stringify ! ( char_found ) )) ;

}
 extern "C" {
# [link_name = "\u{1}_Z15str_locate_charPK5mystrc"] pub fn str_locate_char (p_str : * const mystr , look_char : :: std :: os :: raw :: c_char) -> str_locate_result ;

}
 extern "C" {
# [link_name = "\u{1}_Z14str_locate_strPK5mystrS1_"] pub fn str_locate_str (p_str : * const mystr , p_look_str : * const mystr) -> str_locate_result ;

}
 extern "C" {
# [link_name = "\u{1}_Z22str_locate_str_reversePK5mystrS1_"] pub fn str_locate_str_reverse (p_str : * const mystr , p_look_str : * const mystr) -> str_locate_result ;

}
 extern "C" {
# [link_name = "\u{1}_Z15str_locate_textPK5mystrPKc"] pub fn str_locate_text (p_str : * const mystr , p_text : * const :: std :: os :: raw :: c_char) -> str_locate_result ;

}
 extern "C" {
# [link_name = "\u{1}_Z23str_locate_text_reversePK5mystrPKc"] pub fn str_locate_text_reverse (p_str : * const mystr , p_text : * const :: std :: os :: raw :: c_char) -> str_locate_result ;

}
 extern "C" {
# [link_name = "\u{1}_Z16str_locate_charsPK5mystrPKc"] pub fn str_locate_chars (p_str : * const mystr , p_chars : * const :: std :: os :: raw :: c_char) -> str_locate_result ;

}
 extern "C" {
# [link_name = "\u{1}_Z8str_leftPK5mystrPS_j"] pub fn str_left (p_str : * const mystr , p_out : * mut mystr , chars : :: std :: os :: raw :: c_uint) ;

}
 extern "C" {
# [link_name = "\u{1}_Z9str_rightPK5mystrPS_j"] pub fn str_right (p_str : * const mystr , p_out : * mut mystr , chars : :: std :: os :: raw :: c_uint) ;

}
 extern "C" {
# [link_name = "\u{1}_Z14str_mid_to_endPK5mystrPS_j"] pub fn str_mid_to_end (p_str : * const mystr , p_out : * mut mystr , indexx : :: std :: os :: raw :: c_uint) ;

}
 extern "C" {
# [link_name = "\u{1}_Z15str_get_char_atPK5mystrj"] pub fn str_get_char_at (p_str : * const mystr , indexx : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_char ;

}
 extern "C" {
# [link_name = "\u{1}_Z18str_contains_spacePK5mystr"] pub fn str_contains_space (p_str : * const mystr) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z13str_all_spacePK5mystr"] pub fn str_all_space (p_str : * const mystr) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z24str_contains_unprintablePK5mystr"] pub fn str_contains_unprintable (p_str : * const mystr) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z23str_replace_unprintableP5mystrc"] pub fn str_replace_unprintable (p_str : * mut mystr , new_char : :: std :: os :: raw :: c_char) ;

}
 extern "C" {
# [link_name = "\u{1}_Z8str_atoiPK5mystr"] pub fn str_atoi (p_str : * const mystr) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z19str_a_to_filesize_tPK5mystr"] pub fn str_a_to_filesize_t (p_str : * const mystr) -> filesize_t ;

}
 extern "C" {
# [link_name = "\u{1}_Z17str_octal_to_uintPK5mystr"] pub fn str_octal_to_uint (p_str : * const mystr) -> :: std :: os :: raw :: c_uint ;

}
 extern "C" {
# [link_name = "\u{1}_Z11str_getlinePK5mystrPS_Pj"] pub fn str_getline (p_str : * const mystr , p_line_str : * mut mystr , p_pos : * mut :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z17str_contains_linePK5mystrS1_"] pub fn str_contains_line (p_str : * const mystr , p_line_str : * const mystr) -> :: std :: os :: raw :: c_int ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct vsf_session {
pub p_local_addr : * mut vsf_sysutil_sockaddr , pub p_remote_addr : * mut vsf_sysutil_sockaddr , pub p_control_line_buf : * mut :: std :: os :: raw :: c_char , pub idle_timeout : :: std :: os :: raw :: c_int , pub data_timeout : :: std :: os :: raw :: c_int , pub pasv_listen_fd : :: std :: os :: raw :: c_int , pub p_port_sockaddr : * mut vsf_sysutil_sockaddr , pub data_fd : :: std :: os :: raw :: c_int , pub data_progress : :: std :: os :: raw :: c_int , pub bw_rate_max : :: std :: os :: raw :: c_uint , pub bw_send_start_sec : :: std :: os :: raw :: c_long , pub bw_send_start_usec : :: std :: os :: raw :: c_long , pub is_anonymous : :: std :: os :: raw :: c_int , pub is_guest : :: std :: os :: raw :: c_int , pub user_str : mystr , pub anon_pass_str : mystr , pub restart_pos : filesize_t , pub is_ascii : :: std :: os :: raw :: c_int , pub rnfr_filename_str : mystr , pub abor_received : :: std :: os :: raw :: c_int , pub epsv_all : :: std :: os :: raw :: c_int , pub is_http : :: std :: os :: raw :: c_int , pub http_get_arg : mystr , pub p_visited_dir_list : * mut mystr_list , pub anon_ftp_uid : :: std :: os :: raw :: c_int , pub guest_user_uid : :: std :: os :: raw :: c_int , pub anon_upload_chown_uid : :: std :: os :: raw :: c_int , pub banned_email_str : mystr , pub email_passwords_str : mystr , pub userlist_str : mystr , pub banner_str : mystr , pub tcp_wrapper_ok : :: std :: os :: raw :: c_int , pub xferlog_fd : :: std :: os :: raw :: c_int , pub vsftpd_log_fd : :: std :: os :: raw :: c_int , pub remote_ip_str : mystr , pub log_type : :: std :: os :: raw :: c_ulong , pub log_start_sec : :: std :: os :: raw :: c_long , pub log_start_usec : :: std :: os :: raw :: c_long , pub log_str : mystr , pub transfer_size : filesize_t , pub ftp_cmd_str : mystr , pub ftp_arg_str : mystr , pub parent_fd : :: std :: os :: raw :: c_int , pub child_fd : :: std :: os :: raw :: c_int , pub num_clients : :: std :: os :: raw :: c_uint , pub num_this_ip : :: std :: os :: raw :: c_uint , pub home_str : mystr , pub control_use_ssl : :: std :: os :: raw :: c_int , pub data_use_ssl : :: std :: os :: raw :: c_int , pub p_ssl_ctx : * mut :: std :: os :: raw :: c_void , pub p_control_ssl : * mut :: std :: os :: raw :: c_void , pub p_data_ssl : * mut :: std :: os :: raw :: c_void , pub control_cert_digest : mystr , pub ssl_slave_active : :: std :: os :: raw :: c_int , pub ssl_slave_fd : :: std :: os :: raw :: c_int , pub ssl_consumer_fd : :: std :: os :: raw :: c_int , pub login_fails : :: std :: os :: raw :: c_uint ,
}
 # [test] fn bindgen_test_layout_vsf_session () {
assert_eq ! (:: std :: mem :: size_of :: < vsf_session > ( ) , 480usize , concat ! ( "Size of: " , stringify ! ( vsf_session ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < vsf_session > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( vsf_session ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . p_local_addr as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( p_local_addr ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . p_remote_addr as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( p_remote_addr ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . p_control_line_buf as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( p_control_line_buf ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . idle_timeout as * const _ as usize } , 24usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( idle_timeout ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . data_timeout as * const _ as usize } , 28usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( data_timeout ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . pasv_listen_fd as * const _ as usize } , 32usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( pasv_listen_fd ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . p_port_sockaddr as * const _ as usize } , 40usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( p_port_sockaddr ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . data_fd as * const _ as usize } , 48usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( data_fd ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . data_progress as * const _ as usize } , 52usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( data_progress ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . bw_rate_max as * const _ as usize } , 56usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( bw_rate_max ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . bw_send_start_sec as * const _ as usize } , 64usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( bw_send_start_sec ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . bw_send_start_usec as * const _ as usize } , 72usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( bw_send_start_usec ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . is_anonymous as * const _ as usize } , 80usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( is_anonymous ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . is_guest as * const _ as usize } , 84usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( is_guest ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . user_str as * const _ as usize } , 88usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( user_str ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . anon_pass_str as * const _ as usize } , 104usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( anon_pass_str ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . restart_pos as * const _ as usize } , 120usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( restart_pos ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . is_ascii as * const _ as usize } , 128usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( is_ascii ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . rnfr_filename_str as * const _ as usize } , 136usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( rnfr_filename_str ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . abor_received as * const _ as usize } , 152usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( abor_received ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . epsv_all as * const _ as usize } , 156usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( epsv_all ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . is_http as * const _ as usize } , 160usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( is_http ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . http_get_arg as * const _ as usize } , 168usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( http_get_arg ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . p_visited_dir_list as * const _ as usize } , 184usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( p_visited_dir_list ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . anon_ftp_uid as * const _ as usize } , 192usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( anon_ftp_uid ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . guest_user_uid as * const _ as usize } , 196usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( guest_user_uid ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . anon_upload_chown_uid as * const _ as usize } , 200usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( anon_upload_chown_uid ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . banned_email_str as * const _ as usize } , 208usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( banned_email_str ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . email_passwords_str as * const _ as usize } , 224usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( email_passwords_str ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . userlist_str as * const _ as usize } , 240usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( userlist_str ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . banner_str as * const _ as usize } , 256usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( banner_str ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . tcp_wrapper_ok as * const _ as usize } , 272usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( tcp_wrapper_ok ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . xferlog_fd as * const _ as usize } , 276usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( xferlog_fd ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . vsftpd_log_fd as * const _ as usize } , 280usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( vsftpd_log_fd ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . remote_ip_str as * const _ as usize } , 288usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( remote_ip_str ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . log_type as * const _ as usize } , 304usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( log_type ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . log_start_sec as * const _ as usize } , 312usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( log_start_sec ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . log_start_usec as * const _ as usize } , 320usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( log_start_usec ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . log_str as * const _ as usize } , 328usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( log_str ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . transfer_size as * const _ as usize } , 344usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( transfer_size ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . ftp_cmd_str as * const _ as usize } , 352usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( ftp_cmd_str ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . ftp_arg_str as * const _ as usize } , 368usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( ftp_arg_str ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . parent_fd as * const _ as usize } , 384usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( parent_fd ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . child_fd as * const _ as usize } , 388usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( child_fd ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . num_clients as * const _ as usize } , 392usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( num_clients ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . num_this_ip as * const _ as usize } , 396usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( num_this_ip ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . home_str as * const _ as usize } , 400usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( home_str ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . control_use_ssl as * const _ as usize } , 416usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( control_use_ssl ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . data_use_ssl as * const _ as usize } , 420usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( data_use_ssl ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . p_ssl_ctx as * const _ as usize } , 424usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( p_ssl_ctx ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . p_control_ssl as * const _ as usize } , 432usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( p_control_ssl ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . p_data_ssl as * const _ as usize } , 440usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( p_data_ssl ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . control_cert_digest as * const _ as usize } , 448usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( control_cert_digest ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . ssl_slave_active as * const _ as usize } , 464usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( ssl_slave_active ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . ssl_slave_fd as * const _ as usize } , 468usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( ssl_slave_fd ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . ssl_consumer_fd as * const _ as usize } , 472usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( ssl_consumer_fd ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . login_fails as * const _ as usize } , 476usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( login_fails ) )) ;

}
 extern "C" {
# [link_name = "\u{1}_Z8ssl_readP11vsf_sessionPvPcj"] pub fn ssl_read (p_sess : * mut vsf_session , p_ssl : * mut :: std :: os :: raw :: c_void , p_buf : * mut :: std :: os :: raw :: c_char , len : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z8ssl_peekP11vsf_sessionPvPcj"] pub fn ssl_peek (p_sess : * mut vsf_session , p_ssl : * mut :: std :: os :: raw :: c_void , p_buf : * mut :: std :: os :: raw :: c_char , len : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z9ssl_writePvPKcj"] pub fn ssl_write (p_ssl : * mut :: std :: os :: raw :: c_void , p_buf : * const :: std :: os :: raw :: c_char , len : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z13ssl_write_strPvPK5mystr"] pub fn ssl_write_str (p_ssl : * mut :: std :: os :: raw :: c_void , p_str : * const mystr) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z17ssl_read_into_strP11vsf_sessionPvP5mystr"] pub fn ssl_read_into_str (p_sess : * mut vsf_session , p_ssl : * mut :: std :: os :: raw :: c_void , p_str : * mut mystr) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z8ssl_initP11vsf_session"] pub fn ssl_init (p_sess : * mut vsf_session) ;

}
 extern "C" {
# [link_name = "\u{1}_Z10ssl_acceptP11vsf_sessioni"] pub fn ssl_accept (p_sess : * mut vsf_session , fd : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z14ssl_data_closeP11vsf_session"] pub fn ssl_data_close (p_sess : * mut vsf_session) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z21ssl_comm_channel_initP11vsf_session"] pub fn ssl_comm_channel_init (p_sess : * mut vsf_session) ;

}
 extern "C" {
# [link_name = "\u{1}_Z37ssl_comm_channel_set_consumer_contextP11vsf_session"] pub fn ssl_comm_channel_set_consumer_context (p_sess : * mut vsf_session) ;

}
 extern "C" {
# [link_name = "\u{1}_Z37ssl_comm_channel_set_producer_contextP11vsf_session"] pub fn ssl_comm_channel_set_producer_context (p_sess : * mut vsf_session) ;

}
 extern "C" {
# [link_name = "\u{1}_Z11handle_authP11vsf_session"] pub fn handle_auth (p_sess : * mut vsf_session) ;

}
 extern "C" {
# [link_name = "\u{1}_Z11handle_pbszP11vsf_session"] pub fn handle_pbsz (p_sess : * mut vsf_session) ;

}
 extern "C" {
# [link_name = "\u{1}_Z11handle_protP11vsf_session"] pub fn handle_prot (p_sess : * mut vsf_session) ;

}
 extern "C" {
# [link_name = "\u{1}_Z21ssl_control_handshakeP11vsf_session"] pub fn ssl_control_handshake (p_sess : * mut vsf_session) ;

}
 extern "C" {
# [link_name = "\u{1}_Z15ssl_add_entropyP11vsf_session"] pub fn ssl_add_entropy (p_sess : * mut vsf_session) ;

}
 extern "C" {
# [link_name = "\u{1}_Z9ssl_slaveP11vsf_session"] pub fn ssl_slave (p_sess : * mut vsf_session) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct vsf_client_launch {
pub num_children : :: std :: os :: raw :: c_uint , pub num_this_ip : :: std :: os :: raw :: c_uint ,
}
 # [test] fn bindgen_test_layout_vsf_client_launch () {
assert_eq ! (:: std :: mem :: size_of :: < vsf_client_launch > ( ) , 8usize , concat ! ( "Size of: " , stringify ! ( vsf_client_launch ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < vsf_client_launch > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( vsf_client_launch ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_client_launch > ( ) ) ) . num_children as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( vsf_client_launch ) , "::" , stringify ! ( num_children ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_client_launch > ( ) ) ) . num_this_ip as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( vsf_client_launch ) , "::" , stringify ! ( num_this_ip ) )) ;

}
 extern "C" {
# [link_name = "\u{1}_Z19vsf_standalone_mainv"] pub fn vsf_standalone_main () -> vsf_client_launch ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct mystr_list_node {
_unused : [u8 ; 0] ,
}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct mystr_list {
pub PRIVATE_HANDS_OFF_alloc_len : :: std :: os :: raw :: c_uint , pub PRIVATE_HANDS_OFF_list_len : :: std :: os :: raw :: c_uint , pub PRIVATE_HANDS_OFF_p_nodes : * mut mystr_list_node ,
}
 # [test] fn bindgen_test_layout_mystr_list () {
assert_eq ! (:: std :: mem :: size_of :: < mystr_list > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( mystr_list ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < mystr_list > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( mystr_list ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < mystr_list > ( ) ) ) . PRIVATE_HANDS_OFF_alloc_len as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( mystr_list ) , "::" , stringify ! ( PRIVATE_HANDS_OFF_alloc_len ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < mystr_list > ( ) ) ) . PRIVATE_HANDS_OFF_list_len as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( mystr_list ) , "::" , stringify ! ( PRIVATE_HANDS_OFF_list_len ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < mystr_list > ( ) ) ) . PRIVATE_HANDS_OFF_p_nodes as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( mystr_list ) , "::" , stringify ! ( PRIVATE_HANDS_OFF_p_nodes ) )) ;

}
 extern "C" {
# [link_name = "\u{1}_Z13str_list_freeP10mystr_list"] pub fn str_list_free (p_list : * mut mystr_list) ;

}
 extern "C" {
# [link_name = "\u{1}_Z12str_list_addP10mystr_listPK5mystrS3_"] pub fn str_list_add (p_list : * mut mystr_list , p_str : * const mystr , p_sort_key_str : * const mystr) ;

}
 extern "C" {
# [link_name = "\u{1}_Z13str_list_sortP10mystr_listi"] pub fn str_list_sort (p_list : * mut mystr_list , reverse : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
# [link_name = "\u{1}_Z19str_list_get_lengthPK10mystr_list"] pub fn str_list_get_length (p_list : * const mystr_list) -> :: std :: os :: raw :: c_uint ;

}
 extern "C" {
# [link_name = "\u{1}_Z21str_list_contains_strPK10mystr_listPK5mystr"] pub fn str_list_contains_str (p_list : * const mystr_list , p_str : * const mystr) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z17str_list_get_pstrPK10mystr_listj"] pub fn str_list_get_pstr (p_list : * const mystr_list , indexx : :: std :: os :: raw :: c_uint) -> * const mystr ;

}
 extern "C" {
# [link_name = "\u{1}_Z21vsf_sysdep_check_authP5mystrPKS_S2_"] pub fn vsf_sysdep_check_auth (p_user : * mut mystr , p_pass : * const mystr , p_remote_host : * const mystr) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z27vsf_sysdep_has_capabilitiesv"] pub fn vsf_sysdep_has_capabilities () -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z39vsf_sysdep_has_capabilities_as_non_rootv"] pub fn vsf_sysdep_has_capabilities_as_non_root () -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z28vsf_sysdep_keep_capabilitiesv"] pub fn vsf_sysdep_keep_capabilities () ;

}
 pub const ESysdepCapabilities_kCapabilityCAP_CHOWN : ESysdepCapabilities = 1 ;
 pub const ESysdepCapabilities_kCapabilityCAP_NET_BIND_SERVICE : ESysdepCapabilities = 2 ;
 pub type ESysdepCapabilities = u32 ;
 extern "C" {
# [link_name = "\u{1}_Z29vsf_sysdep_adopt_capabilitiesj"] pub fn vsf_sysdep_adopt_capabilities (caps : :: std :: os :: raw :: c_uint) ;

}
 extern "C" {
# [link_name = "\u{1}_Z20vsf_sysutil_sendfileiiPxxj"] pub fn vsf_sysutil_sendfile (out_fd : :: std :: os :: raw :: c_int , in_fd : :: std :: os :: raw :: c_int , p_offset : * mut filesize_t , num_send : filesize_t , max_chunk : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z29vsf_sysutil_setproctitle_initiPPKc"] pub fn vsf_sysutil_setproctitle_init (argc : :: std :: os :: raw :: c_int , argv : * mut * const :: std :: os :: raw :: c_char) ;

}
 extern "C" {
# [link_name = "\u{1}_Z24vsf_sysutil_setproctitlePKc"] pub fn vsf_sysutil_setproctitle (p_text : * const :: std :: os :: raw :: c_char) ;

}
 extern "C" {
# [link_name = "\u{1}_Z28vsf_sysutil_setproctitle_strPK5mystr"] pub fn vsf_sysutil_setproctitle_str (p_str : * const mystr) ;

}
 extern "C" {
# [link_name = "\u{1}_Z32vsf_sysutil_set_proctitle_prefixPK5mystr"] pub fn vsf_sysutil_set_proctitle_prefix (p_str : * const mystr) ;

}
 extern "C" {
# [link_name = "\u{1}_Z31vsf_sysutil_map_anon_pages_initv"] pub fn vsf_sysutil_map_anon_pages_init () ;

}
 extern "C" {
# [link_name = "\u{1}_Z26vsf_sysutil_map_anon_pagesj"] pub fn vsf_sysutil_map_anon_pages (length : :: std :: os :: raw :: c_uint) -> * mut :: std :: os :: raw :: c_void ;

}
 extern "C" {
# [link_name = "\u{1}_Z19vsf_sysutil_send_fdii"] pub fn vsf_sysutil_send_fd (sock_fd : :: std :: os :: raw :: c_int , send_fd : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
# [link_name = "\u{1}_Z19vsf_sysutil_recv_fdi"] pub fn vsf_sysutil_recv_fd (sock_fd : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z26vsf_set_die_if_parent_diesv"] pub fn vsf_set_die_if_parent_dies () ;

}
 extern "C" {
# [link_name = "\u{1}_Z27vsf_set_term_if_parent_diesv"] pub fn vsf_set_term_if_parent_dies () ;

}
 extern "C" {
# [link_name = "\u{1}_Z31vsf_sysutil_fork_isolate_failokv"] pub fn vsf_sysutil_fork_isolate_failok () -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z35vsf_sysutil_fork_isolate_all_failokv"] pub fn vsf_sysutil_fork_isolate_all_failok () -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z23vsf_sysutil_fork_newnetv"] pub fn vsf_sysutil_fork_newnet () -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z26vsf_sysutil_getpid_nocachev"] pub fn vsf_sysutil_getpid_nocache () -> :: std :: os :: raw :: c_int ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct vsf_sysutil_statbuf {
_unused : [u8 ; 0] ,
}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct vsf_sysutil_user {
_unused : [u8 ; 0] ,
}
 extern "C" {
# [link_name = "\u{1}_Z10str_getcwdP5mystr"] pub fn str_getcwd (p_str : * mut mystr) ;

}
 extern "C" {
# [link_name = "\u{1}_Z12str_readlinkP5mystrPKS_"] pub fn str_readlink (p_str : * mut mystr , p_filename_str : * const mystr) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z14str_write_loopPK5mystri"] pub fn str_write_loop (p_str : * const mystr , fd : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z13str_read_loopP5mystri"] pub fn str_read_loop (p_str : * mut mystr , fd : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z9str_mkdirPK5mystrj"] pub fn str_mkdir (p_str : * const mystr , mode : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z9str_rmdirPK5mystr"] pub fn str_rmdir (p_str : * const mystr) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z10str_unlinkPK5mystr"] pub fn str_unlink (p_str : * const mystr) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z9str_chdirPK5mystr"] pub fn str_chdir (p_str : * const mystr) -> :: std :: os :: raw :: c_int ;

}
 pub const EVSFSysStrOpenMode_kVSFSysStrOpenUnknown : EVSFSysStrOpenMode = 0 ;
 pub const EVSFSysStrOpenMode_kVSFSysStrOpenReadOnly : EVSFSysStrOpenMode = 1 ;
 pub type EVSFSysStrOpenMode = u32 ;
 extern "C" {
# [link_name = "\u{1}_Z8str_openPK5mystr18EVSFSysStrOpenMode"] pub fn str_open (p_str : * const mystr , mode : EVSFSysStrOpenMode) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z10str_createPK5mystr"] pub fn str_create (p_str : * const mystr) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z20str_create_exclusivePK5mystr"] pub fn str_create_exclusive (p_str : * const mystr) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z9str_chmodPK5mystrj"] pub fn str_chmod (p_str : * const mystr , mode : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z8str_statPK5mystrPP19vsf_sysutil_statbuf"] pub fn str_stat (p_str : * const mystr , p_ptr : * mut * mut vsf_sysutil_statbuf) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z9str_lstatPK5mystrPP19vsf_sysutil_statbuf"] pub fn str_lstat (p_str : * const mystr , p_ptr : * mut * mut vsf_sysutil_statbuf) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z10str_renamePK5mystrS1_"] pub fn str_rename (p_from_str : * const mystr , p_to_str : * const mystr) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z11str_opendirPK5mystr"] pub fn str_opendir (p_str : * const mystr) -> * mut vsf_sysutil_dir ;

}
 extern "C" {
# [link_name = "\u{1}_Z15str_next_direntP5mystrP15vsf_sysutil_dir"] pub fn str_next_dirent (p_filename_str : * mut mystr , p_dir : * mut vsf_sysutil_dir) ;

}
 extern "C" {
# [link_name = "\u{1}_Z12str_getpwnamPK5mystr"] pub fn str_getpwnam (p_user_str : * const mystr) -> * mut vsf_sysutil_user ;

}
 extern "C" {
# [link_name = "\u{1}_Z10str_syslogPK5mystri"] pub fn str_syslog (p_str : * const mystr , severe : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
# [link_name = "\u{1}_Z27vsf_sysutil_retval_is_errori"] pub fn vsf_sysutil_retval_is_error (retval : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 pub const EVSFSysUtilError_kVSFSysUtilErrUnknown : EVSFSysUtilError = 1 ;
 pub const EVSFSysUtilError_kVSFSysUtilErrADDRINUSE : EVSFSysUtilError = 2 ;
 pub const EVSFSysUtilError_kVSFSysUtilErrNOSYS : EVSFSysUtilError = 3 ;
 pub const EVSFSysUtilError_kVSFSysUtilErrINTR : EVSFSysUtilError = 4 ;
 pub const EVSFSysUtilError_kVSFSysUtilErrINVAL : EVSFSysUtilError = 5 ;
 pub const EVSFSysUtilError_kVSFSysUtilErrOPNOTSUPP : EVSFSysUtilError = 6 ;
 pub const EVSFSysUtilError_kVSFSysUtilErrACCES : EVSFSysUtilError = 7 ;
 pub const EVSFSysUtilError_kVSFSysUtilErrNOENT : EVSFSysUtilError = 8 ;
 pub type EVSFSysUtilError = u32 ;
 extern "C" {
# [link_name = "\u{1}_Z21vsf_sysutil_get_errorv"] pub fn vsf_sysutil_get_error () -> EVSFSysUtilError ;

}
 pub const EVSFSysUtilSignal_kVSFSysUtilSigALRM : EVSFSysUtilSignal = 1 ;
 pub const EVSFSysUtilSignal_kVSFSysUtilSigTERM : EVSFSysUtilSignal = 2 ;
 pub const EVSFSysUtilSignal_kVSFSysUtilSigCHLD : EVSFSysUtilSignal = 3 ;
 pub const EVSFSysUtilSignal_kVSFSysUtilSigPIPE : EVSFSysUtilSignal = 4 ;
 pub const EVSFSysUtilSignal_kVSFSysUtilSigURG : EVSFSysUtilSignal = 5 ;
 pub const EVSFSysUtilSignal_kVSFSysUtilSigHUP : EVSFSysUtilSignal = 6 ;
 pub type EVSFSysUtilSignal = u32 ;
 pub const EVSFSysUtilInterruptContext_kVSFSysUtilUnknown : EVSFSysUtilInterruptContext = 0 ;
 pub const EVSFSysUtilInterruptContext_kVSFSysUtilIO : EVSFSysUtilInterruptContext = 1 ;
 pub type EVSFSysUtilInterruptContext = u32 ;
 pub type vsf_sighandle_t = :: std :: option :: Option < unsafe extern "C" fn (arg1 : * mut :: std :: os :: raw :: c_void) > ;
 pub type vsf_async_sighandle_t = :: std :: option :: Option < unsafe extern "C" fn (arg1 : :: std :: os :: raw :: c_int) > ;
 pub type vsf_context_io_t = :: std :: option :: Option < unsafe extern "C" fn (arg1 : :: std :: os :: raw :: c_int , arg2 : :: std :: os :: raw :: c_int , arg3 : * mut :: std :: os :: raw :: c_void) > ;
 extern "C" {
# [link_name = "\u{1}_Z35vsf_sysutil_install_null_sighandler17EVSFSysUtilSignal"] pub fn vsf_sysutil_install_null_sighandler (sig : EVSFSysUtilSignal) ;

}
 extern "C" {
# [link_name = "\u{1}_Z30vsf_sysutil_install_sighandler17EVSFSysUtilSignalPFvPvES0_i"] pub fn vsf_sysutil_install_sighandler (arg1 : EVSFSysUtilSignal , handler : vsf_sighandle_t , p_private : * mut :: std :: os :: raw :: c_void , use_alarm : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
# [link_name = "\u{1}_Z36vsf_sysutil_install_async_sighandler17EVSFSysUtilSignalPFviE"] pub fn vsf_sysutil_install_async_sighandler (sig : EVSFSysUtilSignal , handler : vsf_async_sighandle_t) ;

}
 extern "C" {
# [link_name = "\u{1}_Z23vsf_sysutil_default_sig17EVSFSysUtilSignal"] pub fn vsf_sysutil_default_sig (sig : EVSFSysUtilSignal) ;

}
 extern "C" {
# [link_name = "\u{1}_Z30vsf_sysutil_install_io_handlerPFviiPvES_"] pub fn vsf_sysutil_install_io_handler (handler : vsf_context_io_t , p_private : * mut :: std :: os :: raw :: c_void) ;

}
 extern "C" {
# [link_name = "\u{1}_Z32vsf_sysutil_uninstall_io_handlerv"] pub fn vsf_sysutil_uninstall_io_handler () ;

}
 extern "C" {
# [link_name = "\u{1}_Z33vsf_sysutil_check_pending_actions27EVSFSysUtilInterruptContextii"] pub fn vsf_sysutil_check_pending_actions (context : EVSFSysUtilInterruptContext , retval : :: std :: os :: raw :: c_int , fd : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
# [link_name = "\u{1}_Z21vsf_sysutil_block_sig17EVSFSysUtilSignal"] pub fn vsf_sysutil_block_sig (sig : EVSFSysUtilSignal) ;

}
 extern "C" {
# [link_name = "\u{1}_Z23vsf_sysutil_unblock_sig17EVSFSysUtilSignal"] pub fn vsf_sysutil_unblock_sig (sig : EVSFSysUtilSignal) ;

}
 extern "C" {
# [link_name = "\u{1}_Z21vsf_sysutil_set_alarmj"] pub fn vsf_sysutil_set_alarm (trigger_seconds : :: std :: os :: raw :: c_uint) ;

}
 extern "C" {
# [link_name = "\u{1}_Z23vsf_sysutil_clear_alarmv"] pub fn vsf_sysutil_clear_alarm () ;

}
 extern "C" {
# [link_name = "\u{1}_Z18vsf_sysutil_getcwdPcj"] pub fn vsf_sysutil_getcwd (p_dest : * mut :: std :: os :: raw :: c_char , buf_size : :: std :: os :: raw :: c_uint) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
# [link_name = "\u{1}_Z17vsf_sysutil_mkdirPKcj"] pub fn vsf_sysutil_mkdir (p_dirname : * const :: std :: os :: raw :: c_char , mode : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z17vsf_sysutil_rmdirPKc"] pub fn vsf_sysutil_rmdir (p_dirname : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z17vsf_sysutil_chdirPKc"] pub fn vsf_sysutil_chdir (p_dirname : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z18vsf_sysutil_renamePKcS0_"] pub fn vsf_sysutil_rename (p_from : * const :: std :: os :: raw :: c_char , p_to : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z19vsf_sysutil_opendirPKc"] pub fn vsf_sysutil_opendir (p_dirname : * const :: std :: os :: raw :: c_char) -> * mut vsf_sysutil_dir ;

}
 extern "C" {
# [link_name = "\u{1}_Z20vsf_sysutil_closedirP15vsf_sysutil_dir"] pub fn vsf_sysutil_closedir (p_dir : * mut vsf_sysutil_dir) ;

}
 extern "C" {
# [link_name = "\u{1}_Z23vsf_sysutil_next_direntP15vsf_sysutil_dir"] pub fn vsf_sysutil_next_dirent (p_dir : * mut vsf_sysutil_dir) -> * const :: std :: os :: raw :: c_char ;

}
 pub const EVSFSysUtilOpenMode_kVSFSysUtilOpenReadOnly : EVSFSysUtilOpenMode = 1 ;
 pub const EVSFSysUtilOpenMode_kVSFSysUtilOpenWriteOnly : EVSFSysUtilOpenMode = 2 ;
 pub const EVSFSysUtilOpenMode_kVSFSysUtilOpenReadWrite : EVSFSysUtilOpenMode = 3 ;
 pub type EVSFSysUtilOpenMode = u32 ;
 extern "C" {
# [link_name = "\u{1}_Z21vsf_sysutil_open_filePKc19EVSFSysUtilOpenMode"] pub fn vsf_sysutil_open_file (p_filename : * const :: std :: os :: raw :: c_char , arg1 : EVSFSysUtilOpenMode) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z33vsf_sysutil_create_file_exclusivePKc"] pub fn vsf_sysutil_create_file_exclusive (p_filename : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z38vsf_sysutil_create_or_open_file_appendPKcj"] pub fn vsf_sysutil_create_or_open_file_append (p_filename : * const :: std :: os :: raw :: c_char , mode : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z31vsf_sysutil_create_or_open_filePKcj"] pub fn vsf_sysutil_create_or_open_file (p_filename : * const :: std :: os :: raw :: c_char , mode : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z18vsf_sysutil_dupfd2ii"] pub fn vsf_sysutil_dupfd2 (old_fd : :: std :: os :: raw :: c_int , new_fd : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
# [link_name = "\u{1}_Z17vsf_sysutil_closei"] pub fn vsf_sysutil_close (fd : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
# [link_name = "\u{1}_Z24vsf_sysutil_close_failoki"] pub fn vsf_sysutil_close_failok (fd : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z18vsf_sysutil_unlinkPKc"] pub fn vsf_sysutil_unlink (p_dead : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z24vsf_sysutil_write_accessPKc"] pub fn vsf_sysutil_write_access (p_filename : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z21vsf_sysutil_ftruncatei"] pub fn vsf_sysutil_ftruncate (fd : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
# [link_name = "\u{1}_Z20vsf_sysutil_lseek_toix"] pub fn vsf_sysutil_lseek_to (fd : :: std :: os :: raw :: c_int , seek_pos : filesize_t) ;

}
 extern "C" {
# [link_name = "\u{1}_Z21vsf_sysutil_lseek_endi"] pub fn vsf_sysutil_lseek_end (fd : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
# [link_name = "\u{1}_Z27vsf_sysutil_get_file_offseti"] pub fn vsf_sysutil_get_file_offset (file_fd : :: std :: os :: raw :: c_int) -> filesize_t ;

}
 extern "C" {
# [link_name = "\u{1}_Z16vsf_sysutil_readiPvj"] pub fn vsf_sysutil_read (fd : :: std :: os :: raw :: c_int , p_buf : * mut :: std :: os :: raw :: c_void , size : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z17vsf_sysutil_writeiPKvj"] pub fn vsf_sysutil_write (fd : :: std :: os :: raw :: c_int , p_buf : * const :: std :: os :: raw :: c_void , size : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z21vsf_sysutil_read_loopiPvj"] pub fn vsf_sysutil_read_loop (fd : :: std :: os :: raw :: c_int , p_buf : * mut :: std :: os :: raw :: c_void , size : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z22vsf_sysutil_write_loopiPKvj"] pub fn vsf_sysutil_write_loop (fd : :: std :: os :: raw :: c_int , p_buf : * const :: std :: os :: raw :: c_void , size : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z16vsf_sysutil_statPKcPP19vsf_sysutil_statbuf"] pub fn vsf_sysutil_stat (p_name : * const :: std :: os :: raw :: c_char , p_ptr : * mut * mut vsf_sysutil_statbuf) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z17vsf_sysutil_lstatPKcPP19vsf_sysutil_statbuf"] pub fn vsf_sysutil_lstat (p_name : * const :: std :: os :: raw :: c_char , p_ptr : * mut * mut vsf_sysutil_statbuf) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z17vsf_sysutil_fstatiPP19vsf_sysutil_statbuf"] pub fn vsf_sysutil_fstat (fd : :: std :: os :: raw :: c_int , p_ptr : * mut * mut vsf_sysutil_statbuf) ;

}
 extern "C" {
# [link_name = "\u{1}_Z20vsf_sysutil_dir_statPK15vsf_sysutil_dirPP19vsf_sysutil_statbuf"] pub fn vsf_sysutil_dir_stat (p_dir : * const vsf_sysutil_dir , p_ptr : * mut * mut vsf_sysutil_statbuf) ;

}
 extern "C" {
# [link_name = "\u{1}_Z30vsf_sysutil_statbuf_is_regfilePK19vsf_sysutil_statbuf"] pub fn vsf_sysutil_statbuf_is_regfile (p_stat : * const vsf_sysutil_statbuf) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z30vsf_sysutil_statbuf_is_symlinkPK19vsf_sysutil_statbuf"] pub fn vsf_sysutil_statbuf_is_symlink (p_stat : * const vsf_sysutil_statbuf) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z29vsf_sysutil_statbuf_is_socketPK19vsf_sysutil_statbuf"] pub fn vsf_sysutil_statbuf_is_socket (p_stat : * const vsf_sysutil_statbuf) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z26vsf_sysutil_statbuf_is_dirPK19vsf_sysutil_statbuf"] pub fn vsf_sysutil_statbuf_is_dir (p_stat : * const vsf_sysutil_statbuf) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z28vsf_sysutil_statbuf_get_sizePK19vsf_sysutil_statbuf"] pub fn vsf_sysutil_statbuf_get_size (p_stat : * const vsf_sysutil_statbuf) -> filesize_t ;

}
 extern "C" {
# [link_name = "\u{1}_Z29vsf_sysutil_statbuf_get_permsPK19vsf_sysutil_statbuf"] pub fn vsf_sysutil_statbuf_get_perms (p_stat : * const vsf_sysutil_statbuf) -> * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
# [link_name = "\u{1}_Z28vsf_sysutil_statbuf_get_datePK19vsf_sysutil_statbufil"] pub fn vsf_sysutil_statbuf_get_date (p_stat : * const vsf_sysutil_statbuf , use_localtime : :: std :: os :: raw :: c_int , curr_time : :: std :: os :: raw :: c_long) -> * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
# [link_name = "\u{1}_Z36vsf_sysutil_statbuf_get_numeric_datePK19vsf_sysutil_statbufi"] pub fn vsf_sysutil_statbuf_get_numeric_date (p_stat : * const vsf_sysutil_statbuf , use_localtime : :: std :: os :: raw :: c_int) -> * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
# [link_name = "\u{1}_Z29vsf_sysutil_statbuf_get_linksPK19vsf_sysutil_statbuf"] pub fn vsf_sysutil_statbuf_get_links (p_stat : * const vsf_sysutil_statbuf) -> :: std :: os :: raw :: c_uint ;

}
 extern "C" {
# [link_name = "\u{1}_Z27vsf_sysutil_statbuf_get_uidPK19vsf_sysutil_statbuf"] pub fn vsf_sysutil_statbuf_get_uid (p_stat : * const vsf_sysutil_statbuf) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z27vsf_sysutil_statbuf_get_gidPK19vsf_sysutil_statbuf"] pub fn vsf_sysutil_statbuf_get_gid (p_stat : * const vsf_sysutil_statbuf) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z37vsf_sysutil_statbuf_is_readable_otherPK19vsf_sysutil_statbuf"] pub fn vsf_sysutil_statbuf_is_readable_other (p_stat : * const vsf_sysutil_statbuf) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z37vsf_sysutil_statbuf_get_sortkey_mtimePK19vsf_sysutil_statbuf"] pub fn vsf_sysutil_statbuf_get_sortkey_mtime (p_stat : * const vsf_sysutil_statbuf) -> * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
# [link_name = "\u{1}_Z17vsf_sysutil_chmodPKcj"] pub fn vsf_sysutil_chmod (p_filename : * const :: std :: os :: raw :: c_char , mode : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z18vsf_sysutil_fchowniii"] pub fn vsf_sysutil_fchown (fd : :: std :: os :: raw :: c_int , uid : :: std :: os :: raw :: c_int , gid : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
# [link_name = "\u{1}_Z18vsf_sysutil_fchmodij"] pub fn vsf_sysutil_fchmod (fd : :: std :: os :: raw :: c_int , mode : :: std :: os :: raw :: c_uint) ;

}
 extern "C" {
# [link_name = "\u{1}_Z20vsf_sysutil_readlinkPKcPcj"] pub fn vsf_sysutil_readlink (p_filename : * const :: std :: os :: raw :: c_char , p_dest : * mut :: std :: os :: raw :: c_char , bufsiz : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z27vsf_sysutil_lock_file_writei"] pub fn vsf_sysutil_lock_file_write (fd : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z26vsf_sysutil_lock_file_readi"] pub fn vsf_sysutil_lock_file_read (fd : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z23vsf_sysutil_unlock_filei"] pub fn vsf_sysutil_unlock_file (fd : :: std :: os :: raw :: c_int) ;

}
 pub const EVSFSysUtilMapPermission_kVSFSysUtilMapProtReadOnly : EVSFSysUtilMapPermission = 1 ;
 pub const EVSFSysUtilMapPermission_kVSFSysUtilMapProtNone : EVSFSysUtilMapPermission = 2 ;
 pub type EVSFSysUtilMapPermission = u32 ;
 extern "C" {
# [link_name = "\u{1}_Z22vsf_sysutil_memprotectPvj24EVSFSysUtilMapPermission"] pub fn vsf_sysutil_memprotect (p_addr : * mut :: std :: os :: raw :: c_void , len : :: std :: os :: raw :: c_uint , perm : EVSFSysUtilMapPermission) ;

}
 extern "C" {
# [link_name = "\u{1}_Z20vsf_sysutil_memunmapPvj"] pub fn vsf_sysutil_memunmap (p_start : * mut :: std :: os :: raw :: c_void , length : :: std :: os :: raw :: c_uint) ;

}
 extern "C" {
# [link_name = "\u{1}_Z18vsf_sysutil_mallocj"] pub fn vsf_sysutil_malloc (size : :: std :: os :: raw :: c_uint) -> * mut :: std :: os :: raw :: c_void ;

}
 extern "C" {
# [link_name = "\u{1}_Z19vsf_sysutil_reallocPvj"] pub fn vsf_sysutil_realloc (p_ptr : * mut :: std :: os :: raw :: c_void , size : :: std :: os :: raw :: c_uint) -> * mut :: std :: os :: raw :: c_void ;

}
 extern "C" {
# [link_name = "\u{1}_Z16vsf_sysutil_freePv"] pub fn vsf_sysutil_free (p_ptr : * mut :: std :: os :: raw :: c_void) ;

}
 extern "C" {
# [link_name = "\u{1}_Z18vsf_sysutil_getpidv"] pub fn vsf_sysutil_getpid () -> :: std :: os :: raw :: c_uint ;

}
 extern "C" {
# [link_name = "\u{1}_Z21vsf_sysutil_post_forkv"] pub fn vsf_sysutil_post_fork () ;

}
 extern "C" {
# [link_name = "\u{1}_Z16vsf_sysutil_forkv"] pub fn vsf_sysutil_fork () -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z23vsf_sysutil_fork_failokv"] pub fn vsf_sysutil_fork_failok () -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z16vsf_sysutil_exiti"] pub fn vsf_sysutil_exit (exit_code : :: std :: os :: raw :: c_int) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct vsf_sysutil_wait_retval {
pub PRIVATE_HANDS_OFF_syscall_retval : :: std :: os :: raw :: c_int , pub PRIVATE_HANDS_OFF_exit_status : :: std :: os :: raw :: c_int ,
}
 # [test] fn bindgen_test_layout_vsf_sysutil_wait_retval () {
assert_eq ! (:: std :: mem :: size_of :: < vsf_sysutil_wait_retval > ( ) , 8usize , concat ! ( "Size of: " , stringify ! ( vsf_sysutil_wait_retval ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < vsf_sysutil_wait_retval > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( vsf_sysutil_wait_retval ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_sysutil_wait_retval > ( ) ) ) . PRIVATE_HANDS_OFF_syscall_retval as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( vsf_sysutil_wait_retval ) , "::" , stringify ! ( PRIVATE_HANDS_OFF_syscall_retval ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_sysutil_wait_retval > ( ) ) ) . PRIVATE_HANDS_OFF_exit_status as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( vsf_sysutil_wait_retval ) , "::" , stringify ! ( PRIVATE_HANDS_OFF_exit_status ) )) ;

}
 extern "C" {
# [link_name = "\u{1}_Z16vsf_sysutil_waitv"] pub fn vsf_sysutil_wait () -> vsf_sysutil_wait_retval ;

}
 extern "C" {
# [link_name = "\u{1}_Z25vsf_sysutil_wait_reap_onev"] pub fn vsf_sysutil_wait_reap_one () -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z27vsf_sysutil_wait_get_retvalPK23vsf_sysutil_wait_retval"] pub fn vsf_sysutil_wait_get_retval (p_waitret : * const vsf_sysutil_wait_retval) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z32vsf_sysutil_wait_exited_normallyPK23vsf_sysutil_wait_retval"] pub fn vsf_sysutil_wait_exited_normally (p_waitret : * const vsf_sysutil_wait_retval) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z29vsf_sysutil_wait_get_exitcodePK23vsf_sysutil_wait_retval"] pub fn vsf_sysutil_wait_get_exitcode (p_waitret : * const vsf_sysutil_wait_retval) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z18vsf_sysutil_strlenPKc"] pub fn vsf_sysutil_strlen (p_text : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_uint ;

}
 extern "C" {
# [link_name = "\u{1}_Z18vsf_sysutil_strdupPKc"] pub fn vsf_sysutil_strdup (p_str : * const :: std :: os :: raw :: c_char) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
# [link_name = "\u{1}_Z18vsf_sysutil_memclrPvj"] pub fn vsf_sysutil_memclr (p_dest : * mut :: std :: os :: raw :: c_void , size : :: std :: os :: raw :: c_uint) ;

}
 extern "C" {
# [link_name = "\u{1}_Z18vsf_sysutil_memcpyPvPKvj"] pub fn vsf_sysutil_memcpy (p_dest : * mut :: std :: os :: raw :: c_void , p_src : * const :: std :: os :: raw :: c_void , size : :: std :: os :: raw :: c_uint) ;

}
 extern "C" {
# [link_name = "\u{1}_Z18vsf_sysutil_strcpyPcPKcj"] pub fn vsf_sysutil_strcpy (p_dest : * mut :: std :: os :: raw :: c_char , p_src : * const :: std :: os :: raw :: c_char , maxsize : :: std :: os :: raw :: c_uint) ;

}
 extern "C" {
# [link_name = "\u{1}_Z18vsf_sysutil_memcmpPKvS0_j"] pub fn vsf_sysutil_memcmp (p_src1 : * const :: std :: os :: raw :: c_void , p_src2 : * const :: std :: os :: raw :: c_void , size : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z18vsf_sysutil_strcmpPKcS0_"] pub fn vsf_sysutil_strcmp (p_src1 : * const :: std :: os :: raw :: c_char , p_src2 : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z16vsf_sysutil_atoiPKc"] pub fn vsf_sysutil_atoi (p_str : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z27vsf_sysutil_a_to_filesize_tPKc"] pub fn vsf_sysutil_a_to_filesize_t (p_str : * const :: std :: os :: raw :: c_char) -> filesize_t ;

}
 extern "C" {
# [link_name = "\u{1}_Z24vsf_sysutil_ulong_to_strm"] pub fn vsf_sysutil_ulong_to_str (the_ulong : :: std :: os :: raw :: c_ulong) -> * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
# [link_name = "\u{1}_Z29vsf_sysutil_filesize_t_to_strx"] pub fn vsf_sysutil_filesize_t_to_str (the_filesize : filesize_t) -> * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
# [link_name = "\u{1}_Z25vsf_sysutil_double_to_strd"] pub fn vsf_sysutil_double_to_str (the_double : f64) -> * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
# [link_name = "\u{1}_Z25vsf_sysutil_uint_to_octalj"] pub fn vsf_sysutil_uint_to_octal (the_uint : :: std :: os :: raw :: c_uint) -> * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
# [link_name = "\u{1}_Z25vsf_sysutil_octal_to_uintPKc"] pub fn vsf_sysutil_octal_to_uint (p_str : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_uint ;

}
 extern "C" {
# [link_name = "\u{1}_Z19vsf_sysutil_toupperi"] pub fn vsf_sysutil_toupper (the_char : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z19vsf_sysutil_isspacei"] pub fn vsf_sysutil_isspace (the_char : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z19vsf_sysutil_isprinti"] pub fn vsf_sysutil_isprint (the_char : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z19vsf_sysutil_isalnumi"] pub fn vsf_sysutil_isalnum (the_char : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z19vsf_sysutil_isdigiti"] pub fn vsf_sysutil_isdigit (the_char : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct vsf_sysutil_socketpair_retval {
pub socket_one : :: std :: os :: raw :: c_int , pub socket_two : :: std :: os :: raw :: c_int ,
}
 # [test] fn bindgen_test_layout_vsf_sysutil_socketpair_retval () {
assert_eq ! (:: std :: mem :: size_of :: < vsf_sysutil_socketpair_retval > ( ) , 8usize , concat ! ( "Size of: " , stringify ! ( vsf_sysutil_socketpair_retval ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < vsf_sysutil_socketpair_retval > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( vsf_sysutil_socketpair_retval ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_sysutil_socketpair_retval > ( ) ) ) . socket_one as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( vsf_sysutil_socketpair_retval ) , "::" , stringify ! ( socket_one ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_sysutil_socketpair_retval > ( ) ) ) . socket_two as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( vsf_sysutil_socketpair_retval ) , "::" , stringify ! ( socket_two ) )) ;

}
 extern "C" {
# [link_name = "\u{1}_Z26vsf_sysutil_sockaddr_allocPP20vsf_sysutil_sockaddr"] pub fn vsf_sysutil_sockaddr_alloc (p_sockptr : * mut * mut vsf_sysutil_sockaddr) ;

}
 extern "C" {
# [link_name = "\u{1}_Z26vsf_sysutil_sockaddr_clearPP20vsf_sysutil_sockaddr"] pub fn vsf_sysutil_sockaddr_clear (p_sockptr : * mut * mut vsf_sysutil_sockaddr) ;

}
 extern "C" {
# [link_name = "\u{1}_Z31vsf_sysutil_sockaddr_alloc_ipv4PP20vsf_sysutil_sockaddr"] pub fn vsf_sysutil_sockaddr_alloc_ipv4 (p_sockptr : * mut * mut vsf_sysutil_sockaddr) ;

}
 extern "C" {
# [link_name = "\u{1}_Z31vsf_sysutil_sockaddr_alloc_ipv6PP20vsf_sysutil_sockaddr"] pub fn vsf_sysutil_sockaddr_alloc_ipv6 (p_sockptr : * mut * mut vsf_sysutil_sockaddr) ;

}
 extern "C" {
# [link_name = "\u{1}_Z26vsf_sysutil_sockaddr_clonePP20vsf_sysutil_sockaddrPKS_"] pub fn vsf_sysutil_sockaddr_clone (p_sockptr : * mut * mut vsf_sysutil_sockaddr , p_src : * const vsf_sysutil_sockaddr) ;

}
 extern "C" {
# [link_name = "\u{1}_Z31vsf_sysutil_sockaddr_addr_equalPK20vsf_sysutil_sockaddrS1_"] pub fn vsf_sysutil_sockaddr_addr_equal (p1 : * const vsf_sysutil_sockaddr , p2 : * const vsf_sysutil_sockaddr) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z28vsf_sysutil_sockaddr_is_ipv6PK20vsf_sysutil_sockaddr"] pub fn vsf_sysutil_sockaddr_is_ipv6 (p_sockaddr : * const vsf_sysutil_sockaddr) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z33vsf_sysutil_sockaddr_set_ipv4addrP20vsf_sysutil_sockaddrPKh"] pub fn vsf_sysutil_sockaddr_set_ipv4addr (p_sockptr : * mut vsf_sysutil_sockaddr , p_raw : * const :: std :: os :: raw :: c_uchar) ;

}
 extern "C" {
# [link_name = "\u{1}_Z33vsf_sysutil_sockaddr_set_ipv6addrP20vsf_sysutil_sockaddrPKh"] pub fn vsf_sysutil_sockaddr_set_ipv6addr (p_sockptr : * mut vsf_sysutil_sockaddr , p_raw : * const :: std :: os :: raw :: c_uchar) ;

}
 extern "C" {
# [link_name = "\u{1}_Z28vsf_sysutil_sockaddr_set_anyP20vsf_sysutil_sockaddr"] pub fn vsf_sysutil_sockaddr_set_any (p_sockaddr : * mut vsf_sysutil_sockaddr) ;

}
 extern "C" {
# [link_name = "\u{1}_Z29vsf_sysutil_sockaddr_get_portPK20vsf_sysutil_sockaddr"] pub fn vsf_sysutil_sockaddr_get_port (p_sockptr : * const vsf_sysutil_sockaddr) -> :: std :: os :: raw :: c_ushort ;

}
 extern "C" {
# [link_name = "\u{1}_Z29vsf_sysutil_sockaddr_set_portP20vsf_sysutil_sockaddrt"] pub fn vsf_sysutil_sockaddr_set_port (p_sockptr : * mut vsf_sysutil_sockaddr , the_port : :: std :: os :: raw :: c_ushort) ;

}
 extern "C" {
# [link_name = "\u{1}_Z28vsf_sysutil_is_port_reservedt"] pub fn vsf_sysutil_is_port_reserved (port : :: std :: os :: raw :: c_ushort) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z22vsf_sysutil_get_ipsockPK20vsf_sysutil_sockaddr"] pub fn vsf_sysutil_get_ipsock (p_sockaddr : * const vsf_sysutil_sockaddr) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z27vsf_sysutil_get_ipaddr_sizev"] pub fn vsf_sysutil_get_ipaddr_size () -> :: std :: os :: raw :: c_uint ;

}
 extern "C" {
# [link_name = "\u{1}_Z33vsf_sysutil_sockaddr_get_raw_addrP20vsf_sysutil_sockaddr"] pub fn vsf_sysutil_sockaddr_get_raw_addr (p_sockaddr : * mut vsf_sysutil_sockaddr) -> * mut :: std :: os :: raw :: c_void ;

}
 extern "C" {
# [link_name = "\u{1}_Z28vsf_sysutil_sockaddr_ipv6_v4PK20vsf_sysutil_sockaddr"] pub fn vsf_sysutil_sockaddr_ipv6_v4 (p_sockaddr : * const vsf_sysutil_sockaddr) -> * const :: std :: os :: raw :: c_void ;

}
 extern "C" {
# [link_name = "\u{1}_Z28vsf_sysutil_sockaddr_ipv4_v6PK20vsf_sysutil_sockaddr"] pub fn vsf_sysutil_sockaddr_ipv4_v6 (p_sockaddr : * const vsf_sysutil_sockaddr) -> * const :: std :: os :: raw :: c_void ;

}
 extern "C" {
# [link_name = "\u{1}_Z25vsf_sysutil_get_ipv4_sockv"] pub fn vsf_sysutil_get_ipv4_sock () -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z25vsf_sysutil_get_ipv6_sockv"] pub fn vsf_sysutil_get_ipv6_sock () -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z34vsf_sysutil_unix_stream_socketpairv"] pub fn vsf_sysutil_unix_stream_socketpair () -> vsf_sysutil_socketpair_retval ;

}
 extern "C" {
# [link_name = "\u{1}_Z16vsf_sysutil_bindiPK20vsf_sysutil_sockaddr"] pub fn vsf_sysutil_bind (fd : :: std :: os :: raw :: c_int , p_sockptr : * const vsf_sysutil_sockaddr) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z18vsf_sysutil_listenij"] pub fn vsf_sysutil_listen (fd : :: std :: os :: raw :: c_int , backlog : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z23vsf_sysutil_getsocknameiPP20vsf_sysutil_sockaddr"] pub fn vsf_sysutil_getsockname (fd : :: std :: os :: raw :: c_int , p_sockptr : * mut * mut vsf_sysutil_sockaddr) ;

}
 extern "C" {
# [link_name = "\u{1}_Z23vsf_sysutil_getpeernameiPP20vsf_sysutil_sockaddr"] pub fn vsf_sysutil_getpeername (fd : :: std :: os :: raw :: c_int , p_sockptr : * mut * mut vsf_sysutil_sockaddr) ;

}
 extern "C" {
# [link_name = "\u{1}_Z26vsf_sysutil_accept_timeoutiP20vsf_sysutil_sockaddrj"] pub fn vsf_sysutil_accept_timeout (fd : :: std :: os :: raw :: c_int , p_sockaddr : * mut vsf_sysutil_sockaddr , wait_seconds : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z27vsf_sysutil_connect_timeoutiPK20vsf_sysutil_sockaddrj"] pub fn vsf_sysutil_connect_timeout (fd : :: std :: os :: raw :: c_int , p_sockaddr : * const vsf_sysutil_sockaddr , wait_seconds : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z23vsf_sysutil_dns_resolvePP20vsf_sysutil_sockaddrPKc"] pub fn vsf_sysutil_dns_resolve (p_sockptr : * mut * mut vsf_sysutil_sockaddr , p_name : * const :: std :: os :: raw :: c_char) ;

}
 extern "C" {
# [link_name = "\u{1}_Z30vsf_sysutil_activate_keepalivei"] pub fn vsf_sysutil_activate_keepalive (fd : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
# [link_name = "\u{1}_Z32vsf_sysutil_set_iptos_throughputi"] pub fn vsf_sysutil_set_iptos_throughput (fd : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
# [link_name = "\u{1}_Z30vsf_sysutil_activate_reuseaddri"] pub fn vsf_sysutil_activate_reuseaddr (fd : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
# [link_name = "\u{1}_Z23vsf_sysutil_set_nodelayi"] pub fn vsf_sysutil_set_nodelay (fd : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
# [link_name = "\u{1}_Z27vsf_sysutil_activate_sigurgi"] pub fn vsf_sysutil_activate_sigurg (fd : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
# [link_name = "\u{1}_Z30vsf_sysutil_activate_oobinlinei"] pub fn vsf_sysutil_activate_oobinline (fd : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
# [link_name = "\u{1}_Z27vsf_sysutil_activate_lingeri"] pub fn vsf_sysutil_activate_linger (fd : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
# [link_name = "\u{1}_Z36vsf_sysutil_deactivate_linger_failoki"] pub fn vsf_sysutil_deactivate_linger_failok (fd : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
# [link_name = "\u{1}_Z28vsf_sysutil_activate_noblocki"] pub fn vsf_sysutil_activate_noblock (fd : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
# [link_name = "\u{1}_Z30vsf_sysutil_deactivate_noblocki"] pub fn vsf_sysutil_deactivate_noblock (fd : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
# [link_name = "\u{1}_Z27vsf_sysutil_shutdown_failoki"] pub fn vsf_sysutil_shutdown_failok (fd : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
# [link_name = "\u{1}_Z32vsf_sysutil_shutdown_read_failoki"] pub fn vsf_sysutil_shutdown_read_failok (fd : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
# [link_name = "\u{1}_Z21vsf_sysutil_recv_peekiPvj"] pub fn vsf_sysutil_recv_peek (fd : :: std :: os :: raw :: c_int , p_buf : * mut :: std :: os :: raw :: c_void , len : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z21vsf_sysutil_inet_ntopPK20vsf_sysutil_sockaddr"] pub fn vsf_sysutil_inet_ntop (p_sockptr : * const vsf_sysutil_sockaddr) -> * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
# [link_name = "\u{1}_Z21vsf_sysutil_inet_ntoaPKv"] pub fn vsf_sysutil_inet_ntoa (p_raw_addr : * const :: std :: os :: raw :: c_void) -> * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
# [link_name = "\u{1}_Z21vsf_sysutil_inet_atonPKcP20vsf_sysutil_sockaddr"] pub fn vsf_sysutil_inet_aton (p_text : * const :: std :: os :: raw :: c_char , p_addr : * mut vsf_sysutil_sockaddr) -> :: std :: os :: raw :: c_int ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct vsf_sysutil_group {
_unused : [u8 ; 0] ,
}
 extern "C" {
# [link_name = "\u{1}_Z20vsf_sysutil_getpwuidi"] pub fn vsf_sysutil_getpwuid (uid : :: std :: os :: raw :: c_int) -> * mut vsf_sysutil_user ;

}
 extern "C" {
# [link_name = "\u{1}_Z20vsf_sysutil_getpwnamPKc"] pub fn vsf_sysutil_getpwnam (p_user : * const :: std :: os :: raw :: c_char) -> * mut vsf_sysutil_user ;

}
 extern "C" {
# [link_name = "\u{1}_Z24vsf_sysutil_user_getnamePK16vsf_sysutil_user"] pub fn vsf_sysutil_user_getname (p_user : * const vsf_sysutil_user) -> * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
# [link_name = "\u{1}_Z28vsf_sysutil_user_get_homedirPK16vsf_sysutil_user"] pub fn vsf_sysutil_user_get_homedir (p_user : * const vsf_sysutil_user) -> * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
# [link_name = "\u{1}_Z23vsf_sysutil_user_getuidPK16vsf_sysutil_user"] pub fn vsf_sysutil_user_getuid (p_user : * const vsf_sysutil_user) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z23vsf_sysutil_user_getgidPK16vsf_sysutil_user"] pub fn vsf_sysutil_user_getgid (p_user : * const vsf_sysutil_user) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z20vsf_sysutil_getgrgidi"] pub fn vsf_sysutil_getgrgid (gid : :: std :: os :: raw :: c_int) -> * mut vsf_sysutil_group ;

}
 extern "C" {
# [link_name = "\u{1}_Z25vsf_sysutil_group_getnamePK17vsf_sysutil_group"] pub fn vsf_sysutil_group_getname (p_group : * const vsf_sysutil_group) -> * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
# [link_name = "\u{1}_Z23vsf_sysutil_getpagesizev"] pub fn vsf_sysutil_getpagesize () -> :: std :: os :: raw :: c_uint ;

}
 extern "C" {
# [link_name = "\u{1}_Z27vsf_sysutil_get_random_bytev"] pub fn vsf_sysutil_get_random_byte () -> :: std :: os :: raw :: c_uchar ;

}
 extern "C" {
# [link_name = "\u{1}_Z21vsf_sysutil_get_umaskv"] pub fn vsf_sysutil_get_umask () -> :: std :: os :: raw :: c_uint ;

}
 extern "C" {
# [link_name = "\u{1}_Z21vsf_sysutil_set_umaskj"] pub fn vsf_sysutil_set_umask (umask : :: std :: os :: raw :: c_uint) ;

}
 extern "C" {
# [link_name = "\u{1}_Z31vsf_sysutil_make_session_leaderv"] pub fn vsf_sysutil_make_session_leader () ;

}
 extern "C" {
# [link_name = "\u{1}_Z31vsf_sysutil_reopen_standard_fdsv"] pub fn vsf_sysutil_reopen_standard_fds () ;

}
 extern "C" {
# [link_name = "\u{1}_Z17vsf_sysutil_tzsetv"] pub fn vsf_sysutil_tzset () ;

}
 extern "C" {
# [link_name = "\u{1}_Z28vsf_sysutil_get_current_datev"] pub fn vsf_sysutil_get_current_date () -> * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
# [link_name = "\u{1}_Z17vsf_sysutil_qsortPvjjPFiPKvS1_E"] pub fn vsf_sysutil_qsort (p_base : * mut :: std :: os :: raw :: c_void , num_elem : :: std :: os :: raw :: c_uint , elem_size : :: std :: os :: raw :: c_uint , p_compar : :: std :: option :: Option < unsafe extern "C" fn ( arg1 : * const :: std :: os :: raw :: c_void , arg2 : * const :: std :: os :: raw :: c_void ) -> :: std :: os :: raw :: c_int >) ;

}
 extern "C" {
# [link_name = "\u{1}_Z18vsf_sysutil_getenvPKc"] pub fn vsf_sysutil_getenv (p_var : * const :: std :: os :: raw :: c_char) -> * mut :: std :: os :: raw :: c_char ;

}
 pub type exitfunc_t = :: std :: option :: Option < unsafe extern "C" fn () > ;
 extern "C" {
# [link_name = "\u{1}_Z25vsf_sysutil_set_exit_funcPFvvE"] pub fn vsf_sysutil_set_exit_func (exitfunc : exitfunc_t) ;

}
 extern "C" {
# [link_name = "\u{1}_Z18vsf_sysutil_getuidv"] pub fn vsf_sysutil_getuid () -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z19vsf_sysutil_openlogi"] pub fn vsf_sysutil_openlog (force : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
# [link_name = "\u{1}_Z18vsf_sysutil_syslogPKci"] pub fn vsf_sysutil_syslog (p_text : * const :: std :: os :: raw :: c_char , severe : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
# [link_name = "\u{1}_Z20vsf_sysutil_closelogv"] pub fn vsf_sysutil_closelog () ;

}
 extern "C" {
# [link_name = "\u{1}_Z27vsf_sysutil_running_as_rootv"] pub fn vsf_sysutil_running_as_root () -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z18vsf_sysutil_setuidPK16vsf_sysutil_user"] pub fn vsf_sysutil_setuid (p_user : * const vsf_sysutil_user) ;

}
 extern "C" {
# [link_name = "\u{1}_Z18vsf_sysutil_setgidPK16vsf_sysutil_user"] pub fn vsf_sysutil_setgid (p_user : * const vsf_sysutil_user) ;

}
 extern "C" {
# [link_name = "\u{1}_Z26vsf_sysutil_setuid_numerici"] pub fn vsf_sysutil_setuid_numeric (uid : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
# [link_name = "\u{1}_Z26vsf_sysutil_setgid_numerici"] pub fn vsf_sysutil_setgid_numeric (gid : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
# [link_name = "\u{1}_Z19vsf_sysutil_geteuidv"] pub fn vsf_sysutil_geteuid () -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z19vsf_sysutil_getegidv"] pub fn vsf_sysutil_getegid () -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z19vsf_sysutil_seteuidPK16vsf_sysutil_user"] pub fn vsf_sysutil_seteuid (p_user : * const vsf_sysutil_user) ;

}
 extern "C" {
# [link_name = "\u{1}_Z19vsf_sysutil_setegidPK16vsf_sysutil_user"] pub fn vsf_sysutil_setegid (p_user : * const vsf_sysutil_user) ;

}
 extern "C" {
# [link_name = "\u{1}_Z27vsf_sysutil_seteuid_numerici"] pub fn vsf_sysutil_seteuid_numeric (uid : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
# [link_name = "\u{1}_Z27vsf_sysutil_setegid_numerici"] pub fn vsf_sysutil_setegid_numeric (gid : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
# [link_name = "\u{1}_Z29vsf_sysutil_clear_supp_groupsv"] pub fn vsf_sysutil_clear_supp_groups () ;

}
 extern "C" {
# [link_name = "\u{1}_Z22vsf_sysutil_initgroupsPK16vsf_sysutil_user"] pub fn vsf_sysutil_initgroups (p_user : * const vsf_sysutil_user) ;

}
 extern "C" {
# [link_name = "\u{1}_Z18vsf_sysutil_chrootPKc"] pub fn vsf_sysutil_chroot (p_root_path : * const :: std :: os :: raw :: c_char) ;

}
 extern "C" {
# [link_name = "\u{1}_Z24vsf_sysutil_get_time_secv"] pub fn vsf_sysutil_get_time_sec () -> :: std :: os :: raw :: c_long ;

}
 extern "C" {
# [link_name = "\u{1}_Z25vsf_sysutil_get_time_usecv"] pub fn vsf_sysutil_get_time_usec () -> :: std :: os :: raw :: c_long ;

}
 extern "C" {
# [link_name = "\u{1}_Z22vsf_sysutil_parse_timePKc"] pub fn vsf_sysutil_parse_time (p_text : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_long ;

}
 extern "C" {
# [link_name = "\u{1}_Z17vsf_sysutil_sleepd"] pub fn vsf_sysutil_sleep (seconds : f64) ;

}
 extern "C" {
# [link_name = "\u{1}_Z22vsf_sysutil_setmodtimePKcli"] pub fn vsf_sysutil_setmodtime (p_file : * const :: std :: os :: raw :: c_char , the_time : :: std :: os :: raw :: c_long , is_localtime : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z35vsf_sysutil_set_address_space_limitm"] pub fn vsf_sysutil_set_address_space_limit (bytes : :: std :: os :: raw :: c_ulong) ;

}
 extern "C" {
# [link_name = "\u{1}_Z22vsf_sysutil_set_no_fdsv"] pub fn vsf_sysutil_set_no_fds () ;

}
 extern "C" {
# [link_name = "\u{1}_Z24vsf_sysutil_set_no_procsv"] pub fn vsf_sysutil_set_no_procs () ;

}
 extern "C" {
# [link_name = "\u{1}_Z18vsf_tcp_wrapper_oki"] pub fn vsf_tcp_wrapper_ok (remote_fd : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z22tunables_load_defaultsv"] pub fn tunables_load_defaults () ;

}
 extern "C" {
pub static mut tunable_anonymous_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_local_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_pasv_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_port_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_chroot_local_user : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_write_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_anon_upload_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_anon_mkdir_write_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_anon_other_write_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_chown_uploads : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_connect_from_port_20 : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_xferlog_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_dirmessage_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_anon_world_readable_only : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_async_abor_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_ascii_upload_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_ascii_download_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_one_process_model : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_xferlog_std_format : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_pasv_promiscuous : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_deny_email_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_chroot_list_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_setproctitle_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_text_userdb_names : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_ls_recurse_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_log_ftp_protocol : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_guest_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_userlist_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_userlist_deny : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_use_localtime : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_check_shell : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_hide_ids : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_listen : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_port_promiscuous : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_passwd_chroot_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_no_anon_password : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_tcp_wrappers : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_use_sendfile : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_force_dot_files : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_listen_ipv6 : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_dual_log_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_syslog_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_background : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_virtual_use_local_privs : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_session_support : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_download_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_dirlist_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_chmod_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_secure_email_list_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_run_as_launching_user : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_no_log_lock : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_ssl_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_allow_anon_ssl : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_force_local_logins_ssl : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_force_local_data_ssl : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_sslv2 : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_sslv3 : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_tlsv1 : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_tilde_user_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_force_anon_logins_ssl : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_force_anon_data_ssl : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_mdtm_write : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_lock_upload_files : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_pasv_addr_resolve : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_debug_ssl : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_require_cert : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_validate_cert : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_strict_ssl_read_eof : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_strict_ssl_write_shutdown : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_ssl_request_cert : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_delete_failed_uploads : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_implicit_ssl : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_ptrace_sandbox : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_require_ssl_reuse : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_isolate : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_isolate_network : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_ftp_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_http_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_seccomp_sandbox : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_allow_writeable_chroot : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_accept_timeout : :: std :: os :: raw :: c_uint ;

}
 extern "C" {
pub static mut tunable_connect_timeout : :: std :: os :: raw :: c_uint ;

}
 extern "C" {
pub static mut tunable_local_umask : :: std :: os :: raw :: c_uint ;

}
 extern "C" {
pub static mut tunable_anon_umask : :: std :: os :: raw :: c_uint ;

}
 extern "C" {
pub static mut tunable_ftp_data_port : :: std :: os :: raw :: c_uint ;

}
 extern "C" {
pub static mut tunable_idle_session_timeout : :: std :: os :: raw :: c_uint ;

}
 extern "C" {
pub static mut tunable_data_connection_timeout : :: std :: os :: raw :: c_uint ;

}
 extern "C" {
pub static mut tunable_pasv_min_port : :: std :: os :: raw :: c_uint ;

}
 extern "C" {
pub static mut tunable_pasv_max_port : :: std :: os :: raw :: c_uint ;

}
 extern "C" {
pub static mut tunable_anon_max_rate : :: std :: os :: raw :: c_uint ;

}
 extern "C" {
pub static mut tunable_local_max_rate : :: std :: os :: raw :: c_uint ;

}
 extern "C" {
pub static mut tunable_listen_port : :: std :: os :: raw :: c_uint ;

}
 extern "C" {
pub static mut tunable_max_clients : :: std :: os :: raw :: c_uint ;

}
 extern "C" {
pub static mut tunable_file_open_mode : :: std :: os :: raw :: c_uint ;

}
 extern "C" {
pub static mut tunable_max_per_ip : :: std :: os :: raw :: c_uint ;

}
 extern "C" {
pub static mut tunable_trans_chunk_size : :: std :: os :: raw :: c_uint ;

}
 extern "C" {
pub static mut tunable_delay_failed_login : :: std :: os :: raw :: c_uint ;

}
 extern "C" {
pub static mut tunable_delay_successful_login : :: std :: os :: raw :: c_uint ;

}
 extern "C" {
pub static mut tunable_max_login_fails : :: std :: os :: raw :: c_uint ;

}
 extern "C" {
pub static mut tunable_chown_upload_mode : :: std :: os :: raw :: c_uint ;

}
 extern "C" {
pub static mut tunable_secure_chroot_dir : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_ftp_username : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_chown_username : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_xferlog_file : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_vsftpd_log_file : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_message_file : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_nopriv_user : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_ftpd_banner : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_banned_email_file : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_chroot_list_file : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_pam_service_name : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_guest_username : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_userlist_file : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_anon_root : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_local_root : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_banner_file : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_pasv_address : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_listen_address : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_user_config_dir : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_listen_address6 : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_cmds_allowed : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_hide_file : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_deny_file : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_user_sub_token : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_email_password_file : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_rsa_cert_file : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_dsa_cert_file : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_ssl_ciphers : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_rsa_private_key_file : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_dsa_private_key_file : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_ca_certs_file : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_cmds_denied : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
# [link_name = "\u{1}_Z21vsf_two_process_startP11vsf_session"] pub fn vsf_two_process_start (p_sess : * mut vsf_session) ;

}
 extern "C" {
# [link_name = "\u{1}_Z21vsf_two_process_loginP11vsf_sessionPK5mystr"] pub fn vsf_two_process_login (p_sess : * mut vsf_session , p_pass_str : * const mystr) ;

}
 extern "C" {
# [link_name = "\u{1}_Z34vsf_two_process_get_priv_data_sockP11vsf_session"] pub fn vsf_two_process_get_priv_data_sock (p_sess : * mut vsf_session) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z28vsf_two_process_pasv_cleanupP11vsf_session"] pub fn vsf_two_process_pasv_cleanup (p_sess : * mut vsf_session) ;

}
 extern "C" {
# [link_name = "\u{1}_Z27vsf_two_process_pasv_activeP11vsf_session"] pub fn vsf_two_process_pasv_active (p_sess : * mut vsf_session) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z22vsf_two_process_listenP11vsf_session"] pub fn vsf_two_process_listen (p_sess : * mut vsf_session) -> :: std :: os :: raw :: c_ushort ;

}
 extern "C" {
# [link_name = "\u{1}_Z27vsf_two_process_get_pasv_fdP11vsf_session"] pub fn vsf_two_process_get_pasv_fd (p_sess : * mut vsf_session) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
# [link_name = "\u{1}_Z28vsf_two_process_chown_uploadP11vsf_sessioni"] pub fn vsf_two_process_chown_upload (p_sess : * mut vsf_session , fd : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
# [link_name = "\u{1}_Z3diePKc"] pub fn die (p_text : * const :: std :: os :: raw :: c_char) ;

}
 extern "C" {
# [link_name = "\u{1}_Z4die2PKcS0_"] pub fn die2 (p_text1 : * const :: std :: os :: raw :: c_char , p_text2 : * const :: std :: os :: raw :: c_char) ;

}
 extern "C" {
# [link_name = "\u{1}_Z3bugPKc"] pub fn bug (p_text : * const :: std :: os :: raw :: c_char) ;

}
 extern "C" {
# [link_name = "\u{1}_Z8vsf_exitPKc"] pub fn vsf_exit (p_text : * const :: std :: os :: raw :: c_char) ;

}
