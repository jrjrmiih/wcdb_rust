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

#include "OrderingTermRust.h"

#include "OrderingTermBridge.h"

void* WCDBRustOrderingTermClassMethod(create, int type, void* expression) {
    CPPCommonValue common_expression;
    common_expression.type = type;
    common_expression.intValue = (long long)expression;
    return (void*)WCDBOrderingTermCreate2(common_expression).innerValue;
}

// jlong WCDBRustOrderingTermClassMethod(create, jint type, jlong expression)
//{
//     CPPCommonValue common_expression;
//     common_expression.type = type;
//     common_expression.intValue = expression;
//     return (jlong) WCDBOrderingTermCreate2(common_expression).innerValue;
// }
//
// void WCDBRustOrderingTermClassMethod(configCollation, jlong object, jstring collation)
//{
//     WCDBRustBridgeStruct(CPPOrderingTerm, object);
//     WCDBRustGetStringCritical(collation);
//     WCDBOrderingTermConfigCollation(objectStruct, collationString);
//     WCDBRustReleaseStringCritical(collation);
// }

void WCDBRustOrderingTermClassMethod(configOrder, void* object, int order) {
    WCDBRustBridgeStruct(CPPOrderingTerm, object);
    WCDBOrderingTermConfigOrder(objectStruct, order);
}