// <copyright file="Node.cs" company="PlaceholderCompany">
// Copyright (c) PlaceholderCompany. All rights reserved.
// </copyright>

using Valkyrie.Core.Lexer;

namespace Valkyrie.Core.Ast;

public class Node
{
    public TokenType Type { get; }
}

public class ProgramNode : Node
{
    public List<StatementNode> Statements = [];
}


public class StatementNode: Node
{

}
