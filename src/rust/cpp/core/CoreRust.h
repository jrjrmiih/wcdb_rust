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

// #define WCDBRustCoreFuncName(funcName) WCDBRust(Core, funcName)
// #def ine WCDBRustCoreObjectMethod(funcName, ...)                                 \
//    WCDBRustObjectMethod(Core, funcName, __VA_ARGS__)
// #def ine WCDBRustCoreObjectMethodWithNoArg(funcName)                             \
//    WCDBRustObjectMethodWithNoArg(Core, funcName)
// #def ine WCDBRustCoreClassMethodWithNoArg(funcName)                              \
//    WCDBRustClassMethodWithNoArg(Core, funcName)
#define WCDBRustCoreClassMethod(funcName, ...) WCDBRustClassMethod(Core, funcName, __VA_ARGS__)

void* WCDBRustCoreClassMethod(createDatabase, const char* path);
void WCDBRustCoreClassMethod(setDefaultCipherConfig, int version);
// void WCDBRustCoreClassMethodWithNoArg(purgeAllDatabase);
// void WCDBRustCoreClassMethod(releaseSQLiteMemory, jint bytes);
// void WCDBRustCoreClassMethod(setSoftHeapLimit, jlong limit);
// void WCDBRustCoreClassMethod(setAutoCheckpointMinFrames, jint frames);
// jlong WCDBRustCoreClassMethodWithNoArg(getThreadedError);