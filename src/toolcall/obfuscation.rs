//! 工具名稱混淆，對應 Python `services/tool_name_obfuscation.py`。
//! 將客戶端工具名映射成 Qwen 不會攔截的安全別名，並可反向還原。

/// 客戶端名 → Qwen 安全名。
pub fn to_qwen_name(name: &str) -> String {
    match name {
        "Read" => "fs_open_file".into(),
        "Write" => "fs_put_file".into(),
        "Edit" => "fs_patch_file".into(),
        "MultiEdit" => "fs_multi_patch".into(),
        "Bash" => "shell_run".into(),
        "Grep" => "text_search".into(),
        "Glob" => "path_find".into(),
        "NotebookEdit" => "notebook_patch".into(),
        "WebFetch" => "http_get_url".into(),
        "WebSearch" => "web_query".into(),
        "LS" => "dir_list".into(),
        "TodoWrite" => "todo_put".into(),
        "Task" => "agent_task".into(),
        other => {
            // 其餘：前綴 u_ + 清理非法字元
            let cleaned: String = other
                .chars()
                .map(|c| if c.is_ascii_alphanumeric() || c == '_' { c } else { '_' })
                .collect();
            format!("u_{cleaned}")
        }
    }
}

/// 標準化鍵（小寫、去非英數），用於反向匹配。
pub fn norm_key(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 抽樣對照表項：避免逐項複製，只確認對照表確實生效。
    #[test]
    fn known_names_map_to_safe_aliases() {
        assert_eq!(to_qwen_name("Read"), "fs_open_file");
        assert_eq!(to_qwen_name("Bash"), "shell_run");
        assert_eq!(to_qwen_name("WebFetch"), "http_get_url");
    }

    /// fallback：非映射表名一律 `u_` 前綴 + 非法字元替換為 `_`（含底線保留、空字串、unicode）。
    #[test]
    fn unknown_names_get_u_prefix_and_cleanup() {
        // 純合法字元（含底線）→ 原樣
        assert_eq!(to_qwen_name("my_tool"), "u_my_tool");
        // `-`、`.`、空白 → `_`
        assert_eq!(to_qwen_name("my-cool.tool 2"), "u_my_cool_tool_2");
        // 空字串 → 只剩前綴
        assert_eq!(to_qwen_name(""), "u_");
        // 非 ASCII（中文）：chars().map() 一字一替 → 兩字 → 兩底線
        assert_eq!(to_qwen_name("查詢"), "u___");
    }

    /// 反向匹配用：去掉非英數、轉小寫；非 ASCII 一律丟棄。
    #[test]
    fn norm_key_drops_punct_and_lowercases() {
        assert_eq!(norm_key("Web-Fetch"), "webfetch");
        assert_eq!(norm_key("  My_Tool_v2  "), "mytoolv2");
        // 中文被 is_ascii_alphanumeric 過濾掉，只剩 abc
        assert_eq!(norm_key("查詢abc"), "abc");
        assert_eq!(norm_key(""), "");
    }
}
