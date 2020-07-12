# rCoreTutorial
os tutorial summer of code
用Rust实现rCore的所有实验

https://rcore-os.github.io/rCore-Tutorial-deploy/docs/lab-0/guide/intro.html

## 7.12  

- 开始lab0，遇到不少问题
  - rust的版本控制rust-toolchain文件不能有UTF8的BOOM头，否则rust相关的工具无法运行

  - 在windows下进行lab0编译，和实验指导的效果不同，出现各种奇怪点，开始用wsl+vscode开发

  - wsl完全卸载：
    - Get-AppxPackage -allusers | Select Name, PackageFullName
    - get-appxpackage CanonicalGroupLimited.Ubuntu16.04onWindows | remove-Appxpackage
    
  - 在wsl中安装rust：

    - 先更换软件源为阿里源
    - 再apt update和apt upgrade
    - 安装glibc
      - wget http://ftp.gnu.org/gnu/glibc/glibc-2.31.tar.gz
      - tar -zxvf glibc-2.31.tar.gz
      - mkdir glibc-2.31-build 并进入
      - ../glibc-2.31/configure --prefix=/usr，如果提示有些包没有安则需要自己安装
      - make 再 make install
    - 2

  - 2

    
- 