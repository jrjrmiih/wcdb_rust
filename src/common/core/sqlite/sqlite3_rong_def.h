//
// Created by Qi on 2025/3/14.
//

#ifndef CPP_SQLITE3_RONG_DEF_H
#define CPP_SQLITE3_RONG_DEF_H

#include "../../../sqlcipher/sqlite3.h"
// 将 融云加密 需要的 sqlite 变量、结构体等手动声明一下

#ifdef __cplusplus
extern "C" {
#endif

#ifndef PENDING_BYTE
#ifdef SQLITE_OMIT_WSD
# define PENDING_BYTE     (0x40000000)
#else
# define PENDING_BYTE      0x40000000 //sqlite3PendingByte
#endif
#endif

#ifndef SQLITE_N_LIMIT
#define SQLITE_N_LIMIT (SQLITE_LIMIT_WORKER_THREADS+1)
#endif


typedef struct Btree Btree;
typedef struct PgHdr DbPage;
typedef struct Schema Schema;
typedef struct CollSeq CollSeq;
typedef struct PreUpdate PreUpdate;
typedef struct Pager Pager;
typedef struct VtabCtx VtabCtx;
typedef struct VTable VTable;
typedef struct BusyHandler BusyHandler;
typedef struct Savepoint Savepoint;
typedef struct Bitvec Bitvec;
typedef struct PagerSavepoint PagerSavepoint;
typedef struct PCache PCache;
typedef struct Wal Wal;
typedef struct Hash Hash;
typedef struct Vdbe Vdbe;
typedef struct Db Db;
typedef struct Lookaside Lookaside;
typedef struct PgHdr PgHdr;


#ifndef UINT32_TYPE
# ifdef HAVE_UINT32_T
#  define UINT32_TYPE uint32_t
# else
#  define UINT32_TYPE unsigned int
# endif
#endif
#ifndef UINT16_TYPE
# ifdef HAVE_UINT16_T
#  define UINT16_TYPE uint16_t
# else
#  define UINT16_TYPE unsigned short int
# endif
#endif
#ifndef INT16_TYPE
# ifdef HAVE_INT16_T
#  define INT16_TYPE int16_t
# else
#  define INT16_TYPE short int
# endif
#endif
#ifndef UINT8_TYPE
# ifdef HAVE_UINT8_T
#  define UINT8_TYPE uint8_t
# else
#  define UINT8_TYPE unsigned char
# endif
#endif
#ifndef INT8_TYPE
# ifdef HAVE_INT8_T
#  define INT8_TYPE int8_t
# else
#  define INT8_TYPE signed char
# endif
#endif
#ifndef LONGDOUBLE_TYPE
# define LONGDOUBLE_TYPE long double
#endif
typedef sqlite_int64 i64;          /* 8-byte signed integer */
typedef sqlite_uint64 u64;         /* 8-byte unsigned integer */
typedef UINT32_TYPE u32;           /* 4-byte unsigned integer */
typedef UINT16_TYPE u16;           /* 2-byte unsigned integer */
typedef INT16_TYPE i16;            /* 2-byte signed integer */
typedef UINT8_TYPE u8;             /* 1-byte unsigned integer */
typedef INT8_TYPE i8;              /* 1-byte signed integer */
typedef sqlite_int64 i64;          /* 8-byte signed integer */
typedef sqlite_uint64 u64;         /* 8-byte unsigned integer */
typedef UINT32_TYPE u32;           /* 4-byte unsigned integer */
typedef UINT16_TYPE u16;           /* 2-byte unsigned integer */
typedef INT16_TYPE i16;            /* 2-byte signed integer */
typedef UINT8_TYPE u8;             /* 1-byte unsigned integer */
typedef INT8_TYPE i8;              /* 1-byte signed integer */


typedef u32 Pgno;




#ifdef SQLITE_USER_AUTHENTICATION
typedef int (*sqlite3_xauth)(void*,int,const char*,const char*,const char*,
                               const char*, const char*);
#else
typedef int (*sqlite3_xauth)(void*,int,const char*,const char*,const char*,
                             const char*);
#endif

typedef struct HashElem HashElem;
struct Hash {
    unsigned int htsize;      /* Number of buckets in the hash table */
    unsigned int count;       /* Number of entries in this table */
    HashElem *first;          /* The first element of the array */
    struct _ht {              /* the hash table */
        int count;                 /* Number of entries with this hash */
        HashElem *chain;           /* Pointer to first entry with this hash */
    } *ht;
};

typedef struct BusyHandler BusyHandler;
struct BusyHandler {
    int (*xBusyHandler)(void *,int);  /* The busy callback */
    void *pBusyArg;                   /* First arg to busy callback */
    int nBusy;                        /* Incremented with each busy call */
    u8 bExtraFileArg;                 /* Include sqlite3_file as callback arg */
};

typedef struct LookasideSlot LookasideSlot;
struct Lookaside {
    u32 bDisable;           /* Only operate the lookaside when zero */
    u16 sz;                 /* Size of each buffer in bytes */
    u8 bMalloced;           /* True if pStart obtained from sqlite3_malloc() */
    u32 nSlot;              /* Number of lookaside slots allocated */
    u32 anStat[3];          /* 0: hits.  1: size misses.  2: full misses */
    LookasideSlot *pInit;   /* List of buffers not previously used */
    LookasideSlot *pFree;   /* List of available buffers */
    void *pStart;           /* First byte of available memory space */
    void *pEnd;             /* First byte past end of available space */
};

struct Db {
    char *zDbSName;      /* Name of this database. (schema name, not filename) */
    Btree *pBt;          /* The B*Tree structure for this database file */
    u8 safety_level;     /* How aggressive at syncing data to disk */
    u8 bSyncSet;         /* True if "PRAGMA synchronous=N" has been run */
    Schema *pSchema;     /* Pointer to database schema (possibly shared) */
};
struct sqlite3 {
    sqlite3_vfs *pVfs;            /* OS Interface */
    Vdbe *pVdbe;           /* List of active virtual machines */
    CollSeq *pDfltColl;           /* The default collating sequence (BINARY) */
    sqlite3_mutex *mutex;         /* Connection mutex */
    Db *aDb;                      /* All backends */
    int nDb;                      /* Number of backends currently in use */
    u32 mDbFlags;                 /* flags recording internal state */
    u64 flags;                    /* flags settable by pragmas. See below */
    i64 lastRowid;                /* ROWID of most recent insert (see above) */
    i64 szMmap;                   /* Default mmap_size setting */
    u32 nSchemaLock;              /* Do not reset the schema when non-zero */
    unsigned int openFlags;       /* Flags passed to sqlite3_vfs.xOpen() */
    int errCode;                  /* Most recent error code (SQLITE_*) */
    int errMask;                  /* & result codes with this before returning */
    int iSysErrno;                /* Errno value from last system error */
    u16 dbOptFlags;               /* Flags to enable/disable optimizations */
    u8 enc;                       /* Text encoding */
    u8 autoCommit;                /* The auto-commit flag. */
    u8 temp_store;                /* 1: file 2: memory 0: default */
    u8 mallocFailed;              /* True if we have seen a malloc failure */
    u8 bBenignMalloc;             /* Do not require OOMs if true */
    u8 dfltLockMode;              /* Default locking-mode for attached dbs */
    signed char nextAutovac;      /* Autovac setting after VACUUM if >=0 */
    u8 suppressErr;               /* Do not issue error messages if true */
    u8 vtabOnConflict;            /* Value to return for s3_vtab_on_conflict() */
    u8 isTransactionSavepoint;    /* True if the outermost savepoint is a TS */
    u8 mTrace;                    /* zero or more SQLITE_TRACE flags */
    u8 noSharedCache;             /* True if no shared-cache backends */
    u8 nSqlExec;                  /* Number of pending OP_SqlExec opcodes */
    int nextPagesize;             /* Pagesize after VACUUM if >0 */
    u32 magic;                    /* Magic number for detect library misuse */
    int nChange;                  /* Value returned by sqlite3_changes() */
    int nTotalChange;             /* Value returned by sqlite3_total_changes() */
    int aLimit[SQLITE_N_LIMIT];   /* Limits */
    int nMaxSorterMmap;           /* Maximum size of regions mapped by sorter */
    struct sqlite3InitInfo {      /* Information used during initialization */
        int newTnum;                /* Rootpage of table being initialized */
        u8 iDb;                     /* Which db file is being initialized */
        u8 busy;                    /* TRUE if currently initializing */
        unsigned orphanTrigger : 1; /* Last statement is orphaned TEMP trigger */
        unsigned imposterTable : 1; /* Building an imposter table */
        unsigned reopenMemdb : 1;   /* ATTACH is really a reopen using MemDB */
    } init;
    int nVdbeActive;              /* Number of VDBEs currently running */
    int nVdbeRead;                /* Number of active VDBEs that read or write */
    int nVdbeWrite;               /* Number of active VDBEs that read and write */
    int nVdbeExec;                /* Number of nested calls to VdbeExec() */
    int nVDestroy;                /* Number of active OP_VDestroy operations */
    int nExtension;               /* Number of loaded extensions */
    void **aExtension;            /* Array of shared library handles */
    int (*xTrace)(u32,void*,void*,void*);     /* Trace function */
    void *pTraceArg;                          /* Argument to the trace function */
#ifndef SQLITE_OMIT_DEPRECATED
    void (*xProfile)(void*,const char*,u64);  /* Profiling function */
  void *pProfileArg;                        /* Argument to profile function */
#endif
    void *pCommitArg;                 /* Argument to xCommitCallback() */
    int (*xCommitCallback)(void*);    /* Invoked at every commit. */
    void *pRollbackArg;               /* Argument to xRollbackCallback() */
    void (*xRollbackCallback)(void*); /* Invoked at every commit. */
    void *pUpdateArg;
    void (*xUpdateCallback)(void*,int, const char*,const char*,sqlite_int64);
#ifdef SQLITE_ENABLE_PREUPDATE_HOOK
    void *pPreUpdateArg;          /* First argument to xPreUpdateCallback */
    void (*xPreUpdateCallback)(   /* Registered using sqlite3_preupdate_hook() */
            void*,sqlite3*,int,char const*,char const*,sqlite3_int64,sqlite3_int64
    );
    PreUpdate *pPreUpdate;        /* Context for active pre-update callback */
#endif /* SQLITE_ENABLE_PREUPDATE_HOOK */
#ifndef SQLITE_OMIT_WAL
    int (*xWalCallback)(void *, sqlite3 *, const char *, int);
    void *pWalArg;
#ifdef SQLITE_WCDB_CHECKPOINT_HANDLER
    void (*xCheckpointCallback)(void *, sqlite3 *, const char *);
    void *pCheckpointArg;
#endif
#endif
    void(*xCollNeeded)(void*,sqlite3*,int eTextRep,const char*);
    void(*xCollNeeded16)(void*,sqlite3*,int eTextRep,const void*);
    void *pCollNeededArg;
    sqlite3_value *pErr;          /* Most recent error message */
    union {
#ifdef SQLITE_HAS_STDATOMIC
        atomic_int isInterrupted;
#else
        volatile int isInterrupted; /* True if sqlite3_interrupt has been called */
        double notUsed1;            /* Spacer */
#endif
    } u1;
#ifdef SQLITE_HAS_STDATOMIC
    atomic_int suspended;         /* True if sqlite_suspend has been called */
  atomic_int unimpeded;         /* True if interrupt and suspend are ignorable */
#else
    volatile int suspended;       /* True if sqlite_suspend has been called */
    volatile int unimpeded;       /* True if interrupt and suspend are ignorable */
#endif
    Lookaside lookaside;          /* Lookaside malloc configuration */
#ifndef SQLITE_OMIT_AUTHORIZATION
    sqlite3_xauth xAuth;          /* Access authorization function */
    void *pAuthArg;               /* 1st argument to the access auth function */
#endif
#ifndef SQLITE_OMIT_PROGRESS_CALLBACK
    int (*xProgress)(void *);     /* The progress callback */
    void *pProgressArg;           /* Argument to the progress callback */
    unsigned nProgressOps;        /* Number of opcodes for progress callback */
#endif
#ifndef SQLITE_OMIT_VIRTUALTABLE
    int nVTrans;                  /* Allocated size of aVTrans */
    Hash aModule;                 /* populated by sqlite3_create_module() */
    VtabCtx *pVtabCtx;            /* Context for active vtab connect/create */
    VTable **aVTrans;             /* Virtual tables with open transactions */
    VTable *pDisconnect;          /* Disconnect these in next sqlite3_prepare() */
#endif
    Hash aFunc;                   /* Hash table of connection functions */
    Hash aCollSeq;                /* All collating sequences */
    BusyHandler busyHandler;      /* Busy callback */
    Db aDbStatic[2];              /* Static space for the 2 default backends */
    Savepoint *pSavepoint;        /* List of active savepoints */
    int busyTimeout;              /* Busy handler timeout, in msec */
    int nSavepoint;               /* Number of non-transaction savepoints */
    int nStatement;               /* Number of nested statement-transactions  */
    i64 nDeferredCons;            /* Net deferred constraints this transaction. */
    i64 nDeferredImmCons;         /* Net deferred immediate constraints */
    int *pnBytesFreed;            /* If not NULL, increment this in DbFree() */
#ifdef SQLITE_ENABLE_UNLOCK_NOTIFY
    /* The following variables are all protected by the STATIC_MASTER
  ** mutex, not by sqlite3.mutex. They are used by code in notify.c.
  **
  ** When X.pUnlockConnection==Y, that means that X is waiting for Y to
  ** unlock so that it can proceed.
  **
  ** When X.pBlockingConnection==Y, that means that something that X tried
  ** tried to do recently failed with an SQLITE_LOCKED error due to locks
  ** held by Y.
  */
  sqlite3 *pBlockingConnection; /* Connection that caused SQLITE_LOCKED */
  sqlite3 *pUnlockConnection;           /* Connection to watch for unlock */
  void *pUnlockArg;                     /* Argument to xUnlockNotify */
  void (*xUnlockNotify)(void **, int);  /* Unlock notify callback */
  sqlite3 *pNextBlocked;        /* Next in list of all blocked connections */
#endif
#ifdef SQLITE_USER_AUTHENTICATION
    sqlite3_userauth auth;        /* User authentication information */
#endif
#ifdef SQLITE_WCDB
    u8 revertCommit;
#endif
};

struct Pager {
    sqlite3_vfs *pVfs;          /* OS functions to use for IO */
    u8 exclusiveMode;           /* Boolean. True if locking_mode==EXCLUSIVE */
    u8 journalMode;             /* One of the PAGER_JOURNALMODE_* values */
    u8 useJournal;              /* Use a rollback journal on this file */
    u8 noSync;                  /* Do not sync the journal if true */
    u8 fullSync;                /* Do extra syncs of the journal for robustness */
    u8 extraSync;               /* sync directory after journal delete */
    u8 syncFlags;               /* SYNC_NORMAL or SYNC_FULL otherwise */
    u8 walSyncFlags;            /* See description above */
    u8 tempFile;                /* zFilename is a temporary or immutable file */
    u8 noLock;                  /* Do not lock (except in WAL mode) */
    u8 readOnly;                /* True for a read-only database */
    u8 memDb;                   /* True to inhibit all file I/O */

    /**************************************************************************
    ** The following block contains those class members that change during
    ** routine operation.  Class members not in this block are either fixed
    ** when the pager is first created or else only change when there is a
    ** significant mode change (such as changing the page_size, locking_mode,
    ** or the journal_mode).  From another view, these class members describe
    ** the "state" of the pager, while other class members describe the
    ** "configuration" of the pager.
    */
    u8 eState;                  /* Pager state (OPEN, READER, WRITER_LOCKED..) */
    u8 eLock;                   /* Current lock held on database file */
    u8 changeCountDone;         /* Set after incrementing the change-counter */
    u8 setMaster;               /* True if a m-j name has been written to jrnl */
    u8 doNotSpill;              /* Do not spill the cache when non-zero */
    u8 subjInMemory;            /* True to use in-memory sub-journals */
    u8 bUseFetch;               /* True to use xFetch() */
    u8 hasHeldSharedLock;       /* True if a shared lock has ever been held */
    Pgno dbSize;                /* Number of pages in the database */
    Pgno dbOrigSize;            /* dbSize before the current transaction */
    Pgno dbFileSize;            /* Number of pages in the database file */
    Pgno dbHintSize;            /* Value passed to FCNTL_SIZE_HINT call */
    int errCode;                /* One of several kinds of errors */
    int nRec;                   /* Pages journalled since last j-header written */
    u32 cksumInit;              /* Quasi-random value added to every checksum */
    u32 nSubRec;                /* Number of records written to sub-journal */
    Bitvec *pInJournal;         /* One bit for each page in the database file */
    sqlite3_file *fd;           /* File descriptor for database */
    sqlite3_file *jfd;          /* File descriptor for main journal */
    sqlite3_file *sjfd;         /* File descriptor for sub-journal */
    i64 journalOff;             /* Current write offset in the journal file */
    i64 journalHdr;             /* Byte offset to previous journal header */
    sqlite3_backup *pBackup;    /* Pointer to list of ongoing backup processes */
    PagerSavepoint *aSavepoint; /* Array of active savepoints */
    int nSavepoint;             /* Number of elements in aSavepoint[] */
    u32 iDataVersion;           /* Changes whenever database content changes */
    char dbFileVers[16];        /* Changes whenever database file changes */

    int nMmapOut;               /* Number of mmap pages currently outstanding */
    sqlite3_int64 szMmap;       /* Desired maximum mmap size */
    PgHdr *pMmapFreelist;       /* List of free mmap page headers (pDirty) */
    /*
    ** End of the routinely-changing class members
    ***************************************************************************/

    u16 nExtra;                 /* Add this many bytes to each in-memory page */
    i16 nReserve;               /* Number of unused bytes at end of each page */
    u32 vfsFlags;               /* Flags for sqlite3_vfs.xOpen() */
    u32 sectorSize;             /* Assumed sector size during rollback */
    int pageSize;               /* Number of bytes in a page */
    Pgno mxPgno;                /* Maximum allowed size of the database */
    i64 journalSizeLimit;       /* Size limit for persistent journal files */
    char *zFilename;            /* Name of the database file */
    char *zJournal;             /* Name of the journal file */
    int (*xBusyHandler)(void*); /* Function to call when busy */
    void *pBusyHandlerArg;      /* Context argument for xBusyHandler */
    int aStat[4];               /* Total cache hits, misses, writes, spills */
#ifdef SQLITE_TEST
    int nRead;                  /* Database pages read */
#endif
    void (*xReiniter)(DbPage*); /* Call this routine when reloading pages */
    int (*xGet)(Pager*,Pgno,DbPage**,int); /* Routine to fetch a patch */
#ifdef SQLITE_HAS_CODEC
    void *(*xCodec)(void*,void*,Pgno,int); /* Routine for en/decoding data */
    void (*xCodecSizeChng)(void*,int,int); /* Notify of page size changes */
    void (*xCodecFree)(void*);             /* Destructor for the codec */
    void *pCodec;               /* First argument to xCodec... methods */
#endif
    char *pTmpSpace;            /* Pager.pageSize bytes of space for tmp use */
    PCache *pPCache;            /* Pointer to page cache object */
#ifndef SQLITE_OMIT_WAL
    Wal *pWal;                  /* Write-ahead log used by "journal_mode=wal" */
    char *zWal;                 /* File name for write-ahead log */
#endif
};


SQLITE_API int sqlite3_rekey_v2(
        sqlite3 *db,                   /* Database to be rekeyed */
        const char *zDbName,           /* Name of the database */
        const void *pKey, int nKey     /* The new key */
);


extern Pager *sqlite3BtreePager(Btree *p);
extern int sqlite3BtreeBeginTrans(Btree*,int,int*);
extern void sqlite3PagerPagecount(Pager*, int*);
extern int sqlite3PagerGet(Pager *pPager, Pgno pgno, DbPage **ppPage, int clrFlag);
extern int sqlite3PagerWrite(DbPage*);
extern void sqlite3PagerUnref(DbPage*);
extern int sqlite3BtreeCommit(Btree*);
extern int sqlite3BtreeRollback(Btree*,int,int);

#ifdef __cplusplus
}
#endif

#endif //CPP_SQLITE3_RONG_DEF_H
