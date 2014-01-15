// <copyright file="TokenType.cs" company="PlaceholderCompany">
// Copyright (c) PlaceholderCompany. All rights reserved.
// </copyright>

namespace Valkyrie.Core.Lexer;

/// <summary>
/// 标记类型.
/// </summary>
public enum TokenType
{
    /// <summary>
    /// 标识符
    /// </summary>
    SymbolXid,
    SymbolRaw,
    /// <summary>
    /// 关键字
    /// </summary>
    Keyword,
    Number,
    String,
    Boolean,
    Null,
    Operator,
    /// <summary>
    /// 文件结束
    /// </summary>
    Eof
}
