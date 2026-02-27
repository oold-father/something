/**
 * 标签面板组件
 */
import { useEffect } from 'react';
import { Tag, X } from 'lucide-react';
import { api } from '../../lib/api';
import { useStore } from '../../stores/useStore';

export default function TagPanel() {
  const tags = useStore((s) => s.tags);
  const setTags = useStore((s) => s.setTags);
  const selectedTags = useStore((s) => s.selectedTags);
  const toggleTag = useStore((s) => s.toggleTag);

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

  // 按类型分组标签
  const systemTags = tags.filter((t) => t.tagType === 'system');
  const customTags = tags.filter((t) => t.tagType === 'custom');

  return (
    <div className="flex flex-col h-full bg-card border-r border-border">
      <div className="p-4 border-b border-border">
        <h2 className="font-semibold text-lg">标签</h2>
        <p className="text-sm text-muted-foreground">点击标签进行筛选</p>
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
    </div>
  );
}

function TagButton({ tag, isSelected, onClick }: { tag: any; isSelected: boolean; onClick: () => void }) {
  return (
    <button
      onClick={onClick}
      className={`
        flex items-center gap-1 px-3 py-1.5 rounded-full text-sm transition-colors
        ${isSelected ? 'bg-primary text-primary-foreground' : 'bg-secondary text-secondary-foreground hover:bg-secondary/80'}
      `}
      style={isSelected ? { backgroundColor: tag.color } : undefined}
    >
      <Tag className="w-3 h-3" />
      {tag.displayName || tag.name}
      {tag.useCount > 0 && (
        <span className="ml-1 text-xs opacity-70">({tag.useCount})</span>
      )}
    </button>
  );
}
