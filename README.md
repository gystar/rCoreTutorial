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
    - 安装wls2  https://docs.microsoft.com/zh-cn/windows/wsl/install-win10

    - 更换软件源为阿里源

    - 新建/etc/wsl.conf，并配置一些类容，否则无法联网：

      ```
      [network]
      generateHosts = true
      generateResolvConf = true
      ```

    - 再apt update和apt upgrade

    - 用官网的方式安装rust，期间可能需要安装一些依赖的包。


## 7.13  

- 终于搞定了wls2的编译环境
-  完成了lab0的代码

## 7.14 

- 完成了lab1的代码
- 完成lab1的结果在wsl2中运行
- win10下也可以编译通过了，需要先cargo clean
- 直接用win10下的vscode来下编写代码。wsl对windows系统的文件进行了挂载，在wsl中编译并执运行结果。

## 7.15

- 完成了lab2的代码