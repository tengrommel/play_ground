# 第一章 新时代的语言

## 1.2 设计哲学

- 内存安全
- 零成本抽象
- 实用性

### 1.2.1 内存安全

类型系统提供了以下好处：

- 允许编译器侦测无意义甚至无效的代码，暴露程序中隐含的错误
- 可以为编译器提供有意义的类型信息，帮助优化代码
- 可以增强代码的可读性，更直白地阐述开发者的意图
- 提供了一定程度的高级抽象，提升开发效率

与haskell的不同

- Haskell是一门纯函数式编程语言，它的类型系统主要用于承载其"纯函数式"的思想，是范畴论的体现
- 而对Rust来说，它的类型系统要承载其"内存安全"的思想。所以还需要有一个安全内存管理模型，并通过类型系统表达出来，才能保证内存安全

内存安全 -> 不会出现内存访问错误

常见的内存错误
- 引用空指针
- 使用未初始化内存
- 释放后使用，也就是使用悬垂指针
- 缓冲区溢出，比如数组越界
- 非法释放已释放过的指针或未分配的指针，也就是重复释放

内存管理模型

- 所有权系统
> 每个被分配的内存都有一个独占其所有权的指针。只有当该指针被销毁时，其对应的内存才能随之被释放。
- 借用和生命周期
> 每个变量都有其生命周期，一旦超出生命周期，变量就会被自动释放。如果是借用，则可以通过标记生命周期参数供编译器检查的方式，防止出现悬垂指针，也就是释放后使用的情况。

Rust从Haskell的类型系统借鉴了以下特性：

- 没有空指针
- 默认不可变
- 表达式
- 高阶函数
- 代数数据类型
- 模式匹配
- 泛型
- trait和关联类型
- 本地类型推导

为了内存安全，Rust还具备以下独有的特性
- 反射类型（Affine Type），该类型用来表达Rust所有权中的Move语义
- 借用、生命周期

### 1.2.2 零成本抽象
> 编程语言如果想做高效开发，就必须拥有一定的抽象表达能力

### 1.2.3 实用性

- 实践性
    - 失败：对于失败的情况，可以使用断言工具。
    - 错误：对于错误，Rust提供了基于返回值的分层错误处理方式，比如Option<T>可以用来处理可能存在空值的情况，而Result<T>就专门用来处理可以被合理解决并需要传播的错误
    - 异常：对于异常，Rust将其看作无法被合理解决的问题，提供了线程恐慌机制，在发生异常的时候，线程可以安全地退出
- 有益性
- 稳定性

# 第二章 语言精要

## 2.1 Rust语言的基本构成
- 语言规范
- 编译器
- 核心库
- 标准库
- 包管理器

## 2.2 语句与表达式
- Rust 会为每个crate都自动引入标准库模块，除非[no_std]属性明确指定了不需要标准库
- Rust 编译器在解析代码的时候，如果碰到分号，就会继续往后面执行；如果碰到语句，则执行语句；
如果碰到表达式，则会对表达式求值，如果分号后面什么都没有，就会补上单元值()。

## 2.3 变量与绑定
> 通过let关键字来创建变量，let创建的变量一般称为绑定（Binding），它表明了标识符(Identifier)和值(Value)之间建立的一种关联关系。

### 2.3.1 位置表达式和值表达式
> Rust中的表达式一般可以分为位置表达式(Place Expression)和值表达式(Value Expression)。在其它语言中，一般称为左值(LValue)和右值(RValue)。

位置表达式：内存位置的表达式
- 本地变量
- 静态变量
- 解引用(*expr)
- 数组索引(expr[expr])
- 字段引用(expr.field)
- 位置表达式组合

从语义角度来说，位置表达式代表了持久性数据，值表达式代表了临时数据。
位置表达式一般有持久的状态，值表达式要么是字面量，要么是表达式求值过程中创建的临时值。

求值上下文也分为位置上下文（Place Context）和值上下文（Value Context）

位置上下文
- 赋值或者复合赋值语句左侧的操作数
- 一元引用表达式的独立操作数
- 包含隐式借用（引用）的操作数
- match判别式或let绑定右侧在使用ref模式匹配的时候也是位置上下文

### 2.3.2 不可变绑定与可变绑定
> 使用let关键字声明的位置表达式默认不可变，为不可变绑定

### 2.3.3 所有权与引用
> 当位置表达式出现在值上下文中时，该位置表达式将会把内存地址转移给另外一个位置表达式，这其实是所有权的转移。

在语义上，每个变量绑定实际上都拥有该存储单元的所有权，这种转移内存地址的行为就是所有权(OwnerShip)的转移，
在Rust中称为移动(Move)语义，那种不转移的情况实际上一种复制(Copy)语义。

*在日常开发中，有时候并不需要转移所有权。Rust提供引用操作符(&)，可以直接获取表达式的存储单元地址，即内存位置。可以通过该内存位置进行读取。*

## 2.4 函数与闭包

### 2.4.1 函数定义
函数是通过关键字fn定义的。

### 2.4.2 作用域与生命周期
> Rust 语言的作用域是静态作用域，即词法作用域

连续用let定义同名变量的做法叫变量遮蔽(Variable Shadow)

### 2.4.3 函数指针

### 2.4.6 闭包
闭包也叫匿名函数
- 可以像函数一样被调用
- 可以捕获上下文环境中的自由变量
- 可以自动推断输入和返回的类型

*Rust中闭包实际上就是由一个匿名结构体和trait来组合实现的*

### 2.6.7 字符串类型

出于内存安全的考虑，Rust将字符串分为两种类型，
- 一种是固定长度字符串，不可随便更改其长度，就是str字符串
- 一种是可增长字符串，可以随意改变其长度，就是String字符串

### 2.6.8 原生指针
> 原生指针主要用于Unsafe Rust中

### 2.6.9 never类型

rust中提供了一种特殊数据类型，never类型，即!。

该类型用于表示永远不可能有返回值的计算类型

## 2.7 复合数据类型

Rust提供了4种复合数据类型，分别是：
- 元组 (Tuple)
> 异构有限序列
- 结构体 (Struct)
   - 具名结构体(Named-Field Struct) 
   - 元组结构体(Tuple-Like Struct)
   - 单元结构体(Unit-Like Struct)
- 枚举体 (Enum)
- 联合体 (Union)

## 2.8 常用集合类型

### 2.8.1 线性序列：向量
    Vec<T>
### 2.8.2 线性序列：双端队列
    VecDeque<T>
### 2.8.3 链表
    LinkedList<T>
### 2.8.4 HashMap和BTreeMap
    HashMap<K,V> 无序
    BTreeMap<K,V> 有序
### 2.8.5 集合:HashSet和BTreeSet

## 2.9 智能指针

- Rust中的值默认被分配到栈内存
- 可以通过Box<T>将值装箱（在堆内存中分配）

## 2.10 泛型和trait

trait
- trait是Rust唯一的接口抽象方法
- 可以静态生成，也可以动态调用
- 可以当作标记类型拥有某些特定行为的"标签"来使用

# 第三章 类型系统

## 3.1 通用概念
> 在类型系统中，一切皆类型。基于类型定义的一系列组合、运算和转换等方法，可以看作类型的行为。

### 3.1.1 类型系统的作用
- 排查错误
- 抽象
- 文档
- 优化效率
- 类型安全