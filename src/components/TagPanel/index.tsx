/**
 * 标签面板组件
 */
import { useEffect, useState } from 'react';
import { Tag, X, Plus, MoreVertical, Trash2, Edit } from 'lucide-react';
import { api } from '../../lib/api';
import { useStore } from '../../stores/useStore';
import type { Tag as TagType } from '../../types/api';

export default function TagPanel() {
  const tags = useStore((s) => s.tags);
  const setTags = useStore((s) => s.setTags);
  const selectedTags = useStore((s) => s.selectedTags);
  const toggleTag = useStore((s) => s.toggleTag);
  const addNotification = useStore((s) => s.addNotification);

  const [showAddDialog, setShowAddDialog] = useState(false);
  const [showEditDialog, setShowEditDialog] = useState(false);
  const [showDeleteDialog, setShowDeleteDialog] = useState(false);
  const [editingTag, setEditingTag] = useState<TagType | null>(null);
  const [newTagName, setNewTagName] = useState('');
  const [newTagDisplayName, setNewTagDisplayName] = useState('');
  const [newTagColor, setNewTagColor] = useState('#3b82f6');
  const [isSubmitting, setIsSubmitting] = useState(false);

  useEffect(() => {
    loadTags();
  }, []);

  const loadTags = async () => {
    try {
      const data = await api.getAllTags();
      setTags(data);
    } catch (error) {
      console.error('加载标签失败:', error);
    }
  };

  const handleAddTag = async () => {
    if (!newTagName.trim()) {
      addNotification({ type: 'error', message: '请输入标签名称' });
      return;
    }

    setIsSubmitting(true);
    try {
      await api.createTag({
        name: newTagName.trim(),
        displayName: newTagDisplayName.trim() || newTagName.trim(),
        color: newTagColor,
      });
      await loadTags();
      setShowAddDialog(false);
      setNewTagName('');
      setNewTagDisplayName('');
      setNewTagColor('#3b82f6');
      addNotification({ type: 'success', message: '标签创建成功' });
    } catch (error) {
      console.error('创建标签失败:', error);
      addNotification({ type: 'error', message: '创建标签失败' });
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleUpdateTag = async () => {
    if (!editingTag) return;
    if (!newTagName.trim()) {
      addNotification({ type: 'error', message: '请输入标签名称' });
      return;
    }

    setIsSubmitting(true);
    try {
      await api.updateTag(editingTag.id!, {
        name: newTagName.trim(),
        displayName: newTagDisplayName.trim() || newTagName.trim(),
        color: newTagColor,
      });
      await loadTags();
      setShowEditDialog(false);
      setEditingTag(null);
      addNotification({ type: 'success', message: '标签更新成功' });
    } catch (error) {
      console.error('更新标签失败:', error);
      addNotification({ type: 'error', message: '更新标签失败' });
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleDeleteTag = async () => {
    if (!editingTag) return;

    setIsSubmitting(true);
    try {
      await api.deleteTag(editingTag.id!);
      await loadTags();
      setShowDeleteDialog(false);
      setEditingTag(null);
      addNotification({ type: 'success', message: '标签删除成功' });
    } catch (error) {
      console.error('删除标签失败:', error);
      addNotification({ type: 'error', message: '删除标签失败' });
    } finally {
      setIsSubmitting(false);
    }
  };

  const openEditDialog = (tag: TagType) => {
    setEditingTag(tag);
    setNewTagName(tag.name);
    setNewTagDisplayName(tag.displayName);
    setNewTagColor(tag.color);
    setShowEditDialog(true);
  };

  const openDeleteDialog = (tag: TagType) => {
    setEditingTag(tag);
    setShowDeleteDialog(true);
  };

  // 按类型分组标签
  const systemTags = tags.filter((t) => t.tagType === 'system');
  const customTags = tags.filter((t) => t.tagType === 'custom');

  return (
    <div className="flex flex-col h-full bg-card border-r border-border">
      <div className="p-4 border-b border-border flex items-center justify-between">
        <div>
          <h2 className="font-semibold text-lg">标签</h2>
          <p className="text-sm text-muted-foreground">点击标签进行筛选</p>
        </div>
        <button
          onClick={() => {
            setShowAddDialog(true);
            setNewTagName('');
            setNewTagDisplayName('');
            setNewTagColor('#3b82f6');
          }}
          className="p-2 text-blue-600 hover:bg-blue-50 dark:hover:bg-blue-900/20 rounded-lg transition-colors"
          title="添加标签"
        >
          <Plus size={20} />
        </button>
      </div>

      {selectedTags.length > 0 && (
        <div className="p-4 border-b border-border bg-muted/30">
          <div className="text-xs text-muted-foreground mb-2">已选择:</div>
          <div className="flex flex-wrap gap-2">
            {selectedTags.map((tagName) => (
              <button
                key={tagName}
                onClick={() => toggleTag(tagName)}
                className="flex items-center gap-1 px-2 py-1 text-xs rounded-full bg-primary text-primary-foreground hover:bg-primary/90 transition-colors"
              >
                {tagName}
                <X className="w-3 h-3" />
              </button>
            ))}
            <button
              onClick={() => useStore.getState().clearSelectedTags()}
              className="px-2 py-1 text-xs rounded-full bg-secondary text-secondary-foreground hover:bg-secondary/80 transition-colors"
            >
              清除
            </button>
          </div>
        </div>
      )}

      <div className="flex-1 overflow-y-auto p-4">
        {systemTags.length > 0 && (
          <div className="mb-6">
            <h3 className="text-sm font-medium text-muted-foreground mb-2">系统标签</h3>
            <div className="flex flex-wrap gap-2">
              {systemTags.map((tag) => (
                <TagButton
                  key={tag.id || tag.name}
                  tag={tag}
                  isSelected={selectedTags.includes(tag.name)}
                  onClick={() => toggleTag(tag.name)}
                  showActions={false}
                />
              ))}
            </div>
          </div>
        )}

        {customTags.length > 0 && (
          <div>
            <h3 className="text-sm font-medium text-muted-foreground mb-2">自定义标签</h3>
            <div className="flex flex-wrap gap-2">
              {customTags.map((tag) => (
                <TagButton
                  key={tag.id || tag.name}
                  tag={tag}
                  isSelected={selectedTags.includes(tag.name)}
                  onClick={() => toggleTag(tag.name)}
                  showActions={true}
                  onEdit={() => openEditDialog(tag)}
                  onDelete={() => openDeleteDialog(tag)}
                />
              ))}
            </div>
          </div>
        )}

        {tags.length === 0 && (
          <div className="text-center text-muted-foreground text-sm py-8">
            暂无标签
          </div>
        )}
      </div>

      {/* 添加标签对话框 */}
      <TagDialog
        isOpen={showAddDialog}
        onClose={() => setShowAddDialog(false)}
        onSubmit={handleAddTag}
        isSubmitting={isSubmitting}
        title="添加标签"
        name={newTagName}
        displayName={newTagDisplayName}
        color={newTagColor}
        onNameChange={setNewTagName}
        onDisplayNameChange={setNewTagDisplayName}
        onColorChange={setNewTagColor}
      />

      {/* 编辑标签对话框 */}
      <TagDialog
        isOpen={showEditDialog}
        onClose={() => setShowEditDialog(false)}
        onSubmit={handleUpdateTag}
        isSubmitting={isSubmitting}
        title="编辑标签"
        name={newTagName}
        displayName={newTagDisplayName}
        color={newTagColor}
        onNameChange={setNewTagName}
        onDisplayNameChange={setNewTagDisplayName}
        onColorChange={setNewTagColor}
      />

      {/* 删除确认对话框 */}
      {showDeleteDialog && editingTag && (
        <ConfirmDialog
          title="删除标签"
          message={`确定要删除标签"${editingTag.displayName}"吗？此操作不可恢复。`}
          onConfirm={handleDeleteTag}
          onCancel={() => setShowDeleteDialog(false)}
          isSubmitting={isSubmitting}
        />
      )}
    </div>
  );
}

interface TagButtonProps {
  tag: TagType;
  isSelected: boolean;
  onClick: () => void;
  showActions?: boolean;
  onEdit?: () => void;
  onDelete?: () => void;
}

function TagButton({ tag, isSelected, onClick, showActions = false, onEdit, onDelete }: TagButtonProps) {
  const [showMenu, setShowMenu] = useState(false);

  return (
    <div className="relative group">
      <button
        onClick={onClick}
        className={`
          flex items-center gap-1 px-3 py-1.5 rounded-full text-sm transition-colors
          ${isSelected ? 'text-white' : 'bg-secondary text-secondary-foreground hover:bg-secondary/80'}
        `}
        style={isSelected ? { backgroundColor: tag.color } : undefined}
      >
        <Tag className="w-3 h-3" />
        <span>{tag.displayName || tag.name}</span>
        {tag.useCount > 0 && (
          <span className="ml-1 text-xs opacity-70">({tag.useCount})</span>
        )}
      </button>

      {showActions && (
        <button
          onClick={(e) => {
            e.stopPropagation();
            setShowMenu(!showMenu);
          }}
          className="absolute -right-1 -top-1 p-1 bg-white dark:bg-gray-800 rounded-full shadow-md opacity-0 group-hover:opacity-100 transition-opacity"
        >
          <MoreVertical size={12} className="text-gray-600 dark:text-gray-400" />
        </button>
      )}

      {showMenu && showActions && (
        <div className="absolute right-0 top-full mt-1 bg-white dark:bg-gray-800 rounded-lg shadow-lg border border-gray-200 dark:border-gray-700 py-1 z-10 min-w-[100px]">
          <button
            onClick={(e) => {
              e.stopPropagation();
              setShowMenu(false);
              onEdit?.();
            }}
            className="flex items-center gap-2 px-3 py-2 text-sm text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 w-full"
          >
            <Edit size={14} />
            编辑
          </button>
          <button
            onClick={(e) => {
              e.stopPropagation();
              setShowMenu(false);
              onDelete?.();
            }}
            className="flex items-center gap-2 px-3 py-2 text-sm text-red-600 hover:bg-red-50 dark:hover:bg-red-900/20 w-full"
          >
            <Trash2 size={14} />
            删除
          </button>
        </div>
      )}
    </div>
  );
}

interface TagDialogProps {
  isOpen: boolean;
  onClose: () => void;
  onSubmit: () => void;
  isSubmitting: boolean;
  title: string;
  name: string;
  displayName: string;
  color: string;
  onNameChange: (value: string) => void;
  onDisplayNameChange: (value: string) => void;
  onColorChange: (value: string) => void;
}

function TagDialog({
  isOpen,
  onClose,
  onSubmit,
  isSubmitting,
  title,
  name,
  displayName,
  color,
  onNameChange,
  onDisplayNameChange,
  onColorChange,
}: TagDialogProps) {
  if (!isOpen) return null;

  const colors = [
    '#ef4444', '#f97316', '#f59e0b', '#eab308', '#84cc16',
    '#22c55e', '#10b981', '#14b8a6', '#06b6d4', '#0ea5e9',
    '#3b82f6', '#6366f1', '#8b5cf6', '#a855f7', '#d946ef',
    '#ec4899', '#f43f5e'
  ];

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow-xl w-full max-w-md p-6">
        <h3 className="text-lg font-semibold mb-4">{title}</h3>

        <div className="space-y-4">
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              标签名称
            </label>
            <input
              type="text"
              value={name}
              onChange={(e) => onNameChange(e.target.value)}
              placeholder="例如: 重要文件"
              className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            />
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              显示名称（可选）
            </label>
            <input
              type="text"
              value={displayName}
              onChange={(e) => onDisplayNameChange(e.target.value)}
              placeholder="例如: 重要"
              className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            />
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              颜色
            </label>
            <div className="flex flex-wrap gap-2">
              {colors.map((c) => (
                <button
                  key={c}
                  type="button"
                  onClick={() => onColorChange(c)}
                  className={`w-8 h-8 rounded-full border-2 transition-transform hover:scale-110 ${
                    color === c ? 'border-gray-900 dark:border-white scale-110' : 'border-transparent'
                  }`}
                  style={{ backgroundColor: c }}
                />
              ))}
            </div>
          </div>
        </div>

        <div className="flex justify-end gap-3 mt-6">
          <button
            onClick={onClose}
            className="px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg"
          >
            取消
          </button>
          <button
            onClick={onSubmit}
            disabled={isSubmitting}
            className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50"
          >
            {isSubmitting ? '保存中...' : '保存'}
          </button>
        </div>
      </div>
    </div>
  );
}

interface ConfirmDialogProps {
  title: string;
  message: string;
  onConfirm: () => void;
  onCancel: () => void;
  isSubmitting: boolean;
}

function ConfirmDialog({ title, message, onConfirm, onCancel, isSubmitting }: ConfirmDialogProps) {
  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow-xl w-full max-w-md p-6">
        <h3 className="text-lg font-semibold mb-4">{title}</h3>
        <p className="text-gray-700 dark:text-gray-300 mb-6">{message}</p>

        <div className="flex justify-end gap-3">
          <button
            onClick={onCancel}
            disabled={isSubmitting}
            className="px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg disabled:opacity-50"
          >
            取消
          </button>
          <button
            onClick={onConfirm}
            disabled={isSubmitting}
            className="px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 disabled:opacity-50"
          >
            {isSubmitting ? '删除中...' : '删除'}
          </button>
        </div>
      </div>
    </div>
  );
}
