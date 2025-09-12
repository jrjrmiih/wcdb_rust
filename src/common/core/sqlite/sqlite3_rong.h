//
// Created by Qi on 2025/3/14.
//

#ifndef CPP_SQLITE3_RONG_H
#define CPP_SQLITE3_RONG_H

/************** Begin RongCloud protocol encrypt declaration ***********************/
#include "sqlite3_rong_def.h"
#ifdef SQLITE_HAS_CODEC

#ifdef __cplusplus
extern "C" {
#endif

#define RC_CRYPT_OFFSET 8
#define RC_DB_KEY_LENGTH   32
#define RC_DB_KEY_PADDING  0x33
typedef struct _CryptBlock {
    unsigned char* ReadKey;
    unsigned char* WriteKey;
    unsigned char* Data;
    int PageSize;
} CryptBlock, *RC_LPCryptBlock;
SQLITE_API int rc_sqlite3_key(sqlite3*, const void*, int);
SQLITE_API int rc_sqlite3_key_v2(sqlite3*, const char*, const void*, int);
SQLITE_API int rc_sqlite3_rekey(sqlite3*, const void*, int);
SQLITE_API int rc_sqlite3_rekey_v2(sqlite3*, const char*, const void*, int);
static int rc_sqlite3_find_db_index(sqlite3*, const char*);
static int rc_sqlite3CodecAttach(sqlite3*, int, const void*, int);
static unsigned char* rc_DeriveKey(const void*, int);
static RC_LPCryptBlock rc_CreateCryptBlock(unsigned char*, Pager*, RC_LPCryptBlock);
static void rc_DestroyCryptBlock(RC_LPCryptBlock);
static void* rc_sqlite3Codec(void*, void*, Pgno, int);
static void rc_sqlite3pager_set_codec(Pager*, void *(*xCodec)(void*, void*, Pgno, int), void*);
static void* rc_sqlite3pager_get_codecarg(Pager*);

#ifdef __cplusplus
}
#endif

#endif // SQLITE_HAS_CODEC
/************** End of RongCloud protocol encrypt declaration **********************/

#endif //CPP_SQLITE3_RONG_H
