/**
 * 搜索栏组件 - 支持多关键字标签
 */
import { useState } from 'react';
import { Search, X, XCircle, Check } from 'lucide-react';
import { useStore } from '../../stores/useStore';

export default function SearchBar() {
  const [inputValue, setInputValue] = useState('');
  const setSearchKeywords = useStore((s) => s.setSearchKeywords);
  const searchOperator = useStore((s) => s.searchOperator);
  const setSearchOperator = useStore((s) => s.setSearchOperator);
  const setIsLoading = useStore((s) => s.setIsLoading);

  // 从store获取当前搜索的关键字（用于需求2：搜索后保留关键字）
  const currentKeywords = useStore((s) => s.searchKeywords);

  const handleSearch = () => {
    if (currentKeywords.length === 0) return;

    setIsLoading(true);
    // 搜索逻辑在主应用中处理
  };

  const handleClearAll = () => {
    setSearchKeywords([]);
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      // 回车将输入的关键字添加为标签
      if (inputValue.trim()) {
        addKeyword(inputValue.trim());
        // 如果已经是多个标签，不自动搜索，用户需点击搜索按钮
        // 只有一个标签时自动搜索，保持原有体验
        if (currentKeywords.length === 0) {
          handleSearch();
        }
      } else {
        // 如果输入框为空且有标签，直接搜索
        if (currentKeywords.length > 0) {
          handleSearch();
        }
      }
    } else if (e.key === 'Backspace') {
      // 退格键删除最后一个标签
      if (!inputValue && currentKeywords.length > 0) {
        removeKeyword(currentKeywords.length - 1);
      }
    }
  };

  const addKeyword = (keyword: string) => {
    if (keyword && !currentKeywords.includes(keyword)) {
      setSearchKeywords([...currentKeywords, keyword]);
      setInputValue('');
    }
  };

  const removeKeyword = (index: number) => {
    const newKeywords = currentKeywords.filter((_, i) => i !== index);
    setSearchKeywords(newKeywords);
  };

  return (
    <div className="flex flex-col gap-3 bg-card border border-border rounded-lg p-3">
      {/* 第一行：搜索icon | 搜索框 | AND按钮 */}
      <div className="flex items-center gap-2">
        {/* 左侧：搜索icon，无底色，放在搜索框外 */}
        <div className="shrink-0">
          <Search className="w-5 h-5 text-muted-foreground" />
        </div>

        {/* 中间：搜索框，尽量保持宽度 */}
        <div className="flex-1">
          <input
            type="text"
            placeholder={currentKeywords.length > 0 ? "继续输入关键字..." : "搜索文件..."}
            value={inputValue}
            onChange={(e) => setInputValue(e.target.value)}
            onKeyDown={handleKeyDown}
            className="w-full bg-transparent outline-none text-foreground placeholder:text-muted-foreground"
          />
        </div>

        {/* 右侧：AND按钮 */}
        <button
          onClick={() => setSearchOperator(searchOperator === 'AND' ? 'OR' : 'AND')}
          className="px-3 py-1.5 text-sm rounded bg-secondary text-secondary-foreground hover:bg-secondary/80 transition-colors shrink-0"
        >
          {searchOperator}
        </button>
      </div>

      {/* 第二行：关键字标签，仅当放不下时换行 */}
      {currentKeywords.length > 0 && (
        <div className="flex flex-wrap items-center gap-2 mt-2">
          {currentKeywords.map((keyword, index) => (
            <div
              key={index}
              className="flex items-center gap-1 px-3 py-1.5 bg-primary/10 text-primary rounded-md border border-primary/30 shrink-0"
            >
              <span className="text-sm">{keyword}</span>
              <button
                onClick={() => removeKeyword(index)}
                className="p-0.5 hover:bg-primary/20 rounded-full transition-colors shrink-0"
                title="删除此关键字"
              >
                <XCircle className="w-4 h-4 text-primary/70 hover:text-primary" />
              </button>
            </div>
          ))}
          {/* 清除全部按钮 */}
          <button
            onClick={handleClearAll}
            className="p-1.5 hover:bg-muted rounded transition-colors"
            title="清除全部"
          >
            <X className="w-4 h-4 text-muted-foreground" />
          </button>
        </div>
      )}

      {/* 第三行：确定和搜索按钮 */}
      {(currentKeywords.length > 0 || inputValue) && (
        <div className="flex items-center justify-end gap-2 mt-2">
          {inputValue && (
            <button
              onClick={() => addKeyword(inputValue.trim())}
              className="flex items-center gap-1.5 px-3 py-1.5 text-sm rounded bg-primary text-primary-foreground hover:bg-primary/90 transition-colors"
            >
              <Check className="w-4 h-4" />
              确定
            </button>
          )}
          <button
            onClick={handleSearch}
            className="px-3 py-1.5 text-sm rounded bg-primary text-primary-foreground hover:bg-primary/90 transition-colors"
          >
            搜索
          </button>
        </div>
      )}
    </div>
  );
}
