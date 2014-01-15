Valkyrie C# 项目
=====================

Valkyrie 语言的 C# 实现项目

## 项目结构

```
valkyrie.cs/
├── Valkyrie.sln                    # C# 解决方案文件
├── Readme.md                       # 项目说明
├── License.md                      # 许可证文件
└── projects/                       # C# 包目录
    └── Valkyrie.Core/              # Valkyrie 语言核心实现
        ├── Valkyrie.Core.csproj    # 项目文件
        ├── ValkyrieLanguage.cs     # 语言核心类
        ├── Lexer.cs                # 词法分析器
        ├── Parser.cs               # 语法分析器
        └── ValkyrieLanguageTests.cs # 单元测试
```

## Valkyrie.Core 项目

Valkyrie.Core 是 Valkyrie 语言的核心 C# 实现，包含：

- **词法分析器 (Lexer)**: 将源代码分解为词法标记
- **语法分析器 (Parser)**: 构建抽象语法树 (AST)
- **语言核心 (ValkyrieLanguage)**: 提供解析和执行功能

### 功能特性

- 支持基本数据类型（数字、字符串、布尔值）
- 支持变量声明和赋值
- 支持函数定义
- 支持控制流语句（if、while）
- 支持表达式运算

## 快速开始

### 构建项目

```bash
dotnet build
```

### 运行测试

```bash
dotnet test
```

### 使用示例

```csharp
var language = new ValkyrieLanguage();

// 解析代码
var parseResult = language.Parse("let x = 10; x + 5;");

// 执行代码
var executeResult = language.Execute("let x = 10; x + 5;");
```

## 开发指南

### 项目结构
```
valkyrie.cs/
├── Valkyrie.sln                 # 解决方案文件
├── projects/
│   ├── Valkyrie.Core/           # 核心语言实现
│   │   ├── Valkyrie.Core.csproj
│   │   ├── ValkyrieLanguage.cs  # 语言核心类
│   │   ├── Lexer.cs             # 词法分析器
│   │   ├── Parser.cs            # 语法分析器
│   │   └── ValkyrieLanguageTests.cs # 单元测试
│   └── Valkyrie.Translator/     # 翻译器项目（待实现）
├── .editorconfig                # 代码风格配置
├── Directory.Build.props        # 项目构建配置
├── stylecop.json                # StyleCop代码分析配置
├── format.ps1                   # 代码格式化脚本
├── .github/workflows/dotnet.yml # CI/CD工作流
├── .gitignore                   # Git忽略文件
└── Readme.md                    # 项目说明
```

### 快速开始
1. 确保已安装.NET 8.0 SDK
2. 克隆项目并进入项目目录
3. 运行 `dotnet restore` 恢复依赖
4. 运行 `dotnet build` 构建项目
5. 运行 `dotnet test` 执行测试

### 代码格式化
项目配置了完整的代码格式化工具链：

#### 自动格式化
```powershell
# 使用PowerShell脚本自动格式化
.\format.ps1

# 或者直接使用dotnet format
dotnet format
```

#### 格式检查
```powershell
# 检查代码格式（不修改文件）
.\format.ps1 -CheckOnly

# 或者使用dotnet format检查
dotnet format --verify-no-changes
```

#### 代码风格规则
- **强制大括号**: if、for、while等语句必须使用大括号
- **命名规范**: 遵循PascalCase和camelCase命名约定
- **缩进规则**: 使用4个空格缩进
- **导入排序**: System命名空间导入优先

### 功能特性
- **词法分析**: 支持标识符、数字、字符串、操作符等标记
- **语法分析**: 构建抽象语法树，支持函数、变量、条件、循环等语句
- **错误处理**: 完善的错误报告机制
- **测试覆盖**: 完整的单元测试套件
- **代码质量**: 集成Roslyn分析器，确保代码质量

### CI/CD流程
项目配置了GitHub Actions工作流：
- **自动构建**: 在push和pull_request时自动构建
- **格式检查**: PR时自动检查代码格式
- **自动格式化**: dev分支push时自动格式化代码
- **多平台测试**: 支持Windows、Linux、macOS平台

### 开发流程
1. 修改代码后运行 `dotnet build` 确保编译通过
2. 运行 `dotnet test` 验证功能正确性
3. 运行 `dotnet format` 或 `./format.ps1` 格式化代码
4. 提交代码前确保所有测试通过和格式检查通过

## 许可证

详见 [License.md](License.md)
