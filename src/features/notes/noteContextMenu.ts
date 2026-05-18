export type NoteContextMenuAction = "export" | "move" | "openInExplorer" | "delete";

export interface NoteContextMenuItem {
  action: NoteContextMenuAction;
  label: string;
  tone?: "danger";
}

export const noteContextMenuItems: NoteContextMenuItem[] = [
  { action: "export", label: "导出 Markdown" },
  { action: "move", label: "移动到分类…" },
  { action: "openInExplorer", label: "在文件资源管理器中打开" },
  { action: "delete", label: "删除便签", tone: "danger" },
];
