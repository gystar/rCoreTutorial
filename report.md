- ## Rust学习报告，repo：https://github.com/gystar/HelloRust

  - 进行每日学习的简短记录【[README.md](https://github.com/gystar/HelloRust/blob/master/README.md)】
  - 学习《Rust编程之道》，对于阅读过的章节做了详细的读书笔记，目录 【[doc/学习日志](https://github.com/gystar/HelloRust/tree/master/doc/学习日志)】。
  - 完成csv_chanllenge，熟悉cargo的项目创建和包管理等，目录 【[csv_challenge](https://github.com/gystar/HelloRust/tree/master/csv_challenge)】
  - 通过所有的rustlings小测验，目录【[rustlings](https://github.com/gystar/HelloRust/tree/master/rustlings)】
  - 完成hardway学习c中的部分练习题（14道），目录【[exercises](https://github.com/gystar/HelloRust/tree/master/rustlings/exercises)】。前面的一些题目都比较简单，我觉得比较麻烦的是下面几个：
    - ex17 小型数据库
    - ex45 socket连接
    - ex32 双向链表。这道没有使用任何unsafe的用法，用Rc来实现，挺费劲的，对Rust深入了解了不少
    - ex33 链表算法：冒泡排序和归并排序。
  - 在pat平台上选了输入简单的7道题做了，没有做连续的题目主要是因为rust对于控制台的格式化输入解析比较麻烦，目录【[pat_exercises](https://github.com/gystar/HelloRust/tree/master/pat_exercises)】。每道题目都有对应的题目链接以及用C++实现的代码。

- ## rCore学习报告，repo：https://github.com/gystar/rCoreTutorial

  - ### 一步步的进行实验指导

    - 对照实验指导，一步步的从0开始，加入了中断、物理内存、虚拟内存、线程/进程、设备树、系统调用模块。每个实验指导都建立了相应的分支labx，并且后一个实验都是前一个分支上进行必要的增量完成的，没有多余模块。
    - lab0-lab4基本上从一个个的函数，一个个的模块进行搭建的，代码主要是从实验指导和rCore的代码库中参考的，会加上我的一些注释，调整一些顺序。
    - lab5和lab6涉及的代码非常多，而且时间很紧，因此主要是参考rCore的master的源码进行拆分，lab5中没有系统调用模块（提供的实验框架中有这个多余的模块）。lab6基本和rCore的master相同。
    - lab5和lab6的主要任务是深入理解代码结构框架，熟悉整个过程。

  - ### 完成实验题目

    - #### 【实验一 ：中断】

      - 因为有答案，这里略过。

    - #### 【实验二：内存分配】 实现分支：lab2

      - ##### 实现了线段树算法。使用完全二叉排序树来构成一个堆存储，不足的补上一些额外的结点。

        - 分配算法的时间复杂度O(lgn)，总的结点数 <  页面总数*2
        - 空间复杂度 O(n)，但是常数很小，每个结点只占用3个bool的空间
        - 缺陷：因为分配的顺序是先序遍历的方式，前后分配的两个物理页面一般不连续，不适用于后面块设备等的物理内存分配。
        - 源码【[bitmap_vector_allocator.rs](https://github.com/gystar/rCoreTutorial/blob/lab2/os/src/algorithm/src/allocator/bitmap_vector_allocator.rs)】

      - ##### 修改 `FrameAllocator`，令其使用未被分配的页面空间（而不是全局变量）来存放页面使用状态。

        - 这个直接在内核后面的物理内存中，预留足够的大小来放分配器的状态信息即可
        - 在master分支上可以看到这部分的代码，比较分散，也比较简单就略过

      - ##### 挑战实验：使用伙伴分配算法来实现动态堆存储分配的VectorAllocator trait

        - 技术难点：
          - 实现传统的伙伴分配算法，空闲链表初始化->大块分割->合并兄弟块	
          - 分配的地址有aligin对齐要求，大块不一定能分出合适的小块。
          - 这个算法是用来进行动态内存分配，因此初始化的时候不能使用动态内存分配创建链表否则就互相new死锁了，但是伙伴算法是基于动态内存分配的大小动态改变的链表结构。
          - 这里我实现了一个特殊的链表：一开始只有一个静态的结点，后续节点动态分配。由于此算法开始每个空闲链表最多一个结点，刚好满足。
        - 伙伴分配算法源码【[buddy_vector_allocator.rs](https://github.com/gystar/rCoreTutorial/blob/lab2/os/src/algorithm/src/allocator/buddy_vector_allocator.rs)】
        - 链表实现源码【[dynamic_linked_list.rs](https://github.com/gystar/rCoreTutorial/blob/lab2/os/src/algorithm/src/unity/dynamic_linked_list.rs)】

    - #### 【实验三：虚实地址转换】实现分支lab3+

      - ##### 了解并实现时钟页面置换算法

        - 淘汰的规则：对于在列表中的各项，比较其PTE的标志位(A,D)，取出A*2+D最小的表项，进行淘汰，即淘汰顺序：(0,0) < (0,1) < (1,0) < (1,1)

          ```rust
          //通过比较标志位(ACCESSED, DIRTY)来决定大小
          //(0,0) < (0,1) < (1,0) < (1,1)
          fn cmp_flags(a: usize, b: usize) -> bool {
              let entry_a = unsafe { *(a as *mut PageTableEntry) };
              let entry_b = unsafe { *(b as *mut PageTableEntry) };
              let (x1, x0) = entry_a.rw_falgs();
              let (y1, y0) = entry_b.rw_falgs();
              let x = (x1 as usize) * 2 + x0 as usize;
              let y = (y1 as usize) * 2 + y0 as usize;
              x < y
          }
          ```

          

        - 每次进行淘汰之后，重置所有的表项的(A,D)标志位（PS:这样可能会出问题，以后再优化）

        - ClockSwapper 实现的源代码【[swapper.rs](https://github.com/gystar/rCoreTutorial/blob/lab3%2B/os/src/memory/mapping/swapper.rs)】

    - #### 【实验四（上）：线程】分支lab4

      - ##### 当键盘按下 Ctrl + C 时，结束当前线程

        - 捕获到键盘中断的时候，getchar = 3，即知道获取了Ctrl + C
        - 将当前线程标记为dead，在handler里面结束掉即可
        - 代码较简单，则分支中可以测试

      - ##### 实现线程的 clone()，按下C键复制当前线程

        - 捕获到键盘中断的时候，getchar = 99，即知道获取了 C
        - 以此线程的父进程为父进程，分配并且复制当前线程的栈即可
        - 代码较简单，在分支上可以测试

    - #### 【实验四（下）：线程调度】分支lab4

      - ##### 了解并实现 Stride Scheduling 调度算法

        - 技术难点：

          - 累加溢出，这里进行无符号的数相加，只保留后面溢出的部分

            ```rust
            //将无符号的a和b相加，如果溢出，只保留后面低位
            fn overflow_add(a: &mut UNSIGNED, b: UNSIGNED) {
                if b <= UNSIGNED::MAX - *a {
                    *a += b;
                } else {
                    *a = *a - (UNSIGNED::MAX - b + 1);
                }
            }
            ```

            

          - MAX_Stride - MIN_Stride < MAX_Pass的证明

            ```go
            归纳法证明：
            初始状态：所有的stride均为0
            假设，进行了k轮调度后：stride由小到大为x1,x2,...xn， 且有|xj - xi| < MAX_Pass
            进行k+1轮调度：x1为最小值，选择之后stride序列变为x1+pass1, x2...xn
            只考虑变化过的x1+pass1,则有：
            |x1+pass1 - xi|=                                                       (i= 2..n )
            1.x1+pass1-xi >0 
            x1-xi+pass1 < 0+pass1 < MAX_Pass
            2.x1+pass1-xi <0
            0<xi-x1-pass1 < MAX_Pass - pass1 < MAX_Pass
            综上，进行k+1轮调度之后仍然有|xj - xi| < MAX_Pass
            
            则由第一归纳法的知，MAX_Stride - Min_Stride < MAX_Pas
            	 
            ```

            

          - 溢出stride大小比较，这里选用usize作为存储结构，强行转换为isize，通过其正负来得到比较的大小

            ```rust
            //当x和y之差的绝对值小于SIGNED的最大绝对值的时候，可以比较大小
            fn overflow_cmp(x: UNSIGNED, y: UNSIGNED) -> Ordering {
                if x == y {
                    Ordering::Equal
                } else if x >= y {
                    if ((x - y) as SIGNED) < 0 {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                } else {
                    if ((y - x) as SIGNED) < 0 {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                }
            }
            ```

            

        - 源码【[stride_pass_scheduler.rs](https://github.com/gystar/rCoreTutorial/blob/lab4/os/src/algorithm/src/scheduler/stride_pass_scheduler.rs)】

      - ##### 分析Stride Scheduling 调度算法

        - 在 Stride Scheduling 算法下，如果一个线程进入了一段时间的等待（例如等待输入，此时它不会被运行)，从调度池中移除进而进入休眠
        - 对于两个优先级分别为 9 和 1 的线程，连续 10 个时间片中前者的运行次数不一定多，例如开始的时候后者的初始stride很小，通过10个时间片之后依然不大于前者的stride
        - 不足之处：每个线程的priority不好设置，因为是使用了MAX_STRIDE/priority来得到每个线程的pass，设置方法不够直观

    - #### 【实验六（上）：系统调度】分支lab6

      - ##### 如果要让用户线程能够使用 `Vec` 等,需要做哪些工作？

        - 需要预留出一个静态分配的连续空间作为堆
        - 使用相应的算法实现动态全局分配器

      - ##### 实现 `get_tid` 系统调用，使得用户线程可以获取自身的线程 ID。并且为每个进程添加了进程ID，可以通过系统调用获取到

        - 在handler.rs解析系统调用的相关参数，获取到pid和tid之后返回即可

      - ##### 实现 `sys_fork` 系统调用，按下C键之后，复制当前进程，复制当前线程，加入线程池

        - 捕获C键位中断
        - 复制当前进程中的memory_set等实现进程的复制，复制线程的入口函数等实现线程的复制。
