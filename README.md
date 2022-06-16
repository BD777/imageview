# ImageView | 图阅
A image viewer by tauri.
用来看图的软件，当然，看漫画也是很方便的。

## 简单演示
![image](https://user-images.githubusercontent.com/14173335/173492730-fafc4f81-48a0-4a49-bff6-3c5834419af5.png)
![image](https://user-images.githubusercontent.com/14173335/173492762-ca4f8389-d14b-4e16-bfbf-81bf5aba0b96.png)
![image](https://user-images.githubusercontent.com/14173335/173492681-f5a4a096-8a1e-4d2b-8cbf-aff8b5ec1c8c.png)

## 简介
能在PC上较为舒适地看图看漫画。

之前有用electron做过一个，心血来潮，用tauri再做一个。体积比electron小了不少。

## 使用
### 1. 直接去release下载，打开，一气呵成。
[去release页下载](https://github.com/BD777/imageview/releases)

### 2. 我要自己编译
 - 照着这份 [tauri环境要求](https://tauri.studio/v1/guides/getting-started/prerequisites) 所说的去做
 - ```npm install```
 - ```npm run tauri:build```
 - 然后大概会在`src-tauri/target`里获得可执行文件

## 已知问题
 - `dire`在`osx`下貌似有bug，须手动将`osx.rs:26`的`let mut config_home = path;`改为`let mut config_home = path.clone();`方能编译通过。

## 鸣谢 Thx
 - [tauri](https://tauri.studio/)
 - [NaiveUI](https://www.naiveui.com/zh-CN/os-theme) // 很合我口味
 - [Vue3](https://v3.cn.vuejs.org/guide/introduction.html)
 - [vue-cli-plugin-tauri](https://github.com/tauri-apps/vue-cli-plugin-tauri)
