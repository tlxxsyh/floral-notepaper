import { invoke } from "@tauri-apps/api/core";
import type { Note, NoteMetadata, SaveNoteRequest } from "./types";

export function listNotes(): Promise<NoteMetadata[]> {
  return invoke("notes_list");
}

export function getNote(id: string): Promise<Note> {
  return invoke("notes_get", { id });
}

export function createNote(request: SaveNoteRequest): Promise<Note> {
  return invoke("notes_create", { request });
}

export function updateNote(id: string, request: SaveNoteRequest): Promise<Note> {
  return invoke("notes_update", { id, request });
}

export function deleteNote(id: string): Promise<void> {
  return invoke("notes_delete", { id });
}

export function moveNoteCategory(id: string, category: string): Promise<NoteMetadata> {
  return invoke("notes_move_category", { id, category });
}

export function listCategories(): Promise<string[]> {
  return invoke("categories_list");
}

export function createCategory(name: string): Promise<void> {
  return invoke("categories_create", { name });
}

export function renameCategory(oldName: string, newName: string): Promise<void> {
  return invoke("categories_rename", { oldName, newName });
}

export function deleteCategory(name: string): Promise<void> {
  return invoke("categories_delete", { name });
}

export function readExternalFile(path: string): Promise<string> {
  return invoke("read_external_file", { path });
}

export function saveExternalFile(path: string, content: string): Promise<void> {
  return invoke("save_external_file", { path, content });
}

export function getFileModifiedTime(path: string): Promise<number> {
  return invoke("get_file_modified_time", { path });
}

export function listTags(): Promise<string[]> {
  return invoke("tags_list");
}

export function saveAssetBytes(name: string, data: number[]): Promise<string> {
  return invoke("save_asset_bytes", { name, data });
}

export function copyAssetFile(source: string): Promise<string> {
  return invoke("copy_asset_file", { source });
}

export function getAssetBase64(relativePath: string): Promise<string> {
  return invoke("asset_base64", { relativePath });
}

export function openInExplorer(id: string): Promise<void> {
  return invoke("open_in_explorer", { id });
}

export function getErrorMessage(error: unknown): string {
  if (typeof error === "string") return error;
  if (error && typeof error === "object" && "message" in error) {
    return String((error as { message: unknown }).message);
  }
  return "操作失败";
}
