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

#define WCDBRustColumnDefFuncName(funcName) WCDBRust(ColumnDef, funcName)
#define WCDBRustColumnDefObjectMethod(funcName, ...) \
    WCDBRustObjectMethod(ColumnDef, funcName, __VA_ARGS__)
#define WCDBRustColumnDefClassMethodWithNoArg(funcName) \
    WCDBRustClassMethodWithNoArg(ColumnDef, funcName)
#define WCDBRustColumnDefClassMethod(funcName, ...) \
    WCDBRustClassMethod(ColumnDef, funcName, __VA_ARGS__)

void* WCDBRustColumnDefClassMethod(create, WCDBRustObjectOrStringParameter(column), int columnType);

void WCDBRustColumnDefClassMethod(constraint, void* columnDef, void* constraint);