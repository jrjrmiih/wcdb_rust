/*
 * Tencent is pleased to support the open source community by making
 * WCDB available.
 *
 * Copyright (C) 2017 THL A29 Limited, a Tencent company.
 * All rights reserved.
 *
 * Licensed under the BSD 3-Clause License (the "License"); you may not use
 * this file except in compliance with the License. You may obtain a copy of
 * the License at
 *
 *       https://opensource.org/licenses/BSD-3-Clause
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#pragma once

#include "WCDBRust.h"

#define WCDBRustStatementUpdateFuncName(funcName) WCDBRust(StatementUpdate, funcName)
#define WCDBRustStatementUpdateObjectMethod(funcName, ...) \
    WCDBRustObjectMethod(StatementUpdate, funcName, __VA_ARGS__)
#define WCDBRustStatementUpdateObjectMethodWithNoArg(funcName) \
    WCDBRustObjectMethodWithNoArg(StatementUpdate, funcName)
#define WCDBRustStatementUpdateClassMethodWithNoArg(funcName) \
    WCDBRustClassMethodWithNoArg(StatementUpdate, funcName)
#define WCDBRustStatementUpdateClassMethod(funcName, ...) \
    WCDBRustClassMethod(StatementUpdate, funcName, __VA_ARGS__)

void* WCDBRustStatementUpdateClassMethodWithNoArg(create);

void WCDBRustStatementUpdateClassMethod(configWith,
                                        void* self,
                                        void** expressions,
                                        int expressionsLength);
// void WCDBRustStatementUpdateClassMethod(configRecursive, jlong self);
//
void WCDBRustStatementUpdateClassMethod(configTable,
                                        void* self,
                                        WCDBRustObjectOrStringParameter(table));

void WCDBRustStatementUpdateClassMethod(configConfliction, void* self, int action);
void WCDBRustStatementUpdateClassMethod(configColumns,
                                        void* self,
                                        WCDBRustObjectOrStringArrayParameter(columns));
void WCDBRustStatementUpdateClassMethod(configValue,
                                        void* self,
                                        WCDBRustCommonValueParameter(value));
// void WCDBRustStatementUpdateClassMethod(configColumnsToValues,
//                                         void* self,
//                                         WCDBRustObjectOrStringArrayParameter(columns),
//                                         WCDBRustMultiTypeArrayParameter(values));
void WCDBRustStatementUpdateClassMethod(configColumnsToBindParameters,
                                        void* self,
                                        WCDBRustObjectOrStringArrayParameter(columns));
void WCDBRustStatementUpdateClassMethod(configCondition, void* self, void* condition);
void WCDBRustStatementUpdateClassMethod(configOrders, void* self, void** orders, size_t len);
void WCDBRustStatementUpdateClassMethod(configLimitRange,
                                        void* self,
                                        int fromType,
                                        long from,
                                        int toType,
                                        long to);
void WCDBRustStatementUpdateClassMethod(configLimitCount, void* self, int type, long limit);
void WCDBRustStatementUpdateClassMethod(configOffset, void* self, int type, long offset);
