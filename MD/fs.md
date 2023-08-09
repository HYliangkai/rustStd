## 文件系统
I/O 是一个大类,所有涉及到数据交换的行为都可以称之为I/O,rust将I/O分成了3大块 : io，fs 和 net

一般情况下fs代表的都是获取文件句柄的方式

### fs下的直接func

+ `canonicalize(path)->Result<PathBuf>` : 获取文件的绝对路径

+ `copy(Frompath,Topath)->Result<u64>` : 将一个文件的内容复制到另一个文件。此函数还将复制原始文件的权限位到目标文件。该函数将覆盖 to 的内容。请注意，如果 from 和 to 都指向同一个文件，则此操作可能会截断该文件。成功后，将返回复制的字节总数，该总数等于 metadata 报告的 to 文件的长度。

+ `create_dir(path)->Result<()>` : 创建空目录,如果父目录不存在就报错

+ `create_dir_all(path)->Result<()>` : 创建空目录,如果父目录不存在就创建

+ `hard_link(OrgPath,LinkPath)->Result<()>` : 创建*硬*链接

+ `metadata(path)->Result<Metadata>` : 获取文件信息

+ `read(path) -> Result<Vec<u8>>` : 以u8读取path

+ `read_dir()->Result<ReadDir>`	返回目录中条目的迭代器。

+ `read_link()`	读取符号链接，返回链接指向的文件。

+ `read_to_string()`	将文件的全部内容读取为字符串。

+ `remove_dir()`	删除一个空目录。

+ `remove_dir_all()`	删除目录中的所有内容后，将在此路径中删除该目录。小心使用！

+ `remove_file()`	从文件系统中删除文件。

+ `rename()`	将文件或目录重命名为新名称，如果 to 已经存在，则替换原始文件。

+ `set_permissions()`	更改在文件或目录上找到的权限。

+ `symlink_metadata()`	查询有关文件的元数据，而无需遵循符号链接。

+ `write()`	写一个切片作为文件的全部内容。




### 获取句柄 File
文件离开作用域时将自动关闭。Drop 的实现将忽略在关闭时检测到的错误。
+ `File::create(path)->File` :  以只写模式打开文件。如果该函数不存在，则此函数将创建一个文件，如果存在则将截断该文件。

+ `File::open(path)->File` : 尝试以只读模式打开文件。

+ `File:options()->OpenOptions` : 对文件的打开方式进行(必要)设置
#### OpenOptions
可通过OpenOptions配置细化的打开规则来获得一个文件句柄,不需要File::open()
+`::new()->OpenOptions` : 创建一个可复制的OpenOptions
+ `open(path)->Result(File)` : 打开文件,用于尾调用
+ `append(bool)->OpenOptions` : 追加权限
+ `read(bool)->OpenOptions` : 读取权限
+ `write(bool)->OpenOptions` : 写权限
+ `create(bool)->OpenOptions` : 新建权限(如果不存在就创建)
+ `create_new(bool)->OpenOptions` : 只创建权限(如果文件已经存在就报错)

### File的函数
+ `metadata(path)`：获取指定路径的文件的元数据，并返回一个Result<Metadata>类型的值。

+ `read_to_string(path)`：读取指定路径的文件的内容，并返回一个Result<String>类型的值。

+ `write(path, data)`：将指定的数据写入到指定路径的文件中，并返回一个Result<()>类型的值。

+ `read(&mut buffer)`：从文件中读取数据到指定的缓冲区中，并返回一个Result<usize>类型的值，表示实际读取的字节数。

+ `write(&data)`：将指定的数据写入到文件中，并返回一个Result<usize>类型的值，表示实际写入的字节数。

+ `seek(position)`：将文件的当前位置设置为指定的位置，并返回一个Result<u64>类型的值，表示实际设置的位置。

+ `set_len(length)`：将文件的长度设置为指定的长度，并返回一个Result<()>类型的值。

+ `sync_all()`：将文件的所有缓冲区的数据刷新到磁盘，并返回一个Result<()>类型的值。

+ `metadata()`：获取文件的元数据，并返回一个Result<Metadata>类型的值。

+ `file_type()`：获取文件的类型，并返回一个Result<FileType>类型的值。