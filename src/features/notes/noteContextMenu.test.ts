import { describe, expect, test } from "vitest";
import { noteContextMenuItems } from "./noteContextMenu";

describe("noteContextMenuItems", () => {
  test("includes export, move, openInExplorer, and delete actions", () => {
    expect(noteContextMenuItems).toEqual([
      { action: "export", label: "导出 Markdown" },
      { action: "move", label: "移动到分类…" },
      { action: "openInExplorer", label: "在文件资源管理器中打开" },
      { action: "delete", label: "删除便签", tone: "danger" },
    ]);
  });
});
