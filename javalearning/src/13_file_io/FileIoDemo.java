// ============================================================
// 第13章 · 文件 IO
// ============================================================
// 运行: java src/13_file_io/FileIoDemo.java
// 学习要点:
//   1. Files.writeString / readString 一行读写（Java 11+）
//   2. BufferedReader 逐行读大文件
//   3. BufferedWriter 写入与追加模式
//   4. Path 路径操作与文件管理
//   5. 遍历目录 walk
//   （本课把文件写到系统临时目录，结束后自动清理）
// ============================================================

import java.io.BufferedReader;
import java.io.BufferedWriter;
import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.StandardOpenOption;
import java.util.List;
import java.util.stream.Stream;

public class FileIoDemo {

    public static void main(String[] args) {
        // 在临时目录建工作区，避免污染项目文件夹
        Path dir = Path.of(System.getProperty("java.io.tmpdir"), "java_learning_io_demo");
        Path file = dir.resolve("notes.txt");   // resolve = 拼接路径，跨平台自动用 \ 或 /

        try {
            // ==== 0. 准备目录 ====
            Files.createDirectories(dir);       // 类似 mkdir -p，已存在不报错

            // ==== 1. 一行读写小文件（最常用）====
            System.out.println("=== 一行读写 ===");
            Files.writeString(file, "第一行：Hello Java IO\n第二行：你好，世界\n",
                    StandardCharsets.UTF_8);
            // 对比 Python: open(...).write(...)；对比 C: fopen/fwrite/fclose 一套流程
            // Java 11+ 的 writeString/readString 内部帮你搞定打开和关闭

            String content = Files.readString(file, StandardCharsets.UTF_8);
            System.out.println("读回的内容:\n" + content);

            // ==== 2. 逐行读：大文件必用 ====
            System.out.println("=== 逐行读 ===");
            // readString 会把整个文件塞进内存——几个 G 的日志会直接 OOM！
            // BufferedReader 流式逐行读，内存占用恒定。
            try (BufferedReader reader = Files.newBufferedReader(file, StandardCharsets.UTF_8)) {
                String line;
                int lineNo = 1;
                while ((line = reader.readLine()) != null) {   // readLine 到末尾返回 null
                    System.out.println("第" + lineNo++ + "行: " + line);
                }
            }                                                  // try-with-resources 自动关闭！

            // 或者更简单的 readAllLines（中小文件适用）：
            List<String> lines = Files.readAllLines(file);
            System.out.println("共 " + lines.size() + " 行");

            // ==== 3. 追加写入 ====
            System.out.println("\n=== 追加写 ===");
            try (BufferedWriter writer = Files.newBufferedWriter(file,
                    StandardCharsets.UTF_8,
                    StandardOpenOption.APPEND)) {              // 默认是覆盖(CREATE)，APPEND 是追加
                writer.write("第三行：追加进来的");
                writer.newLine();                              // 跨平台换行
            }
            System.out.println("追加后最后一行: " + Files.readAllLines(file).get(2));

            // ==== 4. 文件管理操作 ====
            System.out.println("\n=== 文件管理 ===");
            System.out.println("存在? " + Files.exists(file));
            System.out.println("大小: " + Files.size(file) + " 字节");

            Path backup = dir.resolve("notes_backup.txt");
            Files.copy(file, backup);                          // 复制；移动用 Files.move
            System.out.println("备份存在? " + Files.exists(backup));
            Files.delete(backup);                              // 删除（不存在会抛异常）
            System.out.println("删除备份后存在? " + Files.exists(backup));

            // ==== 5. 遍历目录 ====
            System.out.println("\n=== 遍历目录 ===");
            try (Stream<Path> entries = Files.walk(dir)) {     // walk 深度遍历子目录
                entries.forEach(p -> System.out.println("  " + p.getFileName()));
            }

        } catch (IOException e) {
            // 所有文件操作都可能抛 IOException（checked 异常），必须处理——见第11章
            System.out.println("IO 出错: " + e.getMessage());
        } finally {
            try {
                // 清理现场
                try (Stream<Path> entries = Files.walk(dir)) {
                    entries.sorted(java.util.Comparator.reverseOrder())  // 先删文件再删目录
                           .forEach(p -> p.toFile().delete());
                }
                System.out.println("\n临时目录已清理");
            } catch (IOException ignored) { }
        }

        /*
         * ---- 新旧 API ----
         * java.io.File 是老 API（Java 1.0），方法少还爱抛 null；
         * java.nio.file.Path + Files 是现代标准（Java 7+），一律用它。
         *
         * ---- 一句话总结 ----
         * 小文件一行 writeString/readString；
         * 大文件 newBufferedReader 逐行流式处理；
         * 记住配对口诀：开资源 → try-with-resources 包起来。
         */
    }
}
