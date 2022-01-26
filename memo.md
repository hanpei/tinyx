todo list

[x] 处理 block_statement, {}问题
[x] 处理 decimal，exponent, 负数
[x] 处理“/” 开头的 comment
[x] function scope

[] string 里的 unicode 解析
[] identifier 开头是字母，可以包含数字与\_

---

## resolving variables:

> - A block statement introduces a new scope for the statements it contains.
> - A function declaration introduces a new scope for its body and binds its parameters in that scope.
> - A variable declaration adds a new variable to the current scope.
> - Variable and assignment expressions need to have their variables resolved.
