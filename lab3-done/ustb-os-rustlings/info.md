如果已经安装了`Docker`桌面端和`Vscode`桌面端，可以按照下面步骤启动项目.
1. 启动你的`Docker`桌面端（看到蓝色的小鲸鱼在喷水）
2. 在`Vscode`的终端里执行下面操作
   ```bash
   # 从仓库拷贝源代码
   # Github非科学上网可能会被墙，给出一种镜像访问的指令
   git clone https://gh-proxy.com/https://github.com/Source-of-USTB/ustb-os-rustlings.git

   # 启动Docker容器
   docker run -it --name ustb-os -v "${PWD}:/mnt" -w /mnt rust:latest /bin/bash

   # 进容器后安装
   cargo install --force --path .

   # 完成所有代码补全练习并且去掉相关注释之后，执行
   root@229a283a1fde:/mnt# rustlings watch  # 确保你在Docker环境里执行
                                            # root@229a283a1fde:/mnt是正确的文件位置，而不是本机上的C/D/E盘
    ```
3. 看到以下输出，代表所有测试点全部通过.
   ```bash
    ★ All exercises completed! ★

    +----------------------------------------------------+
    |          You made it to the Fe-nish line!          |
    +--------------------------  ------------------------+
                            \\/
        ▒▒          ▒▒▒▒▒▒▒▒      ▒▒▒▒▒▒▒▒          ▒▒
    ▒▒▒▒  ▒▒    ▒▒        ▒▒  ▒▒        ▒▒    ▒▒  ▒▒▒▒
    ▒▒▒▒  ▒▒  ▒▒            ▒▒            ▒▒  ▒▒  ▒▒▒▒
    ░░▒▒▒▒░░▒▒  ▒▒            ▒▒            ▒▒  ▒▒░░▒▒▒▒
    ▓▓▓▓▓▓▓▓  ▓▓      ▓▓██  ▓▓  ▓▓██      ▓▓  ▓▓▓▓▓▓▓▓
        ▒▒▒▒    ▒▒      ████  ▒▒  ████      ▒▒░░  ▒▒▒▒
        ▒▒  ▒▒▒▒▒▒        ▒▒▒▒▒▒        ▒▒▒▒▒▒  ▒▒
            ▒▒▒▒▒▒▒▒▒▒▓▓▓▓▓▓▒▒▒▒▒▒▒▒▓▓▒▒▓▓▒▒▒▒▒▒▒▒
                ▒▒▒▒▒▒▒▒▒▒██▒▒▒▒▒▒██▒▒▒▒▒▒▒▒▒▒
            ▒▒  ▒▒▒▒▒▒▒▒▒▒██████▒▒▒▒▒▒▒▒▒▒  ▒▒
            ▒▒    ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒    ▒▒
        ▒▒    ▒▒    ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒    ▒▒    ▒▒

    We hope you enjoyed learning about the various aspects of Rust!
    If you noticed any issues, please don't hesitate to report them to our repo.
    You can also contribute your own exercises to help the greater community!
    https://github.com/rust-lang/rustlings/blob/main/CONTRIBUTING.md
```


