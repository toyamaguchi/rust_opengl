商業誌版『RustではじめるOpenGL』
==================================================

このレポジトリは、書籍『RustではじめるOpenGL』で扱っているサンプルコードを収めています。

それぞれのディレクトリは、各章に紐付いています。

- [001_dev_env](https://github.com/toyamaguchi/rust_opengl/tree/master/001_dev_env): 開発環境の準備
    - 「cargo new」してできあがった「Hello, World!」を表示するプログラムです。
- [002_sdl](https://github.com/toyamaguchi/rust_opengl/tree/master/002_sdl): SDL
    - SDLを使ってウィンドウの表示をします。キーボードのイベントを所得して、Escキーで終了します。
- [003_opengl](https://github.com/toyamaguchi/rust_opengl/tree/master/003_opengl): OpenGL
    - ウィンドウ内に三角形を表示します。2種類のシェーダーや、モデル行列、ビュー行列、射影行列を使います。
- [004_imgui](https://github.com/toyamaguchi/rust_opengl/tree/master/004_imgui): Dear ImGui
    - Dear ImGuiを使って、OpenGLの描画領域にウィンドウやウィジェットを描画します。
- [005_3d_object](https://github.com/toyamaguchi/rust_opengl/tree/master/005_3d_object): 3Dオブジェクト
    - 立方体の描画をします。デバッグ用のウィジェットでパラメーターの切り替えができるようにします。
- [006_texture](https://github.com/toyamaguchi/rust_opengl/tree/master/006_texture): テクスチャー
    - 立方体にテクスチャを貼ります。照明の光も導入し、ウィジェットで操作できるようにします。
- [007_frame_buffer](https://github.com/toyamaguchi/rust_opengl/tree/master/007_frame_buffer): フレームバッファーオブジェクト
    - フレームバッファーオブジェクトへ描画し、異なるシェーダーを使って3種類のエフェクトをかけます。
