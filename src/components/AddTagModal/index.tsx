/**
 * 添加标签对话框组件
 */
import { useState, useEffect } from 'react';
import { X, Check, Plus } from 'lucide-react';
import { api } from '../../lib/api';
import { useStore } from '../../stores/useStore';
import type { File as FileType, Tag } from '../../types/api';

// id 为必填的 Tag 类型
type TagWithId = Tag & { id: number };

interface AddTagModalProps {
  isOpen: boolean;
  onClose: () => void;
  onSuccess?: () => void;  // 成功添加标签后的回调
  selectedFiles: FileType[];
}

export default function AddTagModal({ isOpen, onClose, onSuccess, selectedFiles }: AddTagModalProps) {
  const [selectedTags, setSelectedTags] = useState<number[]>([]);
  const [newTagName, setNewTagName] = useState('');
  const [isCreating, setIsCreating] = useState(false);
  const [searchTerm, setSearchTerm] = useState('');

  // 从 store 获取所有标签
  const allTags = useStore((s) => s.tags);
  const setAllTags = useStore((s) => s.setTags);

  // 加载所有标签
  useEffect(() => {
    const loadTags = async () => {
      try {
        const tags = await api.getAllTags();
        setAllTags(tags);
      } catch (error) {
        console.error('加载标签失败:', error);
      }
    };

    if (isOpen) {
      loadTags();
    }
  }, [isOpen]);

  const handleSelectTag = (tagId: number) => {
    if (selectedTags.includes(tagId)) {
      setSelectedTags(selectedTags.filter(id => id !== tagId));
    } else {
      setSelectedTags([...selectedTags, tagId]);
    }
  };

  const handleCreateTag = async () => {
    if (!newTagName.trim()) return;

    setIsCreating(true);
    try {
      const tagId = await api.createTag({
        name: newTagName,
        displayName: newTagName,
        color: '#007ACC',
      });

      // 选择新创建的标签
      setSelectedTags([...selectedTags, tagId]);

      // 重新加载标签列表
      const tags = await api.getAllTags();
      setAllTags(tags);

      setNewTagName('');
    } catch (error) {
      console.error('创建标签失败:', error);
    } finally {
      setIsCreating(false);
    }
  };

  const handleConfirm = async () => {
    if (selectedTags.length === 0) return;

    try {
      // 批量添加标签到文件
      await api.batchAddTags(selectedFiles.map(f => f.id!), selectedTags.map(id => {
        const tag = allTags.find(t => t.id === id);
        return tag?.name || '';
      }));

      // 刷新标签列表
      const tags = await api.getAllTags();
      setAllTags(tags);

      // 通知父组件成功添加标签，刷新文件列表
      onSuccess?.();

      // 关闭对话框
      onClose();
    } catch (error) {
      console.error('添加标签失败:', error);
    }
  };

  const filteredTags = searchTerm
    ? allTags.filter(tag =>
        tag.name.toLowerCase().includes(searchTerm.toLowerCase())
      )
    : allTags;

  return (
    <div
      className={`fixed inset-0 z-50 bg-black/50 flex items-center justify-center transition-opacity ${isOpen ? 'opacity-100' : 'opacity-0 pointer-events-none'}`}
      onClick={onClose}
    >
      <div
        className="bg-card dark:bg-gray-800 rounded-lg shadow-xl w-[600px] max-h-[80vh] overflow-y-auto"
        onClick={(e) => e.stopPropagation()}
      >
        {/* 头部 */}
        <div className="flex items-center justify-between p-6 border-b border-border">
          <h3 className="text-lg font-semibold">
            为 {selectedFiles.length} 个文件添加标签
          </h3>
          <button
            onClick={onClose}
            className="p-1 hover:bg-muted rounded-full transition-colors"
          >
            <X className="w-5 h-5 text-muted-foreground" />
          </button>
        </div>

        {/* 文件列表 */}
        <div className="px-6 py-4 max-h-40 overflow-y-auto border-b border-border">
          {selectedFiles.map((file, index) => (
            <div
              key={file.id || file.path}
              className="flex items-center gap-3 p-2 hover:bg-muted rounded-lg transition-colors"
            >
              <div className="text-sm text-muted-foreground truncate">
                {index + 1}. {file.name}
              </div>
            </div>
          ))}
        </div>

        {/* 已选标签 */}
        <div className="px-6 py-4">
          <h4 className="text-sm font-semibold mb-3 text-muted-foreground">
            已选标签 ({selectedTags.length})
          </h4>
          <div className="flex flex-wrap gap-2">
            {selectedTags
              .map(tagId => allTags.find(t => t.id === tagId))
              .filter((tag): tag is TagWithId => tag !== undefined && tag.id !== undefined)
              .map(tag => (
                <button
                  key={tag.id}
                  onClick={() => handleSelectTag(tag.id)}
                  className={`px-3 py-1.5 text-sm rounded-full transition-colors ${
                    selectedTags.includes(tag.id)
                      ? 'bg-primary text-primary-foreground'
                      : 'bg-muted text-muted-foreground hover:bg-muted/80'
                  }`}
                >
                  <span className="truncate max-w-32">
                    {tag.name}
                  </span>
                </button>
              ))}
          </div>
        </div>

        {/* 创建新标签 */}
        <div className="px-6 py-4 border-b border-border">
          <div className="flex items-center gap-2">
            <input
              type="text"
              placeholder="输入新标签名..."
              value={newTagName}
              onChange={(e) => setNewTagName(e.target.value)}
              onKeyDown={(e) => {
                if (e.key === 'Enter') {
                  e.preventDefault();
                  handleCreateTag();
                }
              }}
              disabled={isCreating}
              className="flex-1 bg-transparent outline-none px-3 py-1.5 rounded border border-border text-foreground placeholder:text-muted-foreground"
            />
            <button
              onClick={handleCreateTag}
              disabled={isCreating || !newTagName.trim()}
              className="p-2 bg-primary text-primary-foreground hover:bg-primary/90 rounded-lg disabled:opacity-50 transition-colors"
            >
              {isCreating ? (
                <span>创建中...</span>
              ) : (
                <div className="flex items-center gap-1">
                  <Plus size={16} />
                  创建
                </div>
              )}
            </button>
          </div>
        </div>

        {/* 标签搜索 */}
        <div className="px-6 py-4 border-b border-border">
          <input
            type="text"
            placeholder="搜索已有标签..."
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
            className="w-full bg-transparent outline-none px-3 py-1.5 rounded border border-border text-foreground placeholder:text-muted-foreground"
          />
        </div>

        {/* 标签列表 */}
        <div className="px-6 py-4 flex-1 overflow-y-auto">
          <h4 className="text-sm font-semibold mb-3 text-muted-foreground">
            选择标签
          </h4>
          {allTags.length === 0 ? (
            <p className="text-sm text-muted-foreground">
              暂无标签，请先创建
            </p>
          ) : (
            <div className="flex flex-wrap gap-2">
              {filteredTags
                .filter((tag): tag is TagWithId => tag.id !== undefined)
                .map(tag => (
                  <button
                    key={tag.id}
                    onClick={() => handleSelectTag(tag.id)}
                    className={`px-3 py-1.5 text-sm rounded-full transition-colors ${
                      selectedTags.includes(tag.id)
                        ? 'bg-primary text-primary-foreground'
                        : 'bg-muted text-muted-foreground hover:bg-muted/80'
                    }`}
                  >
                    <span className="truncate max-w-32">
                      {tag.name}
                    </span>
                  </button>
                ))}
            </div>
          )}
        </div>

        {/* 底部按钮 */}
        <div className="px-6 py-4 flex justify-end gap-3">
          <button
            onClick={onClose}
            className="px-4 py-2 bg-secondary text-secondary-foreground hover:bg-secondary/80 rounded-lg transition-colors"
          >
            取消
          </button>
          <button
            onClick={handleConfirm}
            disabled={selectedTags.length === 0 || isCreating}
            className="px-4 py-2 bg-primary text-primary-foreground hover:bg-primary/90 rounded-lg disabled:opacity-50 transition-colors"
          >
            {isCreating ? (
              <span>添加中...</span>
            ) : (
              <div className="flex items-center gap-1">
                <Check size={16} />
                确定
              </div>
            )}
          </button>
        </div>
      </div>
    </div>
  );
}
