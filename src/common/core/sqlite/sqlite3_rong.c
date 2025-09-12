//
// Created by Qi on 2025/3/14.
//
/************** Begin RongCloud protocol encrypt defination ***********************/
#ifdef SQLITE_HAS_CODEC

#ifdef __cplusplus
extern "C" {
#endif

#include "sqlite3_rong.h"
#include <stddef.h>
#include <string.h>

#define PAGER_MJ_PGNO(x) ((Pgno)((PENDING_BYTE/((x)->pageSize))+1))
SQLITE_API int rc_sqlite3_key(sqlite3 *db, const void *pKey, int keyLen) {
    return rc_sqlite3_key_v2(db, "main", pKey, keyLen);
}
int rc_sqlite3_key_v2(sqlite3 *db, const char *dbName, const void *pKey, int keyLen) {
    if (db && pKey && keyLen)
    {
        int index = rc_sqlite3_find_db_index(db, dbName);
        return rc_sqlite3CodecAttach(db, index, pKey, keyLen);
    }
    return SQLITE_ERROR;
}
int rc_sqlite3_rekey(sqlite3 *db, const void *pKey, int keyLen) {
    return rc_sqlite3_rekey_v2(db, "main", pKey, keyLen);
}
int rc_sqlite3_rekey_v2(sqlite3 *db, const char *dbName, const void *pKey, int keyLen) {
    if(db && pKey) {
        int db_index = rc_sqlite3_find_db_index(db, dbName);
        Btree *pbt = db->aDb[db_index].pBt;
        Pager *p = sqlite3BtreePager(pbt);
        RC_LPCryptBlock pBlock = (RC_LPCryptBlock) rc_sqlite3pager_get_codecarg(p);
        unsigned char * hKey = rc_DeriveKey(pKey, keyLen);
        int rc = SQLITE_ERROR;
        if (!pBlock && !hKey)
            return SQLITE_OK;
        if (!pBlock) {
            pBlock = rc_CreateCryptBlock(hKey, p, NULL);
            pBlock->ReadKey = 0;
            rc_sqlite3pager_set_codec(sqlite3BtreePager(pbt), rc_sqlite3Codec, pBlock);
        } else {
            pBlock->WriteKey = hKey;
        }

        rc = sqlite3BtreeBeginTrans(pbt, 1, NULL);
        if (!rc) {
            int nPage;
            Pgno nSkip = PAGER_MJ_PGNO(p);
            DbPage *pPage;
            Pgno n;
            sqlite3PagerPagecount(p, &nPage);
            for (n = 1; rc == SQLITE_OK && n <= (unsigned int)nPage; n++) {
                if (n == nSkip)
                    continue;
                rc = sqlite3PagerGet(p, n, &pPage, 0);
                if (!rc) {
                    rc = sqlite3PagerWrite(pPage);
                    sqlite3PagerUnref(pPage);
                }
            }
        }
        if (!rc) {
            rc = sqlite3BtreeCommit(pbt);
        }
        if (rc) {
            sqlite3BtreeRollback(pbt,SQLITE_OK,SQLITE_OK);
        }
        if (!rc) {
            if (pBlock->ReadKey)
            {
                sqlite3_free(pBlock->ReadKey);
            }
            pBlock->ReadKey = pBlock->WriteKey;
        } else {
            if (pBlock->WriteKey) {
                sqlite3_free(pBlock->WriteKey);
            }
            pBlock->WriteKey = pBlock->ReadKey;
        }
        if (!pBlock->ReadKey && !pBlock->WriteKey) {
            rc_sqlite3pager_set_codec(p, NULL, NULL);
            rc_DestroyCryptBlock(pBlock);
        }
        return rc;
    }
    return SQLITE_ERROR;
}
static int rc_sqlite3_encrypt(unsigned char *pData, unsigned int dataLen, unsigned char *key, unsigned int keyLen) {
    unsigned int i;
    for (i = 0; i < dataLen; ++i) {
        pData[i] ^= 0xce;
        pData[i] ^= key[i % keyLen];
    }
    return 0;
}
static int rc_sqlite3_decrypt(unsigned char *pData, unsigned int dataLen, unsigned char *key, unsigned int keyLen) {
    unsigned int i;
    for (i = 0; i < dataLen; ++i) {
        pData[i] ^= key[i % keyLen];
        pData[i] ^= 0xce;
    }
    return 0;
}
static int rc_sqlite3_find_db_index(sqlite3 *db, const char *zDb) {
    int db_index;
    if(zDb == NULL) {
        return 0;
    }
    for(db_index = 0; db_index < db->nDb; db_index++) {
        struct Db *pDb = &db->aDb[db_index];
        if(strcmp(pDb->zDbSName, zDb) == 0) {
            return db_index;
        }
    }
    return 0;
}
int rc_sqlite3CodecAttach(sqlite3 *db, int nDb, const void *pKey, int keyLen) {
    int rc = SQLITE_ERROR;
    unsigned char* hKey = 0;
    if (!pKey || !keyLen) {
        if (!nDb) {
            return SQLITE_OK;
        } else {
            RC_LPCryptBlock pBlock = (RC_LPCryptBlock) rc_sqlite3pager_get_codecarg(sqlite3BtreePager(db->aDb[0].pBt));
            if (!pBlock)
                return SQLITE_OK;
            if (!pBlock->ReadKey)
                return SQLITE_OK;
            memcpy(pBlock->ReadKey, &hKey, RC_DB_KEY_LENGTH);
        }
    } else {
        hKey = rc_DeriveKey(pKey, keyLen);
    }
    if (hKey) {
        RC_LPCryptBlock pBlock = rc_CreateCryptBlock(hKey, sqlite3BtreePager(db->aDb[nDb].pBt), NULL);
        rc_sqlite3pager_set_codec(sqlite3BtreePager(db->aDb[nDb].pBt), rc_sqlite3Codec, pBlock);
        rc = SQLITE_OK;
    }
    return rc;
}
static unsigned char* rc_DeriveKey(const void *pKey, int keyLen) {
    unsigned char *hKey = NULL;
    if (pKey == NULL || keyLen == 0) {
        return NULL;
    }
    hKey = sqlite3_malloc(RC_DB_KEY_LENGTH + 1);
    if (hKey == NULL) {
        return NULL;
    }
    hKey[RC_DB_KEY_LENGTH] = 0;
    if (keyLen < RC_DB_KEY_LENGTH) {
        memcpy(hKey, pKey, keyLen);
        memset(hKey+keyLen, RC_DB_KEY_PADDING, RC_DB_KEY_LENGTH-keyLen);
    } else {
        memcpy(hKey, pKey, RC_DB_KEY_LENGTH);
    }
    return hKey;
}
static RC_LPCryptBlock rc_CreateCryptBlock(unsigned char *hKey, Pager *pager, RC_LPCryptBlock pExisting) {
    RC_LPCryptBlock pBlock;
    if (!pExisting) {
        pBlock = sqlite3_malloc(sizeof(CryptBlock));
        memset(pBlock, 0, sizeof(CryptBlock));
        pBlock->ReadKey = hKey;
        pBlock->WriteKey = hKey;
        pBlock->PageSize = pager->pageSize;
        pBlock->Data = (unsigned char*) sqlite3_malloc(pBlock->PageSize + RC_CRYPT_OFFSET);
    } else {
        pBlock = pExisting;
        if (pBlock->PageSize != pager->pageSize && !pBlock->Data) {
            sqlite3_free(pBlock->Data);
            pBlock->PageSize = pager->pageSize;
            pBlock->Data = (unsigned char*) sqlite3_malloc(pBlock->PageSize + RC_CRYPT_OFFSET);
        }
    }
    memset(pBlock->Data, 0, pBlock->PageSize + RC_CRYPT_OFFSET);
    return pBlock;
}
static void rc_DestroyCryptBlock(RC_LPCryptBlock pBlock) {
    if (pBlock->ReadKey) {
        sqlite3_free(pBlock->ReadKey);
    }
    if (pBlock->WriteKey && pBlock->WriteKey != pBlock->ReadKey) {
        sqlite3_free(pBlock->WriteKey);
    }
    if (pBlock->Data) {
        sqlite3_free(pBlock->Data);
    }
    sqlite3_free(pBlock);
}
void* rc_sqlite3Codec(void *pArg, void *data, Pgno nPageNum, int nMode) {
    RC_LPCryptBlock pBlock = (RC_LPCryptBlock) pArg;
    unsigned int dwPageSize = 0;
    if (!pBlock)
        return data;
    switch (nMode) {
        case 0:
        case 2:
        case 3:
            if (!pBlock->ReadKey)
                break;

            dwPageSize = pBlock->PageSize;
            rc_sqlite3_decrypt(data, dwPageSize, pBlock->ReadKey, RC_DB_KEY_LENGTH);
            break;
        case 6:
            if (!pBlock->WriteKey)
                break;

            memcpy(pBlock->Data + RC_CRYPT_OFFSET, data, pBlock->PageSize);
            data = pBlock->Data + RC_CRYPT_OFFSET;
            dwPageSize = pBlock->PageSize;

            rc_sqlite3_encrypt(data, dwPageSize, pBlock->WriteKey, RC_DB_KEY_LENGTH);
            break;
        case 7:
            if (!pBlock->ReadKey)
                break;
            memcpy(pBlock->Data + RC_CRYPT_OFFSET, data, pBlock->PageSize);
            data = pBlock->Data + RC_CRYPT_OFFSET;
            dwPageSize = pBlock->PageSize;
            rc_sqlite3_encrypt(data, dwPageSize, pBlock->ReadKey, RC_DB_KEY_LENGTH);
            break;
    }
    return data;
}
void rc_sqlite3pager_set_codec(Pager *pPager, void *(*xCodec)(void *, void *, Pgno, int), void *pCodec) {
    pPager->xCodec = xCodec;
    pPager->pCodec = pCodec;
}
static void * rc_sqlite3pager_get_codecarg(Pager *pPager) {
    return (pPager->xCodec) ? pPager->pCodec : NULL;
}

#ifdef __cplusplus
}
#endif

#endif // SQLITE_HAS_CODEC
/************** End of RongCloud protocol encrypt defination **********************/
