## io
io核心的trait是Read 和 Write traits，它们提供用于读取和写入输入和输出的最通用接口。

因为它们是 traits，所以 Read 和 Write 由许多其他类型实现,因此io代表的是一种能力,可以通过实现获得，您也可以为您的类型实现它们。

实现了Read-trait的叫做`Reader` , 实现了Write-trait的叫做`Writer`
+ `Read` : [详情](https://rustwiki.org/zh-CN/std/io/trait.Read.html)
+ `Write` : [详情](https://rustwiki.org/zh-CN/std/io/trait.Write.html)

### Seek 和 BufRead : 控制Reader读取的方式

+ 实现了`Seek-trait`的数据可以跳转到Reader任意位置进行读取
+ 而实现了`BufRead`的数据可以缓冲读取,增加读取速度

#### BufReader / BufWriter

实现了 BufRead-trait 的数据称之为 BufReader : 拥有了缓冲读取(也就是整行读取)的函数:`read_line()`
+ `BufReader::new(File)->BufReader`: 从file中读取并生成BufReader

BufWriter没有新增函数(所以没有BufWriter这个东西在代码实现上),只是自动增加了缓冲写入的方式(离开作用域自动刷新缓冲区)

#### 对buffer进行迭代

+ `BufReader.Lines()->Iterator` : 用于对BufReader的缓冲数据进行迭代读取

### 作用于io上的直接函数
+ `copy()`	将 reader 的全部内容复制到 writer 中。

+ `empty()`	为空的 reader 创建一个新的句柄。

+ `repeat()`	创建 reader 的实例，该实例无限重复一个字节。

+ `sink()`	创建 writer 的实例，该实例将成功消费所有数据。

+ `stderr()`	为当前进程的标准错误创建一个新的句柄。

+ `stdin()`	为当前进程的标准输入创建一个新的句柄。

+ `stdout()`	为当前进程的标准输出创建一个新的句柄。