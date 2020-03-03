## 回溯和递归

递归就是自我调用，经常作为一种编程的实现方式，比如题主问题中的DFS 、动态规划、回溯法都可以用递归来实现，当然也可以用非递归来实现。很多时候一个概念也可以用递归的方式来定义（比如gnu）。

回溯是一种通用的算法，把问题分步解决，在每一步都试验所有的可能，当发现已经找到一种方式或者目前这种方式不可能是结果的时候，退回上一步继续尝试其他可能。很多时候每一步的处理都是一致的，这时候用递归来实现就很自然。

当回溯用于树的时候，就是深度优先搜索。当然了，几乎所有可以用回溯解决的问题都可以表示为树。那么这俩在这里就几乎同义了。如果一个问题解决的时候显式地使用了树，那么我们就叫它dfs。很多时候没有用树我们也管它叫dfs严格地说是不对的，但是dfs比回溯打字的时候好输入。别的回答里提到了砍枝，实际上这二者都可以砍枝。

回溯可以用于所有用穷举法可以解决的问题，而DP只用于具有**最优子结构**的问题。所以不是所有问题都适合用dp来解决，比如八皇后。dp需要存贮子问题的解，回溯不需要。

## 这道题

既然回溯可以用于所有用穷举法可以解决的问题，那么很显然非常适合这道题。

注意递归的时候不要返回值，vector复制效率大大下降。

### 什么是尾递归

递归是指函数直接或间接地调用自己。

尾递归是指所有递归形式的调用，一定是发生在函数的末尾。

举例来说，这是在 C 语言中用循环来计算 1 ~ 100 的和，要改写成递归形式的话，它的定义如下：

sum(n) = sum(n - 1) + n (n > 0)
sum(0) = 0

可以看到，函数最后执行的是 + 运算，而不是 sum 的递归调用，因此这属于一般的线性递归。
这种线性递归一般是需要避免的，因为它需要保留每次函数调用的栈帧，直到最深的递归开始返回。我把参数增大到一亿后，8 GB 的内存也扛不住，直接死机了……