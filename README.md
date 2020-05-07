windows下的终端版2048，因为我没有找到调用winAPI合适的crate，无法进行清屏处理。所以游戏大概是如下这样的。


```
       +------+------+------+------+
       |      |      |      |      |
       +------+------+------+------+
       |      |      |      |      |
       +------+------+------+------+
       |      |      |      |      |
       +------+------+------+------+
       |   2  |      |      |      |
       +------+------+------+------+

   'w'    
       +------+------+------+------+
       |   2  |      |      |      |
       +------+------+------+------+
       |      |      |      |      |
       +------+------+------+------+
       |      |  4   |      |      |
       +------+------+------+------+
       |      |      |      |      |
       +------+------+------+------+
```
但是我的游戏虽然简陋，但是逻辑完备。包含测试代码在内代码量极少，充分利用了`rust`语法。处理上下左右移动逻辑，只用到如下一个函数。
```
  fn rs_move(&mut self, x: i8, y: i8, c: i8, r: i8) {
        ...
        
        
        match (current, target) {
           (None, _) => {}
            (Some(v1), Some(v2)) if v1 == v2 => {
               ...

            }
            (Some(v1), None) => {
                ...
            }
            (Some(_), Some(_)) => {
                ....
                }

            }
```